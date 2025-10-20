use turtle_energy::Result;
use turtle_energy::position::position::PositionModel;

fn main() -> Result<()> {
    let _res = PositionModel::from_csv("./test.csv")?;

    Ok(())
}
