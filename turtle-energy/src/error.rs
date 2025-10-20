use chrono_tz;
use csv;
use derive_more::{From, Display};
use std::{num::{ParseFloatError, ParseIntError}, str::ParseBoolError};

#[derive(Debug, From, Display)]
pub enum Error {
    Custom(String),
    ParseInput(String),

    #[from]
    ParseBoolError(ParseBoolError),

    #[from]
    ParseIntError(ParseIntError),

    #[from]
    ParseFloatError(ParseFloatError),

    #[from]
    TimeZoneError(chrono_tz::ParseError),

    #[from]
    DateTimeError(chrono::ParseError),

    #[from]
    CsvError(csv::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
