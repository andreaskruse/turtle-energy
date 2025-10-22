use std::fs::File;
use csv;

use crate::{Result, Error};

pub struct Curve {
    pub prices: Vec<Price>,
    pub errors: Vec<ErrorPrice>
}

impl Curve {
    pub fn from_csv(filename: &str) -> Result<Curve> {
        let file = File::open(filename).expect("Could not read .csv file with prices");
        let mut reader = csv::Reader::from_reader(file);

        let expected_headers: [&'static str; 3] = [
            "delivery_start",
            "delivery_end",
            "value",
        ];
        let csv_headers = reader.headers()?;
        for i in 0..expected_headers.len() {
            if expected_headers[i] != &csv_headers[i] {
                return Err(Error::ParseInput(format!(
                    "Wrong CSV format for prices, expected {:?}",
                    expected_headers
                )));
            }
        }

        let mut out: Vec<Price> = Vec::new();
        let mut errors: Vec<ErrorPrice> = Vec::new();
        for row in reader.records() {
            let record = row?;
            match Price::from_csv(&record[0], &record[1], &record[2]) {
                Ok(val) => out.push(val),
                Err(err) => errors.push(ErrorPrice { error: err.to_string() }),
            }
        }

        println!("Successfully loaded {}/{} prices", out.len(), out.len() + errors.len());

        Ok(Curve { prices: out, errors: errors })

    }
}

pub struct ErrorPrice {
    pub error: String,
}

pub struct Price {
    pub delivery_start: i64,
    pub delivery_end: i64,
    pub value: f64,
    pub error_code: u32,
}

impl Price {
    fn from_csv(delivery_start: &str, delivery_end: &str, value: &str) -> Result<Self> {
        Ok(Price {
            delivery_start: delivery_start.parse()?,
            delivery_end: delivery_end.parse()?,
            value: value.parse()?,
            error_code: 0,
        })
    }
}
