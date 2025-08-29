//a Imports
use std::cell::RefCell;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::Error;
use crate::{AccountDesc, Amount, DatabaseRebuild, Date, DbId};

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

    //mp rebuild
    pub fn rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        if !self.related_party.is_none() {
            self.related_party =
                database_rebuild.get_new_id("BankTransaction related party", self.related_party)?;
        }
        self.account_id =
            database_rebuild.get_new_id("BankTransaction account ID", self.account_id)?;
        Ok(())
    }
}

//tp DbBankTransaction
crate::make_db_item!(DbBankTransaction, BankTransaction);

//a DbBankTransactions
//tp DbBankTransactionsState
/// All the members in the database
#[derive(Debug, Default)]
pub struct DbBankTransactionsState {
    /// All the transactions
    array: Vec<DbBankTransaction>,
    // Indexed by description
    // index: HashMap<String, DbBankTransaction>,
}

//tp DbBankTransactions
/// All the related parties in the database
#[derive(Debug, Default)]
pub struct DbBankTransactions {
    state: RefCell<DbBankTransactionsState>,
}

//ip DbBankTransactions
impl DbBankTransactions {
    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp rebuild_add_bank_transaction
    pub fn rebuild_add_bank_transaction(
        &self,
        db_bank_transaction: DbBankTransaction,
        database_rebuild: &DatabaseRebuild,
    ) -> Result<(), Error> {
        if !self.add_transaction(db_bank_transaction.clone()) {
            return Err(format!(
                "Failed to rebuild bank transaction {}:{}, already present?",
                db_bank_transaction.inner().date(),
                db_bank_transaction.inner().description(),
            )
            .into());
        }
        db_bank_transaction.inner_mut().rebuild(database_rebuild)
    }

    //mp add_transaction
    pub fn add_transaction(&self, db_transaction: DbBankTransaction) -> bool {
        let mut state = self.state.borrow_mut();
        state.array.push(db_transaction.clone());
        true
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
        let state = self.state.borrow();
        let mut seq = serializer.serialize_seq(Some(state.array.len()))?;
        for db_acc in state.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
