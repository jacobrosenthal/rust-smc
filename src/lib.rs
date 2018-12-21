//these need to be in the root crate apparently if if theyre needed for the subfiles
#![feature(slice_patterns)]
#![feature(try_trait)]
#![feature(plugin)]
#![plugin(phf_macros)]

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, ToString};

mod error;
pub use self::error::{SmcError, SmcResult};

mod smc_kit;
use self::smc_kit::{
    SMCKeyData_keyInfo_t, SMCKeyData_t, KERNEL_INDEX_SMC, SMC_CMD_READ_BYTES, SMC_CMD_READ_KEYINFO,
};

use libc::c_void;
use mach::kern_return::KERN_SUCCESS;
use mach::traps::mach_task_self;
use std::ffi::CString;
use std::fmt;
use std::mem;
use IOKit_sys::{
    io_connect_t, io_iterator_t, io_object_t, kIOMasterPortDefault, IOConnectCallStructMethod,
    IOIteratorNext, IOObjectRelease, IOServiceClose, IOServiceGetMatchingServices,
    IOServiceMatching, IOServiceOpen,
};

pub struct Smc {
    connection: io_connect_t,
}

impl<'a> Smc {
    pub fn new() -> SmcResult<Smc> {
        let service = get_service("AppleSMC")?;

        let mut connection = Default::default();
        let kern_result = unsafe { IOServiceOpen(service, mach_task_self(), 0, &mut connection) };
        unsafe {
            IOObjectRelease(service);
        };

        if kern_result != KERN_SUCCESS {
            return Err(SmcError::new(""));
        }

        Ok(Smc { connection })
    }

    pub fn iter(&self) -> KeyIter {
        Key::iter()
    }

    pub fn find<F: Clone + Fn(&Key) -> bool>(
        &self,
        pred: F,
    ) -> impl Iterator<Item = Sensor> + Clone {
        self.iter()
            .filter(pred)
            .map(move |key| Sensor::new(key, &self))
            .filter_map(Result::ok)
    }
    pub fn get_sensor(&'a self, key: Key) -> SmcResult<Sensor<'a>> {
        Sensor::new(key, &self)
    }

    fn read(&self, mut in_struct: SMCKeyData_t) -> SmcResult<SMCKeyData_t> {
        let innn: *const c_void = &mut in_struct as *const _ as *const c_void;

        let mut out_struct: SMCKeyData_t = Default::default();
        let out: *mut c_void = &mut out_struct as *mut _ as *mut c_void;

        let in_struct_size = mem::size_of::<SMCKeyData_t>();
        let mut out_struct_size = std::mem::size_of::<SMCKeyData_t>();

        let kern_result = unsafe {
            IOConnectCallStructMethod(
                self.connection,
                KERNEL_INDEX_SMC,
                innn,
                in_struct_size,
                out,
                &mut out_struct_size,
            )
        };
        if kern_result != KERN_SUCCESS {
            return Err(SmcError::new(""));
        }

        if out_struct.result > 0 {
            return Err(SmcError::new(""));
        }

        Ok(out_struct)
    }

    fn read_key_info(&self, key_sum: u32) -> SmcResult<SMCKeyData_keyInfo_t> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_KEYINFO,
            key: key_sum,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;
        Ok(out_struct.key_info)
    }

    pub fn read_key(&self, key: &Key, key_info: SMCKeyData_keyInfo_t) -> SmcResult<f32> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_BYTES,
            key: key.value(),
            key_info,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;

        parse_value(key_info, out_struct.bytes)
    }
}

fn parse_value(key_info: SMCKeyData_keyInfo_t, bytes: [u8; 32]) -> SmcResult<f32> {
    let data_type = parse_type(key_info.data_type)?;

    let thing = match key_info.data_size {
        2 => {
            let &[a, b, ..] = &bytes;
            let two: [u8; 2] = [a, b];
            u16::from_be_bytes(two) as u32
        }
        4 => {
            let &[a, b, c, d, ..] = &bytes;
            let four: [u8; 4] = [a, b, c, d];
            u32::from_be_bytes(four)
        }
        _ => 0,
    };

    let value = match data_type {
        Type::sp1e => thing as f32 / 16384.0,
        Type::sp3c => thing as f32 / 4096.0,
        Type::sp4b => thing as f32 / 2048.0,
        Type::sp5a => thing as f32 / 1024.0,
        Type::sp69 => thing as f32 / 512.0,
        Type::sp78 => thing as f32 / 256.0,
        Type::sp87 => thing as f32 / 128.0,
        Type::sp96 => thing as f32 / 64.0,
        Type::spb4 => thing as f32 / 16.0,
        Type::spf0 => thing as f32,
        Type::fp1f => thing as f32 / 32768.0,
        Type::fp4c => thing as f32 / 4096.0,
        Type::fp5b => thing as f32 / 2048.0,
        Type::fp6a => thing as f32 / 1024.0,
        Type::fp79 => thing as f32 / 512.0,
        Type::fp88 => thing as f32 / 256.0,
        Type::fpa6 => thing as f32 / 64.0,
        Type::fpc4 => thing as f32 / 16.0,
        Type::fpe2 => thing as f32 / 4.0,
        Type::flt => thing as f32,
        Type::ui32 => {
            thing as f32 //todo as u32
        }
        Type::ui16 => {
            thing as f32 //todo as u16
        }
        Type::ui8 => bytes[0] as f32, //todo as u8
        _ => 0.0,
    };

    Ok(value)
}

impl Drop for Smc {
    fn drop(&mut self) {
        unsafe {
            IOServiceClose(self.connection);
        }
    }
}

fn get_service(key: &str) -> SmcResult<io_object_t> {
    let cstring = CString::new(key)?;

    let matching_dictionary = unsafe { IOServiceMatching(cstring.as_ptr()) };
    if matching_dictionary.is_null() {
        return Err(SmcError::new(""));
    }

    let mut iterator: io_iterator_t = Default::default();

    let result = unsafe {
        IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dictionary, &mut iterator)
    };
    if result != KERN_SUCCESS {
        return Err(SmcError::new(""));
    }

    let service = unsafe { IOIteratorNext(iterator) };
    unsafe {
        IOObjectRelease(iterator);
    }

    if service == 0 {
        return Err(SmcError::new(""));
    }

    Ok(service)
}

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
            Kind::Unknown => kind = "👽".to_string(),
        }

        let reading = self.read().unwrap_or_else(|_| 0.0);

        write!(f, "{}, {}, {:.*}°C", kind, self.name(), 2, reading)
    }
}

impl<'a> Sensor<'a> {
    pub fn new(key: Key, smc: &'a Smc) -> SmcResult<Sensor<'a>> {
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

    fn value(&self) -> u32 {
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

#[derive(PartialEq)]
pub enum Subsystem {
    Cpu,
    Memory,
    Bus,
    Gpu,
    Sensor,
    Battery,
    Mainboard,
    SystemAgent,
    Unknown,
}

#[derive(PartialEq)]
pub enum Kind {
    Temperature,
    Fan,
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, EnumString, EnumIter, ToString)]
pub enum Type {
    fp1f,
    fp4c,
    fp5b,
    fp6a,
    fp79,
    fp88,
    fpa6,
    fpc4,
    fpe2,
    sp1e,
    sp3c,
    sp4b,
    sp5a,
    sp96,
    sp78,
    sp87,
    spb4,
    spf0,
    sp69,
    pwm,
    ui32,
    ui8,
    flag,
    ui16,
    hex,
    ch8,
    ali,
    alp,
    alc,
    alv,
    si16,
    flt,
    si8,
    clc,
    clh,
    hdi,
    lim,
    lkb,
    lks,
    fds,
    mss,
    rev,
    char,
}

static TYPES: phf::Map<u32, Type> = phf_map! {
    1718629730u32 => Type::fp5b,
    1718640950u32 => Type::fpa6,
    1718641460u32 => Type::fpc4,
    1718629475u32 => Type::fp4c,
    1936732517u32 => Type::sp1e,
    1936733027u32 => Type::sp3c,
    1936733282u32 => Type::sp4b,
    1936734518u32 => Type::sp96,
    1936745012u32 => Type::spb4,
    1936746032u32 => Type::spf0,
    2070968173u32 => Type::pwm,
    1969828658u32 => Type::ui32,
    1969829920u32 => Type::ui8,
    1718378855u32 => Type::flag,
    1969828150u32 => Type::ui16,
    1751480415u32 => Type::hex,
    1667774506u32 => Type::ch8,
    1718630456u32 => Type::fp88,
    2069982313u32 => Type::ali,
    2069982320u32 => Type::alp,
    2069982307u32 => Type::alc,
    1718628710u32 => Type::fp1f,
    2069982326u32 => Type::alv,
    1936273718u32 => Type::si16,
    1936734263u32 => Type::sp87,
    1936734008u32 => Type::sp78,
    1936733753u32 => Type::sp69,
    1718383648u32 => Type::flt,
    1936733537u32 => Type::sp5a,
    1936275488u32 => Type::si8,
    2070113379u32 => Type::clc,
    2070113384u32 => Type::clh,
    2070439017u32 => Type::hdi,
    2070702445u32 => Type::lim,
    2070702946u32 => Type::lkb,
    2070702963u32 => Type::lks,
    1718641970u32 => Type::fpe2,
    2070307955u32 => Type::fds,
    1718630201u32 => Type::fp79,
    1718629985u32 => Type::fp6a,
    2070770547u32 => Type::mss,
    1919252000u32 => Type::rev,
    1667785074u32 => Type::char,
};

pub fn parse_type(type_: u32) -> Option<Type> {
    TYPES.get(&type_).cloned()
}
