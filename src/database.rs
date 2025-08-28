//a Documentation
#![allow(dead_code)]

//! The database consists of tables:
//!
//! * BankTransactions
//!
//!     All of the bank transactions for the bank accounts
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
use serde::{Deserialize, Deserializer, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::indexed_vec::Idx;

use crate::{Account, DbAccounts, RelatedParties};
use crate::{BankTransaction, DbBankTransactions, DbMembers, DbRelatedParties, Member};
use crate::{DbId, DbItem};
use crate::{Error, FileFormat};

//a Database
//tp Database
/// The database of all items
///
/// The database is maintained first as a dictionary of item id to
/// item.
///
/// They are reorganized as:
///
/// * DbAccounts, which 'contain' the DbAccTransactions.
///
/// * DbRelatedParties, which 'contain' the related parties; when an
///    account transaction is added, it is matched to a related party
///    by a best-estimate
///
/// * DbBankTransactions, which are the centre of the database; these are
///     expected (once reconcilable) to have a set of debits and a set
///     of credits that balance. They are somewhat free-form - they
///     could be for a particular event, a particular season or year,
///     or for a particular category (such as subscriptions for
///     members). This is handled by a set of transaction tags
///
/// A Database can be serialized and deserialized, and will provide
/// other mechanisms for saving (such as export to MySql datatbase, or
/// sqlite3)
#[derive(Default)]
pub struct DatabaseState {
    /// Next ID to hand out to an entity
    next_db_id: DbId,

    /// Hash map from DbId to the individual items
    items: HashMap<DbId, DbItem>,
}
impl DatabaseState {
    //mi assign_next_free_db_id
    fn assign_next_free_db_id(&mut self) -> DbId {
        loop {
            let db_id = self.next_db_id;
            self.next_db_id = self.next_db_id.increment();
            if db_id.is_none() {
                continue;
            }
            if self.items.contains_key(&db_id) {
                continue;
            }
            return db_id;
        }
    }
}
#[derive(Default)]
pub struct Database {
    /// next_db_id and the items
    state: RefCell<DatabaseState>,

    /// All of the accounts in the database
    accounts: DbAccounts,

    /// All of the members in the database
    members: DbMembers,

    /// All of the accounts in the database
    related_parties: DbRelatedParties,

    /// All of the transactions in the database
    bank_transactions: DbBankTransactions,

    /// Related parties caches
    account_related_parties: RefCell<RelatedParties>,
}

//ip Database
impl Database {
    //ap accounts
    pub fn accounts(&self) -> &DbAccounts {
        &self.accounts
    }

    //ap members
    pub fn members(&self) -> &DbMembers {
        &self.members
    }

    //ap bank_transactions
    pub fn bank_transactions(&self) -> &DbBankTransactions {
        &self.bank_transactions
    }

    //mp get
    pub fn get(&self, id: DbId) -> Option<DbItem> {
        self.state.borrow().items.get(&id).cloned()
    }

    //mi add_item
    fn add_item<I>(&self, item: I) -> (DbId, DbItem)
    where
        DbItem: From<(DbId, I)>,
    {
        let mut state = self.state.borrow_mut();
        let db_id = state.assign_next_free_db_id();
        let item = DbItem::from((db_id, item));
        state.items.insert(db_id, item.clone());
        (db_id, item)
    }

    //mp add_member
    pub fn add_member(&self, member: Member) -> DbId {
        let (db_id, item) = self.add_item(member);
        self.members.add_member(item.member().unwrap());
        db_id
    }

    //mp add_account
    pub fn add_account(&self, account: Account) -> DbId {
        let (db_id, item) = self.add_item(account);
        self.accounts.add_account(item.account().unwrap());
        db_id
    }

    //mp add_bank_transaction
    pub fn add_bank_transaction(&self, bank_transaction: BankTransaction) -> DbId {
        let (db_id, item) = self.add_item(bank_transaction);
        self.bank_transactions
            .add_transaction(item.bank_transaction().unwrap());
        db_id
    }

    //mp clear_related_parties
    pub fn clear_related_parties(&self) {
        *self.account_related_parties.borrow_mut() = RelatedParties::new(6, 16, 4);
    }

    //mp find_account_related_party
    pub fn find_account_related_party(&self, descr: &str) -> DbId {
        if self.account_related_parties.borrow().is_none() {
            self.clear_related_parties();
        }
        let Some(db_id) = self
            .account_related_parties
            .borrow_mut()
            .find_item_with_collisions(descr)
        else {
            eprintln!("find_account_related_party: None");
            return DbId::none();
        };
        if !db_id.is_none() {
            eprintln!("find_account_related_party: {db_id}");
            return db_id;
        }
        self.add_new_account_related_party_cache();
        self.find_account_related_party(descr)
    }

    //mp add_new_account_related_party_cache
    pub fn add_new_account_related_party_cache(&self) {
        let descr_of_db = |id, f: &mut (dyn for<'a> FnMut(DbId, &'a str) -> ())| {
            let state = self.state.borrow();
            let Some(db_item) = state.items.get(&id) else {
                panic!("Bug - ids should all exist");
            };
            if let Some(d) = db_item.member() {
                for s in d.inner().account_descrs() {
                    f(id, s);
                }
            }
        };
        if self
            .account_related_parties
            .borrow_mut()
            .add_new_cache(self.state.borrow().items.keys().copied(), descr_of_db)
            .is_err()
        {
            panic!("Descriptions are not enough");
        }
    }

    //mp serialize_as_array
    pub fn serialize_as_array<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut sorted_ids: Vec<DbId> = self.state.borrow().items.keys().cloned().collect();
        sorted_ids.sort();
        let len = sorted_ids.len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in sorted_ids.into_iter() {
            seq.serialize_element(&self.state.borrow().items[&i])?;
        }
        seq.end()
    }

    //mp deserialize_from_array
    pub fn deserialize_from_array<'de, D>(deserializer: D) -> Result<Self, Error>
    where
        D: Deserializer<'de>,
    {
        let array = Vec::<DbItem>::deserialize(deserializer)
            .map_err(|e| Error::Deserialization(e.to_string()))?;
        Self::try_from(array)
    }

    //cp deserialize
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
        ifmt: FileFormat,
    ) -> Result<Self, Error> {
        match ifmt {
            FileFormat::Array => Self::deserialize_from_array(deserializer),
            _ => Err(Error::FormatNotSupported(ifmt, "database")),
        }
    }

    //zz All done
}

//ip TryFrom<Vec<DbItem>> for Database
impl std::convert::TryFrom<Vec<DbItem>> for Database {
    type Error = Error;
    fn try_from(array: Vec<DbItem>) -> Result<Database, Error> {
        let mut db = Database::default();
        let mut state = DatabaseState::default();
        let mut next_db_id = DbId::default();
        next_db_id = state.assign_next_free_db_id();
        let mut old_to_new_id_map = HashMap::new();
        let mut new_to_old_id_map = HashMap::new();
        for item in array {
            let old_id = item.id();
            if old_to_new_id_map.contains_key(&old_id) {
                return Err(Error::DuplicateItemId(old_id));
            }
            state.items.insert(next_db_id, item);
            old_to_new_id_map.insert(old_id, next_db_id);
            new_to_old_id_map.insert(next_db_id, old_id);
            next_db_id = state.assign_next_free_db_id();
        }
        db.state = state.into();
        // Run through all items - look for accounts, and rebuild the Accounts from the database
        // Run through all items - look for transactions, and rebuild the BankTransactions from the database
        // Run through all items - look for related parties, and rebuild the RelatedParties from the database
        Ok(db)
    }
}
