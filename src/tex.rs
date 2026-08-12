// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::tex::TextureAttribute;
use physis::tex::{Texture, TextureFormat};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Texture {
    p_ptr: *mut Texture,

    attribute: TextureAttribute,
    format: TextureFormat,
    width: u16,
    height: u16,
    depth: u16,
    mip_levels: u8,
    layers: u8,
    data_size: u32,
    data: *mut u8,
}

impl Default for physis_Texture {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            attribute: TextureAttribute::MANAGED,
            format: TextureFormat::A8_UNORM,
            width: 0,
            height: 0,
            depth: 0,
            mip_levels: 0,
            layers: 0,
            data_size: 0,
            data: null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_texture_parse(
    platform: Platform,
    buffer: physis_Buffer,
) -> physis_Texture {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(texture) = Texture::from_existing(platform, data) {
        let mut texture = Box::new(texture);

        let mut tex = physis_Texture {
            p_ptr: null_mut(),
            attribute: texture.attribute,
            format: texture.format,
            width: texture.width,
            height: texture.height,
            depth: texture.depth,
            mip_levels: texture.mip_levels,
            layers: texture.layers(),
            data_size: texture.data.len() as u32,
            data: texture.data.as_mut_ptr(),
        };
        tex.p_ptr = Box::into_raw(texture);

        tex
    } else {
        physis_Texture::default()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TextureRgba {
    rgba_size: u32,
    rgba: *mut u8,
}

impl Default for physis_TextureRgba {
    fn default() -> Self {
        Self {
            rgba_size: 0,
            rgba: null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_texture_to_rgba(texture: physis_Texture) -> physis_TextureRgba {
    unsafe {
        if let Some(mut parsed) = (*texture.p_ptr).to_rgba() {
            let rgba = physis_TextureRgba {
                rgba_size: parsed.len() as u32,
                rgba: parsed.as_mut_ptr(),
            };

            mem::forget(parsed);

            rgba
        } else {
            physis_TextureRgba::default()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_tex_free(tex: &physis_Texture) {
    if tex.p_ptr.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(tex.p_ptr));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_tex_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    match Texture::from_existing(platform, data) {
        Ok(tex) => ffi_to_c_string(&format!("{tex:#?}")),
        Err(err) => ffi_to_c_string(&format!("{err:#?}")),
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TextureMipData {
    width: u16,
    height: u16,
    start: usize,
    end: usize,
}

impl Default for physis_TextureMipData {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            start: 0,
            end: 0,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_tex_mip_data(
    texture: &physis_Texture,
    level: u8,
) -> physis_TextureMipData {
    if texture.p_ptr == null_mut() {
        return physis_TextureMipData::default();
    }

    unsafe {
        if let Some((start, end)) = (*texture.p_ptr).mip_data(level) {
            let (width, height) = (*texture.p_ptr).mip_size(level);

            physis_TextureMipData {
                width,
                height,
                start,
                end,
            }
        } else {
            physis_TextureMipData::default()
        }
    }
}
