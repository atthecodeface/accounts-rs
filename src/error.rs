use thiserror::Error;

use crate::DbId;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Csv {0}")]
    Csv(#[from] csv::Error),
    #[error("failed to parse transaction {0}")]
    ParseTransaction(String),
    #[error("failed to parse date {0}")]
    ParseDate(String),
    #[error("failed to parse account {0}")]
    ParseAccount(String),
    #[error("transaction log {0}")]
    TransactionLog(String),
    #[error("deserialization error {0}")]
    Deserialization(String),
    #[error("duplicate item id {0}")]
    DuplicateItemId(DbId),
}
