// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem;
use std::os::raw::c_char;
use std::ptr::null;

use physis::sqpack::{Hash, SqPackIndex};

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

#[unsafe(no_mangle)]
pub extern "C" fn physis_index_parse(path: *const c_char) -> physis_IndexEntries {
    let Some(r_path) = ffi_from_c_string(path) else {
        return physis_IndexEntries::default();
    };

    if let Some(idx_file) = SqPackIndex::from_existing(&r_path) {
        let mut c_dir_entries = vec![];
        let mut c_file_entries = vec![];

        for entry in &idx_file.entries {
            match &entry.hash {
                Hash::SplitPath { name, path } => {
                    c_file_entries.push(*name);
                    c_dir_entries.push((path >> 32) as u32);
                }
                Hash::FullPath(hash) => {
                    c_file_entries.push(*hash); // TODO: is this really correct?
                }
            }
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

#[unsafe(no_mangle)]
pub extern "C" fn physis_generate_partial_hash(name: *const c_char) -> u32 {
    let Some(r_name) = ffi_from_c_string(name) else {
        return 0;
    };

    SqPackIndex::calculate_partial_hash(&r_name)
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_calculate_hash(
    index_file_path: *const c_char,
    path: *const c_char,
) -> u64 {
    let Some(r_path) = ffi_from_c_string(path) else {
        return 0;
    };

    let Some(r_index_file_path) = ffi_from_c_string(index_file_path) else {
        return 0;
    };

    // TODO: this is not ideal, we should just expose IndexFile in the C API
    if let Some(idx_file) = SqPackIndex::from_existing(&r_index_file_path) {
        match &idx_file.calculate_hash(&r_path) {
            Hash::SplitPath { name, path } => (*path as u64) << 32 | (*name as u64),
            Hash::FullPath(hash) => *hash as u64,
        }
    } else {
        0
    }
}
