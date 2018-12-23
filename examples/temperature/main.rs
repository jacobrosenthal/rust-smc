use smc::{Kind, Smc, SmcResult};
use std::{thread, time};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    // sensors have nontrivial newup cost so you may wish to keep these around
    let sensors = smc.find(|key| key.kind() == Kind::Temperature);

    let count = sensors.clone().count();
    loop {
        for sensor in sensors.clone() {
            println!("{}", sensor);
        }

        thread::sleep(time::Duration::from_millis(1000));

        for _x in 0..count {
            //back up and clear the line so we write over the top of it
            print!("\x1b[1A\x1b[K");
        }
    }
}
