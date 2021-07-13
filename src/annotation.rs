// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/8-annotations/
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// https://spdx.github.io/spdx-spec/8-annotations/#81-annotator
    pub annotator: String,

    /// https://spdx.github.io/spdx-spec/8-annotations/#82-annotation-date
    pub annotation_date: DateTime<Utc>,

    /// https://spdx.github.io/spdx-spec/8-annotations/#83-annotation-type
    pub annotation_type: AnnotationType,

    /// https://spdx.github.io/spdx-spec/8-annotations/#84-spdx-identifier-reference
    // TODO: According to the spec this is mandatory, but the example file doesn't
    // have it.
    pub spdx_identifier_reference: Option<String>,

    /// https://spdx.github.io/spdx-spec/8-annotations/#85-annotation-comment
    #[serde(rename = "comment")]
    pub annotation_comment: String,
}

/// https://spdx.github.io/spdx-spec/8-annotations/#83-annotation-type
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnnotationType {
    Review,
    Other,
}
