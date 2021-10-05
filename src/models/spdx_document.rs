// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use log::info;
use petgraph::graphmap::DiGraphMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graph::{create_graph, path_with_relationships};
use crate::{error::SpdxError, graph::find_path};

use super::{
    Algorithm, Annotation, DocumentCreationInformation, FileInformation,
    OtherLicensingInformationDetected, PackageInformation, Relationship, RelationshipType, Snippet,
};

/// # SPDX 2.2
///
/// Store information about files in SPDX files. Latest spec
/// is currently 2.2. Can be serialized to JSON.  
///     
/// Spec: <https://spdx.github.io/spdx-spec/>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SPDX {
    /// <https://spdx.github.io/spdx-spec/2-document-creation-information/>
    #[serde(flatten)]
    pub document_creation_information: DocumentCreationInformation,

    /// <https://spdx.github.io/spdx-spec/3-package-information/>
    #[serde(rename = "packages")]
    #[serde(default)]
    pub package_information: Vec<PackageInformation>,

    /// <https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/>
    #[serde(rename = "hasExtractedLicensingInfos")]
    #[serde(default)]
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,

    /// <https://spdx.github.io/spdx-spec/4-file-information/>
    #[serde(rename = "files")]
    #[serde(default)]
    pub file_information: Vec<FileInformation>,

    /// <https://spdx.github.io/spdx-spec/5-snippet-information/>
    #[serde(rename = "snippets")]
    #[serde(default)]
    pub snippet_information: Vec<Snippet>,

    /// <https://spdx.github.io/spdx-spec/7-relationships-between-SPDX-elements/>
    #[serde(default)]
    pub relationships: Vec<Relationship>,

    /// <https://spdx.github.io/spdx-spec/8-annotations/>
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
                ..DocumentCreationInformation::default()
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

    /// Get unique hashes for all files the SPDX.
    pub fn get_unique_hashes(&self, algorithm: Algorithm) -> HashSet<String> {
        info!("Getting unique hashes for files in SPDX.");

        let mut unique_hashes: HashSet<String> = HashSet::new();

        for file_information in &self.file_information {
            if let Some(checksum) = file_information.checksum(algorithm) {
                unique_hashes.insert(checksum.to_string());
            }
        }

        unique_hashes
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
                result.push((file, relationship));
            };
        }

        result
    }

    /// Get all license identifiers from the SPDX.
    ///
    /// # Errors
    ///
    /// Returns [`SpdxError`] if parsing of the expressions fails.
    pub fn get_license_ids(&self) -> Result<Vec<String>, SpdxError> {
        info!("Getting all license identifiers from SPDX.");

        let mut license_ids = Vec::new();

        for file in &self.file_information {
            for license in &file.concluded_license.licenses() {
                if !license_ids.contains(license) && license != "NOASSERTION" && license != "NONE" {
                    license_ids.push(license.clone());
                }
            }
        }

        Ok(license_ids)
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
        path.map(|path| {
            path_with_relationships(&graph, path.1)
                .iter()
                .map(|part| (*part).to_string())
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use crate::models::SPDXExpression;

    use super::*;

    #[test]
    fn deserialize_simple_spdx() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();

        assert_eq!(
            spdx_file.document_creation_information.document_name,
            "SPDX-Tools-v2.0".to_string()
        );
    }

    #[test]
    fn find_related_files_for_package() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();

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
            SPDXExpression::parse("LicenseRef-1").unwrap()
        );
    }

    #[test]
    fn get_all_licenses_from_spdx() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();

        let mut actual = spdx_file.get_license_ids().unwrap();
        actual.sort();

        let mut expected: Vec<String> = vec![
            "Apache-2.0".into(),
            "LicenseRef-1".into(),
            "LGPL-2.0".into(),
            "LicenseRef-2".into(),
        ];
        expected.sort();

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_relationships_for_spdx_id() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();

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

        assert_eq!(relationships, expected_relationships);
    }

    #[test]
    fn get_relationships_for_related_spdx_id() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();

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

        assert_eq!(relationships, expected_relationships);
    }

    #[test]
    fn find_path_between_ids_works() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();
        let path = spdx_file
            .find_path_between_spdx_ids("SPDXRef-DOCUMENT", "SPDXRef-Saxon")
            .unwrap();
        dbg!(path);
    }

    #[test]
    fn get_unique_hashes_for_files() {
        let spdx_file: SPDX = serde_json::from_str(
            &read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap(),
        )
        .unwrap();
        let hashes = spdx_file.get_unique_hashes(Algorithm::SHA1);

        let expected = [
            "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12".to_string(),
            "c2b4e1c67a2d28fced849ee1bb76e7391b93f125".to_string(),
            "3ab4e1c67a2d28fced849ee1bb76e7391b93f125".to_string(),
            "d6a770ba38583ed4bb4525bd96e50461655d2758".to_string(),
        ]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

        assert_eq!(hashes, expected);
    }
}
