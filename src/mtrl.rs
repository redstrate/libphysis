// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::mtrl::Material;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Material {
    num_textures: u32,
    textures: *mut *const c_char,
}

impl Default for physis_Material {
    fn default() -> Self {
        Self {
            num_textures: 0,
            textures: null_mut(),
        }
    }
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_material_parse(buffer: physis_Buffer) -> physis_Material {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(material) = Material::from_existing(data) {
        let mut c_strings = vec![];

        for tex in &material.texture_paths {
            c_strings.push(ffi_to_c_string(tex));
        }

        let mat = physis_Material {
            num_textures: c_strings.len() as u32,
            textures: c_strings.as_mut_ptr(),
        };

        mem::forget(c_strings);

        mat
    } else {
        physis_Material::default()
    }
}
