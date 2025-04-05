//a Documentation
//! The database consists of tables:
//!
//! * Transactions
//!
//!     All of the transactions for the bank accounts
//!
//! The database ultimately contains DbItems
//!
//!
//! All DbItems have a unique DbId, are will be of a type such as
//! Transaction, Entity, etc; they implement DbItemKind which provides
//! access to their DbId and their type (which can be referenced as a DbItemType).
//!
//! The DbItems thus have a DbId, DbItemType, and DbItemTypeE.
//!
//! A DbAccounts contains all the accounts as a Vec of DbAccount,
//! which is a DbItem of an Account; it can map from an AccountDesc to
//! a specific DbAccount
//!
//! An account has strings for bank name and account name, an
//! AccounDesc, and a Vec of all of the transactions by reference to
//! their DbId. The Vec is in time-order (or the order in which they
//! are held within the bank)

//a Imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Account, DbAccount, DbAccounts, DbTransaction};

//a DbId
//tp DbId
crate::make_index!(DbId, usize);

//tt trait DbitemKind
pub trait DbItemKind: Clone + Serialize + for<'a> Deserialize<'a> {
    fn id(&self) -> DbId;
    fn itype(&self) -> DbItemType;
}

//mp make_db_item
#[macro_export]
macro_rules! make_db_item {
    {$db_id: ident, $id:ident} => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        pub struct $db_id {
            id : $crate :: DbId,
            inner: std::rc::Rc<std::cell::RefCell<$id>>
        }
        impl $db_id {
            fn inner(&self) -> std::cell::Ref<$id> {
                self.inner.borrow()
            }
        }
        impl $crate :: DbItemKind for $db_id {
            fn id(&self) -> crate :: DbId { self.id }
            fn itype(&self) -> crate :: DbItemType {
                crate :: DbItemType :: $id
            }
        }
        impl From<(crate :: DbId, $id)> for $db_id {
            fn from((id, inner): (crate :: DbId, $id)) -> Self {
                let inner = std::rc::Rc::new(std::cell::RefCell::new(inner));
                Self { id, inner }
            }
        }

    }
}

//tp DbItemType
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum DbItemType {
    Account,
    Transaction,
}

//tp DbItemTypeE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DbItemTypeE {
    Account(DbAccount),
    Transaction(DbTransaction),
}

//a DbItem
//tp DbItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbItem {
    id: DbId,
    itype: DbItemType,
    value: DbItemTypeE,
}

//ip DbItem
impl DbItem {
    pub fn account(&self) -> Option<DbAccount> {
        if let DbItemTypeE::Account(account) = &self.value {
            Some(account.clone())
        } else {
            None
        }
    }
}

//ip From<(DbId, Account)> for DbItem
impl From<(DbId, Account)> for DbItem {
    fn from((id, account): (DbId, Account)) -> Self {
        Self {
            id,
            itype: DbItemType::Account,
            value: DbItemTypeE::Account((id, account).into()),
        }
    }
}

//ip PartialEq for DbItem
impl std::cmp::PartialEq for DbItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

//ip Hash for DbItem
impl std::hash::Hash for DbItem {
    fn hash<H>(&self, hash: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(hash)
    }
}

//a Database
//tp Database
pub struct Database {
    /// Next ID to hand out to an entity
    next_db_id: DbId,

    /// Hash map from DbId to the individual items
    items: HashMap<DbId, DbItem>,

    /// All of the accounts in the database
    accounts: DbAccounts,
}

//tp Default for Database - an empty Database
impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

//ip Database
impl Database {
    //cp new
    pub fn new() -> Self {
        let next_db_id = DbId::default();
        let items = HashMap::new();
        let accounts = DbAccounts::new();
        Self {
            next_db_id,
            items,
            accounts,
        }
    }

    //mp add_account
    pub fn add_account(&mut self, account: Account) {
        let db_id = self.next_db_id;
        self.next_db_id.increment();
        if self.items.contains_key(&db_id) {
            return self.add_account(account);
        }
        let item: DbItem = (db_id, account).into();
        self.items.insert(db_id, item.clone());
        self.accounts.add_account(item.account().unwrap());
    }

    //zz All done
}
