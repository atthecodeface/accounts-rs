use thiserror::Error;

use crate::{DbId, FileFormat, FileType};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Csv {0}")]
    Csv(#[from] csv::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("{0}")]
    ParseTransaction(String),
    #[error("failed to parse date {0}")]
    ParseDate(String),
    #[error("failed to parse account {0}: expected xx-yy-zz:n")]
    ParseAccount(String),
    #[error("transaction log {0}")]
    TransactionLog(String),
    #[error("deserialization error {0}")]
    Deserialization(String),
    #[error("duplicate item id {0}")]
    DuplicateItemId(DbId),
    #[error("unknown file extension {0}")]
    UnknownFileExtension(String),
    #[error("unknown file format {0} (try 'array' or 'dict')")]
    UnknownFileFormat(String),
    #[error("file type {0} not supported for {1}")]
    FileTypeNotSupported(FileType, &'static str),
    #[error("format {0} not supported for {1}")]
    FormatNotSupported(FileFormat, &'static str),
    #[error("{0}")]
    Msg(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}
