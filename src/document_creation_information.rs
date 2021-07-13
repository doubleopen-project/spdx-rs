// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::CreationInfo;

use super::ExternalDocumentReference;
use serde::{Deserialize, Serialize};

/// ## Document Creation Information
///
/// SPDX's [Document Creation Information](https://spdx.github.io/spdx-spec/2-document-creation-information/)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCreationInformation {
    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#21-spdx-version
    pub spdx_version: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#22-data-license
    pub data_license: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#23-spdx-identifier
    #[serde(rename = "SPDXID")]
    pub spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#24-document-name
    #[serde(rename = "name")]
    pub document_name: String,

    ///https://spdx.github.io/spdx-spec/2-document-creation-information/#25-spdx-document-namespace
    #[serde(rename = "documentNamespace")]
    pub spdx_document_namespace: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#26-external-document-references
    #[serde(
        rename = "externalDocumentRefs",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub external_document_references: Vec<ExternalDocumentReference>,

    pub creation_info: CreationInfo,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#211-document-comment
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none", default)]
    pub document_comment: Option<String>,

    /// Doesn't seem to be in spec, but the example contains it.
    /// https://github.com/spdx/spdx-spec/issues/395
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_describes: Vec<String>,
}

impl Default for DocumentCreationInformation {
    fn default() -> Self {
        Self {
            // Current version is 2.2. Might need to support more verisons
            // in the future.
            spdx_version: "SPDX-2.2".to_string(),
            data_license: "CC0-1.0".to_string(),
            spdx_identifier: "SPDXRef-DOCUMENT".to_string(),
            document_name: "NOASSERTION".to_string(),
            spdx_document_namespace: "NOASSERTION".to_string(),
            external_document_references: Vec::new(),
            document_comment: None,
            creation_info: CreationInfo::default(),
            document_describes: Vec::new(),
        }
    }
}
