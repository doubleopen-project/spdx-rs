use chrono::{DateTime, Utc};

use crate::{
    error::SpdxError,
    models::{
        DocumentCreationInformation, ExternalPackageReference, PackageInformation, SPDXExpression,
        SPDX,
    },
    parsers::tag_value::{atoms, Atom},
};

mod tag_value;

/// # Errors
///
/// - If parsing of the tag-value fails.
/// - If parsing of some of the values fail.
pub fn spdx_from_tag_value(input: &str) -> Result<SPDX, SpdxError> {
    let (_, atoms) = atoms(input).map_err(|err| SpdxError::TagValueParse(err.to_string()))?;

    let mut spdx = SPDX::new("SPDX from TV");
    spdx.document_creation_information = document_creation_information_from_atoms(&atoms)?;
    spdx.package_information = packages_from_atoms(&atoms)?;

    Ok(spdx)
}

fn document_creation_information_from_atoms(
    atoms: &[Atom],
) -> Result<DocumentCreationInformation, SpdxError> {
    let mut document_creation_information = DocumentCreationInformation::default();

    // Get document creation information.
    for atom in atoms {
        match atom {
            Atom::SpdxVersion(value) => {
                document_creation_information.spdx_version = value.to_string();
            }
            Atom::DataLicense(value) => {
                document_creation_information.data_license = value.to_string();
            }
            Atom::SPDXID(value) => {
                document_creation_information.spdx_identifier = value.to_string();
            }
            Atom::DocumentName(value) => {
                document_creation_information.document_name = value.to_string();
            }
            Atom::DocumentNamespace(value) => {
                document_creation_information.spdx_document_namespace = value.to_string();
            }
            Atom::ExternalDocumentRef(value) => {
                document_creation_information
                    .external_document_references
                    .push(value.clone());
            }
            Atom::LicenseListVersion(value) => {
                document_creation_information
                    .creation_info
                    .license_list_version = Some(value.to_string());
            }
            Atom::Creator(value) => {
                document_creation_information
                    .creation_info
                    .creators
                    .push(value.to_string());
            }
            Atom::Created(value) => {
                document_creation_information.creation_info.created =
                    DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc);
            }
            Atom::CreatorComment(value) => {
                document_creation_information.creation_info.creator_comment =
                    Some(value.to_string());
            }
            Atom::DocumentComment(value) => {
                document_creation_information.document_comment = Some(value.to_string());
            }
            Atom::TVComment(_) => continue,
            _ => break,
        }
    }
    Ok(document_creation_information)
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn packages_from_atoms(atoms: &[Atom]) -> Result<Vec<PackageInformation>, SpdxError> {
    let mut packages = Vec::new();
    let mut package_in_progress: Option<PackageInformation> = None;
    let mut external_package_ref_in_progress: Option<ExternalPackageReference> = None;

    for atom in atoms {
        match atom {
            Atom::PackageName(value) => {
                if let Some(package) = &package_in_progress {
                    packages.push(package.clone());
                }
                package_in_progress = Some(PackageInformation::default());

                if let Some(package) = &mut package_in_progress {
                    package.package_name = value.to_string();
                }
            }
            Atom::SPDXID(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_spdx_identifier = value.to_string();
                }
            }
            Atom::PackageVersion(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_version = Some(value.to_string());
                }
            }
            Atom::PackageFileName(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_file_name = Some(value.to_string());
                }
            }
            Atom::PackageSupplier(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_supplier = Some(value.to_string());
                }
            }
            Atom::PackageOriginator(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_originator = Some(value.to_string());
                }
            }
            Atom::PackageDownloadLocation(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_download_location = value.to_string();
                }
            }
            Atom::PackageVerificationCode(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_verification_code = Some(value.clone());
                }
            }
            Atom::PackageChecksum(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_checksum.push(value.clone());
                }
            }
            Atom::PackageHomePage(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_home_page = Some(value.clone());
                }
            }
            Atom::PackageSourceInfo(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.source_information = Some(value.clone());
                }
            }
            Atom::PackageLicenseConcluded(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.concluded_license = SPDXExpression::parse(value)?;
                }
            }
            Atom::PackageLicenseInfoFromFiles(value) => {
                if let Some(package) = &mut package_in_progress {
                    package
                        .all_licenses_information_from_files
                        .push(value.clone());
                }
            }
            Atom::PackageLicenseDeclared(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.declared_license = SPDXExpression::parse(value)?;
                }
            }
            Atom::PackageLicenseComments(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.comments_on_license = Some(value.clone());
                }
            }
            Atom::PackageCopyrightText(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.copyright_text = value.clone();
                }
            }
            Atom::PackageSummary(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_summary_description = Some(value.clone());
                }
            }
            Atom::PackageDescription(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_detailed_description = Some(value.clone());
                }
            }
            Atom::PackageAttributionText(value) => {
                if let Some(package) = &mut package_in_progress {
                    package.package_attribution_text.push(value.clone());
                }
            }
            Atom::ExternalRef(value) => {
                if let Some(pkg_ref) = &mut external_package_ref_in_progress {
                    if let Some(package) = &mut package_in_progress {
                        package.external_reference.push(pkg_ref.clone());
                    }
                }
                external_package_ref_in_progress = Some(value.clone());
            }
            Atom::ExternalRefComment(value) => {
                if let Some(pkg_ref) = &mut external_package_ref_in_progress {
                    pkg_ref.reference_comment = Some(value.clone());
                }
            }
            Atom::TVComment(_) => continue,
            _ => {
                if let Some(package) = &mut package_in_progress {
                    if let Some(pkg_ref) = &mut external_package_ref_in_progress {
                        package.external_reference.push(pkg_ref.clone());
                        external_package_ref_in_progress = None;
                    }
                    packages.push(package.clone());
                    package_in_progress = None;
                }
            }
        }
    }

    if let Some(package) = package_in_progress {
        packages.push(package);
    }

    Ok(packages)
}
#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod test_super {
    use std::fs::read_to_string;

    use chrono::TimeZone;

    use crate::models::{
        Algorithm, Checksum, ExternalDocumentReference, ExternalPackageReferenceCategory,
    };

    use super::*;

    #[test]
    fn spdx_creation_info_is_retrieved() {
        let file = read_to_string("tests/data/SPDXTagExample-v2.2.spdx").unwrap();
        let (_, atoms) = atoms(&file).unwrap();
        let document_creation_information =
            document_creation_information_from_atoms(&atoms).unwrap();
        assert_eq!(document_creation_information.spdx_version, "SPDX-2.2");
        assert_eq!(document_creation_information.data_license, "CC0-1.0");
        assert_eq!(
            document_creation_information.spdx_document_namespace,
            "http://spdx.org/spdxdocs/spdx-example-444504E0-4F89-41D3-9A0C-0305E82C3301"
        );
        assert_eq!(
            document_creation_information.document_name,
            "SPDX-Tools-v2.0"
        );
        assert_eq!(
            document_creation_information.spdx_identifier,
            "SPDXRef-DOCUMENT"
        );
        assert_eq!(
            document_creation_information.document_comment,
            Some(
                "This document was created using SPDX 2.0 using licenses from the web site."
                    .to_string()
            )
        );
        assert_eq!(
            document_creation_information.external_document_references,
            vec![ExternalDocumentReference::new(
                "spdx-tool-1.2".to_string(),
                "http://spdx.org/spdxdocs/spdx-tools-v1.2-3F2504E0-4F89-41D3-9A0C-0305E82C3301"
                    .to_string(),
                Checksum::new(Algorithm::SHA1, "d6a770ba38583ed4bb4525bd96e50461655d2759")
            )]
        );
        assert!(document_creation_information
            .creation_info
            .creators
            .contains(&"Tool: LicenseFind-1.0".to_string()));
        assert!(document_creation_information
            .creation_info
            .creators
            .contains(&"Organization: ExampleCodeInspect ()".to_string()));
        assert!(document_creation_information
            .creation_info
            .creators
            .contains(&"Person: Jane Doe ()".to_string()));
        assert_eq!(
            document_creation_information.creation_info.created,
            Utc.ymd(2010, 1, 29).and_hms(18, 30, 22)
        );
        assert_eq!(
            document_creation_information.creation_info.creator_comment,
            Some(
                "This package has been shipped in source and binary form.
The binaries were created with gcc 4.5.1 and expect to link to
compatible system run time libraries."
                    .to_string()
            )
        );
        assert_eq!(
            document_creation_information
                .creation_info
                .license_list_version,
            Some("3.9".to_string())
        );
    }

    #[test]
    fn package_info_is_retrieved() {
        let file = read_to_string("tests/data/SPDXTagExample-v2.2.spdx").unwrap();
        let (_, atoms) = atoms(&file).unwrap();
        let packages = packages_from_atoms(&atoms).unwrap();
        assert_eq!(packages.len(), 4);

        let glibc = packages.iter().find(|p| p.package_name == "glibc").unwrap();
        assert_eq!(glibc.package_spdx_identifier, "SPDXRef-Package");
        assert_eq!(glibc.package_version, Some("2.11.1".to_string()));
        assert_eq!(
            glibc.package_file_name,
            Some("glibc-2.11.1.tar.gz".to_string())
        );
        assert_eq!(
            glibc.package_supplier,
            Some("Person: Jane Doe (jane.doe@example.com)".to_string())
        );
        assert_eq!(
            glibc.package_originator,
            Some("Organization: ExampleCodeInspect (contact@example.com)".to_string())
        );
        assert_eq!(
            glibc.package_download_location,
            "http://ftp.gnu.org/gnu/glibc/glibc-ports-2.15.tar.gz".to_string()
        );
        assert_eq!(
            glibc.package_verification_code.as_ref().unwrap().value,
            "d6a770ba38583ed4bb4525bd96e50461655d2758".to_string()
        );
        assert_eq!(
            glibc.package_verification_code.as_ref().unwrap().excludes,
            vec!["./package.spdx"]
        );
        assert_eq!(
            glibc.package_checksum,
            vec![
                Checksum::new(Algorithm::MD5, "624c1abb3664f4b35547e7c73864ad24"),
                Checksum::new(Algorithm::SHA1, "85ed0817af83a24ad8da68c2b5094de69833983c"),
                Checksum::new(
                    Algorithm::SHA256,
                    "11b6d3ee554eedf79299905a98f9b9a04e498210b59f15094c916c91d150efcd"
                ),
            ]
        );
        assert_eq!(
            glibc.package_home_page,
            Some("http://ftp.gnu.org/gnu/glibc".to_string())
        );
        assert_eq!(
            glibc.source_information,
            Some("uses glibc-2_11-branch from git://sourceware.org/git/glibc.git.".to_string())
        );
        assert_eq!(
            glibc
                .concluded_license
                .requirements()
                .map(|er| er.req.license.id())
                .collect::<Vec<_>>(),
            vec![
                spdx::license_id("LGPL-2.0"),
                spdx::license_id("LicenseRef-3"),
            ]
        );
        assert_eq!(
            glibc.all_licenses_information_from_files,
            vec!["GPL-2.0-only", "LicenseRef-2", "LicenseRef-1"]
        );
        assert_eq!(
            glibc
                .declared_license
                .requirements()
                .map(|er| er.req.license.id())
                .collect::<Vec<_>>(),
            vec![
                spdx::license_id("LGPL-2.0"),
                spdx::license_id("LicenseRef-3"),
            ]
        );
        assert_eq!(glibc.comments_on_license, Some("The license for this project changed with the release of version x.y.  The version of the project included here post-dates the license change.".to_string()));
        assert_eq!(glibc.copyright_text, "Copyright 2008-2010 John Smith");
        assert_eq!(
            glibc.package_summary_description,
            Some("GNU C library.".to_string())
        );
        assert_eq!(glibc.package_detailed_description, Some("The GNU C Library defines functions that are specified by the ISO C standard, as well as additional features specific to POSIX and other derivatives of the Unix operating system, and extensions specific to GNU systems.".to_string()));
        assert_eq!(
            glibc.package_attribution_text,
            vec!["The GNU C Library is free software.  See the file COPYING.LIB for copying conditions, and LICENSES for notices about a few contributions that require these additional notices to be distributed.  License copyright years may be listed using range notation, e.g., 1996-2015, indicating that every year in the range, inclusive, is a copyrightable year that would otherwise be listed individually.".to_string()]
        );
        assert_eq!(
            glibc.external_reference,
            vec![
                ExternalPackageReference::new(
                    ExternalPackageReferenceCategory::Security,
                    "cpe23Type".to_string(),
                    "cpe:2.3:a:pivotal_software:spring_framework:4.1.0:*:*:*:*:*:*:*".to_string(),
                    None
                ),
                ExternalPackageReference::new(
                    ExternalPackageReferenceCategory::Other,
                    "LocationRef-acmeforge".to_string(),
                    "acmecorp/acmenator/4.1.3-alpha".to_string(),
                    Some("This is the external ref for Acme".to_string())
                ),
            ]
        );
        let jena = packages.iter().find(|p| p.package_name == "Jena").unwrap();
        assert_eq!(jena.package_spdx_identifier, "SPDXRef-fromDoap-0");
        assert_eq!(
            jena.external_reference,
            vec![ExternalPackageReference::new(
                ExternalPackageReferenceCategory::PackageManager,
                "purl".to_string(),
                "pkg:maven/org.apache.jena/apache-jena@3.12.0".to_string(),
                None
            ),]
        );
    }
}
