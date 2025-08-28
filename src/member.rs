//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::indexed_vec::Idx;
use crate::{Date, DbId};

//a Member
//tp Member
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Member {
    name: String,
    member_id: usize,
    address: String,
    last_gift_aid: Option<Date>,
    account_descrs: Vec<String>,
    aliases: Vec<String>,
}

//ip Member
impl Member {
    pub fn new(name: String, member_id: usize) -> Self {
        Self {
            name,
            member_id,
            address: "".into(),
            aliases: vec![],
            last_gift_aid: None,
            account_descrs: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn member_id(&self) -> usize {
        self.member_id
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn change_name<I: Into<String>>(&mut self, i: I) {
        self.name = i.into();
    }

    pub fn add_alias<I: Into<String>>(&mut self, i: I) {
        self.aliases.push(i.into());
    }

    pub fn clear_aliases(&mut self) {
        self.aliases.clear();
    }

    pub fn change_address<I: Into<String>>(&mut self, i: I) {
        self.address = i.into();
    }

    pub fn add_account_descr<I: Into<String>>(&mut self, i: I) {
        self.account_descrs.push(i.into());
    }

    pub fn account_descrs<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.account_descrs.iter().map(|a| a.as_str())
    }

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
        for a in db_member.inner().aliases() {
            state.map.insert(a.clone(), db_member.clone());
        }
        true
    }

    //ap has_member
    pub fn has_member(&self, name: &str) -> bool {
        self.state.borrow().map.contains_key(name)
    }

    //ap get_member
    pub fn get_member(&self, name: &str) -> Option<DbMember> {
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
