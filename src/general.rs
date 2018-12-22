use std::convert::AsMut;

use strum_macros::EnumString;

//https://stackoverflow.com/questions/37668886/slice-to-fixed-size-array
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

#[derive(Clone, Copy, Debug, EnumString, PartialEq)]
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

impl Default for Subsystem {
    fn default() -> Subsystem {
        Subsystem::Unknown
    }
}

#[derive(Clone, Copy, Debug, EnumString, PartialEq)]
pub enum Kind {
    Temperature,
    Fan,
    Unknown,
}

impl Default for Kind {
    fn default() -> Kind {
        Kind::Unknown
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
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

pub fn lookup_type(input: u32) -> Option<Type> {
    match input {
        1718629730u32 => Some(Type::fp5b),
        1718640950u32 => Some(Type::fpa6),
        1718641460u32 => Some(Type::fpc4),
        1718629475u32 => Some(Type::fp4c),
        1936732517u32 => Some(Type::sp1e),
        1936733027u32 => Some(Type::sp3c),
        1936733282u32 => Some(Type::sp4b),
        1936734518u32 => Some(Type::sp96),
        1936745012u32 => Some(Type::spb4),
        1936746032u32 => Some(Type::spf0),
        2070968173u32 => Some(Type::pwm),
        1969828658u32 => Some(Type::ui32),
        1969829920u32 => Some(Type::ui8),
        1718378855u32 => Some(Type::flag),
        1969828150u32 => Some(Type::ui16),
        1751480415u32 => Some(Type::hex),
        1667774506u32 => Some(Type::ch8),
        1718630456u32 => Some(Type::fp88),
        2069982313u32 => Some(Type::ali),
        2069982320u32 => Some(Type::alp),
        2069982307u32 => Some(Type::alc),
        1718628710u32 => Some(Type::fp1f),
        2069982326u32 => Some(Type::alv),
        1936273718u32 => Some(Type::si16),
        1936734263u32 => Some(Type::sp87),
        1936734008u32 => Some(Type::sp78),
        1936733753u32 => Some(Type::sp69),
        1718383648u32 => Some(Type::flt),
        1936733537u32 => Some(Type::sp5a),
        1936275488u32 => Some(Type::si8),
        2070113379u32 => Some(Type::clc),
        2070113384u32 => Some(Type::clh),
        2070439017u32 => Some(Type::hdi),
        2070702445u32 => Some(Type::lim),
        2070702946u32 => Some(Type::lkb),
        2070702963u32 => Some(Type::lks),
        1718641970u32 => Some(Type::fpe2),
        2070307955u32 => Some(Type::fds),
        1718630201u32 => Some(Type::fp79),
        1718629985u32 => Some(Type::fp6a),
        2070770547u32 => Some(Type::mss),
        1919252000u32 => Some(Type::rev),
        1667785074u32 => Some(Type::char),
        _ => None,
    }
}

pub fn parse_value(data_size: u32, data_type: Type, bytes: [u8; 32]) -> f32 {
    let thing = match data_size {
        2 => u16::from_be_bytes(clone_into_array(&bytes[0..2])) as u32,
        4 => u32::from_be_bytes(clone_into_array(&bytes[0..4])) as u32,
        _ => 0,
    };

    //
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

    value
}

pub fn translate(name: &str) -> u32 {
    let byte_array_ref = name.as_bytes();
    let byte_array: [u8; 4] = [
        byte_array_ref[0],
        byte_array_ref[1],
        byte_array_ref[2],
        byte_array_ref[3],
    ];
    u32::from_be_bytes(byte_array)
}
