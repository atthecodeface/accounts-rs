//a Imports

use crate::Error;
use serde::{Deserialize, Serialize};

//a AccountDesc
//tp AccountDesc
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AccountDesc {
    #[default]
    None,
    Uk {
        sort_code: u32,
        account: usize,
    },
}

//ip Display for AccountDesc
impl std::fmt::Display for AccountDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            AccountDesc::Uk { sort_code, account } => {
                let sc0 = (sort_code / 10_000) % 100;
                let sc1 = (sort_code / 100) % 100;
                let sc2 = sort_code % 100;
                write!(fmt, "{sc0:02}-{sc1:02}-{sc2:02}:{account:08}")
            }
            AccountDesc::None => {
                write!(fmt, "<None>")
            }
        }
    }
}

//ip AccountDesc
impl AccountDesc {
    pub fn uk(sort_code: u32, account: usize) -> Self {
        Self::Uk { sort_code, account }
    }
    pub fn parse_uk(sc_str: &str, account: usize) -> Result<Self, Error> {
        let re = regex::Regex::new(r"^(\d{2})-(\d{2})-(\d{2})$").unwrap();
        let Some(captures) = re.captures(sc_str) else {
            return Err(Error::ParseAccount(format!("{sc_str}:{account}")));
        };
        let sc0 = captures.get(1).unwrap().as_str().parse::<u32>();
        let sc1 = captures.get(2).unwrap().as_str().parse::<u32>();
        let sc2 = captures.get(3).unwrap().as_str().parse::<u32>();
        if sc0.is_err() || sc1.is_err() || sc2.is_err() {
            Err(Error::ParseAccount(format!("{sc_str}:{account}")))
        } else {
            let sort_code = sc0.unwrap() * 10_000 + sc1.unwrap() * 100 + sc2.unwrap();
            Ok(Self::Uk { sort_code, account })
        }
    }
}
