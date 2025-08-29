//a Imports
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::Error;

//a FileType
//tp FileType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Csv,
    Json,
    Yaml,
}

//ip Display for FileType
impl std::fmt::Display for FileType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FileType::Csv => write!(fmt, "csv"),
            FileType::Json => write!(fmt, "json"),
            FileType::Yaml => write!(fmt, "yaml"),
        }
    }
}

//ip FileType
impl FileType {
    pub fn from_filename(f: &str) -> Result<Self, Error> {
        if f.ends_with(".yaml") {
            Ok(Self::Yaml)
        } else if f.ends_with(".json") {
            Ok(Self::Json)
        } else if f.ends_with(".csv") {
            Ok(Self::Csv)
        } else {
            Err(Error::UnknownFileExtension(f.to_string()))
        }
    }
}

//a FileFormat
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    #[default]
    Array,
    Dictionary,
}
impl std::fmt::Display for FileFormat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FileFormat::Array => write!(fmt, "array"),
            FileFormat::Dictionary => write!(fmt, "dict"),
        }
    }
}
impl std::str::FromStr for FileFormat {
    type Err = Error;

    // Required method
    fn from_str(s: &str) -> Result<Self, Error> {
        if s == "array" {
            Ok(Self::Array)
        } else if s == "dict" || s == "map" || s == "dictionary" {
            Ok(Self::Dictionary)
        } else {
            Err(Error::UnknownFileFormat(s.to_string()))
        }
    }
}

//a Entity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Entity {}

//a Ordering
//tp Ordering
/// A semi-date base ordering
///
/// The default value of '0' indicates unset or unknown
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Ordering {
    /// Nominally this is a Date value +-
    value: usize,
}

impl Ordering {
    pub fn is_none(&self) -> bool {
        self.value == 0
    }
    pub fn from_usize(value: usize) -> Self {
        Self { value }
    }
}

//a Date
//tp Date
/// A Date in the system
///
/// The default value of '0' indicates unset or unknown
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Date {
    /// This is a UTC timestamp
    value: usize,
}

//ip Date
impl Date {
    //ap is_none
    pub fn is_none(&self) -> bool {
        self.value == 0
    }

    //mp as_ordering
    /// Return a usize that an be used to order (at least) 100
    /// transactions on a particular day
    pub fn as_ordering(&self) -> Ordering {
        Ordering::from_usize(self.value)
    }

    //cp parse
    pub fn parse(s: &str, _us_dm: bool) -> Result<Self, Error> {
        if let Ok(date) = NaiveDate::parse_from_str(s, "%d/%m/%Y") {
            let timestamp = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as usize;
            Ok(Self { value: timestamp })
        } else {
            Err(Error::ParseDate(s.into()))
        }
    }

    //cp parse_user
    pub fn parse_user(s: &str) -> Result<Self, Error> {
        if let Ok(date) = NaiveDate::parse_from_str(s, "%d/%m/%Y") {
            let timestamp = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as usize;
            Ok(Self { value: timestamp })
        } else {
            Err(Error::ParseDate(s.into()))
        }
    }

    //mp plus_days
    #[must_use]
    pub fn plus_days(&self, n: usize) -> Self {
        let value = self.value + 24 * 60 * 60 * n;

        Self { value }
    }

    //cp of_dmy
    #[track_caller]
    pub fn of_dmy(day: u32, month: u32, year: i32) -> Self {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let value = Utc.from_utc_datetime(&date.into()).timestamp() as usize;
        Self { value }
    }

    //ap dmy
    pub fn dmy(&self) -> (u32, u32, i32) {
        let date_time = DateTime::<Utc>::from_timestamp(self.value as i64, 0).unwrap();
        let date = date_time.naive_utc().date();
        (date.day(), date.month(), date.year())
    }
}

//ip std::fmt::Display for Date
impl std::fmt::Display for Date {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let date_time = DateTime::<Utc>::from_timestamp(self.value as i64, 0).unwrap();
        write!(fmt, "{}", date_time.format("%d/%m/%Y"),)
    }
}
