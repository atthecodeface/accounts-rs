//a Imports
use std::cell::RefCell;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::Error;
use crate::{AccountDesc, Amount, Date, DbId};

//a TransactionType
//tp TransactionType
/// A transaction type can be a BACS transfer, deposit at the ,
/// direct debit, etc
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TransactionType {
    #[default]
    Unknown,
    StandingOrder,
    BacsIn,
    Fpi,
    Deposit,
    DirectDebit,
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
/// An account transaction, which is one side of one or more
/// interactions
///
/// It might be a debit or a credit; it has a date and account order
///
/// It contains an Option of the related party - when loaded from a
///  CSV, this might need to be a guess
#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Transaction {
    /// Date
    ///
    date: Date,
    /// Account order (usually Data + small offset)
    ///
    /// The ordering in which it is placed within the account
    ///
    /// If this is 'none' then the ordering is unknown
    ordering: usize,
    /// CSV Transaction Type,
    ttype: TransactionType,
    ///  account description that the transaction belongs to
    account_id: DbId,
    ///  account description that the transaction belongs to
    ///
    /// Lose this
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
    related_party: DbId,
}

//ip Transaction
impl Transaction {
    //cp new
    pub fn new(
        date: Date,
        ttype: TransactionType,
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
            account_id: DbId::none(),
            related_party: DbId::none(),
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

    //ap account_id
    pub fn account_id(&self) -> DbId {
        self.account_id
    }

    //ap date
    pub fn date(&self) -> Date {
        self.date
    }

    //ap related_party
    pub fn related_party(&self) -> DbId {
        self.related_party
    }

    //ap description
    pub fn description(&self) -> &str {
        &self.description
    }

    //mp set_related_party
    pub fn set_related_party(&mut self, related_party: DbId) {
        self.related_party = related_party;
    }

    //mp set_account_id
    pub fn set_account_id(&mut self, account_id: DbId) {
        self.account_id = account_id;
    }
}

//tp DbTransaction
crate::make_db_item!(DbTransaction, Transaction);

//a DbTransactions
//tp DbTransactionsState
/// All the members in the database
#[derive(Debug, Default)]
pub struct DbTransactionsState {
    /// All the transactions
    array: Vec<DbTransaction>,
    // Indexed by description
    // index: HashMap<String, DbTransaction>,
}

//tp DbTransactions
/// All the related parties in the database
#[derive(Debug, Default)]
pub struct DbTransactions {
    state: RefCell<DbTransactionsState>,
}

//ip DbTransactions
impl DbTransactions {
    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp add_transaction
    pub fn add_transaction(&self, db_transaction: DbTransaction) -> bool {
        // if self.has_transaction(&db_transaction.inner().description()) {
        // return false;
        // }
        let mut state = self.state.borrow_mut();
        state.array.push(db_transaction.clone());
        // state.index.insert(
        //            db_transaction.inner().description().to_string(),
        // db_transaction.clone(),
        // );
        true
    }

    //zz All done
}

//ip Serialize for DbTransactions
impl Serialize for DbTransactions {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let state = self.state.borrow();
        let mut seq = serializer.serialize_seq(Some(state.array.len()))?;
        for db_acc in state.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
