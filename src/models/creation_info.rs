// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreationInfo {
    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#27-license-list-version
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub license_list_version: Option<String>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#28-creator
    pub creators: Vec<String>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#29-created
    pub created: DateTime<Utc>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#210-creator-comment
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(rename = "comment")]
    pub creator_comment: Option<String>,
}

impl Default for CreationInfo {
    fn default() -> Self {
        Self {
            license_list_version: None,
            creators: vec![
                "Person: Jane Doe ()".into(),
                "Organization: ExampleCodeInspect ()".into(),
                "Tool: LicenseFind-1.0".into(),
            ],
            created: chrono::offset::Utc::now(),
            creator_comment: None,
        }
    }
}
