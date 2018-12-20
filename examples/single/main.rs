use smc::{Key, Smc, SmcResult};
use std::{thread, time};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    let sensor = smc.get_sensor(Key::TCXC)?;

    loop {
        println!("{:?}, {}", sensor.name(), sensor.read()?);

        thread::sleep(time::Duration::from_millis(10000));

        //back up and clear the line so we write over the top of it
        print!("\x1b[1A\x1b[K");
    }
}
