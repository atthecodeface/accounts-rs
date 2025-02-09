use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Account, DbAccount, DbAccounts, DbTransaction};

crate::make_index!(DbId, usize);

pub trait DbItemKind: Clone + Serialize + for<'a> Deserialize<'a> {
    fn id(&self) -> DbId;
    fn itype(&self) -> DbItemType;
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum DbItemType {
    Account,
    Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DbItemTypeE {
    Account(DbAccount),
    Transaction(DbTransaction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbItem {
    id: DbId,
    itype: DbItemType,
    value: DbItemTypeE,
}

impl DbItem {
    pub fn account(&self) -> Option<DbAccount> {
        if let DbItemTypeE::Account(account) = &self.value {
            Some(account.clone())
        } else {
            None
        }
    }
}

impl From<(DbId, Account)> for DbItem {
    fn from((id, account): (DbId, Account)) -> Self {
        Self {
            id,
            itype: DbItemType::Account,
            value: DbItemTypeE::Account((id, account).into()),
        }
    }
}

impl std::cmp::PartialEq for DbItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for DbItem {
    fn hash<H>(&self, hash: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(hash)
    }
}

pub struct Database {
    next_db_id: DbId,
    items: HashMap<DbId, DbItem>,
    accounts: DbAccounts,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
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
}
