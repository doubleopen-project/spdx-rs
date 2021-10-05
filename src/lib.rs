// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

pub mod algorithm;
pub mod annotation;
pub mod checksum;
pub mod creation_info;
pub mod document_creation_information;
pub mod error;
pub mod external_document_reference;
pub mod external_package_reference;
pub mod file_information;
pub mod file_type;
pub mod graph;
pub mod license_list;
pub mod other_licensing_information_detected;
pub mod package_information;
pub mod package_verification_code;
pub mod relationship;
pub mod snippet;
pub mod spdx_expression;
pub use algorithm::*;
pub use annotation::*;
pub use checksum::*;
pub use creation_info::*;
pub use document_creation_information::*;
use error::SpdxError;
pub use external_document_reference::*;
pub use external_package_reference::*;
pub use file_information::*;
pub use file_type::*;
use graph::{create_graph, find_path, path_with_relationships};
use log::info;
pub use other_licensing_information_detected::*;
pub use package_information::*;
pub use package_verification_code::*;
use petgraph::graphmap::DiGraphMap;
pub use relationship::*;
use serde::{Deserialize, Serialize};
pub use snippet::*;
pub use spdx_expression::*;
use std::{fs, io::BufReader, path::Path};
use uuid::Uuid;

use self::Relationship;

/// # SPDX 2.2
///
/// Store information about files in SPDX files. Latest spec
/// is currently 2.2. Can be serialized to JSON.  
///     
/// Spec: https://spdx.github.io/spdx-spec/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SPDX {
    /// https://spdx.github.io/spdx-spec/2-document-creation-information/
    #[serde(flatten)]
    pub document_creation_information: DocumentCreationInformation,

    /// https://spdx.github.io/spdx-spec/3-package-information/
    #[serde(rename = "packages")]
    #[serde(default)]
    pub package_information: Vec<PackageInformation>,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/
    #[serde(rename = "hasExtractedLicensingInfos")]
    #[serde(default)]
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,

    /// https://spdx.github.io/spdx-spec/4-file-information/
    #[serde(rename = "files")]
    #[serde(default)]
    pub file_information: Vec<FileInformation>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/
    #[serde(rename = "snippets")]
    #[serde(default)]
    pub snippet_information: Vec<Snippet>,

    /// https://spdx.github.io/spdx-spec/7-relationships-between-SPDX-elements/
    #[serde(default)]
    pub relationships: Vec<Relationship>,

    /// https://spdx.github.io/spdx-spec/8-annotations/
    #[serde(default)]
    pub annotations: Vec<Annotation>,

    /// Counter for creating SPDXRefs. Is not part of the spec, so don't serialize.
    #[serde(skip)]
    pub spdx_ref_counter: i32,
}

impl SPDX {
    /// Create new SPDX struct.
    pub fn new(name: &str) -> Self {
        info!("Creating SPDX.");

        Self {
            document_creation_information: DocumentCreationInformation {
                document_name: name.to_string(),
                spdx_document_namespace: format!(
                    "http://spdx.org/spdxdocs/{}-{}",
                    name.to_string(),
                    Uuid::new_v4()
                ),
                ..Default::default()
            },
            package_information: Vec::new(),
            other_licensing_information_detected: Vec::new(),
            file_information: Vec::new(),
            relationships: Vec::new(),
            spdx_ref_counter: 0,
            annotations: Vec::new(),
            snippet_information: Vec::new(),
        }
    }

    /// Deserialize from file. Accepts json and yaml.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SpdxError> {
        info!("Deserializing SPDX from {}", path.as_ref().display());

        let path = path.as_ref();
        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);

        match path
            .extension()
            .ok_or_else(|| SpdxError::PathExtension(path.to_string_lossy().to_string()))?
            .to_str()
        {
            Some("yml") => Ok(serde_yaml::from_reader::<_, SPDX>(reader)?),
            Some("json") => Ok(serde_json::from_reader::<_, SPDX>(reader)?),
            None | Some(_) => Err(SpdxError::PathExtension(path.to_string_lossy().to_string())),
        }
    }

    /// Get unique hashes for all files in all packages of the SPDX.
    pub fn get_unique_hashes(&self, algorithm: Algorithm) -> Vec<String> {
        info!("Getting unique hashes for files in SPDX.");

        let mut unique_hashes: Vec<String> = Vec::new();

        for file_information in self.file_information.iter() {
            if let Some(checksum) = file_information
                .file_checksum
                .iter()
                .find(|checksum| checksum.algorithm == algorithm)
            {
                unique_hashes.push(checksum.value.clone());
            }
        }

        unique_hashes.sort();
        unique_hashes.dedup();

        unique_hashes
    }

    /// Save serialized SPDX as json,
    pub fn save_as_json<P: AsRef<Path>>(&self, path: P) -> Result<(), SpdxError> {
        println!("Saving to json...");

        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;

        Ok(())
    }

    /// Find related files of the package with the provided id.
    pub fn get_files_for_package(
        &self,
        package_spdx_id: &str,
    ) -> Vec<(&FileInformation, &Relationship)> {
        info!("Finding related files for package {}.", &package_spdx_id);

        let relationships = self
            .relationships
            .iter()
            .filter(|relationship| relationship.spdx_element_id == package_spdx_id);

        let mut result: Vec<(&FileInformation, &Relationship)> = Vec::new();

        for relationship in relationships {
            let file = self
                .file_information
                .iter()
                .find(|file| file.file_spdx_identifier == relationship.related_spdx_element);
            if let Some(file) = file {
                result.push((&file, &relationship));
            };
        }

        result
    }

    /// Get all license identifiers from the SPDX.
    pub fn get_license_ids(&self) -> Vec<String> {
        info!("Getting all license identifiers from SPDX.");

        let mut license_ids = Vec::new();

        for file in &self.file_information {
            for license in &file.concluded_license.licenses() {
                if !license_ids.contains(license) && license != "NOASSERTION" && license != "NONE" {
                    license_ids.push(license.clone());
                }
            }
        }

        license_ids
    }

    /// Get all relationships where the given SPDX ID is the SPDX element id.
    pub fn relationships_for_spdx_id(&self, spdx_id: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|relationship| relationship.spdx_element_id == spdx_id)
            .collect()
    }

    /// Get all relationships where the given SPDX ID is the related SPDX element id.
    pub fn relationships_for_related_spdx_id(&self, spdx_id: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|relationship| relationship.related_spdx_element == spdx_id)
            .collect()
    }

    /// Create a graph of the relationships in the SPDX Document.
    pub fn graph(&self) -> DiGraphMap<&str, &RelationshipType> {
        create_graph(self)
    }

    /// Find a path between two SPDX IDs.
    pub fn find_path_between_spdx_ids(&self, from: &str, to: &str) -> Option<Vec<String>> {
        let graph = self.graph();
        let path = find_path(&graph, from, to);
        if let Some(path) = path {
            Some(
                path_with_relationships(&graph, path.1)
                    .iter()
                    .map(|part| part.to_string())
                    .collect(),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use chrono::prelude::*;

    use super::*;

    #[test]
    fn deserialize_simple_spdx() {
        let spdx_file = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        assert_eq!(
            spdx_file.document_creation_information.document_name,
            "SPDX-Tools-v2.0".to_string()
        );
    }

    mod correct_information_is_parsed_from_example_spdx {
        use super::*;

        mod document_creation_information {
            use super::*;
            #[test]
            fn spdx_version() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information.spdx_version,
                    "SPDX-2.2".to_string()
                );
            }
            #[test]
            fn data_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.document_creation_information.data_license, "CC0-1.0");
            }
            #[test]
            fn spdx_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information.spdx_identifier,
                    "SPDXRef-DOCUMENT".to_string()
                );
            }
            #[test]
            fn document_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information.document_name,
                    "SPDX-Tools-v2.0".to_string()
                );
            }
            #[test]
            fn spdx_document_namespace() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information.spdx_document_namespace,
                    "http://spdx.org/spdxdocs/spdx-example-444504E0-4F89-41D3-9A0C-0305E82C3301"
                        .to_string()
                );
            }
            #[test]
            fn external_document_references() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(spdx
            .document_creation_information
            .external_document_references
            .contains(&ExternalDocumentReference {
                id_string: "DocumentRef-spdx-tool-1.2".to_string(),
                checksum: Checksum {
                    algorithm: Algorithm::SHA1,
                    value: "d6a770ba38583ed4bb4525bd96e50461655d2759".to_string()
                },
                spdx_document_uri:
                    "http://spdx.org/spdxdocs/spdx-tools-v1.2-3F2504E0-4F89-41D3-9A0C-0305E82C3301"
                        .to_string()
            }));
            }
            #[test]
            fn license_list_version() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information
                        .creation_info
                        .license_list_version,
                    Some("3.9".to_string())
                );
            }
            #[test]
            fn creators() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(spdx
                    .document_creation_information
                    .creation_info
                    .creators
                    .contains(&"Tool: LicenseFind-1.0".to_string()));
                assert!(spdx
                    .document_creation_information
                    .creation_info
                    .creators
                    .contains(&"Organization: ExampleCodeInspect ()".to_string()));
                assert!(spdx
                    .document_creation_information
                    .creation_info
                    .creators
                    .contains(&"Person: Jane Doe ()".to_string()));
            }
            #[test]
            fn created() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information.creation_info.created,
                    Utc.ymd(2010, 1, 29).and_hms(18, 30, 22)
                );
            }
            #[test]
            fn creator_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.document_creation_information
                        .creation_info
                        .creator_comment,
                    Some(
                        r#"This package has been shipped in source and binary form.
The binaries were created with gcc 4.5.1 and expect to link to
compatible system run time libraries."#
                            .to_string()
                    )
                );
            }
            #[test]
            fn document_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                spdx.document_creation_information.document_comment,
                Some(
                    "This document was created using SPDX 2.0 using licenses from the web site."
                        .to_string()
                )
            );
            }
        }

        mod package_information {
            use super::*;

            #[test]
            fn all_packages_are_deserialized() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.package_information.len(), 4);
            }
            #[test]
            fn package_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_name,
                    "glibc".to_string()
                );
            }
            #[test]
            fn package_spdx_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_spdx_identifier,
                    "SPDXRef-Package".to_string()
                );
            }
            #[test]
            fn package_version() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_version,
                    Some("2.11.1".to_string())
                );
            }
            #[test]
            fn package_file_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_file_name,
                    Some("glibc-2.11.1.tar.gz".to_string())
                );
            }
            #[test]
            fn package_supplier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_supplier,
                    Some("Person: Jane Doe (jane.doe@example.com)".to_string())
                );
            }
            #[test]
            fn package_originator() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_originator,
                    Some("Organization: ExampleCodeInspect (contact@example.com)".to_string())
                );
            }
            #[test]
            fn package_download_location() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_download_location,
                    "http://ftp.gnu.org/gnu/glibc/glibc-ports-2.15.tar.gz".to_string()
                );
            }
            #[test]
            fn files_analyzed() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.package_information[0].files_analyzed, Some(true));
            }
            #[test]
            fn package_verification_code() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_verification_code,
                    Some(PackageVerificationCode {
                        value: "d6a770ba38583ed4bb4525bd96e50461655d2758".to_string(),
                        excludes: vec!["./package.spdx".to_string()]
                    })
                );
            }
            #[test]
            fn package_chekcsum() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(spdx.package_information[0]
                    .package_checksum
                    .contains(&Checksum::new(
                        Algorithm::SHA1,
                        "85ed0817af83a24ad8da68c2b5094de69833983c"
                    )));
                assert!(spdx.package_information[0]
                    .package_checksum
                    .contains(&Checksum::new(
                        Algorithm::MD5,
                        "624c1abb3664f4b35547e7c73864ad24"
                    )));
                assert!(spdx.package_information[0]
                    .package_checksum
                    .contains(&Checksum::new(
                        Algorithm::SHA256,
                        "11b6d3ee554eedf79299905a98f9b9a04e498210b59f15094c916c91d150efcd"
                    )));
            }
            #[test]
            fn package_home_page() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_home_page,
                    Some("http://ftp.gnu.org/gnu/glibc".to_string())
                );
            }
            #[test]
            fn source_information() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].source_information,
                    Some(
                        "uses glibc-2_11-branch from git://sourceware.org/git/glibc.git."
                            .to_string()
                    )
                );
            }
            #[test]
            fn concluded_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].concluded_license,
                    SPDXExpression("(LGPL-2.0-only OR LicenseRef-3)".to_string())
                );
            }
            #[test]
            fn all_licenses_information_from_files() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(spdx.package_information[0]
                    .all_licenses_information_from_files
                    .contains(&"GPL-2.0-only".to_string()));
                assert!(spdx.package_information[0]
                    .all_licenses_information_from_files
                    .contains(&"LicenseRef-2".to_string()));
                assert!(spdx.package_information[0]
                    .all_licenses_information_from_files
                    .contains(&"LicenseRef-1".to_string()));
            }
            #[test]
            fn declared_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].declared_license,
                    SPDXExpression("(LGPL-2.0-only AND LicenseRef-3)".to_string())
                );
            }
            #[test]
            fn comments_on_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].comments_on_license,
                    Some("The license for this project changed with the release of version x.y.  The version of the project included here post-dates the license change.".to_string())
                );
            }
            #[test]
            fn copyright_text() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].copyright_text,
                    "Copyright 2008-2010 John Smith".to_string()
                );
            }
            #[test]
            fn package_summary_description() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_summary_description,
                    Some("GNU C library.".to_string())
                );
            }
            #[test]
            fn package_detailed_description() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[0].package_detailed_description,
                    Some("The GNU C Library defines functions that are specified by the ISO C standard, as well as additional features specific to POSIX and other derivatives of the Unix operating system, and extensions specific to GNU systems.".to_string())
                );
            }
            #[test]
            fn package_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.package_information[1].package_comment,
                    Some(
                        "This package was converted from a DOAP Project by the same name"
                            .to_string()
                    )
                );
            }
            #[test]
            fn external_reference() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(
                    spdx.package_information[0].external_reference.contains(&ExternalPackageReference {
                        reference_comment: Some("This is the external ref for Acme".to_string()),
                        reference_category: ExternalPackageReferenceCategory::Other,
                        reference_locator: "acmecorp/acmenator/4.1.3-alpha".to_string(),
                        reference_type: "http://spdx.org/spdxdocs/spdx-example-444504E0-4F89-41D3-9A0C-0305E82C3301#LocationRef-acmeforge".to_string()
                    })
                );
                assert!(spdx.package_information[0].external_reference.contains(
                    &ExternalPackageReference {
                        reference_comment: None,
                        reference_category: ExternalPackageReferenceCategory::Security,
                        reference_locator:
                            "cpe:2.3:a:pivotal_software:spring_framework:4.1.0:*:*:*:*:*:*:*"
                                .to_string(),
                        reference_type: "http://spdx.org/rdf/references/cpe23Type".to_string()
                    }
                ));
            }
            #[test]
            fn package_attribution_text() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert!(
                    spdx.package_information[0].package_attribution_text.contains(&"The GNU C Library is free software.  See the file COPYING.LIB for copying conditions, and LICENSES for notices about a few contributions that require these additional notices to be distributed.  License copyright years may be listed using range notation, e.g., 1996-2015, indicating that every year in the range, inclusive, is a copyrightable year that would otherwise be listed individually.".to_string())
                );
            }
        }

        mod file_information {
            use super::*;

            #[test]
            fn file_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].file_name,
                    "./src/org/spdx/parser/DOAPProject.java"
                );
            }
            #[test]
            fn file_spdx_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].file_spdx_identifier,
                    "SPDXRef-DoapSource"
                );
            }
            #[test]
            fn file_type() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.file_information[0].file_type, vec![FileType::Source]);
            }
            #[test]
            fn file_checksum() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].file_checksum,
                    vec![Checksum {
                        algorithm: Algorithm::SHA1,
                        value: "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12".to_string()
                    }]
                );
            }
            #[test]
            fn concluded_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].concluded_license,
                    SPDXExpression("Apache-2.0".to_string())
                );
            }
            #[test]
            fn license_information_in_file() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].license_information_in_file,
                    vec!["Apache-2.0".to_string()]
                );
            }
            #[test]
            fn comments_on_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[2].comments_on_license,
                    Some("This license is used by Jena".to_string())
                );
            }
            #[test]
            fn copyright_text() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[0].copyright_text,
                    "Copyright 2010, 2011 Source Auditor Inc.".to_string()
                );
            }
            #[test]
            fn file_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[1].file_comment,
                    Some("This file is used by Jena".to_string())
                );
            }
            #[test]
            fn file_notice() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[1].file_notice,
                    Some("Apache Commons Lang\nCopyright 2001-2011 The Apache Software Foundation\n\nThis product includes software developed by\nThe Apache Software Foundation (http://www.apache.org/).\n\nThis product includes software from the Spring Framework,\nunder the Apache License 2.0 (see: StringUtils.containsWhitespace())".to_string())
                );
            }
            #[test]
            fn file_contributor() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.file_information[1].file_contributor,
                    vec!["Apache Software Foundation".to_string()]
                );
            }
        }

        mod snippet_information {
            use super::*;

            #[test]
            fn snippet_spdx_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_spdx_identifier,
                    "SPDXRef-Snippet".to_string()
                );
            }
            #[test]
            fn snippet_from_file_spdx_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_from_file_spdx_identifier,
                    "SPDXRef-DoapSource".to_string()
                );
            }
            #[test]
            fn ranges() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].ranges,
                    vec![
                        Range {
                            end_pointer: EndPointer {
                                line_number: Some(23),
                                reference: Some("SPDXRef-DoapSource".to_string()),
                                offset: None
                            },
                            start_pointer: StartPointer {
                                line_number: Some(5),
                                reference: Some("SPDXRef-DoapSource".to_string()),
                                offset: None
                            }
                        },
                        Range {
                            end_pointer: EndPointer {
                                line_number: None,
                                reference: Some("SPDXRef-DoapSource".to_string()),
                                offset: Some(420)
                            },
                            start_pointer: StartPointer {
                                line_number: None,
                                reference: Some("SPDXRef-DoapSource".to_string()),
                                offset: Some(310)
                            }
                        },
                    ]
                );
            }
            #[test]
            fn snippet_concluded_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_concluded_license,
                    SPDXExpression("GPL-2.0-only".to_string())
                );
            }
            #[test]
            fn license_information_in_snippet() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].license_information_in_snippet,
                    vec!["GPL-2.0-only".to_string()]
                );
            }
            #[test]
            fn snippet_comments_on_license() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_comments_on_license,
                    Some("The concluded license was taken from package xyz, from which the snippet was copied into the current file. The concluded license information was found in the COPYING.txt file in package xyz.".to_string())
                );
            }
            #[test]
            fn snippet_copyright_text() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_copyright_text,
                    "Copyright 2008-2010 John Smith".to_string()
                );
            }
            #[test]
            fn snippet_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_comment,
                    Some("This snippet was identified as significant and highlighted in this Apache-2.0 file, when a commercial scanner identified it as being derived from file foo.c in package xyz which is licensed under GPL-2.0.".to_string())
                );
            }
            #[test]
            fn snippet_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.snippet_information[0].snippet_name,
                    Some("from linux kernel".to_string())
                );
            }
        }

        mod other_licensing_information_detected {
            use super::*;

            #[test]
            fn license_identifier() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.other_licensing_information_detected[0].license_identifier,
                    "LicenseRef-Beerware-4.2".to_string()
                )
            }
            #[test]
            fn extracted_text() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.other_licensing_information_detected[0].extracted_text, "\"THE BEER-WARE LICENSE\" (Revision 42):\nphk@FreeBSD.ORG wrote this file. As long as you retain this notice you\ncan do whatever you want with this stuff. If we meet some day, and you think this stuff is worth it, you can buy me a beer in return Poul-Henning Kamp  </\nLicenseName: Beer-Ware License (Version 42)\nLicenseCrossReference:  http://people.freebsd.org/~phk/\nLicenseComment: \nThe beerware license has a couple of other standard variants.")
            }
            #[test]
            fn license_name() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.other_licensing_information_detected[2].license_name,
                    "CyberNeko License".to_string()
                )
            }
            #[test]
            fn license_cross_reference() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.other_licensing_information_detected[2].license_cross_reference,
                    vec![
                        "http://people.apache.org/~andyc/neko/LICENSE".to_string(),
                        "http://justasample.url.com".to_string()
                    ]
                )
            }
            #[test]
            fn license_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.other_licensing_information_detected[2].license_comment,
                    Some("This is tye CyperNeko License".to_string())
                )
            }
        }

        mod relationships_between_spdx_elements {
            use super::*;

            #[test]
            fn spdx_element_id() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.relationships[0].spdx_element_id,
                    "SPDXRef-DOCUMENT".to_string()
                );
            }
            #[test]
            fn related_spdx_element() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.relationships[0].related_spdx_element,
                    "SPDXRef-Package".to_string()
                );
            }
            #[test]
            fn relationship_type() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.relationships[0].relationship_type,
                    RelationshipType::Contains
                );
                assert_eq!(
                    spdx.relationships[2].relationship_type,
                    RelationshipType::CopyOf
                );
            }
        }

        mod annotations {
            use super::*;

            #[test]
            fn annotator() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.annotations[0].annotator,
                    "Person: Jane Doe ()".to_string()
                );
            }

            #[test]
            fn annotation_date() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.annotations[0].annotation_date,
                    Utc.ymd(2010, 1, 29).and_hms(18, 30, 22)
                );
            }

            #[test]
            fn annotation_type() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(spdx.annotations[0].annotation_type, AnnotationType::Other);
            }

            #[test]
            fn annotation_comment() {
                let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
                assert_eq!(
                    spdx.annotations[0].annotation_comment,
                    "Document level annotation"
                );
            }
        }
    }

    #[test]
    fn find_related_files_for_package() {
        let spdx_file = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();

        let package_1_files = spdx_file.get_files_for_package("SPDXRef-Package");

        assert_eq!(package_1_files.len(), 1);

        let file = package_1_files
            .iter()
            .find(|package_and_relationship| {
                package_and_relationship.0.file_name == *"./lib-source/jena-2.6.3-sources.jar"
            })
            .expect("Should always be found");

        assert_eq!(file.0.file_spdx_identifier, "SPDXRef-JenaLib");
        assert_eq!(file.1.relationship_type, RelationshipType::Contains);

        assert_eq!(
            file.0.concluded_license,
            SPDXExpression("LicenseRef-1".into())
        );
    }

    #[test]
    fn get_all_licenses_from_spdx() {
        let spdx_file = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();

        let mut actual = spdx_file.get_license_ids();
        actual.sort();

        let mut expected: Vec<String> = vec![
            "Apache-2.0".into(),
            "LicenseRef-1".into(),
            "LGPL-2.0-only".into(),
            "LicenseRef-2".into(),
        ];
        expected.sort();

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_relationships_for_spdx_id() {
        let spdx_file = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();

        let relationships = spdx_file.relationships_for_spdx_id("SPDXRef-Package");
        let relationship_1 = Relationship {
            spdx_element_id: "SPDXRef-Package".into(),
            related_spdx_element: "SPDXRef-Saxon".into(),
            relationship_type: RelationshipType::DynamicLink,
            comment: None,
        };
        let relationship_2 = Relationship {
            spdx_element_id: "SPDXRef-Package".into(),
            related_spdx_element: "SPDXRef-JenaLib".into(),
            relationship_type: RelationshipType::Contains,
            comment: None,
        };
        let expected_relationships = vec![&relationship_1, &relationship_2];

        assert_eq!(relationships, expected_relationships)
    }

    #[test]
    fn get_relationships_for_related_spdx_id() {
        let spdx_file = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();

        let relationships = spdx_file.relationships_for_related_spdx_id("SPDXRef-Package");
        let relationship_1 = Relationship {
            spdx_element_id: "SPDXRef-DOCUMENT".into(),
            related_spdx_element: "SPDXRef-Package".into(),
            relationship_type: RelationshipType::Contains,
            comment: None,
        };
        let relationship_2 = Relationship {
            spdx_element_id: "SPDXRef-DOCUMENT".into(),
            related_spdx_element: "SPDXRef-Package".into(),
            relationship_type: RelationshipType::Describes,
            comment: None,
        };
        let relationship_3 = Relationship {
            spdx_element_id: "SPDXRef-JenaLib".into(),
            related_spdx_element: "SPDXRef-Package".into(),
            relationship_type: RelationshipType::Contains,
            comment: None,
        };
        let expected_relationships = vec![&relationship_1, &relationship_2, &relationship_3];

        assert_eq!(relationships, expected_relationships)
    }

    #[test]
    fn find_path_between_ids_works() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        let path = spdx
            .find_path_between_spdx_ids("SPDXRef-DOCUMENT", "SPDXRef-Saxon")
            .unwrap();
        dbg!(path);
    }
}
