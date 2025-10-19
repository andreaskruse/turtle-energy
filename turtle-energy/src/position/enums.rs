use std::str::FromStr;

use crate::Error;

pub enum QuantityUnit {
    MWH,
    CO2TON,
}

impl FromStr for QuantityUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MWH" => Ok(QuantityUnit::MWH),
            "CO2TON" => Ok(QuantityUnit::CO2TON),
            _ => Err(Error::ParseInput(
                "Failed to parse quantity_unit".to_string(),
            )),
        }
    }
}

pub enum CalculationType {
    Forward,
    Future,
    AmericanOption,
    EuropeanOption,
    AsianOption,
}

impl FromStr for CalculationType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FORWARD" => Ok(Self::Forward),
            "FUTURE" => Ok(Self::Future),
            "AMERICANOPTION" => Ok(Self::AmericanOption),
            "EUROPEANOPTION" => Ok(Self::EuropeanOption),
            "ASIANOPTION" => Ok(Self::AsianOption),
            _ => Err(Error::ParseInput(
                "Failed to parse calculation_type".to_string(),
            )),
        }
    }
}

pub enum CalculationModel {
    Standard,
}

impl FromStr for CalculationModel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STANDARD" => Ok(Self::Standard),
            _ => Err(Error::ParseInput(
                "Failed to parse calculation_model".to_string(),
            )),
        }
    }
}

pub enum PriceUnit {
    DKK,
    EUR,
    GBP,
    AUD,
}

impl FromStr for PriceUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DKK" => Ok(Self::DKK),
            "EUR" => Ok(Self::EUR),
            "GBP" => Ok(Self::GBP),
            "AUD" => Ok(Self::AUD),
            _ => Err(Error::ParseInput("Failed to parse price_unit".to_string())),
        }
    }
}
