//a Documentation
//! The accounts system is designed to manage bank-related finances
//!
//! The finances of the organisation split into two areas: income and expenditure:
//!
//! # Income
//!
//! Income is money that comes in as donations, investment income, or
//! in return for a service; in this accounts system these are managed
//! as receiveables and then match up to credit transactions in a bank
//! account.
//!
//! # Expenditure
//!
//! Expenditure is money that goes out, to pay for services used by
//! the organisation; these are handled as liabilities which are then
//! matched to invoices and bank debit transactions
//!
//! # Forecasting
//!
//!

//a TO do
//
// Move transaction to Bank transaction
//
// Add a transaction that has two entities: an internal account and an external entity such as a bank account
//
// Such a transaction can be a forecast
//
// Add ability to add AccTransaction to database
//
// Find AccountDesc in accounts

//a Imports
mod error;
pub use error::Error;

mod indexed_vec;

mod base_types;
pub use base_types::{Date, DateRange, Entity, FileFormat, FileType, Ordering};

mod ordered;
pub use ordered::{OTCursor, OTIndex, OrderedTransactions};

mod amount;
pub use amount::Amount;

mod account_desc;
pub use account_desc::AccountDesc;

mod related_parties;
pub use related_parties::RelatedParties;

mod db_id;
pub use db_id::DbId;

#[macro_use]
mod db_item;
pub use db_item::{DbItem, DbItemKind, DbItemType};

mod db_query;
pub use db_query::DbQuery;

mod related_party;
pub use related_party::{
    DbRelatedParties, DbRelatedParty, RelatedParty, RelatedPartyQuery, RelatedPartyType,
};

mod fund;
pub use fund::{DbFund, DbFunds, Fund};

mod account;
pub use account::{Account, DbAccount, DbAccounts};

mod bank_transaction;
pub use bank_transaction::{
    BankTransaction, BankTransactionType, DbBankTransaction, DbBankTransactions,
};

mod transaction;
pub use transaction::{DbTransaction, DbTransactions, Transaction, TransactionType};

mod receivables;

// mod stocks;
// mod invoices;

pub mod banks;

mod database;
pub use database::{Database, DatabaseRebuild};
