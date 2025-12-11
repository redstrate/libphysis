// SPDX-FileCopyrightText: 2025 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::raw::c_char;
use std::ptr::null;
use std::slice;

use physis::layer::LayerEntryData::{BG, EventNPC, EventObject, MapRange, PopRange};
use physis::layer::*;

use crate::{ffi_to_c_string, physis_Buffer};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_BGInstanceObject {
    asset_path: *const c_char,
    collision_asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_GameInstanceObject {
    base_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventInstanceObject {
    parent_data: physis_GameInstanceObject,
    bound_instance_id: u32,
    linked_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PopRangeInstanceObject {
    pop_type: PopType,
    index: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_NPCInstanceObject {
    parent_data: physis_GameInstanceObject,
    pop_weather: u32,
    pop_time_start: u8,
    pop_time_end: u8,
    move_ai: u32,
    wandering_range: u8,
    route: u8,
    event_group: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ENPCInstanceObject {
    parent_data: physis_NPCInstanceObject,
    behavior: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TriggerBoxInstanceObject {
    trigger_box_shape: TriggerBoxShape,
    priority: i16,
    enabled: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_MapRangeInstanceObject {
    parent_data: physis_TriggerBoxInstanceObject,
    place_name_block: u32,
    place_name_spot: u32,
    rest_bonus_effective: bool,
    discovery_id: u8,
    place_name_enabled: bool,
    discovery_enabled: bool,
    rest_bonus_enabled: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum physis_LayerEntry {
    None, // NOTE: a thing until every layer entry is supported
    BG(physis_BGInstanceObject),
    EventObject(physis_EventInstanceObject),
    PopRange(physis_PopRangeInstanceObject),
    EventNPC(physis_ENPCInstanceObject),
    MapRange(physis_MapRangeInstanceObject),
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_InstanceObject {
    instance_id: u32,
    name: *const c_char,
    transform: Transformation,
    data: physis_LayerEntry,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Layer {
    objects: *const physis_InstanceObject,
    num_objects: u32,
    name: *const c_char,
    id: u32,
    festival_id: u16,
    festival_phase_id: u16,
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

fn convert_data(data: &LayerEntryData) -> physis_LayerEntry {
    match data {
        BG(bg) => physis_LayerEntry::BG(physis_BGInstanceObject {
            asset_path: ffi_to_c_string(&bg.asset_path.value),
            collision_asset_path: ffi_to_c_string(&bg.collision_asset_path.value),
        }),
        EventObject(eobj) => physis_LayerEntry::EventObject(physis_EventInstanceObject {
            parent_data: physis_GameInstanceObject {
                base_id: eobj.parent_data.base_id,
            },
            bound_instance_id: eobj.bound_instance_id,
            linked_instance_id: eobj.linked_instance_id,
        }),
        PopRange(pop) => physis_LayerEntry::PopRange(physis_PopRangeInstanceObject {
            pop_type: pop.pop_type,
            index: pop.index,
        }),
        EventNPC(enpc) => physis_LayerEntry::EventNPC(physis_ENPCInstanceObject {
            parent_data: physis_NPCInstanceObject {
                parent_data: physis_GameInstanceObject {
                    base_id: enpc.parent_data.parent_data.base_id,
                },
                pop_weather: enpc.parent_data.pop_weather,
                pop_time_start: enpc.parent_data.pop_time_start,
                pop_time_end: enpc.parent_data.pop_time_end,
                move_ai: enpc.parent_data.move_ai,
                wandering_range: enpc.parent_data.wandering_range,
                route: enpc.parent_data.route,
                event_group: enpc.parent_data.event_group,
            },
            behavior: enpc.behavior,
        }),
        MapRange(map_range) => physis_LayerEntry::MapRange(physis_MapRangeInstanceObject {
            parent_data: physis_TriggerBoxInstanceObject {
                trigger_box_shape: map_range.parent_data.trigger_box_shape,
                priority: map_range.parent_data.priority,
                enabled: map_range.parent_data.enabled,
            },
            place_name_block: map_range.place_name_block,
            place_name_spot: map_range.place_name_spot,
            rest_bonus_effective: map_range.rest_bonus_effective,
            discovery_id: map_range.discovery_id,
            place_name_enabled: map_range.place_name_enabled,
            discovery_enabled: map_range.discovery_enabled,
            rest_bonus_enabled: map_range.rest_bonus_enabled,
        }),
        _ => physis_LayerEntry::None,
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
