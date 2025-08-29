use regex::Regex;

use crate::indexed_vec::Idx;
use crate::{
    Date, DbAccount, DbBankTransaction, DbFund, DbItemType, DbRelatedParty, DbTransaction,
    RelatedPartyQuery, RelatedPartyType,
};

#[derive(Default, Debug)]
pub struct DbQuery {
    item_type: Option<DbItemType>,
    id: Option<usize>,
    name_match: Option<String>,
    name_re: Option<Regex>,
    desc_match: Option<String>,
    desc_re: Option<Regex>,
    rp_query: RelatedPartyQuery,
    date_range: Option<(Date, Date)>,
}

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
        if !self.matches_id(d.inner().related_party().index()) {
            return false;
        }
        if !self.matches_desc(d.inner().description()) {
            return false;
        }
        true
    }
}
