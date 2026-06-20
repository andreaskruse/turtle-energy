use std::{collections::HashMap, fs::File};
use csv;

use crate::error::{Error, Result, CURVE_START_AFTER_DELIVERY, CURVE_NOT_LONG_ENOUGH, NON_CONTINUOUS};

pub type RawPrice = (f64, f64, f64);
pub struct Price {
    pub start: f64,
    pub end: f64,
    pub value: f64,
    pub error_code: u32,
}

pub struct PriceCache<T>
where T: ReadPrices
{
    pub energy_spot: HashMap<u32, Vec<Price>>,
    pub energy_fwd: HashMap<u32, Vec<Price>>,
    pub fx_spot: HashMap<u32, Vec<Price>>,
    pub fx_fwd: HashMap<u32, Vec<Price>>,
    pub price_getter: T,
}


pub trait ReadPrices {
    fn fetch_energy_fwd() -> HashMap<u32, Vec<Price>>;
    fn fetch_energy_spot() -> HashMap<u32, Vec<Price>>;
    fn fetch_fx_spot() -> HashMap<u32, Vec<Price>>;
    fn fetch_fx_fwd() -> HashMap<u32, Vec<Price>>;
    fn fetch_ir() -> HashMap<u32, Vec<Price>>;
}

struct PriceGetterCsv;
impl ReadPrices for PriceGetterCsv {
    fn fetch_energy_fwd() -> HashMap<u32, Vec<Price>> {
        todo!();
    }
    fn fetch_energy_spot() -> HashMap<u32, Vec<Price>> {
        todo!();
    }
    fn fetch_fx_spot() -> HashMap<u32, Vec<Price>> {
        todo!();
    }
    fn fetch_fx_fwd() -> HashMap<u32, Vec<Price>> {
        todo!();
    }
    fn fetch_ir() -> HashMap<u32, Vec<Price>> {
        todo!();
    }
}

pub fn from_csv(filename: &str) -> Result<Vec<RawPrice>> {
    let file = File::open(filename).unwrap();
    let mut reader = csv::Reader::from_reader(file);

    let expected_headers: [&'static str; 3] = ["start", "end", "value"];
    let csv_headers = reader.headers()?;
    for i in 0..expected_headers.len() {
        if expected_headers[i] != &csv_headers[i] {
            return Err(Error::ParseInput(format!(
                "Wrong CSV format for prices, expected {:?}",
                expected_headers
            )));
        }
    }

    let mut out: Vec<RawPrice> = Vec::new();
    for row in reader.records() {
        let record = row?;

        let start = &record[0].parse::<f64>()?;
        let end = &record[1].parse::<f64>()?;
        let value =&record[1].parse::<f64>()?;
    }

    Ok(out)
}

fn parse_curve(prices: Vec<RawPrice>) -> Result<Vec<Price>> {
    let mut out = Vec::with_capacity(prices.len() + 2);
    let first_price = match prices.get(0) {
        Some(p) => {
                Price{
                    start: 0.0,
                    end: p.0,
                    value: p.2,
                    error_code: CURVE_START_AFTER_DELIVERY,
                }
        }
        None => return Err(Error::NoData(String::from("No prices")))
    };

    let mut last_end = first_price.end;
    let mut last_value = first_price.value;
    out.push(first_price);
    for p in prices {
        if last_end != p.0 {
            out.push(Price { start: last_end, end: p.0, value: last_value, error_code: NON_CONTINUOUS })
        }
        out.push(Price { start: p.0, end: p.1, value: p.2, error_code: 0 });

        last_end = p.1;
        last_value = p.2;
    }
    out.push(Price { start: last_end, end: 30000000000.0, value: last_value, error_code: CURVE_NOT_LONG_ENOUGH });

    Ok(out)
}
