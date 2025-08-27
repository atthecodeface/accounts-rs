//a Imports
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::{AccountDesc, Amount, Date};

//a BankTransactionType
//tp BankTransactionType
/// A transaction type can be a BACS transfer, deposit at the bank,
/// direct debit, etc
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum BankTransactionType {
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    DirectDebit,
    Unknown,
}

//ip BankTransactionType
impl BankTransactionType {
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

//a BankTransaction, DbBankTransaction
//tp BankTransaction
/// A bank transaction
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BankTransaction {
    /// Date
    ///
    pub date: Date,
    /// CSV BankTransaction Type,
    pub ttype: BankTransactionType,
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

//ip BankTransaction
impl BankTransaction {
    pub fn balance_delta(&self) -> Amount {
        (self.credit.value() - self.debit.value()).into()
    }
}

//tp DbBankTransaction
crate::make_db_item!(DbBankTransaction, BankTransaction);

//tp DbBankTransactions
#[derive(Default)]
pub struct DbBankTransactions {}

//ip DbBankTransactions
impl DbBankTransactions {
    pub fn new() -> Self {
        Self::default()
    }
}
