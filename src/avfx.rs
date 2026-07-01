// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::avfx::{Avfx, DrawVertex};
use std::ffi::c_char;
use std::ptr::{null, null_mut};
use std::slice;

#[repr(C)]
pub struct physis_Avfx {
    texture_count: u32,
    textures: *mut *const c_char,

    model_count: u32,
    models: *mut physis_AvfxDrawModel,
}

impl Default for physis_Avfx {
    fn default() -> Self {
        Self {
            texture_count: 0,
            textures: null_mut(),
            model_count: 0,
            models: null_mut(),
        }
    }
}

#[repr(C)]
pub struct physis_AvfxDrawModel {
    vertex_count: u32,
    vertices: *mut DrawVertex,

    index_count: u32,
    indices: *mut u16,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_avfx_parse(platform: Platform, buffer: physis_Buffer) -> physis_Avfx {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(avfx) = Avfx::from_existing(platform, data) {
        let mut c_textures = Vec::new();
        for texture in &avfx.textures {
            c_textures.push(ffi_to_c_string(&texture.path));
        }

        let mut c_models = vec![];
        for model in &avfx.models {
            let mut c_vertices = model.vertices.clone();
            let mut c_indices = Vec::new();
            for triangle in &model.triangles {
                c_indices.push(triangle.indices[0]);
                c_indices.push(triangle.indices[1]);
                c_indices.push(triangle.indices[2]);
            }

            let c_model = physis_AvfxDrawModel {
                vertex_count: c_vertices.len() as u32,
                vertices: c_vertices.as_mut_ptr(),
                index_count: c_indices.len() as u32,
                indices: c_indices.as_mut_ptr(),
            };

            std::mem::forget(c_vertices);
            std::mem::forget(c_indices);

            c_models.push(c_model);
        }

        let c_avfx = physis_Avfx {
            texture_count: c_textures.len() as u32,
            textures: c_textures.as_mut_ptr(),
            model_count: c_models.len() as u32,
            models: c_models.as_mut_ptr(),
        };

        std::mem::forget(c_textures);
        std::mem::forget(c_models);

        c_avfx
    } else {
        physis_Avfx::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_avfx_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(avfx) = Avfx::from_existing(platform, data) {
        ffi_to_c_string(&format!("{avfx:#?}"))
    } else {
        null()
    }
}
