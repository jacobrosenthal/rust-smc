use smc::{Smc, SmcResult};

fn main() -> SmcResult<()> {
    let smc = Smc::new()?;

    for sensor in smc.iter() {
        println!("{}", sensor);
    }

    Ok(())
}
