use chrono::DateTime;
use chrono_tz::Tz;
use csv;
use std::fs;
use std::str::FromStr;

use crate::position::enums::{CalculationModel, CalculationType, PriceUnit, QuantityUnit};
use crate::utils::point::Point;
use crate::{Error, Result};

pub struct PositionModel {
    pub positions: Vec<Position>,
    pub errors: Vec<PositionError>,
}

impl PositionModel {
    fn _from_sql() {}

    pub fn from_csv(filename: &str) -> Result<Self> {
        let file = fs::File::open(filename).expect("Could not read .csv file with positions");
        let mut reader = csv::Reader::from_reader(file);

        let expected_headers: [&'static str; 13] = [
            "position_id",
            "position_leg",
            "delivery_start",
            "delivery_end",
            "quantity",
            "quanity_unit",
            "fixed_price_p0",
            "price_unit",
            "variable_price_flag",
            "contract_valuation_product",
            "market_valuation_product",
            "calculation_type",
            "calculation_model",
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

        let mut out: Vec<Position> = Vec::new();
        let mut errors: Vec<PositionError> = Vec::new();
        for row in reader.records() {
            let record = row?;
            match Position::from_csv(
                &record[0],
                &record[1],
                &record[2],
                &record[3],
                &record[4],
                &record[5],
                &record[6],
                &record[7],
                &record[8],
                &record[9],
                &record[10],
                &record[11],
                &record[12],
                &record[13],
            ) {
                Ok(val) => out.push(val),
                Err(err) => errors.push(PositionError::from_csv(
                    &record[0],
                    &record[1],
                    err.to_string(),
                )?),
            };
        }

        Ok(PositionModel {
            positions: out,
            errors: errors,
        })
    }
}

pub struct PositionError {
    pub position_id: u64,
    pub position_leg: u8,
    pub error: String,
}

impl PositionError {
    pub fn from_csv(position_id: &str, position_leg: &str, error: String) -> Result<Self> {
        Ok(PositionError {
            position_id: position_id
                .parse()
                .expect("position_id must be an integer (u64)"),
            position_leg: position_leg
                .parse()
                .expect("position_leg must be an integer (u8)"),
            error: error,
        })
    }
}

pub struct Position {
    pub position_id: u64,
    pub position_leg: u8,
    pub delivery_start: Point,
    pub delivery_end: Point,
    pub quantity: f32,
    pub quanity_unit: QuantityUnit,
    pub fixed_price_p0: f32,
    pub price_unit: PriceUnit,
    pub variable_price_flag: bool,
    pub contract_valuation_product: String,
    pub market_valation_product: String,
    pub calculation_type: CalculationType,
    pub calculation_model: CalculationModel,
}

impl Position {
    pub fn split_to_months(&self, _split_dt: DateTime<Tz>) {}

    pub fn from(
        position_id: u64,
        position_leg: u8,
        delivery_start: String,
        delivery_end: String,
        quantity: f32,
        quanity_unit: String,
        fixed_price_p0: f32,
        price_unit: String,
        variable_price_flag: bool,
        contract_valuation_product: String,
        market_valation_product: String,
        calculation_type: String,
        calculation_model: String,
        time_zone: String,
    ) -> Result<Self> {
        return Ok(Position {
            position_id: position_id,
            position_leg: position_leg,
            delivery_start: Point::new(&delivery_start, &time_zone)?,
            delivery_end: Point::new(&delivery_end, &time_zone)?,
            quantity: quantity,
            quanity_unit: QuantityUnit::from_str(&quanity_unit)?,
            fixed_price_p0: fixed_price_p0,
            price_unit: PriceUnit::from_str(&price_unit)?,
            variable_price_flag: variable_price_flag,
            contract_valuation_product,
            market_valation_product,
            calculation_type: CalculationType::from_str(&calculation_type)?,
            calculation_model: CalculationModel::from_str(&calculation_model)?,
        });
    }

    pub fn from_csv(
        position_id: &str,
        position_leg: &str,
        delivery_start: &str,
        delivery_end: &str,
        quantity: &str,
        quanity_unit: &str,
        fixed_price_p0: &str,
        price_unit: &str,
        variable_price_flag: &str,
        contract_valuation_product: &str,
        market_valation_product: &str,
        calculation_type: &str,
        calculation_model: &str,
        time_zone: &str,
    ) -> Result<Self> {
        return Ok(Position {
            // position_id and position_leg are required to be correct.
            position_id: position_id
                .parse()
                .expect("position_id must be an integer (u64)"),
            position_leg: position_leg
                .parse()
                .expect("position_leg must be an integer (u8)"),
            delivery_start: Point::new(&delivery_start, &time_zone)?,
            delivery_end: Point::new(&delivery_end, &time_zone)?,
            quantity: quantity.parse()?,
            quanity_unit: QuantityUnit::from_str(&quanity_unit)?,
            fixed_price_p0: fixed_price_p0.parse()?,
            price_unit: PriceUnit::from_str(&price_unit)?,
            variable_price_flag: variable_price_flag.parse()?,
            contract_valuation_product: contract_valuation_product.to_string(),
            market_valation_product: market_valation_product.to_string(),
            calculation_type: CalculationType::from_str(&calculation_type)?,
            calculation_model: CalculationModel::from_str(&calculation_model)?,
        });
    }
}
