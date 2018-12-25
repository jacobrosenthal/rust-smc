use crate::error::SmcResult;
use crate::general::Kind;
use crate::smc::Smc;
use crate::smc_kit::SMCKeyData_keyInfo_t;

use std::fmt;

pub struct Sensor<'a> {
    smc: &'a Smc,
    key: u32,
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

        write!(f, "{} {} {:.2}", kind, self.name(), reading,)
    }
}

impl<'a> Sensor<'a> {
    pub(crate) fn new(key: u32, smc: &'a Smc) -> SmcResult<Sensor<'a>> {
        Ok(Sensor {
            smc: &smc,
            key,
            key_info: smc.get_key_info(key)?,
        })
    }

    pub fn name(&self) -> String {
        let array = self.key.to_be_bytes();
        let name = std::str::from_utf8(&array).unwrap();
        name.to_string()
    }

    pub fn read(&self) -> SmcResult<f32> {
        self.smc.read(self.key, self.key_info)
    }

    pub fn kind(&self) -> Kind {
        let array = self.key.to_be_bytes();

        match array[0] {
            84 => Kind::Temperature,
            70 => Kind::Fan,
            86 => Kind::Voltage,
            73 => Kind::Current,
            80 => Kind::Power,
            _ => Kind::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct SensorIter<'a> {
    smc: &'a Smc,
    index: usize,
    size: usize,
}

impl<'a> SensorIter<'a> {
    pub(crate) fn new(smc: &'a Smc) -> SmcResult<Self> {
        let size = smc.get_key_count()? as usize;

        Ok(Self {
            smc: &smc,
            size,
            index: 0,
        })
    }
}

impl<'a> Iterator for SensorIter<'a> {
    type Item = Sensor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let index = self.index as u32;

            self.index = self.index + 1;

            let sensor = self.smc.get_sensor_by_index(index).unwrap();

            Some(sensor)
        } else {
            None
        }
    }
}
