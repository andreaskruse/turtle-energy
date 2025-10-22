use std::time::Instant;

use turtle_energy::price::curve::Curve;
use turtle_energy::valuation::get_prices::{get_price, get_prices_rs_mpsc};
use turtle_energy::Result;
use turtle_energy::position::naive_position::NaivePositions;

fn main() -> Result<()> {

    let now = Instant::now();
    let deliveries = NaivePositions::from_csv("./positions_light.csv")?;
    println!("Loading positions took: {:?}", now.elapsed());

    let now = Instant::now();
    let curve = Curve::from_csv("./prices.csv")?;
    println!("Loading prices took: {:?}", now.elapsed());
    
    let now = Instant::now();
    let result1 = get_price(&deliveries.positions, &curve.prices);
    println!("Computing result took: {:?}", now.elapsed());

    let now = Instant::now();
    let result2 = get_prices_rs_mpsc(deliveries.positions, curve.prices);
    println!("Computing result with threads took: {:?}", now.elapsed());

    println!("the last result was {:?}", result1[result1.len()-1]);
    println!("the last result was {:?}", result2[result2.len()-1]);

    Ok(())
}
