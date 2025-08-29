//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{Amount, Database, Date, DbId, DbTransaction, OrderedTransactions};

//a Fund
//tp Fund
/// An fund which contains an ordered Vec of references to fund
/// transactions
///
/// This describes a bank fund or an investment fund
///
/// The transactions should really be in the order in which the
/// institution lists them (which may well be time-order)
#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    name: String,
    description: String,
    transactions: OrderedTransactions<DbId>,
    start_balance: Amount,
    end_balance: Option<Amount>,
}

//ip Fund
impl Fund {
    //cp new
    pub fn new(name: &str, description: &str) -> Self {
        let transactions = OrderedTransactions::default();
        let name = name.into();
        let description = description.into();
        Self {
            name,
            description,
            transactions,
            start_balance: Amount::default(),
            end_balance: None,
        }
    }

    //ap name
    pub fn name(&self) -> &str {
        &self.name
    }

    //ap desc
    pub fn desc(&self) -> &str {
        &self.description
    }

    //mp transactions_between_dates
    pub fn transactions_between_dates(&self, start: Date, end: Date) -> Vec<DbId> {
        self.transactions.transactions_between_dates(start, end)
    }

    //mp add_transaction
    /// Add transaction
    pub fn add_transaction(
        &mut self,
        db: &Database,
        fund_id: DbId,
        mut t: DbTransaction,
    ) -> Result<(), DbTransaction> {
        let date = t.inner().date();
        let db_id = t.id();
        if let Some(db_ids) = self.transactions.of_date(date) {
            if db_ids.contains(&db_id) {
                return Err(t);
            }
        }
        self.transactions.push_to_date(date, db_id);
        self.end_balance = None;
        Ok(())
    }
}

//tp DbFund
crate::make_db_item!(DbFund, Fund);

//a DbFunds
//ti DbFundsState
/// The actual DbFunds state
#[derive(Debug)]
struct DbFundsState {
    array: Vec<DbFund>,
    index: HashMap<String, DbFund>,
}

//tp DbFunds
/// A dictionary of FundDesc -> DbFund
///
/// This serializes as an array of DBFund, as the funds themselves include their FundDesc
#[derive(Debug)]
pub struct DbFunds {
    state: RefCell<DbFundsState>,
}

//ip DbFunds
impl DbFunds {
    //mp ids
    pub fn ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp add_fund
    pub fn add_fund(&self, db_fund: DbFund) -> bool {
        if self.has_fund(&db_fund.inner().name) {
            return false;
        }
        let mut state = self.state.borrow_mut();
        state.array.push(db_fund.clone());
        state
            .index
            .insert(db_fund.inner().desc().into(), db_fund.clone());
        true
    }

    //ap has_fund
    pub fn has_fund(&self, name: &str) -> bool {
        self.state.borrow().index.contains_key(name)
    }

    //ap get_fund
    pub fn get_fund(&self, name: &str) -> Option<DbFund> {
        self.state.borrow().index.get(name).cloned()
    }

    //zz All done
}

//ip Serialize for DbFunds
impl Serialize for DbFunds {
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
