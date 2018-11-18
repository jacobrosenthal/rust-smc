use strum::IntoEnumIterator;

use rust_smc::{Key, Kind, Smc, SmcResult, Subsystem};

fn main() -> SmcResult<()> {
    let mut smc = Smc::new()?;

    let iter = Key::iter()
        .filter(|key| key.subsystem() == Subsystem::Cpu && key.kind() == Kind::Temperature);

    for key in iter {
        let value = smc.read_key(&key)?;
        println!("{:?}, {}", key, value);
    }

    Ok(())
}
