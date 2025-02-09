use serde::{Deserialize, Serialize};

use crate::indexed_vec::StringsWithIndex;
use crate::Error;
use crate::{AccountDesc, Amount, Date};
use num_traits::cast::{NumCast, ToPrimitive};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TransactionType {
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    Unknown,
}

impl TransactionType {
    pub fn parse(s: &str, is_debit: bool) -> Result<Self, Error> {
        if s == "SO" {
            Ok(Self::StandingOrder)
        } else if s == "BGC" {
            Ok(Self::BacsIn)
        } else if s == "FPI" {
            Ok(Self::Fpi)
        } else {
            Ok(Self::Unknown)
        }
    }
}

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

impl Transaction {
    pub fn balance_delta(&self) -> Amount {
        (self.credit.value() - self.debit.value()).into()
    }
}

crate::make_db_item!(DbTransaction, Transaction);
