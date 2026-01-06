// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::layer::{convert_data, physis_InstanceObject, physis_Layer};
use crate::{ffi_to_c_string, physis_Buffer};
use physis::ReadableFile;
use physis::common::Platform;
use physis::layer::LayerGroup;
use std::ptr::null;
use std::slice;

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
pub extern "C" fn physis_lgb_parse(platform: Platform, buffer: physis_Buffer) -> physis_LayerGroup {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(lgb) = LayerGroup::from_existing(platform, data) {
        let mut c_chunks = vec![];

        for chunk in &lgb.chunks {
            let mut c_layers = vec![];

            for layer in &chunk.layers {
                let mut c_objects = vec![];

                for object in &layer.objects {
                    c_objects.push(physis_InstanceObject {
                        instance_id: object.instance_id,
                        name: ffi_to_c_string(&object.name.value),
                        transform: object.transform,
                        data: convert_data(&object.data),
                    });
                }

                c_layers.push(physis_Layer {
                    objects: c_objects.as_ptr(),
                    num_objects: c_objects.len() as u32,
                    name: ffi_to_c_string(&layer.header.name.value),
                    id: layer.header.layer_id,
                    festival_id: layer.header.festival_id,
                    festival_phase_id: layer.header.festival_phase_id,
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
