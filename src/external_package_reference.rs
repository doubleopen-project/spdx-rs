// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/3-package-information/#321-external-reference
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPackageReference {
    pub reference_category: ExternalPackageReferenceCategory,
    pub reference_type: String,
    pub reference_locator: String,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reference_comment: Option<String>,
}

/// https://spdx.github.io/spdx-spec/3-package-information/#321-external-reference
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum ExternalPackageReferenceCategory {
    Security,
    PackageManager,
    PersistentID,
    Other,
}
