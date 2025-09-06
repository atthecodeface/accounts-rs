//a Imports
use serde::{Deserialize, Serialize};

use crate::Error;
use num_traits::cast::NumCast;

//a Amount
//tp Amount
/// A value stored in a transaction of some form
///
/// The units are *pence* for UK transactions
///
/// This serializes as an i32
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct Amount {
    value: isize,
}

//ip From<isize> for Amount
impl From<isize> for Amount {
    fn from(value: isize) -> Amount {
        Self { value }
    }
}

//ip Add<Amount> for Amount
impl std::ops::Add<Amount> for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Amount {
        (self.value + other.value).into()
    }
}

//ip AddAssign<Amount> for Amount
impl std::ops::AddAssign<Amount> for Amount {
    fn add_assign(&mut self, other: Amount) {
        self.value = (self.value + other.value).into();
    }
}

//ip Sub<Amount> for Amount
impl std::ops::Sub<Amount> for Amount {
    type Output = Amount;
    fn sub(self, other: Amount) -> Amount {
        (self.value - other.value).into()
    }
}

//ip SubAssign<Amount> for Amount
impl std::ops::SubAssign<Amount> for Amount {
    fn sub_assign(&mut self, other: Amount) {
        self.value = (self.value - other.value).into();
    }
}

//ip Neg for Amount
impl std::ops::Neg for Amount {
    type Output = Self;
    fn neg(self) -> Self {
        (-self.value).into()
    }
}

//ip Amount
impl Amount {
    //ci of_f32
    /// Create an [Amount] from an f32 by rounding to nearest penny
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

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
    pub fn value(&self) -> isize {
        self.value
    }
}

//ip FromStr for Amount
impl std::str::FromStr for Amount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(f) = s.parse::<f32>() {
            Self::of_f32(f)
        } else {
            Err(Error::ParseDate(format!("parsing amount '{s}'")))
        }
    }
}

//ip Display for Amount
impl std::fmt::Display for Amount {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:10.2}", (self.value as f64) / 100.0)
    }
}
