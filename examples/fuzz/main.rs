use passgen::password::PasswordGenerator;
use smc::{Smc, SmcResult};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    let sensors: Vec<String> = smc.iter().map(|sensor| sensor.name()).collect();

    //not exaustive atm, just random search
    //also doesnt handle spaces
    let passgen = PasswordGenerator::new(4)
        .symbols(true)
        .numbers(true)
        .uppercase(true)
        .lowercase(true)
        .begin_with_letter(false);

    loop {
        let name = passgen.generate().unwrap();

        //if this is a known sensor, just end this iteration
        if sensors.contains(&name) {
            continue;
        }

        //if its not a valid sensor, end this iteration
        let sensor = match smc.get_sensor_by_name(&name) {
            Ok(v) => v,
            Err(_e) => continue,
        };

        //sometimes they seem to be valid, but dont get values?
        match sensor.read() {
            Ok(v) => println!("found {} {}", sensor.name(), v),
            Err(_e) => println!("found {}", sensor.name()),
        };
    }
}
