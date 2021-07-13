// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherLicensingInformationDetected {
    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#61-license-identifier
    #[serde(rename = "licenseId")]
    pub license_identifier: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#62-extracted-text
    pub extracted_text: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#63-license-name
    #[serde(rename = "name")]
    #[serde(default = "default_noassertion")]
    pub license_name: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#64-license-cross-reference
    #[serde(rename = "seeAlsos", skip_serializing_if = "Vec::is_empty", default)]
    pub license_cross_reference: Vec<String>,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#65-license-comment
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none", default)]
    pub license_comment: Option<String>,
}

fn default_noassertion() -> String {
    "NOASSERTION".into()
}
