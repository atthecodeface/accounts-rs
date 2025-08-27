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
use std::collections::HashMap;

use crate::{Account, DbAccounts};
use crate::{DbBankTransactions, DbMembers, DbRelatedParties, Member};
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
pub struct Database {
    /// Next ID to hand out to an entity
    next_db_id: DbId,

    /// Hash map from DbId to the individual items
    items: HashMap<DbId, DbItem>,

    /// All of the accounts in the database
    accounts: DbAccounts,

    /// All of the members in the database
    members: DbMembers,

    /// All of the accounts in the database
    related_parties: DbRelatedParties,

    /// All of the transactions in the database
    bank_transactions: DbBankTransactions,
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

    //mi assign_next_free_db_id
    fn assign_next_free_db_id(&mut self) -> DbId {
        loop {
            let db_id = self.next_db_id;
            self.next_db_id = self.next_db_id.increment();
            if !self.items.contains_key(&db_id) {
                return db_id;
            }
        }
    }
    //mp add_member
    pub fn add_member(&mut self, member: Member) -> DbId {
        let db_id = self.assign_next_free_db_id();

        let item: DbItem = (db_id, member).into();
        self.items.insert(db_id, item.clone());
        self.members.add_member(item.member().unwrap());
        db_id
    }

    //mp add_account
    pub fn add_account(&mut self, account: Account) -> DbId {
        let db_id = self.assign_next_free_db_id();

        let item: DbItem = (db_id, account).into();
        self.items.insert(db_id, item.clone());
        self.accounts.add_account(item.account().unwrap());
        db_id
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
        let len = self.items.len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in self.items.values() {
            seq.serialize_element(i)?;
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
        let mut d = Database::default();
        let mut next_db_id = DbId::default();
        let mut old_to_new_id_map = HashMap::new();
        let mut new_to_old_id_map = HashMap::new();
        for item in array {
            let old_id = item.id();
            if old_to_new_id_map.contains_key(&old_id) {
                return Err(Error::DuplicateItemId(old_id));
            }
            d.items.insert(next_db_id, item);
            old_to_new_id_map.insert(old_id, next_db_id);
            new_to_old_id_map.insert(next_db_id, old_id);
            next_db_id = next_db_id.increment();
        }
        d.next_db_id = next_db_id;
        // Run through all items - look for accounts, and rebuild the Accounts from the database
        // Run through all items - look for transactions, and rebuild the BankTransactions from the database
        // Run through all items - look for related parties, and rebuild the RelatedParties from the database
        Ok(d)
    }
}
