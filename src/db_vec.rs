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

use crate::indexed_vec::{Idx, IndexedVec};
use crate::DbId;
use crate::DbItemKind;

//a DbVec
pub struct DbVec<I, D>
where
    I: Idx,
    D: DbItemKind,
{
    array: IndexedVec<I, D, false>,
    map: HashMap<DbId, I>,
}

impl<I, D> std::default::Default for DbVec<I, D>
where
    I: Idx,
    D: DbItemKind,
{
    fn default() -> Self {
        let array = IndexedVec::default();
        let map = HashMap::new();
        Self { array, map }
    }
}

impl<I, D> std::ops::Deref for DbVec<I, D>
where
    I: Idx,
    D: DbItemKind,
{
    type Target = IndexedVec<I, D, false>;
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<I, D> DbVec<I, D>
where
    I: Idx,
    D: DbItemKind,
{
    pub fn contains(&self, id: DbId) -> bool {
        self.map.contains_key(&id)
    }
    pub fn push(&mut self, item: D) -> Option<I> {
        let id = item.id();
        if self.contains(id) {
            None
        } else {
            let idx = self.array.push(item);
            self.map.insert(id, idx);
            Some(idx)
        }
    }
}
