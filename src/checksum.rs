// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use super::Algorithm;
use serde::{Deserialize, Serialize};

/// Representation of SPDX's
/// [Package Checksum](https://spdx.github.io/spdx-spec/3-package-information/#310-package-checksum)
/// and
/// [File Checksum](https://spdx.github.io/spdx-spec/4-file-information/#44-file-checksum).
/// According to the spec, SHA1 is mandatory but we don't currently enforce that.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Checksum {
    /// Algorithm used to calculate the checksum
    pub algorithm: Algorithm,

    /// The checksum value.
    #[serde(rename = "checksumValue")]
    pub value: String,
}

impl Checksum {
    /// Create new checksum.
    pub fn new(algorithm: Algorithm, value: &str) -> Self {
        Self {
            algorithm,
            value: value.to_lowercase(),
        }
    }
}
