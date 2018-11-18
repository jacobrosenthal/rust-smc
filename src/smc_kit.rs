#[repr(C)]
#[derive(Debug, Default)]
pub struct SMCKeyData_vers_t {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub reserved: [u8; 1],
    pub release: u16,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct SMCKeyData_pLimitData_t {
    pub version: u16,
    pub length: u16,
    pub cpu_p_limit: u32,
    pub gpu_p_limit: u32,
    pub mem_p_limit: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SMCKeyData_keyInfo_t {
    pub data_size: u32,
    pub data_type: u32,
    pub data_attributes: u8,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct SMCKeyData_t {
    pub key: u32,
    pub vers: SMCKeyData_vers_t,
    pub p_limit_data: SMCKeyData_pLimitData_t,
    pub key_info: SMCKeyData_keyInfo_t,
    pub result: u8,
    pub status: u8,
    pub data8: u8,
    pub data32: u32,
    pub bytes: [u8; 32],
}

//#define KERNEL_INDEX_SMC      2
pub const KERNEL_INDEX_SMC: u32 = 2;

//#define SMC_CMD_READ_KEYINFO  9
pub const SMC_CMD_READ_KEYINFO: u8 = 9;

// #define SMC_CMD_READ_BYTES    5
pub const SMC_CMD_READ_BYTES: u8 = 5;
