// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_from_c_string, ffi_to_c_string};
use physis::bootdata::BootData;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[unsafe(no_mangle)]
pub extern "C" fn physis_bootdata_get_version(boot_data: &BootData) -> *const c_char {
    ffi_to_c_string(&boot_data.version)
}

/// Initializes a new BootData structure. Path must be a valid boot path, or else it will return NULL.
#[unsafe(no_mangle)]
pub extern "C" fn physis_bootdata_initialize(path: *const c_char) -> *mut BootData {
    let Some(r_path) = ffi_from_c_string(path) else {
        return null_mut();
    };

    if let Some(boot_data) = BootData::from_existing(&r_path) {
        let boxed = Box::new(boot_data);

        Box::leak(boxed)
    } else {
        null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_bootdata_free(boot_data: *mut BootData) {
    unsafe {
        drop(Box::from_raw(boot_data));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_bootdata_apply_patch(bootdata: &BootData, path: *const c_char) -> bool {
    if let Some(r_path) = ffi_from_c_string(path) {
        bootdata.apply_patch(&r_path).is_ok()
    } else {
        false
    }
}
