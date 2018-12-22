#![feature(plugin)]
#![plugin(phf_macros)]

mod general;
pub use crate::general::{Kind, Subsystem, Type};

mod key;
pub use crate::key::Key;

mod error;
pub use crate::error::{SmcError, SmcResult};

mod smc_kit;

mod sensor;
pub use crate::sensor::Sensor;

mod smc;
pub use crate::smc::Smc;
