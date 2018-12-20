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

    pub fn find<F: Fn(&Key) -> bool>(&self, pred: F) -> impl Iterator<Item = Sensor> {
        //explicit move ok?
        self.iter()
            .filter(pred)
            .map(move |key| Sensor::new(key, &self))
    }

    pub fn get_sensor(&'a self, key: Key) -> Sensor<'a> {
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

    // fn format(&self, data_type: Type, bytes: &[u8]) -> SmcResult<f32> {

    //     Ok(a)
    // }

    pub fn read_key(&self, key: &Key, key_info: SMCKeyData_keyInfo_t) -> SmcResult<f32> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_BYTES,
            key: key.value(),
            key_info,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;

        let data_type = parse_type(key_info.data_type)?;

        let a = match data_type {
            Type::sp78 => 1.0f32,
            Type::ui32 => {
                let &[a, b, c, d, ..] = &out_struct.bytes;
                let four: [u8; 4] = [a, b, c, d];
                u32::from_be_bytes(four) as f32
            }
            Type::ui16 => {
                let &[a, b, ..] = &out_struct.bytes;
                let two: [u8; 2] = [a, b];
                u16::from_be_bytes(two) as f32
            }
            Type::ui8 => {
                // let &[a _..] = &out_struct.bytes;
                // let two: [u8; 2] = [a, b];
                // u16::from_be_bytes(two) as f32
                out_struct.bytes[0] as f32
            }

            _ => -1.0,
        };

        Ok(a)
    }
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
}

pub struct Sensor<'a> {
    smc: &'a Smc,
    key: Key,
    key_info: SMCKeyData_keyInfo_t,
}

use std::fmt;

impl<'a> fmt::Display for Sensor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.name())
    }
}

impl<'a> Sensor<'a> {
    pub fn new(key: Key, smc: &'a Smc) -> Sensor<'a> {
        let val = key.value();
        Sensor {
            smc: &smc,
            key: key,
            key_info: smc.read_key_info(val).unwrap(),
        }
    }
    pub fn name(&self) -> &'static str {
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
    pub fn name(&self) -> &'static str {
        match *self {
            Key::TCXC => "TCXC",
            Key::TC0P => "TC0P",
            Key::TM0P => "TM0P",
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
            Key::TC0P => Kind::Fan,
            Key::TM0P => Kind::Unknown,
        }
    }

    #[allow(dead_code)]
    pub fn subsystem(&self) -> Subsystem {
        match *self {
            Key::TCXC => Subsystem::Cpu,
            Key::TC0P => Subsystem::Cpu,
            Key::TM0P => Subsystem::Mainboard,
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
    fp5b,
    fpa6,
    fpc4,
    sp1e,
    sp3c,
    sp4b,
    sp96,
    spb4,
    spf0,
    pwm,
    ui32,
    ui8,
    flag,
    ui16,
    hex,
    ch8,
    fp88,
    ali,
    alp,
    alc,
    fp1f,
    alv,
    si16,
    sp87,
    sp78,
    flt,
    sp5a,
    si8,
    clc,
    clh,
    hdi,
    lim,
    lkb,
    lks,
    fpe2,
    fds,
    fp79,
    fp6a,
    mss,
    rev,
    char,
}

static TYPES: phf::Map<u32, Type> = phf_map! {
    1718629730u32 => Type::fp5b,
    1718640950u32 => Type::fpa6,
    1718641460u32 => Type::fpc4,
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
