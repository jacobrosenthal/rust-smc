use strum_macros::{EnumIter, EnumString, ToString};

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

pub fn parse_value(data_size: u32, data_type: Type, bytes: [u8; 32]) -> f32 {
    let thing = match data_size {
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
