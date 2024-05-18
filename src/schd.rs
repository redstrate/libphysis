// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_from_c_string, ffi_to_c_string, physis_Buffer};
use std::ptr::{null, null_mut};
use std::{mem, slice};
use std::ffi::c_char;
use physis::schd::ShaderStage;
use physis::schd::Schd;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SCHD {
    p_ptr: *mut Schd,
    shader_stage: ShaderStage
}

#[no_mangle]
pub extern "C" fn physis_parse_schd(buffer: physis_Buffer) -> physis_SCHD {
    physis_SCHD {
        p_ptr: null_mut(),
        shader_stage: ShaderStage::Vertex
    }
}