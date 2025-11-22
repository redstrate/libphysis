// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ptr::null_mut;

use physis::shcd::SHCD;
use physis::shcd::ShaderStage;

use crate::physis_Buffer;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SHCD {
    p_ptr: *mut SHCD,
    shader_stage: ShaderStage,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_parse_schd(_buffer: physis_Buffer) -> physis_SHCD {
    physis_SHCD {
        p_ptr: null_mut(),
        shader_stage: ShaderStage::Vertex,
    }
}
