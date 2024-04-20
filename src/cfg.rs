use crate::{ffi_from_c_string, physis_Buffer};
use physis::cfg::ConfigFile;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ConfigFile {
    p_ptr: *mut ConfigFile,
}

#[no_mangle]
pub extern "C" fn physis_cfg_parse(buffer: physis_Buffer) -> physis_ConfigFile {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(cfg) = ConfigFile::from_existing(&data.to_vec()) {
        let cfg_struct = physis_ConfigFile {
            p_ptr: Box::leak(Box::new(cfg)),
        };

        cfg_struct
    } else {
        physis_ConfigFile { p_ptr: null_mut() }
    }
}

#[no_mangle]
pub extern "C" fn physis_cfg_set_value(
    cfg: physis_ConfigFile,
    key: *const c_char,
    value: *const c_char,
) {
    let Some(r_key) = ffi_from_c_string(key) else {
        return
    };

    let Some(r_value) = ffi_from_c_string(value) else {
        return
    };
    
    unsafe {
        (*cfg.p_ptr).set_value(&r_key, &r_value);
    }
}

#[no_mangle]
pub extern "C" fn physis_cfg_write(cfg: physis_ConfigFile) -> physis_Buffer {
    unsafe {
        let mut buffer = (*cfg.p_ptr).write_to_buffer().unwrap();

        let leak_buffer = physis_Buffer {
            size: buffer.len() as u32,
            data: buffer.as_mut_ptr(),
        };

        mem::forget(buffer);

        leak_buffer
    }
}
