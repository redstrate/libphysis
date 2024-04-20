use crate::{ffi_from_c_string, ffi_to_c_string};
use physis::bootdata::BootData;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[no_mangle]
pub extern "C" fn physis_bootdata_get_version(boot_data: &BootData) -> *const c_char {
    ffi_to_c_string(&boot_data.version)
}

/// Initializes a new BootData structure. Path must be a valid boot path, or else it will return NULL.
#[no_mangle]
pub extern "C" fn physis_bootdata_initialize(path: *const c_char) -> *mut BootData {
    if let Some(boot_data) = BootData::from_existing(&ffi_from_c_string(path)) {
        let boxed = Box::new(boot_data);

        Box::leak(boxed)
    } else {
        null_mut()
    }
}

#[no_mangle]
pub extern "C" fn physis_bootdata_free(boot_data: *mut BootData) {
    unsafe {
        drop(Box::from_raw(boot_data));
    }
}

#[no_mangle]
pub extern "C" fn physis_bootdata_apply_patch(bootdata: &BootData, path: *const c_char) -> bool {
    bootdata.apply_patch(&ffi_from_c_string(path)).is_ok()
}
