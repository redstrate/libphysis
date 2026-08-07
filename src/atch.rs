// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::atch::Atch;
use std::ffi::c_char;
use std::slice;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_atch_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    match Atch::from_existing(platform, data) {
        Ok(atch) => ffi_to_c_string(&format!("{atch:#?}")),
        Err(err) => ffi_to_c_string(&format!("{err:#?}")),
    }
}
