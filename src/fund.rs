//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::{Amount, Database, DatabaseRebuild, Date, DateRange, DbId, Error, OrderedTransactions};

//a Fund
//tp Fund
/// An fund which contains an ordered Vec of references to fund
/// transactions
///
/// This describes a bank fund or an investment fund
///
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Fund {
    name: String,
    description: String,
    aliases: Vec<String>,
    /// Transactions on the fund - these may be any kind of transaction
    transactions: OrderedTransactions<DbId>,
    start_balance: Amount,
    end_balance: Option<Amount>,
}

//ip Display for Fund
impl std::fmt::Display for Fund {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, fmt)
    }
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
            aliases: vec![],
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

    //ap aliases
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    //mp add_alias
    pub fn add_alias<I: Into<String>>(&mut self, i: I) {
        self.aliases.push(i.into());
    }

    //mp clear_aliases
    pub fn clear_aliases(&mut self) {
        self.aliases.clear();
    }

    //mp transactions_in_range
    pub fn transactions_in_range(&self, date_range: DateRange) -> Vec<DbId> {
        self.transactions.transactions_in_range(date_range)
    }

    //mp add_transaction
    /// Add transaction
    pub fn add_transaction(&mut self, date: Date, t_id: DbId) -> bool {
        if let Some(db_ids) = self.transactions.of_date(date) {
            if db_ids.contains(&t_id) {
                return false;
            }
        }
        self.transactions.push_to_date(date, t_id);
        self.end_balance = None;
        true
    }

    //mp rebuild
    pub fn rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        self.transactions.rebuild(database_rebuild)
    }

    //ap show
    /// Show for a human
    pub fn show(&self, db: &Database, db_id: DbId) {
        println!("Fund {} : {}", self.name, self.description);
        for a in self.aliases.iter() {
            println!("    alias {a}");
        }
        println!("Starting balance: {}", self.start_balance);
        let mut balance = self.start_balance;
        for t in self.transactions.iter() {
            let t_db_id = self.transactions[t];
            if let Some(db_t) = db.get_transaction(t_db_id) {
                if let Some(delta) = db_t.inner().balance_delta_for(db_id) {
                    println!("  {} : {}", balance, db_t.inner().show_one_line(db));
                    balance += delta;
                } else {
                    println!(
                        "  !!Not for this Fund!! : {}",
                        db_t.inner().show_one_line(db)
                    );
                }
            }
        }
        println!("Ending balance: {}", balance);
    }

    //mp show_name
    pub fn show_name(&self) -> String {
        self.name().to_string()
    }

    //zz All done
}

//tp DbFund
crate::make_db_item!(DbFund, Fund, show_name);

//a DbFunds
//ti DbFundsState
/// The actual DbFunds state
#[derive(Debug, Default)]
struct DbFundsState {
    array: Vec<DbFund>,
    index: HashMap<String, DbFund>,
}

//tp DbFunds
/// A dictionary of FundDesc -> DbFund
///
/// This serializes as an array of DBFund, as the funds themselves include their FundDesc
#[derive(Debug, Default)]
pub struct DbFunds {
    state: RefCell<DbFundsState>,
}

//ip DbFunds
impl DbFunds {
    //ap map_nth
    pub fn map_nth<F, T>(&self, f: F, n: usize) -> Option<T>
    where
        F: FnOnce(&DbFund) -> T,
    {
        self.state.borrow().array.get(n).map(f)
    }

    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp rebuild_add_fund
    pub fn rebuild_add_fund(
        &self,
        db_fund: DbFund,
        database_rebuild: &DatabaseRebuild,
    ) -> Result<(), Error> {
        if !self.add_fund(db_fund.clone()) {
            return Err(format!(
                "Failed to rebuild fund {}, already present?",
                db_fund.inner().name(),
            )
            .into());
        }
        self.add_fund_aliases(&db_fund);
        db_fund.inner_mut().rebuild(database_rebuild)
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
            .insert(db_fund.inner().name().into(), db_fund.clone());
        true
    }

    //mp remove_fund_aliases
    pub fn remove_fund_aliases(&self, db_fund: &DbFund) {
        for a in db_fund.inner().aliases() {
            if self.state.borrow().index.contains_key(a) {
                self.state.borrow_mut().index.remove(a);
            }
        }
    }

    //mp add_fund_aliases
    pub fn add_fund_aliases(&self, db_fund: &DbFund) {
        for a in db_fund.inner().aliases() {
            if !self.state.borrow().index.contains_key(a) {
                self.state
                    .borrow_mut()
                    .index
                    .insert(a.into(), db_fund.clone());
            }
        }
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
