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

use crate::DbQuery;
use crate::RelatedParties;
use crate::{Account, DbAccounts};
use crate::{BankTransaction, DbBankTransactions};
use crate::{DbFunds, Fund};
use crate::{DbId, DbItem, DbItemType};
use crate::{DbRelatedParties, RelatedParty};
use crate::{DbTransactions, Transaction};
use crate::{Error, FileFormat};

//a DatabaseRebuild
#[derive(Debug, Default)]
pub struct DatabaseRebuild {
    old_to_new_id_map: HashMap<DbId, DbId>,
    new_to_old_id_map: HashMap<DbId, DbId>,
}

impl DatabaseRebuild {
    pub fn add_mapping(&mut self, old_id: DbId, new_id: DbId) -> Result<(), Error> {
        if self.old_to_new_id_map.contains_key(&old_id) {
            return Err(Error::DuplicateItemId(old_id));
        }
        self.old_to_new_id_map.insert(old_id, new_id);
        self.new_to_old_id_map.insert(new_id, old_id);
        Ok(())
    }
    pub fn get_new_id(&self, reason: &str, old_id: DbId) -> Result<DbId, Error> {
        let Some(nt) = self.old_to_new_id_map.get(&old_id) else {
            return Err(format!("No old-to-new Db map entry for {old_id} for {reason}").into());
        };
        Ok(*nt)
    }
}

//a DatabaseState
//tp DatabaseState
#[derive(Default)]
pub struct DatabaseState {
    /// Next ID to hand out to an entity
    next_db_id: DbId,

    /// Hash map from DbId to the individual items
    items: HashMap<DbId, DbItem>,
}

//ip DatabaseState
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

//a Database
//tp Database
/// The database of all items
///
/// The database is maintained first as a dictionary of item id to
/// item.
///
/// They are organized as:
///
/// * DbAccounts, which 'contain' all the DbBankTransactions.
///
/// * DbFunds, which 'contain' all the DbTransactions (although a DbTransaction can be in *two* funds).
///
/// * DbRelatedParties, which 'contain' the related parties; when an
///   account transaction is added, it is matched to a related party
///   by a best-estimate
///
/// * DbBankTransactions, which are the centre of the database; these are
///   expected (once reconcilable) to have a set of debits and a set
///   of credits that balance. They are somewhat free-form - they
///   could be for a particular event, a particular season or year,
///   or for a particular category (such as subscriptions for
///   related_parties). This is handled by a set of transaction tags
///
/// A Database can be serialized and deserialized, and will provide
/// other mechanisms for saving (such as export to MySql datatbase, or
/// sqlite3)
#[derive(Default)]
pub struct Database {
    /// next_db_id and the items
    state: RefCell<DatabaseState>,

    /// All of the accounts in the database
    accounts: DbAccounts,

    /// All of the funds in the database
    funds: DbFunds,

    /// All of the related_parties in the database
    related_parties: DbRelatedParties,

    /// All of the bank transactions in the database
    bank_transactions: DbBankTransactions,

    /// All of the transactions in the database
    transactions: DbTransactions,

    /// Related parties caches
    account_related_parties: RefCell<RelatedParties>,
}

//tp DatabaseQueryIter<'a>
pub struct DatabaseQueryIter<'a> {
    query: DbQuery,
    accounts: Option<&'a DbAccounts>,
    funds: Option<&'a DbFunds>,
    related_parties: Option<&'a DbRelatedParties>,
    bank_transactions: Option<&'a DbBankTransactions>,
    transactions: Option<&'a DbTransactions>,
    index: usize,
}
impl<'a> DatabaseQueryIter<'a> {
    pub fn new(db: &'a Database, query: DbQuery) -> Self {
        let accounts = {
            if query.item_type_matches(DbItemType::Account) {
                Some(&db.accounts)
            } else {
                None
            }
        };

        let funds = {
            if query.item_type_matches(DbItemType::Fund) {
                Some(&db.funds)
            } else {
                None
            }
        };

        let related_parties = {
            if query.item_type_matches(DbItemType::RelatedParty) {
                Some(&db.related_parties)
            } else {
                None
            }
        };

        let bank_transactions = {
            if query.item_type_matches(DbItemType::BankTransaction) {
                Some(&db.bank_transactions)
            } else {
                None
            }
        };

        let transactions = {
            if query.item_type_matches(DbItemType::Transaction) {
                Some(&db.transactions)
            } else {
                None
            }
        };

        Self {
            query,
            accounts,
            funds,
            related_parties,
            bank_transactions,
            transactions,
            index: 0,
        }
    }
}

//ip Iterator for DatabaseQueryIter
impl<'a> std::iter::Iterator for DatabaseQueryIter<'a> {
    type Item = DbId;
    fn next(&mut self) -> Option<DbId> {
        loop {
            let opt_opt_db_id = {
                if let Some(accounts) = self.accounts {
                    accounts.map_nth(
                        |d| self.query.matches_account(d).then(|| d.id()),
                        self.index,
                    )
                } else if let Some(funds) = self.funds {
                    funds.map_nth(|d| self.query.matches_fund(d).then(|| d.id()), self.index)
                } else if let Some(related_parties) = self.related_parties {
                    related_parties.map_nth(
                        |d| self.query.matches_related_party(d).then(|| d.id()),
                        self.index,
                    )
                } else if let Some(transactions) = self.transactions {
                    transactions.map_nth(
                        |d| self.query.matches_transaction(d).then(|| d.id()),
                        self.index,
                    )
                } else if let Some(bank_transactions) = self.bank_transactions {
                    bank_transactions.map_nth(
                        |d| self.query.matches_bank_transaction(d).then(|| d.id()),
                        self.index,
                    )
                } else {
                    None
                }
            };
            if let Some(opt_db_id) = opt_opt_db_id {
                self.index += 1;
                if opt_db_id.is_some() {
                    return opt_db_id;
                }
                continue;
            }
            self.index = 0;
            if self.accounts.is_some() {
                self.accounts = None;
                continue;
            }
            if self.funds.is_some() {
                self.funds = None;
                continue;
            }
            if self.related_parties.is_some() {
                self.related_parties = None;
                continue;
            }
            if self.bank_transactions.is_some() {
                self.bank_transactions = None;
                continue;
            }
            if self.transactions.is_some() {
                self.transactions = None;
                continue;
            }
            return None;
        }
    }
}

//ip Database
impl Database {
    //mp try_rebuild
    pub fn try_rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        let state = self.state.borrow_mut();

        for db_id in state.items.keys() {
            let item = &state.items[db_id];
            match item.itype() {
                DbItemType::Fund => {
                    self.funds
                        .rebuild_add_fund(item.fund().unwrap(), database_rebuild)?;
                }
                DbItemType::Account => {
                    self.accounts
                        .rebuild_add_account(item.account().unwrap(), database_rebuild)?;
                }
                DbItemType::RelatedParty => {
                    self.related_parties.rebuild_add_related_party(
                        item.related_party().unwrap(),
                        database_rebuild,
                    )?;
                }
                DbItemType::BankTransaction => {
                    self.bank_transactions.rebuild_add_bank_transaction(
                        item.bank_transaction().unwrap(),
                        database_rebuild,
                    )?;
                }
                _ => (),
            }
            // Run through all items - look for transactions, and rebuild the Transactions from the database
        }
        Ok(())
    }

    //ap accounts
    pub fn accounts(&self) -> &DbAccounts {
        &self.accounts
    }

    //ap funds
    pub fn funds(&self) -> &DbFunds {
        &self.funds
    }

    //ap related_parties
    pub fn related_parties(&self) -> &DbRelatedParties {
        &self.related_parties
    }

    //ap bank_transactions
    pub fn bank_transactions(&self) -> &DbBankTransactions {
        &self.bank_transactions
    }

    //mp get
    pub fn get(&self, id: DbId) -> Option<DbItem> {
        self.state.borrow().items.get(&id).cloned()
    }

    //mp query
    pub fn query(&self, query: DbQuery) -> DatabaseQueryIter {
        DatabaseQueryIter::new(&self, query)
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

    //mp add_related_party
    pub fn add_related_party(&self, related_party: RelatedParty) -> DbId {
        let (db_id, item) = self.add_item(related_party);
        self.related_parties
            .add_related_party(item.related_party().unwrap());
        db_id
    }

    //mp add_account
    pub fn add_account(&self, account: Account) -> DbId {
        let (db_id, item) = self.add_item(account);
        self.accounts.add_account(item.account().unwrap());
        db_id
    }

    //mp add_fund
    pub fn add_fund(&self, fund: Fund) -> DbId {
        let (db_id, item) = self.add_item(fund);
        self.funds.add_fund(item.fund().unwrap());
        db_id
    }

    //mp add_bank_transaction
    /// The bank transaction must *already* have been added to db.bank_transactions
    pub fn add_bank_transaction(&self, bank_transaction: BankTransaction) -> DbId {
        let (db_id, item) = self.add_item(bank_transaction);
        db_id
    }

    //mp clear_account_related_parties
    pub fn clear_account_related_parties(&self) {
        *self.account_related_parties.borrow_mut() = RelatedParties::new(6, 12, 3);
    }

    //mp find_account_related_party
    pub fn find_account_related_party(&self, descr: &str) -> DbId {
        if self.account_related_parties.borrow().is_none() {
            self.clear_account_related_parties();
        }
        let Some(db_id) = self
            .account_related_parties
            .borrow_mut()
            .find_item_with_collisions(descr)
        else {
            // eprintln!("find_account_related_party: None");
            return DbId::none();
        };
        if !db_id.is_none() {
            // eprintln!("find_account_related_party: {db_id}");
            return db_id;
        }
        self.add_new_account_related_party_cache();
        self.find_account_related_party(descr)
    }

    //mp add_new_account_related_party_cache
    pub fn add_new_account_related_party_cache(&self) {
        let descr_of_db = |id, f: &mut (dyn for<'a> FnMut(DbId, &'a str))| {
            let state = self.state.borrow();
            let Some(db_item) = state.items.get(&id) else {
                panic!("Bug - ids should all exist");
            };
            if let Some(d) = db_item.related_party() {
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
        let mut next_db_id = state.assign_next_free_db_id();
        let mut database_rebuild = DatabaseRebuild::default();
        for item in array {
            let old_id = item.id();
            database_rebuild.add_mapping(old_id, next_db_id)?;
            state.items.insert(next_db_id, item);
            next_db_id = state.assign_next_free_db_id();
        }
        db.state = state.into();
        db.try_rebuild(&database_rebuild)?;
        Ok(db)
    }
}
