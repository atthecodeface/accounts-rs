//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{Amount, Database, DatabaseRebuild, Date, DbId, Error, OrderedTransactions};

//a Invoice
//tp Invoice
/// An invoice which is a simple item
///
/// The transactions should really be in the order in which the
/// institution lists them (which may well be time-order)
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    /// Reason for the invoice
    ///
    /// This must be unique within the database
    reason: String,
    /// PDF filename of the invoice
    filename: String,
    /// The invoicer
    supplier_id: DbId,
    /// Amount of the invoice
    amount: Amount,
    /// Transactions that cover the payment of the invoice
    transactions: OrderedTransactions<DbId>,
}

//ip Display for Invoice
impl std::fmt::Display for Invoice {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Invoice by {} for {} by '{}', filename '{}' : with {} transactions",
            self.supplier_id,
            self.amount,
            self.reason,
            self.filename,
            self.transactions.len()
        )
    }
}

//ip Invoice
impl Invoice {
    //cp new
    pub fn new(supplier_id: DbId, reason: String, filename: String, amount: Amount) -> Self {
        let transactions = OrderedTransactions::default();
        Self {
            reason,
            filename,
            supplier_id,
            amount,
            transactions,
        }
    }

    //ap reason
    pub fn reason(&self) -> &str {
        &self.reason
    }

    //ap filename
    pub fn filename(&self) -> &str {
        &self.filename
    }

    //ap supplier_id
    pub fn supplier_id(&self) -> DbId {
        self.supplier_id
    }

    //ap amount
    pub fn amount(&self) -> Amount {
        self.amount
    }

    //mp validate_transaction
    fn validate_transaction(
        &self,
        db: &Database,
        db_id: DbId,
        t_id: DbId,
        result: &mut Vec<String>,
    ) -> Option<(Amount, Date)> {
        let mut okay = true;
        let Some(transaction) = db.get(t_id).and_then(|d| d.transaction()) else {
            result.push(format!("Db item {t_id} is not even a transaction"));
            return None;
        };
        let transaction = transaction.inner();
        if !transaction.ttype().is_to_rp() {
            result.push(format!(
                "Transaction {t_id} has incorrect type {} for an invoice",
                transaction.ttype()
            ));
            okay = false;
        }
        if transaction.db_ids().1 != db_id {
            result.push(format!(
                "Transaction {t_id} does not credit {db_id} but a *different* related party {}",
                transaction.db_ids().1
            ));
            okay = false;
        }
        if okay {
            Some((transaction.amount(), transaction.date()))
        } else {
            None
        }
    }
    //mp validate
    pub fn validate(&self, db: &Database, db_id: DbId) -> Vec<String> {
        let mut result = vec![];
        let mut balance = self.amount;
        for ot_c in self.transactions.iter() {
            let t_id = self.transactions[ot_c];
            if let Some((amount, _)) = self.validate_transaction(db, db_id, t_id, &mut result) {
                balance -= amount;
            }
        }
        if !balance.is_zero() {
            result.push(format!(
                "Transactions for {db_id} end with outstanding balance {balance}"
            ));
        }
        result
    }

    //mp clear_transactions
    /// Clear the transactions
    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    //mp add_transactions
    /// Add a Vec of transactions to the invoice
    ///
    pub fn add_transactions<I>(&mut self, db: &Database, db_id: DbId, iter: I) -> Vec<String>
    where
        I: Iterator<Item = DbId>,
    {
        let mut result = vec![];
        for t_id in iter {
            if let Some((_, date)) = self.validate_transaction(db, db_id, t_id, &mut result) {
                self.transactions.push_to_date(date, t_id);
            }
        }
        result
    }

    //mp rebuild
    pub fn rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        self.supplier_id = database_rebuild.get_new_id("Invoice supplier", self.supplier_id)?;
        self.transactions.rebuild(database_rebuild)
    }
}

//tp DbInvoice
crate::make_db_item!(DbInvoice, Invoice);

//a DbInvoices
//ti DbInvoicesState
/// The actual DbInvoices state
#[derive(Debug)]
struct DbInvoicesState {
    array: Vec<DbInvoice>,
    map: HashMap<String, DbInvoice>,
}

//tp DbInvoices
/// A dictionary of InvoiceDesc -> DbInvoice
///
/// This serializes as an array of DBInvoice, as the invoices themselves include their InvoiceDesc
#[derive(Debug)]
pub struct DbInvoices {
    state: RefCell<DbInvoicesState>,
}

//ip Default for DbInvoices
impl Default for DbInvoices {
    fn default() -> Self {
        let array = vec![];
        let map = HashMap::new();
        let state = (DbInvoicesState { array, map }).into();
        Self { state }
    }
}

//ip DbInvoices
impl DbInvoices {
    //ap map_nth
    pub fn map_nth<F, T>(&self, f: F, n: usize) -> Option<T>
    where
        F: FnOnce(&DbInvoice) -> T,
    {
        self.state.borrow().array.get(n).map(f)
    }

    //mp ids
    pub fn ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp rebuild_add_invoice
    pub fn rebuild_add_invoice(
        &self,
        db_invoice: DbInvoice,
        database_rebuild: &DatabaseRebuild,
    ) -> Result<(), Error> {
        if !self.add_invoice(db_invoice.clone()) {
            return Err(format!(
                "Failed to rebuild invoice {}, already present?",
                db_invoice.inner().reason()
            )
            .into());
        }
        db_invoice.inner_mut().rebuild(database_rebuild)
    }

    //mp add_invoice
    pub fn add_invoice(&self, db_invoice: DbInvoice) -> bool {
        if self.has_invoice(&db_invoice.inner().reason) {
            return false;
        }
        let mut state = self.state.borrow_mut();
        state.array.push(db_invoice.clone());
        state
            .map
            .insert(db_invoice.inner().reason.clone(), db_invoice.clone());
        true
    }

    //ap has_invoice
    pub fn has_invoice(&self, reason: &str) -> bool {
        self.state.borrow().map.contains_key(reason)
    }

    //ap get_invoice
    pub fn get_invoice(&self, reason: &str) -> Option<DbInvoice> {
        self.state.borrow().map.get(reason).cloned()
    }

    //zz All done
}

//ip Serialize for DbInvoices
impl Serialize for DbInvoices {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let state = self.state.borrow();
        let mut seq = serializer.serialize_seq(Some(state.array.len()))?;
        for db_acc in state.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
