use crate::ffi_from_c_string;
use physis::index::IndexFile;
use std::mem;
use std::os::raw::c_char;
use std::ptr::null_mut;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_IndexEntries {
    num_entries: u32,
    dir_entries: *const u32,
    filename_entries: *const u32,
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_index_parse(path: *const c_char) -> physis_IndexEntries {
    if let Some(idx_file) = IndexFile::from_existing(&ffi_from_c_string(path)) {
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
        physis_IndexEntries {
            num_entries: 0,
            dir_entries: null_mut(),
            filename_entries: null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn physis_generate_partial_hash(name: *const c_char) -> u32 {
    IndexFile::calculate_partial_hash(&ffi_from_c_string(name))
}

#[no_mangle]
pub extern "C" fn physis_calculate_hash(path: *const c_char) -> u64 {
    IndexFile::calculate_hash(&ffi_from_c_string(path))
}
