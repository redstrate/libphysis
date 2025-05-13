// SPDX-FileCopyrightText: 2025 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::raw::c_char;
use std::ptr::null;
use std::slice;

use physis::layer::*;

use crate::{ffi_to_c_string, physis_Buffer};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_InstanceObject {
    instance_id: u32,
    name: *const c_char,
    transform: Transformation,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Layer {
    objects: *const physis_InstanceObject,
    num_objects: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LayerChunk {
    layers: *const physis_Layer,
    num_layers: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LayerGroup {
    chunks: *const physis_LayerChunk,
    num_chunks: u32,
}

impl Default for physis_LayerGroup {
    fn default() -> Self {
        Self {
            chunks: null(),
            num_chunks: 0,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_layergroup_read(buffer: physis_Buffer) -> physis_LayerGroup {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(lgb) = LayerGroup::from_existing(data) {
        let mut c_chunks = vec![];

        for chunk in &lgb.chunks {
            let mut c_layers = vec![];

            for layer in &chunk.layers {
                let mut c_objects = vec![];

                for object in &layer.objects {
                    c_objects.push(physis_InstanceObject {
                        instance_id: object.instance_id,
                        name: ffi_to_c_string(&object.name),
                        transform: object.transform,
                    });
                }

                c_layers.push(physis_Layer {
                    objects: c_objects.as_ptr(),
                    num_objects: c_objects.len() as u32,
                });

                std::mem::forget(c_objects);
            }

            c_chunks.push(physis_LayerChunk {
                layers: c_layers.as_ptr(),
                num_layers: c_layers.len() as u32,
            });

            std::mem::forget(c_layers);
        }

        let lgb = physis_LayerGroup {
            chunks: c_chunks.as_ptr(),
            num_chunks: c_chunks.len() as u32,
        };

        std::mem::forget(c_chunks);

        lgb
    } else {
        physis_LayerGroup::default()
    }
}
