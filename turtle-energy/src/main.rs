use turtle_energy::Result;
use turtle_energy::position::read_positions::Positions;

fn main() -> Result<()> {
    let _res = Positions::from_csv("./test.csv");

    Ok(())
}
