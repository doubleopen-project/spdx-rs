// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/3-package-information/#39-package-verification-code
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PackageVerificationCode {
    /// Value of the verification code.
    #[serde(rename = "packageVerificationCodeValue")]
    pub value: String,

    /// Files that were excluded when calculating the verification code.
    #[serde(
        rename = "packageVerificationCodeExcludedFiles",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub excludes: Vec<String>,
}
