use pyo3::prelude::*;

type DeliveryTuple = (f64, f64);
type PriceTuple = (f64, f64, f64, u32);
type ResultTuple = (f64, u32);

#[pyfunction]
pub fn get_prices(deliveries: Vec<DeliveryTuple>, prices: Vec<PriceTuple>) -> Vec<ResultTuple> {
    let mut deliv_len: f64;
    let mut total_overlap: f64;

    let mut error_code: u32 = 0;
    let mut price: f64 = 0.0;

    let mut last_p_idx: usize = 0;
    let mut p_idx: usize = 0;
    let mut last_d_s: f64 = 0.0;
    let mut last_d_e: f64 = 0.0;

    let curve_l: usize = prices.len();
    let mut out: Vec<ResultTuple> = Vec::with_capacity(deliveries.len());
    for d in deliveries {
        if last_d_s == d.0 && last_d_e == d.1 {
            out.push((price, error_code));
            continue;
        }

        deliv_len = d.1 - d.0;
        total_overlap = 0.0;
        price = 0.0;
        error_code = 0;

        // Jump back to the starting point of the last delivery.
        if prices[p_idx].0 > d.0 {
            p_idx = last_p_idx;
        }

        // Jump to the next price. Can we make the code prettier?
        for p in &prices[p_idx..curve_l] {
            if p.1 <= d.0 {
                p_idx += 1;
            } else {
                break;
            }
        }
        last_p_idx = p_idx;

        for p in &prices[p_idx..curve_l] {
            // Compute the overlap
            let overlap = p.1.min(d.1) - d.0.max(p.0);
            total_overlap += overlap;

            price += p.2 * (overlap / deliv_len);
            error_code |= p.3;

            // Done with this delivery.
            if p.1 >= d.1 {
                break;
            }
        }

        // Check for coding mistakes.
        assert_eq!(
            total_overlap, deliv_len,
            "Is data sorted? Overlap does not match delivery length: {} != {}",
            total_overlap, deliv_len
        );

        last_d_s = d.0;
        last_d_e = d.1;

        out.push((price, error_code));
    }

    return out;
}
