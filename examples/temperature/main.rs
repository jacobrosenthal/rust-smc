use smc::{Kind, Sensor, Smc, SmcResult, Subsystem};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    // turn iterator of keys into subset of sensors
    // sensors have nontrivial init cost so we deal in keys instead of sensors
    // as a result you may new up your sensors at your convenience and may wish to keep them around
    let iter = smc
        .iter()
        .filter(|key| key.subsystem() == Subsystem::Cpu && key.kind() == Kind::Temperature)
        // .map(|key| smc.get_sensor(key));
        .map(|key| Sensor::new(key, &smc));

    for sensor in iter {
        let value = sensor.read()?;
        println!("{:?}, {}", sensor.name(), value);
    }

    Ok(())
}
