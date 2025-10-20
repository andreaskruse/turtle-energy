use crate::structs::{CurvePoint, Price};
use std::cmp;

pub fn preprocess_curve(prices: &Vec<CurvePoint>) -> Vec<Price> {
    // Ensure the curve spans the whole sample space.

    if prices.len() == 0 {
        panic!("Input curve is empty");
    }

    const CURVE_START_ERR: u32 = 2;
    const CURVE_SHORT_ERR: u32 = 4;

    let mut out: Vec<Price> = Vec::with_capacity(prices.len() + 2);

    // Ensure a delivery cannot start before the curve starts.
    out.push(Price {
        start: 0.0,
        end: prices[0].0,
        value: prices[0].2,
        error_code: CURVE_START_ERR,
    });

    for p in prices {
        out.push(Price {
            start: p.0,
            end: p.1,
            value: p.2,
            error_code: 0,
        })
    }

    // Ensure a delivery cannot end after the curve ends.
    out.push(Price {
        start: prices[prices.len() - 1].1,
        end: 30000000000.0, // year 2920
        value: prices[prices.len() - 1].2,
        error_code: CURVE_SHORT_ERR,
    });

    return out;
}

pub fn preprocess_curve_cont(prices: &Vec<CurvePoint>) -> Vec<Price> {
    // Ensure that the curve is continuous and span the
    // whole sample space.

    if prices.len() == 0 {
        panic!("Input curve is empty");
    }

    const CURVE_START_ERR: u32 = 2;
    const CURVE_SHORT_ERR: u32 = 4;
    const NON_CONTINUOUS_ERR: u32 = 8;

    // Check if it even matter to allocate a buffer for missing prices(?)
    let inp_len = prices.len();
    let len = inp_len + cmp::max(inp_len + inp_len / 5, 10);
    let mut out: Vec<Price> = Vec::with_capacity(len);

    // Ensure a delivery cannot start before the curve starts.
    out.push(Price {
        start: 0.0,
        end: prices[0].0,
        value: prices[0].2,
        error_code: CURVE_START_ERR,
    });

    // Check if the curve is continuous
    for i in prices.windows(2) {
        out.push(Price {
            start: i[0].0,
            end: i[0].1,
            value: i[0].2,
            error_code: 0,
        });
        if i[0].1 != i[1].0 {
            out.push(Price {
                start: i[0].1,
                end: i[1].0,
                value: i[0].2,
                error_code: NON_CONTINUOUS_ERR,
            });
        }
    }

    // Add the last element
    out.push(Price {
        start: prices[inp_len - 1].0,
        end: prices[inp_len - 1].1,
        value: prices[inp_len - 1].2,
        error_code: 0,
    });

    // Ensure a delivery cannot end after the curve ends.
    out.push(Price {
        start: prices[inp_len - 1].1,
        end: 30000000000.0, // year 2920
        value: prices[inp_len - 1].2,
        error_code: CURVE_SHORT_ERR,
    });

    return out;
}
