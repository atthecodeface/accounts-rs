//a Imports
use regex::Regex;

use crate::indexed_vec::Idx;
use crate::{
    Date, DateRange, DbAccount, DbBankTransaction, DbFund, DbId, DbInvoice, DbItemType,
    DbRelatedParty, DbTransaction, RelatedPartyQuery, RelatedPartyType,
};

//a DbQuery
//tp DbQuery
/// A query of the database, somewhat abstract. The matching for
/// specific DbItemTypes is up to those individual types: normally the
/// first value to check on a query is the item_type itself.
///
/// Subsequent checks might be to see if the name matches the name_re;
/// this does not make sense for (e.g.) a Transaction, so presumably
/// that is not useful to set in a specific query for transactions.
#[derive(Default, Debug, Clone)]
pub struct DbQuery {
    /// Database item type that a match must be
    item_type: Option<DbItemType>,

    /// An ID that the item must have - this might be a related party
    /// ID, for example
    id: Option<usize>,

    /// A DbId that the item must have. This can be DbId::is_none() for no match
    db_id: DbId,

    /// Name that must be start with for a match
    name_match: Option<String>,

    /// Regular expression that a name of the item must match
    name_re: Option<Regex>,

    /// A string that a description in the item must start with
    desc_match: Option<String>,

    /// Regular expression that a description in the item must match
    desc_re: Option<Regex>,

    /// A specific query that a related party must match
    rp_query: RelatedPartyQuery,

    /// A data range that the item must be within
    date_range: DateRange,
}

//ip std::fmt::Display for DbQuery
impl std::fmt::Display for DbQuery {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "DbQuery:")?;
        if let Some(item_type) = &self.item_type {
            writeln!(fmt, "  item type: {item_type:?}")?;
        }
        if let Some(id) = &self.id {
            writeln!(fmt, "  id: {id}")?;
        }
        if let Some(name_match) = &self.name_match {
            writeln!(fmt, "  name: {name_match}")?;
        }
        if let Some(name_re) = &self.name_re {
            writeln!(fmt, "  name_re: {name_re}")?;
        }
        if let Some(desc_match) = &self.desc_match {
            writeln!(fmt, "  desc start: {desc_match}")?;
        }
        if let Some(desc_re) = &self.desc_re {
            writeln!(fmt, "  desc re:{desc_re}")?;
        }
        if !self.rp_query.is_any() {
            writeln!(fmt, "  rp_query: {:?}", self.rp_query)?;
        }
        if !self.date_range.is_empty() {
            writeln!(fmt, "  dates: {}", self.date_range)?;
        }
        Ok(())
    }
}

//ip DbQuery - builders
impl DbQuery {
    //cp with_item_type
    pub fn with_item_type(mut self, opt_item_type: Option<DbItemType>) -> Self {
        self.item_type = opt_item_type;
        self
    }

    //cp with_id
    pub fn with_id(mut self, id: Option<usize>) -> Self {
        self.id = id;
        self
    }

    //cp with_db_id
    pub fn with_db_id(mut self, db_id: DbId) -> Self {
        self.db_id = db_id;
        self
    }

    //cp with_name
    pub fn with_name(mut self, name: &str) -> Self {
        if name.chars().any(|c| "*?$^[]".contains(c)) {
            if let Ok(re) = Regex::new(name) {
                self.name_re = Some(re);
                return self;
            }
        }
        self.name_match = Some(name.into());
        self
    }

    //cp with_desc
    pub fn with_desc(mut self, desc: &str) -> Self {
        if desc.chars().any(|c| "*?$^[]".contains(c)) {
            if let Ok(re) = Regex::new(desc) {
                self.desc_re = Some(re);
                return self;
            }
        }
        self.desc_match = Some(desc.into());
        self
    }

    //cp with_rp_type
    pub fn with_rp_type(mut self, opt_rp_type: Option<RelatedPartyType>) -> Self {
        self.rp_query = opt_rp_type.into();
        self
    }

    //cp with_date_range
    pub fn with_date_range(mut self, date_range: DateRange) -> Self {
        self.date_range = date_range;
        self
    }
}

//ip DbQuery - matching methods
impl DbQuery {
    //mp item_type_matches
    pub fn item_type_matches(&self, db_it: DbItemType) -> bool {
        self.item_type.is_none() || (self.item_type == Some(db_it))
    }

    //mi matches_name
    fn matches_name(&self, s: &str) -> bool {
        if let Some(name) = self.name_match.as_ref() {
            if !s.starts_with(name) {
                return false;
            }
        }
        if let Some(name_re) = self.name_re.as_ref() {
            if !name_re.is_match(s) {
                return false;
            }
        }
        true
    }

    //mi matches_any_name
    fn matches_any_name<'a, I>(&self, okay: bool, iter: I) -> bool
    where
        I: Iterator<Item = &'a str>,
    {
        if okay {
            return true;
        }
        for s in iter {
            if self.matches_name(s) {
                return true;
            }
        }
        false
    }

    //mi matches_desc
    fn matches_desc(&self, s: &str) -> bool {
        if let Some(desc) = self.desc_match.as_ref() {
            if !s.starts_with(desc) {
                return false;
            }
        }
        if let Some(desc_re) = self.desc_re.as_ref() {
            if !desc_re.is_match(s) {
                return false;
            }
        }
        true
    }

    //mi matches_date_range
    fn matches_date_range(&self, d: Date) -> bool {
        if self.date_range.is_empty() {
            true
        } else {
            self.date_range.contains(d)
        }
    }

    //mi matches_id
    fn matches_id(&self, n: usize) -> bool {
        if let Some(id) = self.id {
            id == n
        } else {
            true
        }
    }

    //mi matches_db_id
    fn matches_db_id(&self, db_id: DbId) -> bool {
        if !self.db_id.is_none() {
            db_id == self.db_id
        } else {
            true
        }
    }

    //mp matches_account - check name
    pub fn matches_account(&self, d: &DbAccount) -> bool {
        self.matches_name(d.inner().name())
    }

    //mp matches_fund
    pub fn matches_fund(&self, d: &DbFund) -> bool {
        self.matches_any_name(
            self.matches_any_name(false, [d.inner().name()].iter().map(|s| *s)),
            d.inner().aliases().iter().map(|s| s.as_str()),
        )
    }

    //mp matches_invoice
    pub fn matches_invoice(&self, d: &DbInvoice) -> bool {
        if !self.matches_name(d.inner().reason()) {
            return false;
        }
        if !self.matches_id(d.inner().supplier_id().index()) {
            return false;
        }
        true
    }

    //mp matches_related_party
    pub fn matches_related_party(&self, d: &DbRelatedParty) -> bool {
        if self.name_match.is_some() {
            let mut matched_name = self.matches_name(d.inner().name());
            if d.inner().aliases().iter().any(|s| self.matches_name(s)) {
                matched_name = true;
            }
            if !matched_name {
                return false;
            }
        }
        true
    }

    //mp matches_transaction
    pub fn matches_transaction(&self, d: &DbTransaction) -> bool {
        if !self.matches_date_range(d.inner().date()) {
            return false;
        }
        if !self.matches_db_id(d.inner().db_ids().0) && !self.matches_db_id(d.inner().db_ids().1) {
            return false;
        }
        true
    }

    //mp matches_bank_transaction
    pub fn matches_bank_transaction(&self, d: &DbBankTransaction) -> bool {
        if !self.matches_date_range(d.inner().date()) {
            return false;
        }
        if !self.matches_id(d.inner().related_party().index()) {
            return false;
        }
        if !self.matches_desc(d.inner().description()) {
            return false;
        }
        true
    }

    //zz All done
}
