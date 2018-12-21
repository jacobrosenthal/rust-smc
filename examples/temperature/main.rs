use smc::{Kind, Smc, SmcResult};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    // sensors have nontrivial newup cost so you may wish to keep these around
    let sensors = smc.find(|key| key.kind() == Kind::Temperature);

    for sensor in sensors.clone() {
        println!("{}", sensor);
    }

    Ok(())
}
