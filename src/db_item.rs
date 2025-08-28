//a Documentation
//! The database consists of tables:
//!
//! * BankTransactions
//!
//!     All of the transactions for the bank accounts
//!
//! The database ultimately contains DbItems
//!
//!
//! All DbItems have a unique DbId, are will be of a type such as
//! BankTransaction, Entity, etc; they implement DbItemKind which provides
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

use crate::DbId;
use crate::{Account, BankTransaction, DbAccount, DbBankTransaction, Member};
use crate::{DbMember, DbRelatedParty};

//a DbItemKind
//tt trait DbitemKind
pub trait DbItemKind: Clone + Serialize + for<'a> Deserialize<'a> {
    fn id(&self) -> DbId;
    fn itype(&self) -> DbItemType;
}

//mp make_db_item
/// Construct a type that can be Added to the database as a DbItemType
///
/// The type is a Rc<RefCell<type>>, with an additional DbId id.
#[macro_export]
macro_rules! make_db_item {
    {$db_id: ident, $id:ident} => {

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $db_id {
            id : $crate :: DbId,
            inner: std::rc::Rc<std::cell::RefCell<$id>>
        }

        impl std::cmp::PartialEq for $db_id {
            fn eq(&self, other:&$db_id) -> bool {
                self.id == other.id
            }
        }

        impl std::cmp::Eq for $db_id {
        }

        impl $db_id {
            #[allow(dead_code)]
            pub fn inner(&self) -> std::cell::Ref<$id> {
                self.inner.borrow()
            }
            #[allow(dead_code)]
            pub fn borrow(&self) -> std::cell::Ref<$id> {
                self.inner.borrow()
            }
            #[allow(dead_code)]
            pub fn borrow_mut(&self) -> std::cell::RefMut<$id> {
                self.inner.borrow_mut()
            }
            #[allow(dead_code)]
            pub fn inner_mut(&self) -> std::cell::RefMut<$id> {
                self.inner.borrow_mut()
            }
            #[allow(dead_code)]
            pub fn id(&self) -> $crate :: DbId {
                self.id
            }
        }

        impl $crate :: DbItemKind for $db_id {
            fn id(&self) -> $crate :: DbId { self.id }
            fn itype(&self) -> $crate :: DbItemType {
                $crate :: DbItemType :: $id
            }
        }

        impl From<($crate :: DbId, $id)> for $db_id {
            fn from((id, inner): ($crate :: DbId, $id)) -> Self {
                let inner = std::rc::Rc::new(std::cell::RefCell::new(inner));
                Self { id, inner }
            }
        }

    }
}

//a DbItemType
//tp DbItemType
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum DbItemType {
    Account,
    BankTransaction,
    RelatedParty,
    Member,
}

//tp DbItemTypeE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DbItemTypeE {
    Account(DbAccount),
    BankTransaction(DbBankTransaction),
    RelatedParty(DbRelatedParty),
    Member(DbMember),
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
    pub fn id(&self) -> DbId {
        self.id
    }
    pub fn itype(&self) -> DbItemType {
        self.itype
    }
    pub fn account(&self) -> Option<DbAccount> {
        if let DbItemTypeE::Account(account) = &self.value {
            Some(account.clone())
        } else {
            None
        }
    }
    pub fn member(&self) -> Option<DbMember> {
        if let DbItemTypeE::Member(member) = &self.value {
            Some(member.clone())
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

//ip From<(DbId, Member)> for DbItem
impl From<(DbId, Member)> for DbItem {
    fn from((id, member): (DbId, Member)) -> Self {
        Self {
            id,
            itype: DbItemType::Member,
            value: DbItemTypeE::Member((id, member).into()),
        }
    }
}

//ip From<(DbId, BankTransaction)> for DbItem
impl From<(DbId, BankTransaction)> for DbItem {
    fn from((id, trans): (DbId, BankTransaction)) -> Self {
        Self {
            id,
            itype: DbItemType::BankTransaction,
            value: DbItemTypeE::BankTransaction((id, trans).into()),
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
