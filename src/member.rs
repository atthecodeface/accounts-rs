//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{Date, DbId};

//a Member
//tp Member
#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Member {
    name: String,
    member_id: usize,
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

//ip Member
impl Member {
    //cp new
    pub fn new(name: String, member_id: usize) -> Self {
        let mut s = Self::default();
        s.name = name;
        s.member_id = member_id;
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

    //ap member_id
    pub fn member_id(&self) -> usize {
        self.member_id
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

//tp DbMember
crate::make_db_item!(DbMember, Member);

//a DbMembers
//tp DbMembersState
/// All the members in the database
#[derive(Debug)]
pub struct DbMembersState {
    array: Vec<DbMember>,
    map: HashMap<String, DbMember>,
}

//tp DbMembers
/// All the members in the database
#[derive(Debug)]
pub struct DbMembers {
    state: RefCell<DbMembersState>,
}

//ip Default for DbMembers
impl Default for DbMembers {
    fn default() -> Self {
        let array = vec![];
        let map = HashMap::new();
        let state = (DbMembersState { array, map }).into();
        Self { state }
    }
}

//ip DbMembers
impl DbMembers {
    //mp db_ids
    pub fn db_ids(&self) -> Vec<DbId> {
        self.state.borrow().array.iter().map(|db| db.id()).collect()
    }

    //mp member_ids
    pub fn member_ids(&self) -> Vec<usize> {
        self.state
            .borrow()
            .array
            .iter()
            .map(|db| db.inner().member_id)
            .collect()
    }

    //mp add_member
    pub fn add_member(&self, db_member: DbMember) -> bool {
        if self.has_member_id(db_member.inner().member_id) {
            return false;
        }
        if self.has_member(db_member.inner().name()) {
            return false;
        }
        for a in db_member.inner().aliases() {
            if self.has_member(a) {
                return false;
            }
        }
        let mut state = self.state.borrow_mut();
        state.array.push(db_member.clone());
        state
            .map
            .insert(db_member.inner().name().to_string(), db_member.clone());
        self.add_member_aliases(&db_member);
        true
    }

    //mp remove_member_aliases
    pub fn remove_member_aliases(&self, db_member: &DbMember) {
        for a in db_member.inner().aliases() {
            if self.state.borrow().map.contains_key(a) {
                self.state.borrow_mut().map.remove(a);
            }
        }
    }

    //mp add_member_aliases
    pub fn add_member_aliases(&self, db_member: &DbMember) {
        for a in db_member.inner().aliases() {
            if !self.state.borrow().map.contains_key(a) {
                self.state
                    .borrow_mut()
                    .map
                    .insert(a.into(), db_member.clone());
            }
        }
    }

    //ap has_member
    pub fn has_member(&self, name: &str) -> bool {
        self.state.borrow().map.contains_key(name)
    }

    //ap get_member
    pub fn get_member(&self, name: &str) -> Option<DbMember> {
        if name.chars().all(|c| c.is_digit(10)) {
            match name.parse::<usize>() {
                Ok(n) => {
                    return self.get_member_id(n.into());
                }
                _ => (),
            }
        }
        self.state.borrow().map.get(name).cloned()
    }

    //ap has_member_id
    pub fn has_member_id(&self, id: usize) -> bool {
        self.state
            .borrow()
            .array
            .iter()
            .any(|a| a.inner().member_id == id)
    }

    //ap get_member_id
    pub fn get_member_id(&self, id: usize) -> Option<DbMember> {
        self.state
            .borrow()
            .array
            .iter()
            .find(|a| a.inner().member_id == id)
            .cloned()
    }

    //zz All done
}

//ip Serialize for DbMembers
impl Serialize for DbMembers {
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
