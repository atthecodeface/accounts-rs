//a Imports
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::{AccountDesc, Amount, Date};

//a TransactionType
//tp TransactionType
/// A transaction type can be a BACS transfer, deposit at the bank,
/// direct debit, etc
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TransactionType {
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    DirectDebit,
    Unknown,
}

//ip TransactionType
impl TransactionType {
    //cp parse
    /// Parse a string into a transaction type
    pub fn parse(s: &str, _is_debit: bool) -> Result<Self, Error> {
        if s == "SO" {
            Ok(Self::StandingOrder)
        } else if s == "BGC" {
            Ok(Self::BacsIn)
        } else if s == "FPI" {
            Ok(Self::Fpi)
        } else if s == "DD" {
            Ok(Self::DirectDebit)
        } else {
            Ok(Self::Unknown)
        }
    }
}

//a Transaction, DbTransaction
//tp Transaction
/// A bank transaction
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Transaction {
    /// Date
    ///
    pub date: Date,
    /// CSV Transaction Type,
    pub ttype: TransactionType,
    /// Bank account description that the transaction belongs to
    pub account_desc: AccountDesc,
    /// Description; probably includes user etc
    pub description: String,
    /// Amount of a debit
    pub debit: Amount,
    /// Amount of a credit
    pub credit: Amount,
    /// Balance after the transaction
    pub balance: Amount,
}

//ip Transaction
impl Transaction {
    pub fn balance_delta(&self) -> Amount {
        (self.credit.value() - self.debit.value()).into()
    }
}

//tp DbTransaction
crate::make_db_item!(DbTransaction, Transaction);

//tp DbTransactions
#[derive(Default)]
pub struct DbTransactions {}

//ip DbTransactions
impl DbTransactions {
    pub fn new() -> Self {
        Self::default()
    }
}
