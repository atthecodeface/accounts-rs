//a Imports
use std::cell::RefCell;

use serde::{Deserialize, Serialize, Serializer};

use crate::{Amount, Database, Date, DbId};

//a TransactionType
//tp TransactionType
/// A transaction type can be a BACS transfer, deposit at the ,
/// direct debit, etc
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TransactionType {
    /// Transfer between funds - debit and credit ids will be *fund*s
    #[default]
    FundTransfer,
    /// For subs, tickets, etc - debit id is RP, credit id is *fund*
    FromRp,
    /// For an invoice, expenses, etc - debit id is *fund* credit id is *rp*
    ToRp,
    ///
    CaptialRevaluation,
}

impl TransactionType {
    pub fn is_to_rp(&self) -> bool {
        matches!(self, TransactionType::ToRp)
    }
    pub fn is_from_rp(&self) -> bool {
        matches!(self, TransactionType::ToRp)
    }
    pub fn is_revaluation(&self) -> bool {
        matches!(self, TransactionType::CaptialRevaluation)
    }
    pub fn is_fund_transfer(&self) -> bool {
        matches!(self, TransactionType::FundTransfer)
    }
}

//ip Display for TransactionType
impl std::fmt::Display for TransactionType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TransactionType::FundTransfer => write!(fmt, "FundTransfer"),
            TransactionType::FromRp => write!(fmt, "FromRp"),
            TransactionType::ToRp => write!(fmt, "ToRp"),
            TransactionType::CaptialRevaluation => write!(fmt, "CapitalRevaluation"),
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
    /// CSV Transaction Type,
    ttype: TransactionType,
    /// Debit side of transaction (could be Fund, RelatedParty)
    debit_id: DbId,
    /// Credit side of transaction (could be Fund, RelatedParty)
    credit_id: DbId,
    /// Amount, always positive(?)
    amount: Amount,
    /// Notes
    notes: Vec<String>,
}

//ip Display for Transaction
impl std::fmt::Display for Transaction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}: {}: {} {} -> {} {:?}",
            self.date, self.amount, self.ttype, self.debit_id, self.credit_id, self.notes
        )
    }
}

//ip Transaction
impl Transaction {
    //cp new
    pub fn new(
        date: Date,
        ttype: TransactionType,
        amount: Amount,
        debit_id: DbId,
        credit_id: DbId,
    ) -> Self {
        Self {
            date,
            ttype,
            amount,
            debit_id,
            credit_id,
            notes: vec![],
        }
    }

    //cp new_payment
    pub fn new_payment(date: Date, amount: Amount, from_fund_id: DbId, to_id: DbId) -> Self {
        Self::new(date, TransactionType::ToRp, amount, from_fund_id, to_id)
    }

    //cp new_income
    pub fn new_income(date: Date, amount: Amount, from_id: DbId, to_fund_id: DbId) -> Self {
        Self::new(date, TransactionType::FromRp, amount, from_id, to_fund_id)
    }

    //mp update_related_dbs
    pub fn update_related_dbs(&self, database: &Database, db_id: DbId) -> bool {
        let mut okay = true;
        match self.ttype {
            TransactionType::FromRp => {
                if let Some(db_f) = database.get_fund(self.credit_id) {
                    okay &= db_f.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
                if let Some(db_rp) = database.get_related_party(self.debit_id) {
                    okay &= db_rp.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
            }
            TransactionType::ToRp => {
                if let Some(db_rp) = database.get_related_party(self.credit_id) {
                    okay &= db_rp.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
                if let Some(db_f) = database.get_fund(self.debit_id) {
                    okay &= db_f.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
            }
            _ => {
                if let Some(db_f) = database.get_fund(self.debit_id) {
                    okay &= db_f.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
                if let Some(db_f) = database.get_fund(self.credit_id) {
                    okay &= db_f.inner_mut().add_transaction(self.date, db_id);
                } else {
                    okay = false;
                }
            }
        }
        okay
    }

    //ap date
    pub fn date(&self) -> Date {
        self.date
    }

    //ap amount
    pub fn amount(&self) -> Amount {
        self.amount
    }

    //ap ttype
    pub fn ttype(&self) -> TransactionType {
        self.ttype
    }

    //ap db_ids
    pub fn db_ids(&self) -> (DbId, DbId) {
        (self.debit_id, self.credit_id)
    }

    //ap notes
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    //mp clear_notes
    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }

    //mp add_note
    pub fn add_note<I: Into<String>>(&mut self, s: I) {
        self.notes.push(s.into());
    }

    //mp show_one_line
    pub fn show_one_line(&self, db: &Database) -> String {
        format!(
            "{}: {}:   {} debit:'{}' credit:'{}'",
            self.date,
            self.amount,
            self.ttype,
            db.show_name(self.debit_id),
            db.show_name(self.credit_id)
        )
    }

    //mp balance_delta_for
    pub fn balance_delta_for(&self, db_id: DbId) -> Option<Amount> {
        if self.debit_id == db_id {
            Some(-self.amount)
        } else if self.credit_id == db_id {
            Some(self.amount)
        } else {
            None
        }
    }

    //mp show_name
    pub fn show_name(&self) -> String {
        format!(
            "{} {} '{}'",
            self.date,
            self.amount,
            self.notes.get(0).map(|s| s.as_str()).unwrap_or_default()
        )
    }

    //zz All done
}

//tp DbTransaction
crate::make_db_item!(DbTransaction, Transaction, show_name);

//a DbTransactions
//tp DbTransactionsState
/// All the members in the database
#[derive(Debug, Default)]
pub struct DbTransactionsState {
    /// All the transactions
    array: Vec<DbTransaction>,
}

//tp DbTransactions
/// All the related parties in the database
#[derive(Debug, Default)]
pub struct DbTransactions {
    state: RefCell<DbTransactionsState>,
}

//ip DbTransactions
impl DbTransactions {
    //ap map_nth
    pub fn map_nth<F, T>(&self, f: F, n: usize) -> Option<T>
    where
        F: FnOnce(&DbTransaction) -> T,
    {
        self.state.borrow().array.get(n).map(f)
    }

    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp add_transaction
    pub fn add_transaction(&self, db_transaction: DbTransaction) -> bool {
        let mut state = self.state.borrow_mut();
        state.array.push(db_transaction.clone());
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
