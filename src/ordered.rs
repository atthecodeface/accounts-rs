//a Imports
use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::{Idx, VecWithIndex};
use crate::make_index;
use crate::{Date, DbId};

//a OrderedTransactionId
//tt OrderedTransactionId
pub trait OrderedTransactionId: std::default::Default + Copy + std::fmt::Debug {}

impl<I: Idx + Default> OrderedTransactionId for I {}

//a OTIndex
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
/// An ordering of T - which may be DbId, for example - with a number per day
///
/// This maintains an array of `(Date, Vec<T>)`, with the Vec being
/// the T for that date.
///
/// dat_order is an array is sorted by Date of pairs of (Date, OTIndex), mapping into the transactions_by_date array
#[derive(Debug, Default)]
pub struct OrderedTransactions<T>
where
    T: OrderedTransactionId,
{
    /// Array of transactions for each date
    ///
    /// transactions_by_date.find_key(&Data) -> OTIndex
    ///
    /// transactions_by_date[OTIndex] -> (Date, Vec<DbId>) (with at least one item)
    transactions_by_date: VecWithIndex<'static, Date, OTIndex, (Date, Vec<T>), true>,

    /// Mapping from date to the date index in transactions_by_date
    ///
    /// This is maintained in sorted order
    date_order: Vec<(Date, OTIndex)>,
}

//ip Index<ArrayIndex> for VecWithIndex
impl<T> std::ops::Index<OTCursor> for OrderedTransactions<T>
where
    T: OrderedTransactionId,
{
    type Output = T;
    #[track_caller]
    fn index(&self, cursor: OTCursor) -> &T {
        assert!(cursor.is_valid(), "Invalid cursor used for indexing");
        &self.transactions_by_date[cursor.idx].1[cursor.ofs]
    }
}

pub struct OrderedTransactionsIter<'a, T>
where
    T: OrderedTransactionId,
{
    transactions: &'a OrderedTransactions<T>,
    cursor: OTCursor,
}
impl<'a, T> Iterator for OrderedTransactionsIter<'a, T>
where
    T: OrderedTransactionId,
{
    type Item = OTCursor;
    fn next(&mut self) -> Option<OTCursor> {
        let cursor = self.cursor;
        if cursor.is_valid() {
            self.transactions.cursor_next(&mut self.cursor);
            Some(cursor)
        } else {
            None
        }
    }
}

//ip OrderedTransactions
impl<T> OrderedTransactions<T>
where
    T: OrderedTransactionId,
{
    //ap has_undated_transactions
    pub fn has_undated_transactions(&self) -> bool {
        if let Some(first) = self.date_order.first() {
            first.0.is_none()
        } else {
            false
        }
    }

    //mp iter
    pub fn iter<'a>(&'a self) -> OrderedTransactionsIter<'a, T> {
        OrderedTransactionsIter {
            transactions: self,
            cursor: self.cursor_first(),
        }
    }

    //cp add_iter
    pub fn add_iter<I, F, S>(&mut self, iter: I, date_item: F)
    where
        I: Iterator<Item = S>,
        F: Fn(S) -> (Date, T),
    {
        for s in iter {
            let (d, t) = date_item(s);
            let (added_date, ot_d) = self.transactions_by_date.find_or_add(d, |_| (d, vec![]));
            if added_date {
                self.date_order.push((d, ot_d));
            }
            self.transactions_by_date[ot_d].1.push(t);
        }
        self.sort();
    }

    //mi sort
    fn sort(&mut self) {
        self.date_order.sort_by_key(|d_idx| d_idx.0);
    }

    //ap contains_date
    pub fn contains_date(&self, date: Date) -> bool {
        self.transactions_by_date.contains(&date)
    }

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
    pub fn cursor_id(&self, cursor: &OTCursor) -> Option<T> {
        if cursor.valid {
            Some(self.transactions_by_date[cursor.idx].1[cursor.ofs])
        } else {
            None
        }
    }

    //mp cursor_first
    pub fn cursor_first(&self) -> OTCursor {
        if let Some(first) = self.date_order.first() {
            OTCursor::new(first.1, 0)
        } else {
            OTCursor::invalid()
        }
    }

    //mp cursor_of_date
    /// Return a cursor to the element specified and whether that cursor is accuacte
    ///
    /// The cursor is accurate if the date specified is present.
    ///
    /// The cursor is also accurate for *first of date* if the date
    /// specified is not present, but the cursor returned is pointing
    /// at the end of the last available date prior to that requested.
    ///
    /// The cursor is also accurate for *last of date* if the date if
    /// the cursor returned is pointing at the first transaction of
    /// the earliest *following* date.
    ///
    /// The cursor is *not* accurate for *first of date* if the date
    /// specified is not present and the cursor can only point
    /// *beyond* the specified date.
    ///
    pub fn cursor_of_date(&self, date: Date, first_of_date: bool) -> (OTCursor, bool) {
        if self.date_order.is_empty() {
            (OTCursor::invalid(), false)
        } else {
            match self.date_order.binary_search_by_key(&date, |d_idx| d_idx.0) {
                Ok(index) => {
                    let ot_index = self.date_order[index].1;
                    if first_of_date {
                        (OTCursor::new(ot_index, 0), true)
                    } else {
                        (
                            OTCursor::new(
                                ot_index,
                                self.transactions_by_date[ot_index].1.len() - 1,
                            ),
                            true,
                        )
                    }
                }
                Err(index) => {
                    let index_is_end = index == self.date_order.len();
                    let (use_end_of_previous, is_accurate) = {
                        if index_is_end {
                            // date provided is beyond the last date in the transactions
                            //
                            // Using the last transaction in the array
                            // is inaccurate for requesting the last
                            // transaction of the later date, but
                            // accurate for the first
                            (true, first_of_date)
                        } else if index > 0 && first_of_date {
                            // date provided is not present, and for
                            // first_of_date this must not return an
                            // index later than that date, so use the
                            // end of the previous; except if this is
                            // the first, in which case there is no
                            // choice
                            (true, true)
                        } else if first_of_date {
                            // index == 0
                            // Must use the first transaction, but this is older than the requested date
                            //
                            // This is not accurate
                            (false, false)
                        } else {
                            // index != end, last of date requested
                            // date provided is not present and end of
                            // date is requested; use the
                            // first of the *later* date, and this is accurate
                            (false, true)
                        }
                    };
                    if use_end_of_previous {
                        let index = OTIndex::from_usize(index - 1);
                        (
                            OTCursor::new(index, self.transactions_by_date[index].1.len() - 1),
                            is_accurate,
                        )
                    } else {
                        let index = OTIndex::from_usize(index);
                        (OTCursor::new(index, 0), is_accurate)
                    }
                }
            }
        }
    }
}

//ip Serialize for OrderedTransactions
impl<T> Serialize for OrderedTransactions<T>
where
    T: OrderedTransactionId + Serialize,
{
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
impl<'de, T> Deserialize<'de> for OrderedTransactions<T>
where
    T: OrderedTransactionId + Deserialize<'de>,
{
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: serde::Deserializer<'de>,
    {
        let mut ot = Self::default();
        let v = Vec::<(Date, T)>::deserialize(deserializer)?;
        ot.add_iter(v.into_iter(), |(d, t)| (d, t));
        Ok(ot)
    }
}
