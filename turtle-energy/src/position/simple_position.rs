use chrono::NaiveDateTime;
use std::fs;

use crate::{Result, Error};

pub struct SimplePositionModel {
    pub positions: Vec<SimplePosition>,
    pub errors: Vec<SimplePositionError>,
}

impl SimplePositionModel {
    pub fn from_csv(filename: &str) -> Result<Self> {
        let file = fs::File::open(filename).expect("Could not read .csv file");
        let mut reader = csv::Reader::from_reader(file);

        let expected_headers: [&'static str; 4] = [
            "position_id",
            "delivery_start",
            "delivery_end",
            "quantity",
        ];
        let csv_headers = reader.headers()?;
        for i in 0..expected_headers.len() {
            if expected_headers[i] != &csv_headers[i] {
                return Err(Error::ParseInput(format!(
                    "Wrong CSV file format, expected {:?}",
                    expected_headers
                )));
            }
        }

        let mut out: Vec<SimplePosition> = Vec::new();
        let mut errors: Vec<SimplePositionError> = Vec::new();
        for row in reader.records() {
            let record = row?;
            match SimplePosition::from_csv(
                &record[0],
                &record[1],
                &record[2],
                &record[3],
            ) {
                Ok(val) => out.push(val),
                Err(err) => errors.push(SimplePositionError::from_csv(
                    &record[0],
                    err.to_string(),
                )?),
            };
        }

        Ok(SimplePositionModel {
            positions: out,
            errors: errors,
        })


    }
}

pub struct SimplePositionError {
    pub position_id: u64,
    pub error: String,
}

impl SimplePositionError {
    pub fn from_csv(position_id: &str, error: String) -> Result<Self> {
        Ok(SimplePositionError { position_id: position_id.parse()?, error: error })
    }
}

pub struct SimplePosition {
    pub position_id: u64,
    pub delivery_start: NaiveDateTime,
    pub delivery_end: NaiveDateTime,
    pub quantity: f64,
}

impl SimplePosition {
    pub fn from_csv(
        position_id: &str,
        delivery_start: &str,
        delivery_end: &str,
        quantity: &str,
    ) -> Result<SimplePosition> {
        Ok(SimplePosition {
            position_id: position_id.parse()?,
            delivery_start: delivery_start.parse()?,
            delivery_end: delivery_end.parse()?,
            quantity: quantity.parse()?,
        })
    }
}
