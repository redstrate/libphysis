// SPDX-FileCopyrightText: 2023 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::{CStr, CString};
use std::fs::read;
use std::mem;
use std::os::raw::c_char;
use std::ptr::null;

fn ffi_from_c_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr as *mut i8).to_str().unwrap().to_string() }
}

fn ffi_to_c_string(s: &String) -> *const c_char {
    let s = CString::new(s.as_bytes());

    match s {
        Ok(x) => x.into_raw(),
        Err(_) => null(),
    }
}

fn ffi_to_vec<T>(ptr: *mut T, count: u32) -> Vec<T> {
    unsafe { Vec::from_raw_parts(ptr, count as usize, count as usize) }
}

fn ffi_free_string(ptr: *const c_char) {
    unsafe {
        let str = CString::from_raw(ptr as *mut i8);
        drop(str);
    }
}

#[no_mangle]
pub extern "C" fn physis_get_physis_version() -> *const c_char {
    ffi_to_c_string(&physis::PHYSIS_VERSION.to_string())
}

#[no_mangle]
pub extern "C" fn physis_get_libphysis_version() -> *const c_char {
    ffi_to_c_string(&env!("CARGO_PKG_VERSION").to_string())
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Buffer {
    size: u32,
    data: *mut u8,
}

#[no_mangle]
pub extern "C" fn physis_read_file(path: *const c_char) -> physis_Buffer {
    let mut f = read(ffi_from_c_string(path)).unwrap();
    let buf = physis_Buffer {
        size: f.len() as u32,
        data: f.as_mut_ptr(),
    };

    mem::forget(f);

    buf
}

mod gamedata;

mod bootdata;

#[cfg(feature = "visual_data")]
mod model;

mod race;

mod exl;

mod equipment;

mod blowfish;

#[cfg(feature = "game_install")]
mod installer;

mod exh;

mod exd;

#[cfg(feature = "visual_data")]
mod skeleton;

#[cfg(feature = "visual_data")]
mod tex;

#[cfg(feature = "visual_data")]
mod mtrl;

#[cfg(feature = "visual_data")]
mod shpk;

mod cmp;

mod chardat;

mod cfg;

#[cfg(feature = "visual_data")]
mod pbd;

#[cfg(feature = "visual_data")]
mod tera;

mod dic;

mod index;

mod logging;
