//a Imports
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use crate::Error;
use crate::{AccountDesc, Amount, Date, DbId};

//a BankTransactionType
//tp BankTransactionType
/// A transaction type can be a BACS transfer, deposit at the bank,
/// direct debit, etc
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum BankTransactionType {
    #[default]
    Unknown,
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    DirectDebit,
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
/// An account transaction, which is one side of one or more
/// interactions
///
/// It might be a debit or a credit; it has a date and account order
///
/// It contains an Option of the related party - when loaded from a
/// bank CSV, this might need to be a guess
#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BankTransaction {
    /// Date
    ///
    date: Date,
    /// Account order (usually Data + small offset)
    ///
    /// The ordering in which it is placed within the account
    ///
    /// If this is 'none' then the ordering is unknown
    ordering: usize,
    /// CSV BankTransaction Type,
    ttype: BankTransactionType,
    /// Bank account description that the transaction belongs to
    account_id: DbId,
    /// Bank account description that the transaction belongs to
    account_desc: AccountDesc,
    /// Description; probably includes user etc
    description: String,
    /// Amount of a debit
    debit: Amount,
    /// Amount of a credit
    credit: Amount,
    /// Balance after the transaction
    balance: Amount,
    /// Related party
    #[serde(default)]
    related_party: Option<DbId>,
}

//ip BankTransaction
impl BankTransaction {
    //cp new
    pub fn new(
        date: Date,
        ttype: BankTransactionType,
        account_desc: AccountDesc,
        description: String,
        debit: Amount,
        credit: Amount,
        balance: Amount,
    ) -> Self {
        Self {
            date,
            ttype,
            account_desc,
            description,
            debit,
            credit,
            balance,
            ordering: 0,
            account_id: DbId::default(),
            related_party: None,
        }
    }

    //ap balance
    pub fn balance(&self) -> Amount {
        self.balance
    }

    //ap balance_delta
    pub fn balance_delta(&self) -> Amount {
        (self.credit.value() - self.debit.value()).into()
    }

    //ap account_desc
    pub fn account_desc(&self) -> &AccountDesc {
        &self.account_desc
    }

    //ap date
    pub fn date(&self) -> Date {
        self.date
    }

    //ap related_party
    pub fn related_party(&self) -> Option<DbId> {
        self.related_party
    }

    //ap description
    pub fn description(&self) -> &str {
        &self.description
    }

    //ap account_id
    pub fn account_id(&self) -> DbId {
        self.account_id
    }
}

//tp DbBankTransaction
crate::make_db_item!(DbBankTransaction, BankTransaction);

//a DbBankTransactions
//tp DbBankTransactions
/// All the related parties in the database
#[derive(Debug, Default)]
pub struct DbBankTransactions {
    array: Vec<DbBankTransaction>,
    index: HashMap<String, DbId>,
}

//ip DbBankTransactions
impl DbBankTransactions {
    //mp iter_db_id
    pub fn iter_db_id(&self) -> impl Iterator<Item = DbId> + use<'_> {
        self.array.iter().map(|m| m.id)
    }

    //mp add_transaction
    pub fn add_transaction(&mut self, db_transaction: DbBankTransaction) -> bool {
        if self.has_transaction(&db_transaction.inner().description) {
            return false;
        }
        self.index.insert(
            db_transaction.inner().description.clone(),
            db_transaction.id,
        );
        self.array.push(db_transaction.clone());
        true
    }

    //ap has_transaction
    pub fn has_transaction(&self, description: &str) -> bool {
        self.index.contains_key(description)
    }

    //ap get_transaction
    pub fn get_transaction(&self, description: &str) -> Option<DbId> {
        self.index.get(description).copied()
    }

    //zz All done
}

//ip Serialize for DbBankTransactions
impl Serialize for DbBankTransactions {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
        for db_acc in self.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
