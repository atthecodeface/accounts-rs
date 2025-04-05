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

//a Imports
mod error;
pub use error::Error;

mod indexed_vec;

mod base_types;
pub use base_types::{Date, Entity};

mod amount;
pub use amount::Amount;

mod receivables;

mod account_desc;
pub use account_desc::AccountDesc;

mod account;
pub use account::{Account, DbAccount, DbAccounts};

mod transaction;
pub use transaction::{DbTransaction, Transaction, TransactionType};

// mod invoices;
// mod account_transactions;

mod banks;
pub use banks::lloyds;

#[macro_use]
mod database;
pub use database::{Database, DbId, DbItem, DbItemKind, DbItemType};
