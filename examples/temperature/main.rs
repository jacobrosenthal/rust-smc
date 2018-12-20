use smc::{Kind, Smc, SmcResult, Subsystem};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    // sensors have nontrivial init cost so you may wish to keep them around
    let sensors =
        smc.find(|key| key.subsystem() == Subsystem::Cpu && key.kind() == Kind::Temperature);

    for sensor in sensors {
        println!("{}", sensor);
    }

    Ok(())
}
