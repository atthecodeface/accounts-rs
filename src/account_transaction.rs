//a Imports
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::{AccountDesc, Amount, Date, RelatedParty};

//a AccTransactionType
//tp AccTransactionType
/// A transaction type can be a BACS transfer, deposit at the bank,
/// direct debit, etc
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AccTransactionType {
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    DirectDebit,
    Unknown,
}

//ip AccTransactionType
impl AccTransactionType {
    //cp parse
    /// Parse a string into a transaction type
    pub fn parse(s: &str, is_debit: bool) -> Result<Self, Error> {
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

//a AccTransaction, DbAccTransaction
//tp AccTransaction
/// An account transaction, which is one side of one or more
/// interactions
///
/// It might be a debit or a credit; it has a date and account order
///
/// It contains an Option of the related party - when loaded from a
/// bank CSV, this might need to be a guess
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct AccTransaction {
    /// Date
    ///
    pub date: Date,
    /// CSV AccTransaction Type,
    pub ttype: AccTransactionType,
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
    /// Related party
    #[serde(default)]
    pub related_party: Option<RelatedParty>,
}

//ip AccTransaction
impl AccTransaction {
    pub fn balance_delta(&self) -> Amount {
        (self.credit.value() - self.debit.value()).into()
    }
}

//tp DbAccTransaction
crate::make_db_item!(DbAccTransaction, AccTransaction);
