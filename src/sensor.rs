use crate::error::SmcResult;
use crate::general::{Kind, Subsystem};
use crate::key::Key;
use crate::smc::Smc;
use crate::smc_kit::SMCKeyData_keyInfo_t;

use std::fmt;

pub struct Sensor<'a> {
    smc: &'a Smc,
    key: Key,
    key_info: SMCKeyData_keyInfo_t,
}

impl<'a> fmt::Display for Sensor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut kind;

        match self.kind() {
            Kind::Temperature => kind = "🌡️".to_string(),
            Kind::Fan => kind = "💨".to_string(),
            Kind::Power => kind = "🔌".to_string(),
            Kind::Current => kind = "🚰".to_string(),
            Kind::Voltage => kind = "⚡".to_string(),
            Kind::Unknown => kind = "👽".to_string(),
        }

        let reading = self.read().unwrap_or_else(|_| 0.0);

        write!(
            f,
            "{} {} {:.2} {}",
            kind,
            self.name(),
            reading,
            self.key.detail()
        )
    }
}

impl<'a> Sensor<'a> {
    pub(crate) fn new(key: Key, smc: &'a Smc) -> SmcResult<Sensor<'a>> {
        let val = key.value();
        Ok(Sensor {
            smc: &smc,
            key: key,
            key_info: smc.read_key_info(val)?,
        })
    }

    pub fn name(&self) -> &str {
        self.key.name()
    }

    pub fn read(&self) -> SmcResult<f32> {
        self.smc.read_key(&self.key, self.key_info)
    }

    pub fn subsystem(&self) -> Subsystem {
        self.key.subsystem()
    }

    pub fn kind(&self) -> Kind {
        self.key.kind()
    }
}
