use crate::general::{translate, Kind, Subsystem};
use std::str::FromStr;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};

//cant use display as then Custom's name cant be matched
#[derive(EnumIter, Debug, EnumProperty)]
pub enum Key {
    //cant &str here sadly as we need a lifetime and EnumIter can't handle that case
    Custom(String, Kind, Subsystem),
    #[strum(props(Name = "TCXC", Kind = "Temperature", Subsystem = "Cpu"))]
    TCXC,
    #[strum(props(Name = "TC0P", Kind = "Temperature", Subsystem = "Cpu"))]
    TC0P,
    #[strum(props(Name = "TM0P", Kind = "Temperature", Subsystem = "Cpu"))]
    TM0P,
    #[strum(props(Name = "TC0H", Kind = "Temperature", Subsystem = "Cpu"))]
    TC0H,
    #[strum(props(Name = "TC0D", Kind = "Temperature", Subsystem = "Cpu"))]
    TC0D,
    #[strum(props(Name = "TC0E", Kind = "Temperature", Subsystem = "Cpu"))]
    TC0E,
    #[strum(props(Name = "TC0F", Kind = "Temperature", Subsystem = "Cpu"))]
    TC0F,
    #[strum(props(Name = "TC1C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1C,
    #[strum(props(Name = "TC2C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC2C,
    #[strum(props(Name = "TC3C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC3C,
    #[strum(props(Name = "TC4C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC4C,
    #[strum(props(Name = "TC5C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC5C,
    #[strum(props(Name = "TC6C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC6C,
    #[strum(props(Name = "TC7C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC7C,
    #[strum(props(Name = "TC8C", Kind = "Temperature", Subsystem = "Cpu"))]
    TC8C,
    #[strum(props(Name = "TCAH", Kind = "Temperature", Subsystem = "Cpu"))]
    TCAH,
    #[strum(props(Name = "TCAD", Kind = "Temperature", Subsystem = "Cpu"))]
    TCAD,
    #[strum(props(Name = "TC1P", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1P,
    #[strum(props(Name = "TC1H", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1H,
    #[strum(props(Name = "TC1D", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1D,
    #[strum(props(Name = "TC1E", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1E,
    #[strum(props(Name = "TC1F", Kind = "Temperature", Subsystem = "Cpu"))]
    TC1F,
    #[strum(props(Name = "TCBH", Kind = "Temperature", Subsystem = "Cpu"))]
    TCBH,
    #[strum(props(Name = "TCBD", Kind = "Temperature", Subsystem = "Cpu"))]
    TCBD,
    #[strum(props(Name = "TCSC", Kind = "Temperature", Subsystem = "SystemAgent"))]
    TCSC,
    #[strum(props(Name = "TCSA", Kind = "Temperature", Subsystem = "SystemAgent"))]
    TCSA,
    #[strum(props(Name = "TCGC", Kind = "Temperature", Subsystem = "Gpu"))]
    TCGC,
}

impl Key {
    pub fn name(&self) -> &str {
        match self {
            Key::Custom(name, _kind, _subsystem) => name,
            //explicit fail if someone fat fingers strum strings
            _ => self.get_str("Name").unwrap(),
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Key::Custom(name, _kind, _subsystem) => {
                assert_eq!(4, name.len());
                translate(name)
            }
            _ => translate(&self.name()),
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Key::Custom(_name, kind, _subsystem) => kind.clone(),
            _ => {
                //explicit fail if someone fat fingers strum strings
                let kind = self.get_str("Kind").unwrap();
                Kind::from_str(kind).unwrap()
            }
        }
    }

    pub fn subsystem(&self) -> Subsystem {
        match self {
            Key::Custom(_name, _kind, subsystem) => subsystem.clone(),
            _ => {
                //explicit fail if someone fat fingers strum strings
                let subsystem = self.get_str("Subsystem").unwrap();
                Subsystem::from_str(subsystem).unwrap()
            }
        }
    }
}
