use passgen::password::PasswordGenerator;
use smc::{Key, Kind, Smc, SmcResult, Subsystem};
use std::str::FromStr;

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    //I dont think space is a character in passgen and I know there are key that have a space as fourth char
    let passgen = PasswordGenerator::new(4)
        .symbols(false)
        .numbers(true)
        .uppercase(true)
        .lowercase(true)
        .begin_with_letter(true);

    loop {
        let name = passgen.generate().unwrap();

        //if this is a known key, just end this iteration
        match Key::from_str(&name.clone()) {
            Ok(_v) => continue,
            Err(_e) => {}
        };

        let custom = Key::Custom(name, Kind::Unknown, Subsystem::Unknown);

        //if its not a key, end this iteration
        let sensor = match smc.get_sensor(custom) {
            Ok(v) => v,
            Err(_e) => continue,
        };

        //sometimes they seem to be keys, but dont get values?
        match sensor.read() {
            Ok(v) => println!("found {} {}", sensor.name(), v),
            Err(_e) => println!("found {}", sensor.name()),
        };
    }
}
