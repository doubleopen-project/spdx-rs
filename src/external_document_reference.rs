// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use super::Checksum;
use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/2-document-creation-information/#26-external-document-references
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalDocumentReference {
    /// Unique ID string of the reference.
    #[serde(rename = "externalDocumentId")]
    pub id_string: String,

    /// Unique ID for the external document.
    #[serde(rename = "spdxDocument")]
    pub spdx_document_uri: String,

    /// Checksum of the external document following the checksum format defined
    /// in https://spdx.github.io/spdx-spec/4-file-information/#44-file-checksum.
    pub checksum: Checksum,
}
