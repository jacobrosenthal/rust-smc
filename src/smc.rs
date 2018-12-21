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

use crate::general::{parse_type, parse_value};
pub use crate::general::{Kind, Subsystem, Type};
use strum::IntoEnumIterator;

pub use crate::key::Key;

pub use crate::error::{SmcError, SmcResult};

use crate::smc_kit::{
    SMCKeyData_keyInfo_t, SMCKeyData_t, KERNEL_INDEX_SMC, SMC_CMD_READ_BYTES, SMC_CMD_READ_KEYINFO,
};

pub use crate::sensor::Sensor;

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

    pub fn iter(&self) -> impl Iterator<Item = Key> + Clone {
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

    pub(crate) fn read_key_info(&self, key_sum: u32) -> SmcResult<SMCKeyData_keyInfo_t> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_KEYINFO,
            key: key_sum,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;
        Ok(out_struct.key_info)
    }

    pub(crate) fn read_key(&self, key: &Key, key_info: SMCKeyData_keyInfo_t) -> SmcResult<f32> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_BYTES,
            key: key.value(),
            key_info,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;
        let data_type = parse_type(key_info.data_type)?;
        let value = parse_value(key_info.data_size, data_type, out_struct.bytes);

        Ok(value)
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
