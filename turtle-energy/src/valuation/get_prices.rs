use crate::price::process_curve;
use crate::structs::{CurvePoint, Delivery, Price, Result};
use std::sync::mpsc;
use std::{mem, sync::Arc, thread};

/// Compute the average price for each delivery in the deliveries.
/// Deliveries and the curve are assumed to be sorted, and the curve is
/// assumed continuous.
pub fn get_prices(deliveries: &[Delivery], curve: &[Price]) -> Vec<Result> {
    let prices_len = curve.len();
    let mut out: Vec<Result> = Vec::with_capacity(deliveries.len());
    let mut idx_price = 0;
    let mut deliv_len: f64;
    let mut total_overlap: f64;
    let mut price: f64;
    let mut value: f64;
    let mut error_code: u32;
    for d in deliveries {
        deliv_len = d.1 - d.0;
        total_overlap = 0.0;
        price = 0.0;
        value = 0.0;
        error_code = 0;

        // Could make a check for d==d_lag and reuse price, update value. Maybe faster?
        // Jump back to immediately before delivery start.
        while &curve[idx_price].start > &d.0 {
            idx_price -= 1;
        }

        for p in &curve[idx_price..prices_len] {
            // If the price ends before delivery start, move to next:
            if p.end <= d.0 {
                continue;
            }

            // Compute the overlap
            let overlap = p.end.min(d.1) - d.0.max(p.start);
            total_overlap += overlap;

            price += p.value * (overlap / deliv_len);
            value += price * d.2;
            error_code |= p.error_code;

            // Done with this delivery.
            if p.end >= d.1 {
                break;
            }
        }

        // Check for coding mistakes. Can be removed once we are
        // absolutely sure it works in all cases.
        assert_eq!(
            total_overlap, deliv_len,
            "Is data sorted? Overlap does not match delivery length: {} != {}",
            total_overlap, deliv_len
        );

        out.push((price, value, error_code));
    }

    return out;
}

pub fn get_prices_rs_mpsc(deliveries: Vec<Delivery>, prices: Vec<CurvePoint>) -> Vec<Result> {
    if deliveries.len() == 0 {
        panic!("deliveries is empty")
    }

    let curve = process_curve::preprocess_curve(&prices);

    let threads = thread::available_parallelism().unwrap().get();
    let size: usize = deliveries.len();
    let chunk_size = size / threads;
    let loop_size: Vec<(usize, usize)> = (1..=threads)
        .map(|x| (x - 1 * chunk_size, x * chunk_size))
        .collect();

    let shared_deliveries = Arc::new(deliveries);
    let shared_curve = Arc::new(curve);
    let (sender, receiver) = mpsc::channel();
    for (s, e) in loop_size {
        let shared_deliveries = Arc::clone(&shared_deliveries);
        let shared_curve = Arc::clone(&shared_curve);

        let sender = sender.clone();

        thread::spawn(move || {
            let res = get_prices(&shared_deliveries[s..e], &shared_curve);
            sender.send(res).unwrap();
        });
    }

    mem::drop(sender);

    let mut out = Vec::with_capacity(size);
    for received in receiver {
        out.extend(received);
    }
    out
}

pub fn get_prices_rs_mt(deliveries: Vec<Delivery>, prices: Vec<CurvePoint>) -> Vec<Result> {
    if deliveries.len() == 0 {
        panic!("deliveries is empty")
    }

    let curve = process_curve::preprocess_curve(&prices);

    // let threads = thread::available_parallelism().unwrap().get();
    let size: usize = deliveries.len();
    let shared_deliveries = Arc::new(deliveries);
    let shared_curve = Arc::new(curve);
    let loop_size: [(usize, usize); 7] = [
        (0, 150000),
        (150000, 300000),
        (300000, 450000),
        (450000, 600000),
        (600000, 750000),
        (750000, 900000),
        (900000, 999999),
    ];

    let mut handles = Vec::new();
    for (s, e) in loop_size {
        let shared_deliveries = Arc::clone(&shared_deliveries);
        let shared_curve = Arc::clone(&shared_curve);

        let handle = thread::spawn(move || {
            let res = get_prices(&shared_deliveries[s..e], &shared_curve);
            res
        });
        handles.push(handle);
    }

    let mut out = Vec::with_capacity(size);
    for handle in handles {
        let chunk_res = handle.join().unwrap();

        out.extend(chunk_res);
    }
    out
}
