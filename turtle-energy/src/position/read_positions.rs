use csv;
use std::fs;

use crate::Result;
use crate::position::position::Position;

pub struct Positions {
    pub position: Vec<Position>,
}

impl Positions {
    fn _from_sql() {}

    pub fn from_csv(filename: &str) -> Result<()> {
        let file = fs::File::open(filename).expect("Could not read .csv file");
        let mut reader = csv::Reader::from_reader(file);

        println!("{:?}", reader.headers());

        for row in reader.records() {
            let record = row?;
            println!("{:?}", record);
        }

        Ok(())
    }
}
