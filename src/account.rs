//a Imports
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::{DbId, DbItemKind, Error};

//a AccountDesc
//tp AccountDesc
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AccountDesc {
    #[default]
    None,
    Uk {
        sort_code: u32,
        account: usize,
    },
}

//ip Display for AccountDesc
impl std::fmt::Display for AccountDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AccountDesc::Uk { sort_code, account } => {
                let sc0 = (sort_code / 10_000) % 100;
                let sc1 = (sort_code / 100) % 100;
                let sc2 = sort_code % 100;
                write!(fmt, "{sc0}-{sc1}-{sc2}:{}", account)
            }
            AccountDesc::None => {
                write!(fmt, "<None>")
            }
        }
    }
}

//ip AccountDesc
impl AccountDesc {
    pub fn uk(sort_code: u32, account: usize) -> Self {
        Self::Uk { sort_code, account }
    }
    pub fn parse_uk(sc_str: &str, account: usize) -> Result<Self, Error> {
        let re = regex::Regex::new(r"^(\d{2})-(\d{2})-(\d{2})$").unwrap();
        let Some(captures) = re.captures(sc_str) else {
            return Err(Error::ParseAccount(format!("{sc_str}:{account}")));
        };
        let sc0 = u32::from_str_radix(captures.get(1).unwrap().as_str(), 10);
        let sc1 = u32::from_str_radix(captures.get(2).unwrap().as_str(), 10);
        let sc2 = u32::from_str_radix(captures.get(3).unwrap().as_str(), 10);
        if sc0.is_err() || sc1.is_err() || sc2.is_err() {
            Err(Error::ParseAccount(format!("{sc_str}:{account}")))
        } else {
            let sort_code = sc0.unwrap() * 10_000 + sc1.unwrap() * 100 + sc2.unwrap();
            Ok(Self::Uk { sort_code, account })
        }
    }
}

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
