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
use crate::indexed_vec::Idx;

//a DbId
//tp DbId
crate::make_index!(DbId, usize, Some(0));

impl DbId {
    pub fn of_usize(id: usize) -> Self {
        <Self as Idx>::from_usize(id)
    }
}
