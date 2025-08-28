//a Imports
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::{AccountDesc, BankTransaction, Database, DbId, OrderedTransactions};

//a Account
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

//ip Account
impl Account {
    pub fn new(org: String, name: String, desc: AccountDesc) -> Self {
        let transactions = OrderedTransactions::default();
        Self {
            org,
            name,
            desc,
            transactions,
        }
    }
    pub fn org(&self) -> &str {
        &self.org
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    //mp add_transactions
    /// Add a Vec of transactions to the account
    ///
    /// The transactions must be contiguous as far as the bank is
    /// concerned - that is the balances before and after must match
    /// the debits/credits; they must all be for the same AccountDesc
    ///
    /// For each transaction:
    ///
    /// * Verify that it is for this AccountDesc
    ///
    /// * Check to see if is is a duplicate
    ///
    /// * Find the insertion point
    ///
    /// Return a Vec for the transactions *not* added (in the same
    /// order that they arrived)
    pub fn add_transactions(
        &mut self,
        db: &Database,
        transactions: Vec<BankTransaction>,
        slack: usize,
    ) -> Result<(), Vec<BankTransaction>> {
        if transactions.is_empty() {
            return Ok(());
        }
        for t in transactions.iter() {
            if t.account_desc() != &self.desc {
                return Err(transactions);
            }
        }
        let start_date = transactions[0].date();
        let end_date = transactions.last().unwrap().date();
        let sc = self.transactions.cursor_of_date(start_date, true);
        if sc.is_valid() {
            todo!();
        }
        for t in transactions {
            // let t_id = db.add_transaction(t);
            // self.transactions.add_id(&mut sc, t_id)
        }
        Ok(())
    }
}

//tp DbAccount
crate::make_db_item!(DbAccount, Account);

//a DbAccounts
//tp DbAccounts
/// A dictionary of AccountDesc -> DbAccount
///
/// This serializes as an array of DBAccount, as the accounts themselves include their AccountDesc
#[derive(Debug)]
pub struct DbAccounts {
    array: Vec<DbAccount>,
    map: HashMap<AccountDesc, DbAccount>,
}

//ip Default for DbAccounts
impl Default for DbAccounts {
    fn default() -> Self {
        Self::new()
    }
}

//ip DbAccounts
impl DbAccounts {
    //cp new
    pub fn new() -> Self {
        let array = vec![];
        let map = HashMap::new();
        Self { array, map }
    }

    //mp descs
    pub fn descs(&self) -> impl std::iter::Iterator<Item = &AccountDesc> {
        self.map.keys()
    }

    //mp add_account
    pub fn add_account(&mut self, db_account: DbAccount) -> bool {
        if self.has_account(&db_account.inner().desc) {
            return false;
        }
        self.array.push(db_account.clone());
        self.map.insert(db_account.inner().desc, db_account.clone());
        true
    }

    //ap has_account
    pub fn has_account(&self, desc: &AccountDesc) -> bool {
        self.map.contains_key(desc)
    }

    //ap get_account
    pub fn get_account(&self, desc: &AccountDesc) -> Option<&DbAccount> {
        self.map.get(desc)
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
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
        for db_acc in self.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
