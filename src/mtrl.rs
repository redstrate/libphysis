// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::mtrl::ColorTableRow;
use physis::mtrl::Constant;
use physis::mtrl::Material;
use physis::mtrl::Sampler;
use physis::mtrl::ShaderKey;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ColorTable {
    num_rows: u32,
    rows: *mut ColorTableRow,
}

impl Default for physis_ColorTable {
    fn default() -> Self {
        Self {
            num_rows: 0,
            rows: null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Material {
    shpk_name: *const c_char,
    num_textures: u32,
    textures: *mut *const c_char,
    num_shader_keys: u32,
    shader_keys: *mut ShaderKey,
    num_constants: u32,
    constants: *mut Constant,
    num_samplers: u32,
    samplers: *mut Sampler,
    color_table: physis_ColorTable,
}

impl Default for physis_Material {
    fn default() -> Self {
        Self {
            shpk_name: null(),
            num_textures: 0,
            textures: null_mut(),
            num_shader_keys: 0,
            shader_keys: null_mut(),
            num_constants: 0,
            constants: null_mut(),
            num_samplers: 0,
            samplers: null_mut(),
            color_table: Default::default(),
        }
    }
}

#[no_mangle]
pub extern "C" fn physis_material_parse(buffer: physis_Buffer) -> physis_Material {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(material) = Material::from_existing(data) {
        let mut c_strings = vec![];

        for tex in &material.texture_paths {
            c_strings.push(ffi_to_c_string(tex));
        }

        let mut shader_keys = material.shader_keys.clone();
        let mut constants = material.constants.clone();
        let mut samplers = material.samplers.clone();

        let mut rows = vec![];
        if let Some(color_table) = material.color_table {
            rows.clone_from(&color_table.rows);
        }

        let mat = physis_Material {
            shpk_name: ffi_to_c_string(&material.shader_package_name),
            num_textures: c_strings.len() as u32,
            textures: c_strings.as_mut_ptr(),
            num_shader_keys: shader_keys.len() as u32,
            shader_keys: shader_keys.as_mut_ptr(),
            num_constants: constants.len() as u32,
            constants: constants.as_mut_ptr(),
            num_samplers: samplers.len() as u32,
            samplers: samplers.as_mut_ptr(),
            color_table: physis_ColorTable {
                num_rows: rows.len() as u32,
                rows: if rows.is_empty() {
                    null_mut()
                } else {
                    rows.as_mut_ptr()
                },
            },
        };

        mem::forget(c_strings);
        mem::forget(shader_keys);
        mem::forget(constants);
        mem::forget(samplers);
        mem::forget(rows);

        mat
    } else {
        physis_Material::default()
    }
}
