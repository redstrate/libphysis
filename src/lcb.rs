// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::lcb::{Lcb, LccEntry};
use std::ffi::c_char;
use std::ptr::{null, null_mut};
use std::{mem, slice};

#[repr(C)]
#[derive(Clone)]
pub struct physis_Lcc {
    num_entries: u32,
    entries: *mut LccEntry,
}

#[repr(C)]
#[derive(Clone)]
pub struct physis_Lcb {
    lcc_count: u32,
    lccs: *mut physis_Lcc,
}

impl Default for physis_Lcb {
    fn default() -> Self {
        Self {
            lcc_count: 0,
            lccs: null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_lcb_parse(platform: Platform, buffer: physis_Buffer) -> physis_Lcb {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(lcb) = Lcb::from_existing(platform, data) {
        let mut c_lccs = Vec::new();
        for lcc in &lcb.lccs {
            let mut c_entries = lcc.entries.clone();

            let c_lcc = physis_Lcc {
                num_entries: c_entries.len() as u32,
                entries: c_entries.as_mut_ptr(),
            };

            mem::forget(c_entries);

            c_lccs.push(c_lcc);
        }

        let c_lcb = physis_Lcb {
            lcc_count: c_lccs.len() as u32,
            lccs: c_lccs.as_mut_ptr(),
        };

        mem::forget(c_lccs);

        c_lcb
    } else {
        physis_Lcb::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_lcb_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(lcb) = Lcb::from_existing(platform, data) {
        ffi_to_c_string(&format!("{lcb:#?}"))
    } else {
        null()
    }
}
