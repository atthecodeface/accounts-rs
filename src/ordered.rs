//a Imports
use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::{Idx, VecWithIndex};
use crate::make_index;
use crate::{Date, DbId};

//a OTInde
make_index!(OTIndex, usize);

//a OTCursor
//tp OTCursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OTCursor {
    valid: bool,
    idx: OTIndex,
    ofs: usize,
}

//ip OTCursor
impl OTCursor {
    //cp new
    /// A new cursor into [OrderedTransactions]
    pub fn new(idx: OTIndex, ofs: usize) -> Self {
        Self {
            valid: true,
            idx,
            ofs,
        }
    }

    //cp invalid
    /// Create a new invalid cursor
    pub fn invalid() -> Self {
        Self {
            valid: false,
            idx: OTIndex::default(),
            ofs: 0,
        }
    }

    //ap is_valid
    /// Return true if the cursor is valid
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

//a OrderedTransactions
//tp OrderedTransactions
/// An ordering of DbItems, with a number per day
///
/// This maintains an array of `(Date, Vec<DbId>)`, with the Vec being
/// the DbId for that date. The array is kept sorted by Date, so that
/// all the transactions for a Date can be readily found.
#[derive(Debug, Default)]
pub struct OrderedTransactions {
    /// Array of transactions for each date
    ///
    /// transactions_by_date.find_key(&Data) -> OTIndex
    ///
    /// transactions_by_date[OTIndex] -> (Date, Vec<DbId>) (with at least one item)
    transactions_by_date: VecWithIndex<'static, Date, OTIndex, (Date, Vec<DbId>), true>,
}

//ip OrderedTransactions
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
