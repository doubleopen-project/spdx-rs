// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpdxError {
    #[error("Error with serde_yaml.")]
    SerdeYaml {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("Error with serde_yaml.")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },

    #[error("Error with HTML request.")]
    Request {
        #[from]
        source: reqwest::Error,
    },

    #[error("Error parsing an SPDX Expression: {0}")]
    Parse(String),

    #[error("Path {0} doesn't have an extension.")]
    PathExtension(String),

    #[error("Error with file I/O.")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Error while parsing date.")]
    DateTimeParse {
        #[from]
        source: chrono::ParseError,
    },

    #[error("Error parsing tag-value: {0}")]
    TagValueParse(String),
}
