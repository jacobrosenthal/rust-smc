use crate::general::{Kind, Subsystem};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum Key {
    TCXC,
    TC0P,
    TM0P,
    TC0H,
    TC0D,
    TC0E,
    TC0F,
    TC1C,
    TC2C,
    TC3C,
    TC4C,
    TC5C,
    TC6C,
    TC7C,
    TC8C,
    TCAH,
    TCAD,
    TC1P,
    TC1H,
    TC1D,
    TC1E,
    TC1F,
    TCBH,
    TCBD,
    TCSC,
    TCSA,
    TCGC,
}

impl Key {
    //could use strum for this, but strum only does string atm
    //and I need to define kind and subsytem etc so might as well
    //just do it this way for now
    pub fn name(&self) -> &str {
        match *self {
            Key::TCXC => "TCXC",
            Key::TC0P => "TC0P",
            Key::TM0P => "TM0P",
            Key::TC0H => "TC0H",
            Key::TC0D => "TC0D",
            Key::TC0E => "TC0E",
            Key::TC0F => "TC0F",
            Key::TC1C => "TC1C",
            Key::TC2C => "TC2C",
            Key::TC3C => "TC3C",
            Key::TC4C => "TC4C",
            Key::TC5C => "TC5C",
            Key::TC6C => "TC6C",
            Key::TC7C => "TC7C",
            Key::TC8C => "TC8C",
            Key::TCAH => "TCAH",
            Key::TCAD => "TCAD",
            Key::TC1P => "TC1P",
            Key::TC1H => "TC1H",
            Key::TC1D => "TC1D",
            Key::TC1E => "TC1E",
            Key::TC1F => "TC1F",
            Key::TCBH => "TCBH",
            Key::TCBD => "TCBD",
            Key::TCSC => "TCSC",
            Key::TCSA => "TCSA",
            Key::TCGC => "TCGC",
            // _ => "",
        }
    }

    pub fn value(&self) -> u32 {
        let byte_array_ref = self.name().as_bytes();
        let byte_array: [u8; 4] = [
            byte_array_ref[0],
            byte_array_ref[1],
            byte_array_ref[2],
            byte_array_ref[3],
        ];
        u32::from_be_bytes(byte_array)
    }

    //should there be an unknown? or.. use None?
    #[allow(dead_code)]
    pub fn kind(&self) -> Kind {
        match *self {
            Key::TCXC => Kind::Temperature,
            Key::TC0P => Kind::Temperature,
            Key::TM0P => Kind::Temperature,
            Key::TC0H => Kind::Temperature,
            Key::TC0D => Kind::Temperature,
            Key::TC0E => Kind::Temperature,
            Key::TC0F => Kind::Temperature,
            Key::TC1C => Kind::Temperature,
            Key::TC2C => Kind::Temperature,
            Key::TC3C => Kind::Temperature,
            Key::TC4C => Kind::Temperature,
            Key::TC5C => Kind::Temperature,
            Key::TC6C => Kind::Temperature,
            Key::TC7C => Kind::Temperature,
            Key::TC8C => Kind::Temperature,
            Key::TCAH => Kind::Temperature,
            Key::TCAD => Kind::Temperature,
            Key::TC1P => Kind::Temperature,
            Key::TC1H => Kind::Temperature,
            Key::TC1D => Kind::Temperature,
            Key::TC1E => Kind::Temperature,
            Key::TC1F => Kind::Temperature,
            Key::TCBH => Kind::Temperature,
            Key::TCBD => Kind::Temperature,
            Key::TCSC => Kind::Temperature,
            Key::TCSA => Kind::Temperature,
            Key::TCGC => Kind::Temperature,
        }
    }

    #[allow(dead_code)]
    pub fn subsystem(&self) -> Subsystem {
        match *self {
            Key::TCXC => Subsystem::Cpu,
            Key::TC0P => Subsystem::Cpu,
            Key::TM0P => Subsystem::Cpu,
            Key::TC0H => Subsystem::Cpu,
            Key::TC0D => Subsystem::Cpu,
            Key::TC0E => Subsystem::Cpu,
            Key::TC0F => Subsystem::Cpu,
            Key::TC1C => Subsystem::Cpu,
            Key::TC2C => Subsystem::Cpu,
            Key::TC3C => Subsystem::Cpu,
            Key::TC4C => Subsystem::Cpu,
            Key::TC5C => Subsystem::Cpu,
            Key::TC6C => Subsystem::Cpu,
            Key::TC7C => Subsystem::Cpu,
            Key::TC8C => Subsystem::Cpu,
            Key::TCAH => Subsystem::Cpu,
            Key::TCAD => Subsystem::Cpu,
            Key::TC1P => Subsystem::Cpu,
            Key::TC1H => Subsystem::Cpu,
            Key::TC1D => Subsystem::Cpu,
            Key::TC1E => Subsystem::Cpu,
            Key::TC1F => Subsystem::Cpu,
            Key::TCBH => Subsystem::Cpu,
            Key::TCBD => Subsystem::Cpu,
            Key::TCSC => Subsystem::SystemAgent,
            Key::TCSA => Subsystem::SystemAgent,
            Key::TCGC => Subsystem::Gpu,
        }
    }
}
