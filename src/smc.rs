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

use crate::error::{SmcError, SmcResult};
use crate::general::{lookup_type, parse_value, translate};

use crate::sensor::{Sensor, SensorIter};
use crate::smc_kit::{
    SMCKeyData_keyInfo_t, SMCKeyData_t, KERNEL_INDEX_SMC, SMC_CMD_READ_BYTES, SMC_CMD_READ_INDEX,
    SMC_CMD_READ_KEYINFO,
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
            return Err(SmcError::new(&format!(
                "IOServiceOpen failed {}",
                kern_result
            )));
        }

        Ok(Smc { connection })
    }

    pub fn iter(&self) -> impl Iterator<Item = Sensor> + Clone {
        SensorIter::new(self).unwrap()
    }

    // pub fn find<F: Clone + Fn(&Key) -> bool>(
    //     &self,
    //     pred: F,
    // ) -> impl Iterator<Item = Sensor> + Clone {
    //     self.iter()
    //         .filter(pred)
    //         .map(move |key| Sensor::new(key, &self))
    //         .filter_map(Result::ok)
    // }

    pub fn get_sensor_by_name(&'a self, name: &str) -> SmcResult<Sensor<'a>> {
        assert_eq!(4, name.len());
        Sensor::new(translate(name), &self)
    }

    pub fn get_sensor_by_value(&'a self, value: u32) -> SmcResult<Sensor<'a>> {
        Sensor::new(value, &self)
    }

    pub fn get_sensor_by_index(&'a self, index: u32) -> SmcResult<Sensor<'a>> {
        let value = self.get_key_by_index(index)?;
        Sensor::new(value, &self)
    }

    pub fn get_key_by_index(&self, index: u32) -> SmcResult<u32> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_INDEX,
            data32: index,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;

        Ok(out_struct.key)
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
            return Err(SmcError::new(&format!("read failed {}", kern_result)));
        }

        //todo.. should probably be a different error to differentiate from non recoverable errors
        if out_struct.result == 132 {
            return Err(SmcError::new("read returned but key not found"));
        }

        if out_struct.result > 0 {
            return Err(SmcError::new(&format!(
                "read returned {}",
                out_struct.result
            )));
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

    pub(crate) fn read_key(&self, value: u32, key_info: SMCKeyData_keyInfo_t) -> SmcResult<f32> {
        let in_struct = SMCKeyData_t {
            data8: SMC_CMD_READ_BYTES,
            key: value,
            key_info,
            ..Default::default()
        };

        let out_struct = self.read(in_struct)?;

        let data_type = lookup_type(key_info.data_type);

        let data_type = match data_type {
            Some(file) => file,
            None => return Err(SmcError::new("Parse Key Type failed")),
        };

        let value = parse_value(key_info.data_size, data_type, out_struct.bytes);

        Ok(value)
    }

    pub fn get_key_count(&self) -> SmcResult<f32> {
        let sensor = self.get_sensor_by_name("#KEY")?;
        sensor.read()
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
        return Err(SmcError::new("IOServiceMatching failed"));
    }

    let mut iterator: io_iterator_t = Default::default();

    let result = unsafe {
        IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dictionary, &mut iterator)
    };
    if result != KERN_SUCCESS {
        return Err(SmcError::new(&format!(
            "IOServiceGetMatchingServices failed {}",
            result
        )));
    }

    let service = unsafe { IOIteratorNext(iterator) };
    unsafe {
        IOObjectRelease(iterator);
    }

    if service == 0 {
        return Err(SmcError::new("IOIteratorNext failed"));
    }

    Ok(service)
}
