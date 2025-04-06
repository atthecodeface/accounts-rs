//a Imports
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::{AccountDesc, DbId};

//a Account
//tp Account
/// An account which contains an ordered Vec of references to account
/// transactions
///
/// This describes a bank account or an investment account
///
/// The transactions should really be in the order in which the
/// institution lists them (which may well be time-order)
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Account {
    org: String,
    name: String,
    desc: AccountDesc,
    transaction_log: Vec<DbId>,
}

//ip Account
impl Account {
    pub fn new(org: String, name: String, desc: AccountDesc) -> Self {
        let transaction_log = vec![];
        Self {
            org,
            name,
            desc,
            transaction_log,
        }
    }
    pub fn org(&self) -> &str {
        &self.org
    }
    pub fn name(&self) -> &str {
        &self.name
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
