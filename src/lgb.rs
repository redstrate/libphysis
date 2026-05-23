// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::layer::{
    free_layer, physis_GameInstanceObject, physis_InstanceObject, physis_Layer, physis_LayerEntry,
    to_c_layer,
};
use crate::{ffi_from_c_string, ffi_to_c_string, ffi_to_vec, physis_Buffer};
use physis::ReadableFile;
use physis::layer::{
    BGInstanceObject, EventInstanceObject, GameInstanceObject, InstanceObject, Layer,
    LayerEntryData, LayerHeader, LightInstanceObject, SharedGroupInstance,
    VFXInstanceObject,
};
use physis::lgb::{LayerChunk, Lgb};
use physis::{Platform, WritableFile};
use std::ffi::c_char;
use std::ptr::{null, null_mut};
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LayerChunk {
    layer_group_id: i32,
    name: *const c_char,
    layers: *mut physis_Layer,
    num_layers: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LayerGroup {
    chunks: *mut physis_LayerChunk,
    num_chunks: u32,
}

impl Default for physis_LayerGroup {
    fn default() -> Self {
        Self {
            chunks: null_mut(),
            num_chunks: 0,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_lgb_parse(platform: Platform, buffer: physis_Buffer) -> physis_LayerGroup {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(lgb) = Lgb::from_existing(platform, data) {
        let mut c_chunks = vec![];

        for chunk in &lgb.chunks {
            let mut c_layers = vec![];

            for layer in &chunk.layers {
                c_layers.push(to_c_layer(layer));
            }

            c_chunks.push(physis_LayerChunk {
                layer_group_id: chunk.layer_group_id,
                name: ffi_to_c_string(&chunk.name),
                layers: c_layers.as_mut_ptr(),
                num_layers: c_layers.len() as u32,
            });

            std::mem::forget(c_layers);
        }

        let lgb = physis_LayerGroup {
            chunks: c_chunks.as_mut_ptr(),
            num_chunks: c_chunks.len() as u32,
        };

        std::mem::forget(c_chunks);

        lgb
    } else {
        physis_LayerGroup::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_lgb_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(lgb) = Lgb::from_existing(platform, data) {
        ffi_to_c_string(&format!("{lgb:#?}"))
    } else {
        null()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_lgb_free(lgb: &physis_LayerGroup) {
    if lgb.chunks.is_null() {
        return;
    }

    let data = ffi_to_vec(lgb.chunks, lgb.num_chunks);
    for chunk in &data {
        let data = ffi_to_vec(chunk.layers, chunk.num_layers);
        for layer in &data {
            free_layer(layer);
        }
        drop(data);
    }
    drop(data);
}

fn to_rust_parent_data_game(parent_data: physis_GameInstanceObject) -> GameInstanceObject {
    GameInstanceObject {
        base_id: parent_data.base_id,
    }
}

fn to_rust_object(object: &physis_InstanceObject) -> InstanceObject {
    let data = match object.data {
        physis_LayerEntry::BG(bg) => LayerEntryData::BG(BGInstanceObject {
            asset_path: ffi_from_c_string(bg.asset_path).unwrap().as_str().into(),
            collision_asset_path: ffi_from_c_string(bg.collision_asset_path)
                .unwrap()
                .as_str()
                .into(),
            ..Default::default()
        }),
        physis_LayerEntry::LayLight(light) => LayerEntryData::LayLight(LightInstanceObject {
            light_type: light.light_type,
            diffuse_color_hdri: light.diffuse_color_hdri,
            ..Default::default()
        }),
        physis_LayerEntry::Vfx(vfx) => LayerEntryData::Vfx(VFXInstanceObject {
            asset_path: ffi_from_c_string(vfx.asset_path).unwrap().as_str().into(),
            ..Default::default()
        }),
        physis_LayerEntry::EventObject(eobj) => LayerEntryData::EventObject(EventInstanceObject {
            parent_data: to_rust_parent_data_game(eobj.parent_data),
            bound_instance_id: eobj.bound_instance_id,
            linked_instance_id: eobj.linked_instance_id,
        }),
        physis_LayerEntry::SharedGroup(sgb) => LayerEntryData::SharedGroup(SharedGroupInstance {
            asset_path: ffi_from_c_string(sgb.asset_path).unwrap().as_str().into(),
            ..Default::default()
        }),
        _ => LayerEntryData::None, // TODO
    };

    InstanceObject {
        instance_id: object.instance_id,
        name: ffi_from_c_string(object.name).unwrap().as_str().into(),
        transform: object.transform,
        data,
    }
}

unsafe fn to_rust_layer(layer: &physis_Layer) -> Layer {
    unsafe {
        let mut objects = Vec::new();
        for i in 0..layer.num_objects {
            objects.push(to_rust_object(&*layer.objects.add(i as usize)));
        }

        Layer {
            header: LayerHeader {
                layer_id: layer.id,
                name: ffi_from_c_string(layer.name).unwrap().as_str().into(),
                festival_id: layer.festival_id,
                festival_phase_id: layer.festival_phase_id,
                ..Default::default()
            },
            objects,
        }
    }
}

unsafe fn to_rust_chunk(chunk: &physis_LayerChunk) -> LayerChunk {
    unsafe {
        let mut layers = Vec::new();
        for i in 0..chunk.num_layers {
            layers.push(to_rust_layer(&*chunk.layers.add(i as usize)));
        }

        LayerChunk {
            layer_group_id: chunk.layer_group_id,
            name: ffi_from_c_string(chunk.name).unwrap(),
            layers,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_lgb_write_to_buffer(
    platform: Platform,
    layer_group: physis_LayerGroup,
) -> physis_Buffer {
    unsafe {
        let mut chunks = Vec::new();
        for i in 0..layer_group.num_chunks {
            chunks.push(to_rust_chunk(&*layer_group.chunks.add(i as usize)));
        }

        let lgb = Lgb { chunks };

        if let Ok(mut d) = lgb.write_to_buffer(platform) {
            let b = physis_Buffer {
                size: d.len() as u32,
                data: d.as_mut_ptr(),
            };

            mem::forget(d);

            return b;
        }

        physis_Buffer::default()
    }
}
