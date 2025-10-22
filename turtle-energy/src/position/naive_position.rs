use std::fs;

use crate::{utils::point::Point, Error, Result};

pub struct NaivePositions {
    pub positions: Vec<NaivePosition>,
    pub errors: Vec<NaivePositionError>,
}

impl NaivePositions {
    pub fn from_csv(filename: &str) -> Result<Self> {
        let file = fs::File::open(filename).expect("Could not read .csv file with positions");
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
                    "Wrong CSV file format for positions, expected {:?}",
                    expected_headers
                )));
            }
        }

        let mut out: Vec<NaivePosition> = Vec::new();
        let mut errors: Vec<NaivePositionError> = Vec::new();
        for row in reader.records() {
            let record = row?;
            match NaivePosition::from_csv(
                &record[0],
                &record[1],
                &record[2],
                &record[3],
            ) {
                Ok(val) => out.push(val),
                Err(err) => errors.push(NaivePositionError::from_csv(
                    &record[0],
                    err.to_string(),
                )?),
            };
        }

        println!("Successfully loaded {}/{} positions", out.len(), out.len() + errors.len());

        Ok(NaivePositions {
            positions: out,
            errors: errors,
        })


    }
}

pub struct NaivePositionError {
    pub position_id: u64,
    pub error: String,
}

impl NaivePositionError {
    pub fn from_csv(position_id: &str, error: String) -> Result<Self> {
        Ok(NaivePositionError { position_id: position_id.parse()?, error: error })
    }
}

pub struct NaivePosition {
    pub position_id: u64,
    pub delivery_start: Point,
    pub delivery_end: Point,
    pub quantity: f64,
}

impl NaivePosition {
    pub fn from_csv(
        position_id: &str,
        delivery_start: &str,
        delivery_end: &str,
        quantity: &str,
    ) -> Result<NaivePosition> {
        Ok(NaivePosition {
            position_id: position_id.parse()?,
            delivery_start: Point::new(delivery_start, "Europe/Copenhagen")?,
            delivery_end: Point::new(delivery_end, "Europe/Copenhagen")?,
            quantity: quantity.parse()?,
        })
    }
}
