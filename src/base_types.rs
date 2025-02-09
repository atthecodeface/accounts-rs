use serde::{Deserialize, Serialize};

use crate::indexed_vec::StringsWithIndex;
use crate::Error;

#[derive(Debug)]
pub struct EntityTable<'et> {
    names: StringsWithIndex<'et>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Entity {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Date {
    value: usize,
}

impl Date {
    pub fn parse(s: &str, _us_dm: bool) -> Result<Self, Error> {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y") {
            let timestamp = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as usize;
            Ok(Self { value: timestamp })
        } else {
            Err(Error::ParseDate(s.into()))
        }
    }
}
