// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::physis_Buffer;
use physis::savedata::chardat::CharacterData;
use std::slice;

#[unsafe(no_mangle)]
pub extern "C" fn physis_chardat_parse(buffer: physis_Buffer) -> CharacterData {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    CharacterData::from_existing(data).unwrap()
}
