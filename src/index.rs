// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem;
use std::os::raw::c_char;
use std::ptr::null;

use physis::index::IndexFile;

use crate::ffi_from_c_string;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_IndexEntries {
    num_entries: u32,
    dir_entries: *const u32,
    filename_entries: *const u32,
}

impl Default for physis_IndexEntries {
    fn default() -> Self {
        Self {
            num_entries: 0,
            dir_entries: null(),
            filename_entries: null(),
        }
    }
}

#[no_mangle]
pub extern "C" fn physis_index_parse(path: *const c_char) -> physis_IndexEntries {
    let Some(r_path) = ffi_from_c_string(path) else {
        return physis_IndexEntries::default();
    };

    if let Some(idx_file) = IndexFile::from_existing(&r_path) {
        let mut c_dir_entries = vec![];
        let mut c_file_entries = vec![];

        for entry in &idx_file.entries {
            c_file_entries.push(entry.hash as u32);
            c_dir_entries.push((entry.hash >> 32) as u32);
        }

        let mat = physis_IndexEntries {
            num_entries: c_dir_entries.len() as u32,
            dir_entries: c_dir_entries.as_mut_ptr(),
            filename_entries: c_file_entries.as_mut_ptr(),
        };

        mem::forget(c_dir_entries);
        mem::forget(c_file_entries);

        mat
    } else {
        physis_IndexEntries::default()
    }
}

#[no_mangle]
pub extern "C" fn physis_generate_partial_hash(name: *const c_char) -> u32 {
    let Some(r_name) = ffi_from_c_string(name) else {
        return 0;
    };

    IndexFile::calculate_partial_hash(&r_name)
}

#[no_mangle]
pub extern "C" fn physis_calculate_hash(path: *const c_char) -> u64 {
    let Some(r_path) = ffi_from_c_string(path) else {
        return 0;
    };

    IndexFile::calculate_hash(&r_path)
}
