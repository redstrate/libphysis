// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::scd::Scd;
use physis::scd::{AudioData, AudioFormat};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::slice;

#[repr(C)]
pub struct physis_Scd {
    audio_count: u32,
    audios: *mut physis_ScdAudio,
}

impl Default for physis_Scd {
    fn default() -> Self {
        Self {
            audio_count: 0,
            audios: null_mut(),
        }
    }
}

#[repr(C)]
pub struct physis_ScdAudio {
    format: AudioFormat,
    data_size: u32,
    data: *mut u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_scd_parse(platform: Platform, buffer: physis_Buffer) -> physis_Scd {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(scd) = Scd::from_existing(platform, data) {
        let mut c_audios = Vec::new();
        for audio in &scd.audios {
            let mut c_data;
            match &audio.data {
                AudioData::Empty => c_data = Vec::default(),
                AudioData::OggVorbis { data, .. } => c_data = data.clone(),
                AudioData::Unknown { data, .. } => c_data = data.clone(),
            }

            let c_audio = physis_ScdAudio {
                format: audio.format,
                data_size: c_data.len() as u32,
                data: c_data.as_mut_ptr(),
            };

            c_audios.push(c_audio);

            std::mem::forget(c_data);
        }

        let c_scd = physis_Scd {
            audio_count: c_audios.len() as u32,
            audios: c_audios.as_mut_ptr(),
        };

        std::mem::forget(c_audios);

        c_scd
    } else {
        physis_Scd::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_scd_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    match Scd::from_existing(platform, data) {
        Ok(scd) => ffi_to_c_string(&format!("{scd:#?}")),
        Err(err) => ffi_to_c_string(&format!("{err:#?}")),
    }
}
