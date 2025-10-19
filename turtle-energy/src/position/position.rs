use std::str::FromStr;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::Result;
use crate::position::enums::{CalculationModel, CalculationType, PriceUnit, QuantityUnit};
use crate::utils::point::Point;

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
    pub cvp: String,
    pub mvp: String,
    pub calculation_type: CalculationType,
    pub calculation_model: CalculationModel,
}

pub struct PositionError {
    pub position_id: u64,
    pub position_leg: u8,
    pub error: String,
}

impl Position {
    pub fn new(
        position_id: u64,
        position_leg: u8,
        delivery_start: String,
        delivery_end: String,
        quantity: f32,
        quanity_unit: String,
        fixed_price_p0: f32,
        price_unit: String,
        variable_price_flag: bool,
        cvp: String,
        mvp: String,
        calculation_type: String,
        calculation_model: String,
        time_zone: String,
    ) -> Result<Self> {
        let delivery_start_p = Point::new(&delivery_start, &time_zone)?;
        let delivery_end_p = Point::new(&delivery_end, &time_zone)?;
        let quantity_unit_e = QuantityUnit::from_str(&quanity_unit)?;
        let price_unit_e = PriceUnit::from_str(&price_unit)?;
        let calculation_type = CalculationType::from_str(&calculation_type)?;
        let calculation_model = CalculationModel::from_str(&calculation_model)?;

        return Ok(Position {
            position_id: position_id,
            position_leg: position_leg,
            delivery_start: delivery_start_p,
            delivery_end: delivery_end_p,
            quantity: quantity,
            quanity_unit: quantity_unit_e,
            fixed_price_p0: fixed_price_p0,
            price_unit: price_unit_e,
            variable_price_flag: variable_price_flag,
            cvp: cvp,
            mvp: mvp,
            calculation_type: calculation_type,
            calculation_model: calculation_model,
        });
    }

    pub fn split_to_months(&self, _split_dt: DateTime<Tz>) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_to_months() {
        let raw_data = (
            1,
            1,
            String::from("2025-01-02 00:00:00Z"),
            String::from("2026-01-12 00:00:00Z"),
            100.0,
            String::from("MWH"),
            10.0,
            String::from("EUR"),
            true,
            String::from("DK1"),
            String::from("DK1"),
            String::from("FORWARD"),
            String::from("STANDARD"),
            String::from("Europe/Copenhagen"),
        );

        let _position = Position::new(
            raw_data.0,
            raw_data.1,
            raw_data.2,
            raw_data.3,
            raw_data.4,
            raw_data.5,
            raw_data.6,
            raw_data.7,
            raw_data.8,
            raw_data.9,
            raw_data.10,
            raw_data.11,
            raw_data.12,
            raw_data.13,
        );
    }
}
