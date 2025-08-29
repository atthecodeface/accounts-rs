//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{Date, DbId};

//a RelatedPartyType, RelatedPartyQuery
pub enum RelatedPartyType {
    Member,
    Friend,
    Donor,
    Supplier,
    Musician,
    Director,
}

pub enum RelatedPartyQuery {
    RpType(RelatedPartyType),
    Any,
}

//a RelatedParty
//tp RelatedParty
#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct RelatedParty {
    name: String,
    related_party_id: usize,
    address: String,
    email: String,
    house_number: String,
    postcode: String,
    telephone: String,
    tax_name: String,
    last_gift_aid: Option<Date>,
    account_descrs: Vec<String>,
    aliases: Vec<String>,
}

//ip RelatedParty
impl RelatedParty {
    //cp new
    pub fn new(name: String, related_party_id: usize) -> Self {
        let mut s = Self::default();
        s.name = name;
        s.related_party_id = related_party_id;
        s
    }

    //ap name
    pub fn name(&self) -> &str {
        &self.name
    }

    //ap aliases
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    //ap related_party_id
    pub fn related_party_id(&self) -> usize {
        self.related_party_id
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
    pub fn account_descrs<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.account_descrs.iter().map(|a| a.as_str())
    }

    //ap last_gift_aid
    pub fn last_gift_aid(&self) -> Option<&Date> {
        self.last_gift_aid.as_ref()
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

    //mp related_party_ids
    pub fn related_party_ids(&self) -> Vec<usize> {
        self.state
            .borrow()
            .array
            .iter()
            .map(|db| db.inner().related_party_id)
            .collect()
    }

    //mp add_related_party
    pub fn add_related_party(&self, db_related_party: DbRelatedParty) -> bool {
        if self.has_related_party_id(db_related_party.inner().related_party_id) {
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

    //ap get_party
    pub fn get_party(&self, name: &str, query: RelatedPartyQuery) -> Option<DbRelatedParty> {
        if name.chars().all(|c| c.is_digit(10)) {
            match name.parse::<usize>() {
                Ok(n) => {
                    return self.get_related_party_id(n.into());
                }
                _ => (),
            }
        }
        self.state.borrow().map.get(name).cloned()
    }

    //ap has_related_party_id
    pub fn has_related_party_id(&self, id: usize) -> bool {
        self.state
            .borrow()
            .array
            .iter()
            .any(|a| a.inner().related_party_id == id)
    }

    //ap get_related_party_id
    pub fn get_related_party_id(&self, id: usize) -> Option<DbRelatedParty> {
        self.state
            .borrow()
            .array
            .iter()
            .find(|a| a.inner().related_party_id == id)
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
