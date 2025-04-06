//a Imports
use serde::{Deserialize, Serialize};

use crate::Error;

//a FileType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Csv,
    Json,
    Yaml,
}
impl std::fmt::Display for FileType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FileType::Csv => write!(fmt, "csv"),
            FileType::Json => write!(fmt, "json"),
            FileType::Yaml => write!(fmt, "yaml"),
        }
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
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

//a Date
//tp Date
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
