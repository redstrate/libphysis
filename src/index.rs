// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::{null, null_mut};

use physis::Platform;
use physis::sqpack::{Hash, SqPackIndex};

use crate::ffi_from_c_string;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_IndexEntries {
    p_ptr: *mut SqPackIndex,

    num_hashes: u32,
    hashes: *const Hash,
}

impl Default for physis_IndexEntries {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            num_hashes: 0,
            hashes: null(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_index_parse(
    platform: Platform,
    path: *const c_char,
) -> physis_IndexEntries {
    let Some(r_path) = ffi_from_c_string(path) else {
        return physis_IndexEntries::default();
    };

    if let Some(idx_file) = SqPackIndex::from_existing(platform, Path::new(&r_path)) {
        let mut c_hashes = Vec::new();

        for entry in &idx_file.entries {
            c_hashes.push(entry.hash);
        }

        let boxed = Box::new(idx_file);

        let mat = physis_IndexEntries {
            p_ptr: Box::leak(boxed),
            num_hashes: c_hashes.len() as u32,
            hashes: c_hashes.as_mut_ptr(),
        };

        mem::forget(c_hashes);

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
pub extern "C" fn physis_index_hash_from_offset(entries: physis_IndexEntries, offset: u64) -> Hash {
    unsafe {
        if let Some(hash) = (*entries.p_ptr).find_entry_from_offset(offset) {
            hash
        } else {
            Hash::FullPath(0)
        }
    }
}
