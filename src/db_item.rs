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

use crate::{Account, DbAccount};
use crate::{BankTransaction, DbBankTransaction};
use crate::{DbFund, Fund};
use crate::{DbId, Error};
use crate::{DbInvoice, Invoice};
use crate::{DbRelatedParty, RelatedParty};
use crate::{DbTransaction, Transaction};

//a DbItemKind
//tt trait DbitemKind
pub trait DbItemKindObj: std::fmt::Display {}
impl<T> DbItemKindObj for T where T: std::fmt::Display {}
pub trait DbItemKind: DbItemKindObj + Clone + Serialize + for<'a> Deserialize<'a> {
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

        impl std::fmt::Display for $db_id {
            fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
                self.inner.borrow().fmt(fmt)
            }
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
    Fund,
    Invoice,
    Transaction,
    RelatedParty,
}

//ip FromStr for DbItemType
impl std::str::FromStr for DbItemType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "account" => Ok(Self::Account),
            "bank_transaction" => Ok(Self::BankTransaction),
            "fund" => Ok(Self::Fund),
            "invoice" => Ok(Self::Invoice),
            "related_party" => Ok(Self::RelatedParty),
            "rp" => Ok(Self::RelatedParty),
            "transaction" => Ok(Self::Transaction),
            _ => Err(format!("Unknown db item type {s}").into()),
        }
    }
}

//a DbItemTypeE
//tp DbItemTypeE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DbItemTypeE {
    Account(DbAccount),
    BankTransaction(DbBankTransaction),
    Fund(DbFund),
    Invoice(DbInvoice),
    RelatedParty(DbRelatedParty),
    Transaction(DbTransaction),
}

//ip DbItemTypeE
impl DbItemTypeE {
    //ap as_dyn_item_kind
    pub fn as_dyn_item_kind(&self) -> Option<&dyn DbItemKindObj> {
        match self {
            DbItemTypeE::Account(d) => Some(d),
            DbItemTypeE::Fund(d) => Some(d),
            DbItemTypeE::Invoice(d) => Some(d),
            DbItemTypeE::RelatedParty(d) => Some(d),
            DbItemTypeE::BankTransaction(d) => Some(d),
            DbItemTypeE::Transaction(d) => Some(d),
            // _ => None,
        }
    }

    //ap account
    pub fn account(&self) -> Option<DbAccount> {
        if let DbItemTypeE::Account(account) = &self {
            Some(account.clone())
        } else {
            None
        }
    }

    //ap fund
    pub fn fund(&self) -> Option<DbFund> {
        if let DbItemTypeE::Fund(fund) = &self {
            Some(fund.clone())
        } else {
            None
        }
    }

    //ap invoice
    pub fn invoice(&self) -> Option<DbInvoice> {
        if let DbItemTypeE::Invoice(invoice) = &self {
            Some(invoice.clone())
        } else {
            None
        }
    }

    //ap related_party
    pub fn related_party(&self) -> Option<DbRelatedParty> {
        if let DbItemTypeE::RelatedParty(related_party) = &self {
            Some(related_party.clone())
        } else {
            None
        }
    }

    //ap bank_transaction
    pub fn bank_transaction(&self) -> Option<DbBankTransaction> {
        if let DbItemTypeE::BankTransaction(bank_transaction) = &self {
            Some(bank_transaction.clone())
        } else {
            None
        }
    }

    //ap transaction
    pub fn transaction(&self) -> Option<DbTransaction> {
        if let DbItemTypeE::Transaction(transaction) = &self {
            Some(transaction.clone())
        } else {
            None
        }
    }
}

//a DbItem
//tp DbItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbItem {
    id: DbId,
    itype: DbItemType,
    value: DbItemTypeE,
}

//ip Display for DbItem
impl std::fmt::Display for DbItem {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if let Some(v) = self.value.as_dyn_item_kind() {
            v.fmt(fmt)
        } else {
            write!(fmt, "No display for {}", self.id)
        }
    }
}

//ip DbItem
impl DbItem {
    //ap id
    pub fn id(&self) -> DbId {
        self.id
    }

    //ap itype
    pub fn itype(&self) -> DbItemType {
        self.itype
    }

    //ap account
    pub fn account(&self) -> Option<DbAccount> {
        self.value.account()
    }

    //ap fund
    pub fn fund(&self) -> Option<DbFund> {
        self.value.fund()
    }

    //ap invoice
    pub fn invoice(&self) -> Option<DbInvoice> {
        self.value.invoice()
    }

    //ap related_party
    pub fn related_party(&self) -> Option<DbRelatedParty> {
        self.value.related_party()
    }

    //ap bank_transaction
    pub fn bank_transaction(&self) -> Option<DbBankTransaction> {
        self.value.bank_transaction()
    }

    //ap transaction
    pub fn transaction(&self) -> Option<DbTransaction> {
        self.value.transaction()
    }

    //mp as_json
    pub fn as_json(&self, pretty: bool) -> Result<String, Error> {
        if pretty {
            Ok(serde_json::to_string_pretty(self)?)
        } else {
            Ok(serde_json::to_string(self)?)
        }
    }

    //zz All done
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

//ip From<(DbId, Fund)> for DbItem
impl From<(DbId, Fund)> for DbItem {
    fn from((id, fund): (DbId, Fund)) -> Self {
        Self {
            id,
            itype: DbItemType::Fund,
            value: DbItemTypeE::Fund((id, fund).into()),
        }
    }
}

//ip From<(DbId, Invoice)> for DbItem
impl From<(DbId, Invoice)> for DbItem {
    fn from((id, invoice): (DbId, Invoice)) -> Self {
        Self {
            id,
            itype: DbItemType::Invoice,
            value: DbItemTypeE::Invoice((id, invoice).into()),
        }
    }
}

//ip From<(DbId, RelatedParty)> for DbItem
impl From<(DbId, RelatedParty)> for DbItem {
    fn from((id, related_party): (DbId, RelatedParty)) -> Self {
        Self {
            id,
            itype: DbItemType::RelatedParty,
            value: DbItemTypeE::RelatedParty((id, related_party).into()),
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

//ip From<(DbId, BankTransaction)> for DbItem
impl From<(DbId, Transaction)> for DbItem {
    fn from((id, trans): (DbId, Transaction)) -> Self {
        Self {
            id,
            itype: DbItemType::Transaction,
            value: DbItemTypeE::Transaction((id, trans).into()),
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
