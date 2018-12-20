use smc::{Kind, Smc, SmcResult};
use std::{thread, time};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    // sensors have nontrivial newup cost so you may wish to keep these around
    let sensors = smc.find(|key| key.kind() == Kind::Temperature);

    loop {
        for sensor in sensors.clone() {
            println!("{:?}, {}", sensor.name(), sensor.read()?);
        }
        thread::sleep(time::Duration::from_millis(10000));
    }
}
