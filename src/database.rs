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

use crate::DbTransaction;
use crate::DbTransactions;
use crate::{Account, DbAccTransaction, DbAccount, DbAccounts};
use crate::{DbId, DbItem, DbItemKind, DbItemType};
use crate::{DbRelatedParties, DbRelatedParty};

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
/// * DbTransactions, which are the centre of the database; these are
///     expected (once reconcilable) to have a set of debits and a set
///     of credits that balance. They are somewhat free-form - they
///     could be for a particular event, a particular season or year,
///     or for a particular category (such as subscriptions for
///     members). This is handled by a set of transaction tags
///
/// A Database can be serialized and deserialized, and will provide
/// other mechanisms for saving (such as export to MySql datatbase, or
/// sqlite3)
pub struct Database {
    /// Next ID to hand out to an entity
    next_db_id: DbId,

    /// Hash map from DbId to the individual items
    items: HashMap<DbId, DbItem>,

    /// All of the accounts in the database
    accounts: DbAccounts,

    /// All of the accounts in the database
    related_parties: DbRelatedParties,

    /// All of the transactions in the database
    transactions: DbTransactions,
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
        let related_parties = DbRelatedParties::new();
        let transactions = DbTransactions::new();
        Self {
            next_db_id,
            items,
            accounts,
            related_parties,
            transactions,
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
