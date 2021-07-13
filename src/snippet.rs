// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::SPDXExpression;

/// https://spdx.github.io/spdx-spec/5-snippet-information/
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Snippet {
    /// https://spdx.github.io/spdx-spec/5-snippet-information/#51-snippet-spdx-identifier
    #[serde(rename = "SPDXID")]
    pub snippet_spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#52-snippet-from-file-spdx-identifier
    #[serde(rename = "snippetFromFile")]
    pub snippet_from_file_spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#53-snippet-byte-range
    pub ranges: Vec<Range>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#55-snippet-concluded-license
    #[serde(rename = "licenseConcluded")]
    pub snippet_concluded_license: SPDXExpression,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#56-license-information-in-snippet
    #[serde(
        rename = "licenseInfoInSnippets",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub license_information_in_snippet: Vec<String>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#57-snippet-comments-on-license
    #[serde(
        rename = "licenseComments",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub snippet_comments_on_license: Option<String>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#58-snippet-copyright-text
    #[serde(rename = "copyrightText")]
    pub snippet_copyright_text: String,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#59-snippet-comment
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none", default)]
    pub snippet_comment: Option<String>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#510-snippet-name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub snippet_name: Option<String>,

    /// https://spdx.github.io/spdx-spec/5-snippet-information/#511-snippet-attribution-text
    #[serde(
        rename = "attributionText",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub snippet_attribution_text: Option<String>,
}

/// https://spdx.github.io/spdx-spec/5-snippet-information/#53-snippet-byte-range
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start_pointer: StartPointer,
    pub end_pointer: EndPointer,
}

/// https://spdx.github.io/spdx-spec/5-snippet-information/#53-snippet-byte-range
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartPointer {
    pub reference: Option<String>,
    pub offset: Option<i32>,
    pub line_number: Option<i32>,
}

/// https://spdx.github.io/spdx-spec/5-snippet-information/#53-snippet-byte-range
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EndPointer {
    pub reference: Option<String>,
    pub offset: Option<i32>,
    pub line_number: Option<i32>,
}
