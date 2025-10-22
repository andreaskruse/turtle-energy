use std::sync::mpsc;
use std::{mem, sync::Arc, thread};

use crate::position::naive_position::NaivePosition;
use crate::price::curve::Price;
use crate::structs::Result;

pub fn get_price(deliveries: &[NaivePosition], curve: &[Price]) -> Vec<Result> {
    let prices_len = curve.len();
    let mut out: Vec<Result> = Vec::with_capacity(deliveries.len());
    let mut idx_price = 0;
    let mut deliv_len: i64;
    let mut total_overlap: i64;
    let mut price: f64;
    let mut value: f64;
    let mut error_code: u32;
    for d in deliveries {
        deliv_len = d.delivery_end.ts - d.delivery_start.ts;
        total_overlap = 0;
        price = 0.0;
        value = 0.0;
        error_code = 0;

        // Could make a check for d==d_lag and reuse price, update value. Maybe faster?
        // Jump back to immediately before delivery start.
        while &curve[idx_price].delivery_start > &d.delivery_start.ts {
            idx_price -= 1;
        }

        for p in &curve[idx_price..prices_len] {
            // If the price ends before delivery start, move to next:
            if p.delivery_end <= d.delivery_start.ts {
                continue;
            }

            // Compute the overlap
            let overlap =
                p.delivery_end.min(d.delivery_end.ts) - d.delivery_start.ts.max(p.delivery_start);
            total_overlap += overlap;

            price += p.value * (overlap / deliv_len) as f64;
            value += price * d.quantity;
            error_code |= p.error_code;

            // Done with this delivery.
            if p.delivery_end >= d.delivery_end.ts {
                break;
            }
        }

        // Check for coding mistakes. Can be removed once we are
        // absolutely sure it works in all cases.
        // assert_eq!(
        //     total_overlap, deliv_len,
        //     "Is data sorted? Overlap does not match delivery length: {} != {}",
        //     total_overlap, deliv_len
        // );

        out.push((price, value, error_code));
    }

    return out;
}

pub fn get_prices_rs_mpsc(deliveries: Vec<NaivePosition>, prices: Vec<Price>) -> Vec<Result> {
    if deliveries.len() == 0 {
        panic!("deliveries is empty")
    }

    let threads = thread::available_parallelism().unwrap().get();
    let size: usize = deliveries.len();
    let chunk_size = size / threads;
    let loop_size: Vec<(usize, usize)> = (1..=threads)
        .map(|x| ((x - 1) * chunk_size, x * chunk_size))
        .collect();

    let shared_deliveries = Arc::new(deliveries);
    let shared_curve = Arc::new(prices);
    let (sender, receiver) = mpsc::channel();
    for (s, e) in loop_size {
        let shared_deliveries = Arc::clone(&shared_deliveries);
        let shared_curve = Arc::clone(&shared_curve);

        let sender = sender.clone();

        thread::spawn(move || {
            let res = get_price(&shared_deliveries[s..e], &shared_curve);
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
