//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{
    AccountDesc, BankTransaction, Database, DatabaseRebuild, Date, DateRange, DbId, Error,
    OrderedTransactions,
};

//a Account
//tp AccountSummaryOwned
/// The account for delivery as a summary, without all the transactions
#[derive(Debug, Serialize)]
pub struct AccountSummaryOwned {
    org: String,
    name: String,
    desc: AccountDesc,
    num_transactions: usize,
}

//ip AccountSummaryOwned
impl AccountSummaryOwned {
    //ap summary
    pub fn summary<'a>(&'a self) -> AccountSummary<'a> {
        AccountSummary {
            org: &self.org,
            name: &self.name,
            desc: &self.desc,
            num_transactions: self.num_transactions,
        }
    }
}

//ip Display for AccountSummaryOwned
impl std::fmt::Display for AccountSummaryOwned {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.summary().fmt(fmt)
    }
}

//tp AccountSummary
/// The account for delivery as a summary, without all the transactions
#[derive(Debug, Serialize)]
pub struct AccountSummary<'a> {
    org: &'a str,
    name: &'a str,
    desc: &'a AccountDesc,
    num_transactions: usize,
}

//ip Display for AccountSummary
impl<'a> std::fmt::Display for AccountSummary<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Account '{}' with '{}' : {} : with {} transactions",
            self.name, self.org, self.desc, self.num_transactions
        )
    }
}

//ip AccountSummary
impl<'a> AccountSummary<'a> {
    pub fn to_owned(&self) -> AccountSummaryOwned {
        AccountSummaryOwned {
            org: self.org.to_owned(),
            name: self.name.to_owned(),
            desc: self.desc.clone(),
            num_transactions: self.num_transactions,
        }
    }
}

//tp Account
/// An account which contains an ordered Vec of references to account
/// transactions
///
/// This describes a bank account or an investment account
///
/// The transactions should really be in the order in which the
/// institution lists them (which may well be time-order)
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    org: String,
    name: String,
    desc: AccountDesc,
    transactions: OrderedTransactions<DbId>,
}

//ip Display for Account
impl std::fmt::Display for Account {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.summary().fmt(fmt)
    }
}

//ip Account
impl Account {
    //cp new
    pub fn new(org: String, name: String, desc: AccountDesc) -> Self {
        let transactions = OrderedTransactions::default();
        Self {
            org,
            name,
            desc,
            transactions,
        }
    }

    //ap summary
    pub fn summary<'a>(&'a self) -> AccountSummary<'a> {
        AccountSummary {
            org: &self.org,
            name: &self.name,
            desc: &self.desc,
            num_transactions: self.transactions.len(),
        }
    }

    //ap org
    pub fn org(&self) -> &str {
        &self.org
    }

    //ap name
    pub fn name(&self) -> &str {
        &self.name
    }

    //mp transactions_in_range
    pub fn transactions_in_range(&self, date_range: DateRange) -> Vec<DbId> {
        self.transactions.transactions_in_range(date_range)
    }

    //mp validate_transactions
    pub fn validate_transactions(&self, db: &Database) -> Vec<(DbId, String)> {
        let bt_of_c = |c| {
            db.get(self.transactions[c])
                .unwrap()
                .bank_transaction()
                .unwrap()
        };
        let mut result = vec![];
        let c = self.transactions.cursor_first();
        eprintln!("{c:?}");
        if c.is_valid() {
            let bt = bt_of_c(c);
            let mut balance = bt.inner().balance() - bt.inner().balance_delta(); // balance *before* first transaction
            for c in self.transactions.iter() {
                let bt = bt_of_c(c);
                if bt.inner().balance() != balance + bt.inner().balance_delta() {
                    result.push((
                        bt.id(),
                        format!(
                            "Mismatch in balance: before {}, delta {}, after {}",
                            balance,
                            bt.inner().balance_delta(),
                            bt.inner().balance()
                        ),
                    ));
                }
                balance = bt.inner().balance();
            }
        }
        result
    }

    //mp add_bank_transaction
    /// Add transaction unless it is a duplicate
    pub fn add_bank_transaction(
        &mut self,
        db: &Database,
        account_id: DbId,
        mut bt: BankTransaction,
    ) -> Result<(), BankTransaction> {
        // if db.bank_transactions().has_transaction(bt.description()) {
        // return Err(bt);
        // }
        if bt.related_party().is_none() {
            bt.set_related_party(db.find_account_related_party(bt.description()));
        }
        bt.set_account_id(account_id);

        let date = bt.date();
        let db_id = db.add_bank_transaction(bt);
        let db_bank_transaction = db.get(db_id).unwrap().bank_transaction().unwrap();
        let added = db
            .bank_transactions()
            .add_transaction(db_bank_transaction.clone());
        if !added {
            panic!(
                "Could not add DbBankTransaction {} to DbAccount",
                db_bank_transaction.inner().description()
            );
        }
        self.transactions.push_to_date(date, db_id);
        Ok(())
    }

    //mp add_transactions
    /// Add a Vec of transactions to the account
    ///
    /// Any transactions for the same date should be in the correct
    /// order, and should be able to be appended to those from the
    /// same date already in the account
    ///
    /// Return a Vec for the transactions *not* added (in the same
    /// order that they arrived)
    pub fn add_transactions(
        &mut self,
        db: &Database,
        account_id: DbId,
        transactions: Vec<BankTransaction>,
    ) -> Result<(), Vec<BankTransaction>> {
        if transactions.is_empty() {
            return Ok(());
        }
        for t in transactions.iter() {
            if t.account_desc() != &self.desc {
                return Err(transactions);
            }
        }
        let mut errors = vec![];
        for t in transactions.into_iter() {
            if let Err(e) = self.add_bank_transaction(db, account_id, t) {
                errors.push(e);
            }
        }
        self.transactions.sort();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    //mp rebuild
    pub fn rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        self.transactions.rebuild(database_rebuild)
    }
}

//tp DbAccount
crate::make_db_item!(DbAccount, Account);

//a DbAccounts
//ti DbAccountsState
/// The actual DbAccounts state
#[derive(Debug)]
struct DbAccountsState {
    array: Vec<DbAccount>,
    map: HashMap<AccountDesc, DbAccount>,
}

//tp DbAccounts
/// A dictionary of AccountDesc -> DbAccount
///
/// This serializes as an array of DBAccount, as the accounts themselves include their AccountDesc
#[derive(Debug)]
pub struct DbAccounts {
    state: RefCell<DbAccountsState>,
}

//ip Default for DbAccounts
impl Default for DbAccounts {
    fn default() -> Self {
        let array = vec![];
        let map = HashMap::new();
        let state = (DbAccountsState { array, map }).into();
        Self { state }
    }
}

//ip DbAccounts
impl DbAccounts {
    //ap map_nth
    pub fn map_nth<F, T>(&self, f: F, n: usize) -> Option<T>
    where
        F: FnOnce(&DbAccount) -> T,
    {
        self.state.borrow().array.get(n).map(f)
    }

    //mp ids
    pub fn ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp rebuild_add_account
    pub fn rebuild_add_account(
        &self,
        db_account: DbAccount,
        database_rebuild: &DatabaseRebuild,
    ) -> Result<(), Error> {
        if !self.add_account(db_account.clone()) {
            return Err(format!(
                "Failed to rebuild account {}, already present?",
                db_account.inner().name()
            )
            .into());
        }
        db_account.inner_mut().rebuild(database_rebuild)
    }

    //mp add_account
    pub fn add_account(&self, db_account: DbAccount) -> bool {
        if self.has_account(&db_account.inner().desc) {
            return false;
        }
        let mut state = self.state.borrow_mut();
        state.array.push(db_account.clone());
        state
            .map
            .insert(db_account.inner().desc, db_account.clone());
        true
    }

    //ap has_account
    pub fn has_account(&self, desc: &AccountDesc) -> bool {
        self.state.borrow().map.contains_key(desc)
    }

    //ap get_account
    pub fn get_account(&self, desc: &AccountDesc) -> Option<DbAccount> {
        self.state.borrow().map.get(desc).cloned()
    }

    //ap get_account_by_name
    pub fn get_account_by_name(&self, name: &str) -> Option<DbAccount> {
        self.state
            .borrow()
            .array
            .iter()
            .find(|s| s.inner().name() == name)
            .cloned()
    }

    //zz All done
}

//ip Serialize for DbAccounts
impl Serialize for DbAccounts {
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
