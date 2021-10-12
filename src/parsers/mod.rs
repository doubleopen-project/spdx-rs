use chrono::{DateTime, Utc};

use crate::{
    error::SpdxError,
    models::{DocumentCreationInformation, SPDX},
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

#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use chrono::TimeZone;

    use crate::models::{Algorithm, Checksum, ExternalDocumentReference};

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
}
