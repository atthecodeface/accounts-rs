//a Imports
use serde::{Deserialize, Serialize, Serializer};

use crate::DbItemKind;

//a RelatedParty
//tp RelatedParty
/// Somebody with whom the operation does business
///
/// This may be a member, a supplier, etc
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct RelatedParty {
    name: String,
}

//ip RelatedParty
impl RelatedParty {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

//tp DbRelatedParty
crate::make_db_item!(DbRelatedParty, RelatedParty);

//a DbRelatedParties
//tp DbRelatedParties
/// All the related parties in the database
#[derive(Debug)]
pub struct DbRelatedParties {
    array: Vec<DbRelatedParty>,
}

//ip Default for DbRelatedParties
impl Default for DbRelatedParties {
    fn default() -> Self {
        Self::new()
    }
}

//ip DbRelatedParties
impl DbRelatedParties {
    //cp new
    pub fn new() -> Self {
        let array = vec![];
        Self { array }
    }

    //mp add_party
    pub fn add_party(&mut self, db_related_party: DbRelatedParty) -> bool {
        if self.has_party(&db_related_party.inner().name) {
            return false;
        }
        self.array.push(db_related_party.clone());
        true
    }

    //ap has_party
    pub fn has_party(&self, name: &str) -> bool {
        self.array.iter().any(|a| a.inner().name == name)
    }

    //ap get_party
    pub fn get_party(&self, name: &str) -> Option<&DbRelatedParty> {
        self.array.iter().find(|a| a.inner().name == name)
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
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
        for db_acc in self.array.iter() {
            seq.serialize_element(&*db_acc.inner())?;
        }
        seq.end()
    }
}
