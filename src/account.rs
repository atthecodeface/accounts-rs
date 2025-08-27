//a Imports
use std::collections::BTreeMap;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::{Idx, VecWithIndex};
use crate::make_index;
use crate::{AccTransaction, AccountDesc, Database, Date, DbId, Ordering};

//a OrderedTransactions
make_index!(OTIndex, usize);

/// All the transactions grouped by Date
#[derive(Debug, Default)]
pub struct OrderedTransactions {
    /// Array of transactions for each date
    ///
    /// transactions_by_date.find_key(&Data) -> OTIndex
    ///
    /// transactions_by_date[OTIndex] -> (Date, Vec<DbId>) (with at least one item)
    transactions_by_date: VecWithIndex<'static, Date, OTIndex, (Date, Vec<DbId>), true>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OTCursor {
    valid: bool,
    idx: OTIndex,
    ofs: usize,
}
impl OTCursor {
    pub fn new(idx: OTIndex, ofs: usize) -> Self {
        Self {
            valid: true,
            idx,
            ofs,
        }
    }
    pub fn invalid() -> Self {
        Self {
            valid: false,
            idx: OTIndex::default(),
            ofs: 0,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

impl OrderedTransactions {
    //mp cursor_prev
    pub fn cursor_prev(&self, cursor: &mut OTCursor) -> bool {
        if cursor.valid {
            if cursor.ofs > 0 {
                cursor.ofs -= 1;
                true
            } else if cursor.idx.index() > 0 {
                cursor.idx = cursor.idx.decrement();
                cursor.ofs = self.transactions_by_date[cursor.idx].1.len() - 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    //mp cursor_next
    pub fn cursor_next(&self, cursor: &mut OTCursor) -> bool {
        if cursor.valid {
            if cursor.ofs < self.transactions_by_date[cursor.idx].1.len() {
                cursor.ofs += 1;
                true
            } else if cursor.idx.index() < self.transactions_by_date.len() {
                cursor.idx = cursor.idx.increment();
                cursor.ofs = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    //mp cursor_date
    pub fn cursor_date(&self, cursor: &OTCursor) -> Option<Date> {
        if cursor.valid {
            Some(self.transactions_by_date[cursor.idx].0)
        } else {
            None
        }
    }

    //mp cursor_id
    pub fn cursor_id(&self, cursor: &OTCursor) -> Option<DbId> {
        if cursor.valid {
            Some(self.transactions_by_date[cursor.idx].1[cursor.ofs])
        } else {
            None
        }
    }

    //mp cursor_of_date
    pub fn cursor_of_date(&self, date: Date, first_of_date: bool) -> OTCursor {
        if self.transactions_by_date.is_empty() {
            OTCursor::invalid()
        } else {
            match self
                .transactions_by_date
                .binary_search_by(|r| r.0.cmp(&date))
            {
                Ok(index) => {
                    let index = OTIndex::from_usize(index);
                    if first_of_date {
                        OTCursor::new(index, 0)
                    } else {
                        OTCursor::new(index, self.transactions_by_date[index].1.len() - 1)
                    }
                }
                Err(index) => {
                    let index_is_end = index == self.transactions_by_date.len();
                    let use_end_of_previous = {
                        if index_is_end {
                            true
                        } else if index > 0 && first_of_date {
                            true
                        } else {
                            false
                        }
                    };
                    if use_end_of_previous {
                        let index = OTIndex::from_usize(index - 1);
                        OTCursor::new(index, self.transactions_by_date[index].1.len() - 1)
                    } else {
                        let index = OTIndex::from_usize(index);
                        OTCursor::new(index, 0)
                    }
                }
            }
        }
    }
}

//ip Serialize for OrderedTransactions
impl Serialize for OrderedTransactions {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(None)?;
        for (date, date_transactions) in self.transactions_by_date.iter() {
            for d in date_transactions.iter() {
                seq.serialize_element(&(*date, *d))?;
            }
        }
        seq.end()
    }
}

//ip Deserialize for OrderedTransactions
impl<'de> Deserialize<'de> for OrderedTransactions {
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: serde::Deserializer<'de>,
    {
        let mut ot = Self::default();
        let v = Vec::<(Date, DbId)>::deserialize(deserializer)?;
        for (date, db_id) in v {
            let (_, idx) = ot
                .transactions_by_date
                .find_or_add(date, |_| (date, vec![]));
            ot.transactions_by_date[idx].1.push(db_id);
        }
        Ok(ot)
    }
}

//a Account
//tp Account
/// An account which contains an ordered Vec of references to account
/// transactions
///
/// This describes a bank account or an investment account
///
/// The transactions should really be in the order in which the
/// institution lists them (which may well be time-order)
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    org: String,
    name: String,
    desc: AccountDesc,
    transactions: OrderedTransactions,
}

//ip Account
impl Account {
    pub fn new(org: String, name: String, desc: AccountDesc) -> Self {
        let transactions = OrderedTransactions::default();
        Self {
            org,
            name,
            desc,
            transactions,
        }
    }
    pub fn org(&self) -> &str {
        &self.org
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    //mp add_transactions
    /// Add a Vec of transactions to the account
    ///
    /// The transactions must be contiguous as far as the bank is
    /// concerned - that is the balances before and after must match
    /// the debits/credits; they must all be for the same AccountDesc
    ///
    /// For each transaction:
    ///
    /// * Verify that it is for this AccountDesc
    ///
    /// * Check to see if is is a duplicate
    ///
    /// * Find the insertion point
    ///
    /// Return a Vec for the transactions *not* added (in the same
    /// order that they arrived)
    pub fn add_transactions(
        &mut self,
        db: &Database,
        transactions: Vec<AccTransaction>,
        slack: usize,
    ) -> Result<(), Vec<AccTransaction>> {
        for t in transactions.iter() {
            if t.account_desc() != &self.desc {
                return Err(transactions);
            }
        }
        let start_date = transactions[0].date();
        // let end_date = t.last().unwrap().date();
        let sc = self.transactions.cursor_of_date(start_date, true);
        if sc.is_valid() {
            todo!();
        }
        for t in transactions {
            // let t_id = db.add_transaction(t);
            // self.transactions.add_id(&mut sc, t_id)
        }
        Ok(())
    }
}

//tp DbAccount
crate::make_db_item!(DbAccount, Account);

//a DbAccounts
//tp DbAccounts
/// A dictionary of AccountDesc -> DbAccount
///
/// This serializes as an array of DBAccount, as the accounts themselves include their AccountDesc
#[derive(Debug)]
pub struct DbAccounts {
    array: Vec<DbAccount>,
    map: HashMap<AccountDesc, DbAccount>,
}

//ip Default for DbAccounts
impl Default for DbAccounts {
    fn default() -> Self {
        Self::new()
    }
}

//ip DbAccounts
impl DbAccounts {
    //cp new
    pub fn new() -> Self {
        let array = vec![];
        let map = HashMap::new();
        Self { array, map }
    }

    //mp descs
    pub fn descs(&self) -> impl std::iter::Iterator<Item = &AccountDesc> {
        self.map.keys()
    }

    //mp add_account
    pub fn add_account(&mut self, db_account: DbAccount) -> bool {
        if self.has_account(&db_account.inner().desc) {
            return false;
        }
        self.array.push(db_account.clone());
        self.map.insert(db_account.inner().desc, db_account.clone());
        true
    }

    //ap has_account
    pub fn has_account(&self, desc: &AccountDesc) -> bool {
        self.map.contains_key(desc)
    }

    //ap get_account
    pub fn get_account(&self, desc: &AccountDesc) -> Option<&DbAccount> {
        self.map.get(desc)
    }

    //zz All done
}

//ip Serialize for DbAccounts
impl Serialize for DbAccounts {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
        for db_acc in self.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
