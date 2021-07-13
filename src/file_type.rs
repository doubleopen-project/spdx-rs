// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/4-file-information/#43-file-type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum FileType {
    Source,
    Binary,
    Archive,
    Application,
    Audio,
    Image,
    Text,
    Video,
    Documentation,
    SPDX,
    Other,
}
