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

    #[error("Path {0} doesn't have an extension.")]
    PathExtension(String),

    #[error("Error with file I/O.")]
    Io {
        #[from]
        source: io::Error,
    },
}
