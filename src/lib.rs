mod general;
pub use crate::general::{Kind, Subsystem, Type};

mod error;
pub use crate::error::{SmcError, SmcResult};

mod smc_kit;

mod sensor;
pub use crate::sensor::Sensor;

mod smc;
pub use crate::smc::Smc;
