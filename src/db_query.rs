use regex::Regex;

use crate::indexed_vec::Idx;
use crate::{
    Date, DateRange, DbAccount, DbBankTransaction, DbFund, DbItemType, DbRelatedParty,
    DbTransaction, RelatedPartyQuery, RelatedPartyType,
};

#[derive(Default, Debug, Clone)]
pub struct DbQuery {
    item_type: Option<DbItemType>,
    id: Option<usize>,
    name_match: Option<String>,
    name_re: Option<Regex>,
    desc_match: Option<String>,
    desc_re: Option<Regex>,
    rp_query: RelatedPartyQuery,
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

//ip DbQuery
impl DbQuery {
    pub fn with_item_type(mut self, opt_item_type: Option<DbItemType>) -> Self {
        self.item_type = opt_item_type;
        self
    }

    pub fn with_id(mut self, id: Option<usize>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name_match = Some(name.into());
        self
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc_match = Some(desc.into());
        self
    }

    pub fn with_rp_type(mut self, opt_rp_type: Option<RelatedPartyType>) -> Self {
        self.rp_query = opt_rp_type.into();
        self
    }

    pub fn with_date_range(mut self, date_range: DateRange) -> Self {
        self.date_range = date_range;
        self
    }

    pub fn item_type_matches(&self, db_it: DbItemType) -> bool {
        self.item_type.is_none() || (self.item_type == Some(db_it))
    }

    fn matches_name(&self, s: &str) -> bool {
        if let Some(name) = self.name_match.as_ref() {
            name == s
        } else {
            true
        }
    }

    fn matches_desc(&self, s: &str) -> bool {
        if let Some(desc) = self.desc_match.as_ref() {
            s.starts_with(desc)
        } else {
            true
        }
    }

    pub fn matches_account(&self, d: &DbAccount) -> bool {
        self.matches_name(d.inner().name())
    }

    pub fn matches_date_range(&self, d: Date) -> bool {
        if self.date_range.is_empty() {
            true
        } else {
            self.date_range.contains(d)
        }
    }

    pub fn matches_id(&self, n: usize) -> bool {
        if let Some(id) = self.id {
            id == n
        } else {
            true
        }
    }

    pub fn matches_fund(&self, d: &DbFund) -> bool {
        if let Some(name) = self.name_match.as_ref() {
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

    pub fn matches_related_party(&self, d: &DbRelatedParty) -> bool {
        if let Some(name) = self.name_match.as_ref() {
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

    pub fn matches_transaction(&self, d: &DbTransaction) -> bool {
        true
    }

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
}
