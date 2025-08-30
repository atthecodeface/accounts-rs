//a Imports
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use regex::Regex;
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

//a Date, DateRange
//tp DateRange
#[derive(Debug, Default, Clone, Copy)]
pub struct DateRange {
    start: Date,
    end: Date,
}

//ip DateRange
impl DateRange {
    pub fn validate(self) -> Self {
        if self.start.is_none() {
            Self::default()
        } else if self.end.is_none() {
            Self {
                start: self.start,
                end: self.start.plus_days(1),
            }
        } else if self.end <= self.start {
            Self::default()
        } else {
            self
        }
    }
    pub fn is_empty(&self) -> bool {
        self.start.is_none()
    }
    pub fn len(&self) -> usize {
        self.end.value - self.start.value
    }
    pub fn contains(&self, date: Date) -> bool {
        if date.is_none() {
            false
        } else if self.is_empty() {
            false
        } else {
            ((self.start.value)..(self.end.value)).contains(&date.value)
        }
    }
    pub fn start(&self) -> Date {
        self.start
    }
    pub fn end(&self) -> Date {
        self.end
    }
}

//ip From<Date> for DateRange
impl From<Date> for DateRange {
    fn from(start: Date) -> DateRange {
        (Self {
            start: start,
            end: Date::default(),
        })
        .validate()
    }
}

//ip From<(Date, Date)> for DateRange
impl From<(Date, Date)> for DateRange {
    fn from((start, end): (Date, Date)) -> DateRange {
        (Self { start, end }).validate()
    }
}

//ip From<Option<Date>> for DateRange
impl From<Option<Date>> for DateRange {
    fn from(start: Option<Date>) -> DateRange {
        let start = start.unwrap_or_default();
        (Self {
            start,
            end: Date::default(),
        })
        .validate()
    }
}

//ip std::fmt::Display for DateRange
impl std::fmt::Display for DateRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_empty() {
            write!(fmt, "<no dates>")
        } else {
            write!(fmt, "{} to {}", self.start, self.end)
        }
    }
}

//tp Date
/// A Date in the system
///
/// The default value of '0' indicates unset or unknown
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Date {
    /// This is a UTC timestamp / 24*60*60
    value: usize,
}

//ip From<i64> for Date
impl From<i64> for Date {
    fn from(timestamp: i64) -> Date {
        let value = (timestamp / 60 / 60 / 24) as usize;
        Self { value }
    }
}

//ip From<DateTime<Utc>> for Date
impl From<DateTime<Utc>> for Date {
    fn from(utc: DateTime<Utc>) -> Date {
        utc.timestamp().into()
    }
}

//ip From<Date> for DateTime<Utc>
impl From<Date> for DateTime<Utc> {
    #[track_caller]
    fn from(date: Date) -> DateTime<Utc> {
        if date.is_none() {
            DateTime::<Utc>::from_timestamp(0, 0).unwrap()
        } else {
            DateTime::<Utc>::from_timestamp((date.value * 24 * 60 * 60) as i64, 0).unwrap()
        }
    }
}

//ip From<&Date> for DateTime<Utc>
impl From<&Date> for DateTime<Utc> {
    #[track_caller]
    fn from(date: &Date) -> DateTime<Utc> {
        (*date).into()
    }
}

//ip Date
impl Date {
    //ap is_none
    pub fn is_none(&self) -> bool {
        self.value == 0
    }

    //cp parse
    pub fn parse(s: &str) -> Result<Self, Error> {
        let r1 = Regex::new(r"^([0-9]{1,2})/([0-9]{1,2})/([0-9]{1,2})$").unwrap();
        let r2 = Regex::new(r"^([0-9]{1,2})/([0-9]{1,2})/([0-9]{4,4})$").unwrap();
        let r3 = Regex::new(r"^([0-9]{1,2})/([0-9]{1,2})$").unwrap();
        let r4 = Regex::new(r"^([0-9]{1,2})/([0-9]{4,4})$").unwrap();
        if let Some(c) = r1.captures(s) {
            let day = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let month = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let year = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
            Ok(Self::of_dmy(day, month, year))
        } else if let Some(c) = r2.captures(s) {
            let day = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let month = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let year = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
            Ok(Self::of_dmy(day, month, year))
        } else if let Some(c) = r3.captures(s) {
            let month = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let year = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Ok(Self::of_dmy(1, month, year))
        } else if let Some(c) = r4.captures(s) {
            let month = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let year = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Ok(Self::of_dmy(1, month, year))
        } else {
            Err(Error::ParseDate(s.into()))
        }
    }

    //mp plus_days
    #[must_use]
    pub fn plus_days(&self, n: usize) -> Self {
        if self.is_none() {
            *self
        } else {
            Self {
                value: self.value + n,
            }
        }
    }

    //cp of_dmy
    #[track_caller]
    pub fn of_dmy(day: u32, month: u32, year: i32) -> Self {
        let year = {
            if year < 90 {
                year + 2000
            } else if year < 100 {
                year + 1900
            } else {
                year
            }
        };
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let date = Utc.from_utc_datetime(&date.into()).into();
        date
    }

    //ap dmy
    pub fn dmy(&self) -> (u32, u32, i32) {
        let date_time: DateTime<Utc> = self.into();
        let date = date_time.naive_utc().date();
        (date.day(), date.month(), date.year())
    }
}

//ip std::fmt::Display for Date
impl std::fmt::Display for Date {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_none() {
            write!(fmt, "<None>")
        } else {
            let date_time: DateTime<Utc> = self.into();
            write!(fmt, "{}", date_time.format("%d/%m/%Y"),)
        }
    }
}
