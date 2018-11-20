use strum::IntoEnumIterator;

use smc::{Key, Kind, Smc, SmcResult, Subsystem};

fn main() -> SmcResult<()> {
    let mut smc = Smc::new()?;

    let iter = Smc
        .iter()
        .filter(|key| key.subsystem() == Subsystem::Cpu && key.kind() == Kind::Temperature);

    for sensor in iter {
        let value = sensor.read()?;
        println!("{:?}, {}", sensor.name, sensor.value);
    }

    Ok(())
}
