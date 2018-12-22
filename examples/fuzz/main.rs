use smc::{Key, Kind, Smc, SmcResult, Subsystem};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    let sensors = ["AAAA", "AAAB", "AAAC", "TCXC"]
        .iter()
        .map(|name| {
            let custom = Key::Custom(name.to_string(), Kind::Unknown, Subsystem::Unknown);

            smc.get_sensor(custom)
        })
        .filter_map(Result::ok);

    for sensor in sensors.clone() {
        println!("found {} {}", sensor.name(), sensor.read()?);
    }

    Ok(())
}
