use std::time::Instant;
use std::env;

use turtle_energy::price::curve::Curve;
use turtle_energy::valuation::get_prices::{get_price, get_prices_rs_mpsc};
use turtle_energy::{Result, Error};
use turtle_energy::position::naive_position::NaivePositions;

fn main() -> Result<()> {

    let cl_input = parse_cl_args()?;

    let now = Instant::now();
    let deliveries = NaivePositions::from_csv(&cl_input.position_file)?;
    println!("Loading positions took: {:?}", now.elapsed());

    let now = Instant::now();
    let curve = Curve::from_csv("./prices.csv")?;
    println!("Loading prices took: {:?}", now.elapsed());
    
    let now = Instant::now();
    let result1 = get_price(&deliveries.positions, &curve.prices);
    println!("Computing result took: {:?}", now.elapsed());

    let now = Instant::now();
    let result2 = get_prices_rs_mpsc(deliveries.positions, curve.prices);
    println!("Computing result (multithreaded) took: {:?}", now.elapsed());

    println!("the last result was {:?}", result1[result1.len()-1]);
    println!("the last result was {:?}", result2[result2.len()-1]);

    Ok(())
}


enum CLArgs {

}

#[derive(Debug)]
struct CommandLineInput {
    position_file: String,
    price_file: Option<String>,
}

impl CommandLineInput {
    fn new() -> Self {
        Self {position_file: String::new(), price_file: None }
    }
}

fn parse_cl_args() -> Result<CommandLineInput> {
    let mut args = env::args();
    args.next(); // skip the module name

    let mut cl_input = CommandLineInput::new();
    match args.next().as_deref() {
        Some("-f") => {
            match args.next() {
                Some(val) => {
                    cl_input.position_file = val.to_string();
                },
                None => return Err(Error::Custom(String::from("Expected a filename"))),
            }
        }
        Some(val) => return Err(Error::Custom(format!("Unexpected input: {}", val))),
        None => return Err(Error::Custom(String::from("Need a path to a .csv file with positions: -f <file>")))
    }

    Ok(cl_input)
}
