// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::{Annotation, ExternalPackageReference};

use super::{Checksum, FileInformation, PackageVerificationCode, SPDXExpression};

/// ## Package Information
///
/// SPDX's [Package Information](https://spdx.github.io/spdx-spec/3-package-information/).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackageInformation {
    /// https://spdx.github.io/spdx-spec/3-package-information/#31-package-name
    #[serde(rename = "name")]
    pub package_name: String,

    /// https://spdx.github.io/spdx-spec/3-package-information/#32-package-spdx-identifier
    #[serde(rename = "SPDXID")]
    pub package_spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/3-package-information/#33-package-version
    #[serde(
        rename = "versionInfo",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub package_version: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#34-package-file-name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_file_name: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#35-package-supplier
    #[serde(rename = "supplier", skip_serializing_if = "Option::is_none", default)]
    pub package_supplier: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#36-package-originator
    #[serde(
        rename = "originator",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub package_originator: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#37-package-download-location
    #[serde(rename = "downloadLocation")]
    pub package_download_location: String,

    /// https://spdx.github.io/spdx-spec/3-package-information/#38-files-analyzed
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub files_analyzed: Option<bool>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#39-package-verification-code
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_verification_code: Option<PackageVerificationCode>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#310-package-checksum
    #[serde(rename = "checksums", skip_serializing_if = "Vec::is_empty", default)]
    pub package_checksum: Vec<Checksum>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#311-package-home-page
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none", default)]
    pub package_home_page: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#312-source-information
    #[serde(
        rename = "sourceInfo",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub source_information: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#313-concluded-license
    #[serde(rename = "licenseConcluded")]
    pub concluded_license: SPDXExpression,

    /// https://spdx.github.io/spdx-spec/3-package-information/#314-all-licenses-information-from-files
    #[serde(
        rename = "licenseInfoFromFiles",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub all_licenses_information_from_files: Vec<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#315-declared-license
    #[serde(rename = "licenseDeclared")]
    pub declared_license: SPDXExpression,

    /// https://spdx.github.io/spdx-spec/3-package-information/#316-comments-on-license
    #[serde(
        rename = "licenseComments",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub comments_on_license: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#317-copyright-text
    pub copyright_text: String,

    /// https://spdx.github.io/spdx-spec/3-package-information/#318-package-summary-description
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none", default)]
    pub package_summary_description: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#319-package-detailed-description
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub package_detailed_description: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#320-package-comment
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none", default)]
    pub package_comment: Option<String>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#321-external-reference
    #[serde(
        rename = "externalRefs",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub external_reference: Vec<ExternalPackageReference>,

    /// https://spdx.github.io/spdx-spec/3-package-information/#323-package-attribution-text
    #[serde(
        rename = "attributionTexts",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub package_attribution_text: Vec<String>,

    /// List of "files in the package". Not sure which relationship type this maps to.
    /// Info: https://github.com/spdx/spdx-spec/issues/487
    // Valid SPDX?
    #[serde(rename = "hasFiles", skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub annotations: Vec<Annotation>,
}

impl Default for PackageInformation {
    fn default() -> Self {
        Self {
            package_name: "NOASSERTION".to_string(),
            package_spdx_identifier: "SPDXRef-1".to_string(),
            package_version: None,
            package_file_name: None,
            package_supplier: None,
            package_originator: None,
            package_download_location: "NOASSERTION".to_string(),
            files_analyzed: None,
            package_verification_code: None,
            package_checksum: Vec::new(),
            package_home_page: None,
            source_information: None,
            concluded_license: SPDXExpression("NOASSERTION".to_string()),
            all_licenses_information_from_files: Vec::new(),
            declared_license: SPDXExpression("NOASSERTION".to_string()),
            comments_on_license: None,
            copyright_text: "NOASSERTION".to_string(),
            package_summary_description: None,
            package_detailed_description: None,
            package_comment: None,
            external_reference: Vec::new(),
            package_attribution_text: Vec::new(),
            files: Vec::new(),
            annotations: Vec::new(),
        }
    }
}

impl PackageInformation {
    /// Create new package.
    pub fn new(name: &str, id: &mut i32) -> Self {
        *id += 1;
        Self {
            package_name: name.to_string(),
            package_spdx_identifier: format!("SPDXRef-{}", id),
            ..Default::default()
        }
    }

    /// Find all files of the package.
    pub fn find_files_for_package<'a>(
        &'a self,
        files: &'a [FileInformation],
    ) -> Vec<&'a FileInformation> {
        self.files
            .iter()
            .map(|file| {
                files
                    .iter()
                    .find(|file_information| &file_information.file_spdx_identifier == file)
                    // Unwrap, the file should always exist in files.
                    // TODO: Proper error handling.
                    .unwrap()
            })
            .collect()
    }
}
