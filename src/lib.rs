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
pub use base_types::{Date, Entity, FileFormat, FileType, Ordering};

mod amount;
pub use amount::Amount;

mod member;
pub use member::{DbMember, DbMembers, Member};

mod related_party;
pub use related_party::{DbRelatedParties, DbRelatedParty, RelatedParty};

mod receivables;

mod account_desc;
pub use account_desc::AccountDesc;

mod account;
pub use account::{Account, DbAccount, DbAccounts};

mod bank_transaction;
pub use bank_transaction::{
    BankTransaction, BankTransactionType, DbBankTransaction, DbBankTransactions,
};

// mod invoices;
// mod account_transactions;

pub mod banks;

mod db_id;
pub use db_id::DbId;

mod db_vec;
pub use db_vec::DbVec;

#[macro_use]
mod db_item;
pub use db_item::{DbItem, DbItemKind, DbItemType};

mod database;
pub use database::Database;

pub mod cmdline;
