// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::physis_Buffer;
use crate::scn::{physis_ScnSection, to_c_section};
use physis::Platform;
use physis::ReadableFile;
use physis::sgb::Sgb;
use std::ptr::null;
use std::slice;

#[repr(C)]
pub struct physis_Sgb {
    section_count: u32,
    sections: *const physis_ScnSection,
}

impl Default for physis_Sgb {
    fn default() -> Self {
        Self {
            section_count: 0,
            sections: null(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sgb_parse(platform: Platform, buffer: physis_Buffer) -> physis_Sgb {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(sgb) = Sgb::from_existing(platform, data) {
        let mut c_sections = Vec::new();

        for section in &sgb.sections {
            c_sections.push(to_c_section(section))
        }

        let sgb = physis_Sgb {
            section_count: c_sections.len() as u32,
            sections: c_sections.as_ptr(),
        };

        std::mem::forget(c_sections);

        sgb
    } else {
        physis_Sgb::default()
    }
}
