// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::physis_Buffer;
use physis::Platform;
use physis::sqpack::SqPackData;
use std::io::Cursor;
use std::{mem, slice};

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_read_block(
    platform: Platform,
    buffer: physis_Buffer,
) -> physis_Buffer {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    let mut cursor = Cursor::new(data);

    if let Ok(mut buffer) = SqPackData::read_from_reader(&mut cursor, platform) {
        let leak_buffer = physis_Buffer {
            size: buffer.len() as u32,
            data: buffer.as_mut_ptr(),
        };

        mem::forget(buffer);

        leak_buffer
    } else {
        physis_Buffer::default()
    }
}
