use serde::{Deserialize, Serialize};

use crate::Error;
use num_traits::cast::NumCast;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Amount {
    value: isize,
}
impl From<isize> for Amount {
    fn from(value: isize) -> Amount {
        Self { value }
    }
}
impl std::ops::Add<Amount> for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Amount {
        (self.value + other.value).into()
    }
}
impl Amount {
    fn of_f32(f: f32) -> Result<Self, Error> {
        let pennies = (f * 100.0).round();
        if let Some(value) = <isize as NumCast>::from(pennies) {
            Ok(Self { value })
        } else {
            Err(Error::ParseDate(format!(
                "parsing amount out of range '{f}'"
            )))
        }
    }
    pub fn parse(s: &str) -> Result<Self, Error> {
        if let Ok(f) = s.parse::<f32>() {
            Self::of_f32(f)
        } else {
            Err(Error::ParseDate(format!("parsing amount '{s}'")))
        }
    }
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
    pub fn value(&self) -> isize {
        self.value
    }
}
impl std::fmt::Display for Amount {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt(fmt)
    }
}
