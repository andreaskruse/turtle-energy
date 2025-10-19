use chrono_tz;
use csv;
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),
    ParseInput(String),

    #[from]
    TimeZoneError(chrono_tz::ParseError),

    #[from]
    DateTimeError(chrono::ParseError),

    #[from]
    CsvError(csv::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
