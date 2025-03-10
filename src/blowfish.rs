// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use physis::blowfish::Blowfish;
use std::os::raw::c_uint;
use std::{mem, slice};

#[unsafe(no_mangle)]
pub extern "C" fn physis_blowfish_initialize(key: *mut u8, key_size: c_uint) -> *mut Blowfish {
    let data = unsafe { slice::from_raw_parts(key, key_size as usize) };
    Box::into_raw(Box::new(Blowfish::new(data)))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_blowfish_free(blowfish: *mut Blowfish) {
    unsafe {
        drop(Box::from_raw(blowfish));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_blowfish_encrypt(
    blowfish: &Blowfish,
    in_data: *mut u8,
    in_data_size: c_uint,
    out_data: &mut *mut u8,
    out_data_size: *mut u32,
) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.encrypt(in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_blowfish_decrypt(
    blowfish: &Blowfish,
    in_data: *mut u8,
    in_data_size: c_uint,
    out_data: &mut *mut u8,
    out_data_size: *mut u32,
) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.decrypt(in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false,
    }
}
