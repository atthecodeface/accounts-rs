//a Imports
use serde::{Deserialize, Serialize, Serializer};

use crate::Date;

//a Member
//tp Member
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Member {
    name: String,
    member_id: usize,
    address: String,
    last_gift_aid: Option<Date>,
}

//ip Member
impl Member {
    pub fn new(name: String, member_id: usize) -> Self {
        Self {
            name,
            member_id,
            address: "".into(),
            last_gift_aid: None,
        }
    }
}

//tp DbMember
crate::make_db_item!(DbMember, Member);

//a DbMembers
//tp DbMembers
/// All the related parties in the database
#[derive(Debug)]
pub struct DbMembers {
    array: Vec<DbMember>,
}

//ip Default for DbMembers
impl Default for DbMembers {
    fn default() -> Self {
        Self::new()
    }
}

//ip DbMembers
impl DbMembers {
    //cp new
    pub fn new() -> Self {
        let array = vec![];
        Self { array }
    }

    //mp add_member
    pub fn add_member(&mut self, db_related_member: DbMember) -> bool {
        if self.has_member(&db_related_member.inner().name) {
            return false;
        }
        self.array.push(db_related_member.clone());
        true
    }

    //ap has_member
    pub fn has_member(&self, name: &str) -> bool {
        self.array.iter().any(|a| a.inner().name == name)
    }

    //ap get_member
    pub fn get_member(&self, name: &str) -> Option<&DbMember> {
        self.array.iter().find(|a| a.inner().name == name)
    }

    //ap has_member_id
    pub fn has_member_idr(&self, id: usize) -> bool {
        self.array.iter().any(|a| a.inner().member_id == id)
    }

    //ap get_member_id
    pub fn get_member_id(&self, id: usize) -> Option<&DbMember> {
        self.array.iter().find(|a| a.inner().member_id == id)
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
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
        for db_acc in self.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
