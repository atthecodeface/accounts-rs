//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::{DatabaseRebuild, Date, DbId, Error, OrderedTransactions};

//a RelatedPartyType, RelatedPartyQuery
//tp RelatedPartyType
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RelatedPartyType {
    #[default]
    Member,
    Friend,
    Donor,
    Supplier,
    Musician,
    Director,
}

//ip FromStr for RelatedPartyType
impl std::str::FromStr for RelatedPartyType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let ls = s.to_ascii_lowercase();
        match ls.as_str() {
            "member" => Ok(Self::Member),
            "friend" => Ok(Self::Friend),
            "donor" => Ok(Self::Donor),
            "supplier" => Ok(Self::Supplier),
            "musician" => Ok(Self::Musician),
            "director" => Ok(Self::Director),
            _ => Err(format!("Unknown relate party type {s}").into()),
        }
    }
}

//tp RelatedPartyQuery
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RelatedPartyQuery {
    RpType(RelatedPartyType),
    Any,
}

//ip RelatedPartyQuery
impl RelatedPartyQuery {
    pub fn matches_rp_type(&self, rp_type: RelatedPartyType) -> bool {
        match self {
            Self::Any => true,
            Self::RpType(x) => *x == rp_type,
        }
    }
}

//ip From<Option<RelatedPartyType>> for RelatedPartyQuery
impl From<Option<RelatedPartyType>> for RelatedPartyQuery {
    fn from(opt_rpt: Option<RelatedPartyType>) -> Self {
        if let Some(rpt) = opt_rpt {
            Self::RpType(rpt)
        } else {
            Self::Any
        }
    }
}

//a RelatedParty
//tp RelatedParty
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RelatedParty {
    name: String,
    rp_id: usize,
    rp_type: RelatedPartyType,
    address: String,
    email: String,
    house_number: String,
    postcode: String,
    telephone: String,
    tax_name: String,
    last_gift_aid: Option<Date>,
    account_descrs: Vec<String>,
    aliases: Vec<String>,
    transactions: OrderedTransactions<DbId>,
    invoices: OrderedTransactions<DbId>,
}

//ip RelatedParty
impl RelatedParty {
    //cp new
    pub fn new(name: String, rp_id: usize, rp_type: RelatedPartyType) -> Self {
        Self {
            name,
            rp_id,
            rp_type,
            ..Default::default()
        }
    }

    //ap name
    pub fn name(&self) -> &str {
        &self.name
    }

    //ap aliases
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    //ap rp_id
    pub fn rp_id(&self) -> usize {
        self.rp_id
    }

    //ap rp_type
    pub fn rp_type(&self) -> RelatedPartyType {
        self.rp_type
    }

    //ap address
    pub fn address(&self) -> &str {
        &self.address
    }

    //mp change_name
    pub fn change_name<I: Into<String>>(&mut self, i: I) {
        self.name = i.into();
    }

    //mp add_alias
    pub fn add_alias<I: Into<String>>(&mut self, i: I) {
        self.aliases.push(i.into());
    }

    //mp clear_aliases
    pub fn clear_aliases(&mut self) {
        self.aliases.clear();
    }

    //mp clear_address_info
    pub fn clear_address_info(&mut self) {
        self.address = "".into();
        self.email = "".into();
        self.postcode = "".into();
        self.telephone = "".into();
        self.tax_name = "".into();
        self.house_number = "".into();
    }

    //mp change_postcode
    pub fn change_postcode<I: Into<String>>(&mut self, i: I) {
        self.postcode = i.into();
    }

    //mp change_address
    pub fn change_address<I: Into<String>>(&mut self, i: I) {
        self.address = i.into();
    }

    //mp change_email
    pub fn change_email<I: Into<String>>(&mut self, i: I) {
        self.email = i.into();
    }

    //mp change_house_number
    pub fn change_house_number<I: Into<String>>(&mut self, i: I) {
        self.house_number = i.into();
    }

    //mp change_telephone
    pub fn change_telephone<I: Into<String>>(&mut self, i: I) {
        self.telephone = i.into();
    }

    //mp change_tax_name
    pub fn change_tax_name<I: Into<String>>(&mut self, i: I) {
        self.tax_name = i.into();
    }

    //mp clear_account_descr
    pub fn clear_account_descr(&mut self) {
        self.account_descrs.clear();
    }

    //mp add_account_descr
    pub fn add_account_descr<I: Into<String>>(&mut self, i: I) {
        self.account_descrs.push(i.into());
    }

    //ap account_descrs
    pub fn account_descrs(&self) -> impl Iterator<Item = &str> {
        self.account_descrs.iter().map(|a| a.as_str())
    }

    //ap last_gift_aid
    pub fn last_gift_aid(&self) -> Option<&Date> {
        self.last_gift_aid.as_ref()
    }

    //ap matches_query
    pub fn matches_query(&self, query: &RelatedPartyQuery) -> bool {
        query.matches_rp_type(self.rp_type)
    }

    //mp rebuild
    pub fn rebuild(&mut self, database_rebuild: &DatabaseRebuild) -> Result<(), Error> {
        self.transactions.rebuild(database_rebuild)?;
        self.invoices.rebuild(database_rebuild)
    }
}

//tp DbRelatedParty
crate::make_db_item!(DbRelatedParty, RelatedParty);

//a DbRelatedParties
//tp DbRelatedPartiesState
/// All the related_parties in the database
#[derive(Debug)]
pub struct DbRelatedPartiesState {
    array: Vec<DbRelatedParty>,
    map: HashMap<String, DbRelatedParty>,
}

//tp DbRelatedParties
/// All the related_parties in the database
#[derive(Debug)]
pub struct DbRelatedParties {
    state: RefCell<DbRelatedPartiesState>,
}

//ip Default for DbRelatedParties
impl Default for DbRelatedParties {
    fn default() -> Self {
        let array = vec![];
        let map = HashMap::new();
        let state = (DbRelatedPartiesState { array, map }).into();
        Self { state }
    }
}

//ip DbRelatedParties
impl DbRelatedParties {
    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp rp_ids
    pub fn rp_ids(&self) -> Vec<usize> {
        self.state
            .borrow()
            .array
            .iter()
            .map(|db| db.inner().rp_id)
            .collect()
    }

    //mp rebuild_add_related_party
    pub fn rebuild_add_related_party(
        &self,
        db_related_party: DbRelatedParty,
        database_rebuild: &DatabaseRebuild,
    ) -> Result<(), Error> {
        if !self.add_related_party(db_related_party.clone()) {
            return Err(format!(
                "Failed to rebuild related party {}, already present?",
                db_related_party.inner().name()
            )
            .into());
        }
        db_related_party.inner_mut().rebuild(database_rebuild)
    }

    //mp add_related_party
    pub fn add_related_party(&self, db_related_party: DbRelatedParty) -> bool {
        if self.has_rp_id(db_related_party.inner().rp_id) {
            return false;
        }
        if self
            .state
            .borrow()
            .map
            .contains_key(db_related_party.inner().name())
        {
            return false;
        }
        for a in db_related_party.inner().aliases() {
            if self.state.borrow().map.contains_key(a) {
                return false;
            }
        }
        let mut state = self.state.borrow_mut();
        state.array.push(db_related_party.clone());
        state.map.insert(
            db_related_party.inner().name().to_string(),
            db_related_party.clone(),
        );
        drop(state);
        self.add_related_party_aliases(&db_related_party);
        true
    }

    //mp remove_related_party_aliases
    pub fn remove_related_party_aliases(&self, db_related_party: &DbRelatedParty) {
        for a in db_related_party.inner().aliases() {
            if self.state.borrow().map.contains_key(a) {
                self.state.borrow_mut().map.remove(a);
            }
        }
    }

    //mp add_related_party_aliases
    pub fn add_related_party_aliases(&self, db_related_party: &DbRelatedParty) {
        for a in db_related_party.inner().aliases() {
            if !self.state.borrow().map.contains_key(a) {
                self.state
                    .borrow_mut()
                    .map
                    .insert(a.into(), db_related_party.clone());
            }
        }
    }

    //mi get_party_of_str
    fn get_party_of_str(&self, name: &str) -> Option<DbRelatedParty> {
        if name.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(n) = name.parse::<usize>() {
                return self.get_rp_id(n);
            }
        }
        self.state.borrow().map.get(name).cloned()
    }

    //ap get_party
    pub fn get_party(&self, name: &str, query: RelatedPartyQuery) -> Option<DbRelatedParty> {
        if let Some(db_rp) = self.get_party_of_str(name) {
            if db_rp.inner().matches_query(&query) {
                Some(db_rp)
            } else {
                None
            }
        } else {
            None
        }
    }

    //ap has_rp_id
    pub fn has_rp_id(&self, id: usize) -> bool {
        self.state
            .borrow()
            .array
            .iter()
            .any(|a| a.inner().rp_id == id)
    }

    //ap get_rp_id
    pub fn get_rp_id(&self, id: usize) -> Option<DbRelatedParty> {
        self.state
            .borrow()
            .array
            .iter()
            .find(|a| a.inner().rp_id == id)
            .cloned()
    }

    //zz All done
}

//ip Serialize for DbRelatedParties
impl Serialize for DbRelatedParties {
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
