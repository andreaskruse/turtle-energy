use chrono_tz;
use csv;
use derive_more::{From, Display};
use std::{num::{ParseFloatError, ParseIntError}, str::ParseBoolError};

#[derive(Debug, From, Display)]
pub enum Error {
    Custom(String),
    ParseInput(String),
    NoData(String),

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

// Calculation errors using a binary system.
pub const CURVE_NOT_LONG_ENOUGH: u32 = 1 << 1;
pub const CURVE_START_AFTER_DELIVERY: u32 = 1 << 2;
pub const NON_CONTINUOUS: u32 = 1 << 3;
pub const MISSING_INTEREST_RATE: u32 = 1 << 4;
pub const MISSING_FX_RATE: u32 = 1 << 5;
