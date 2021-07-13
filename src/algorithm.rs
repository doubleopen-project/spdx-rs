// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Possible algorithms to be used for SPDX's
/// [package checksum](https://spdx.github.io/spdx-spec/3-package-information/#310-package-checksum)
/// and [file checksum](https://spdx.github.io/spdx-spec/4-file-information/#44-file-checksum).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Algorithm {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    MD2,
    MD4,
    MD5,
    MD6,
}
