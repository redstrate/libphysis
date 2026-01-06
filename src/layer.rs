// SPDX-FileCopyrightText: 2025 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::raw::c_char;

use crate::ffi_to_c_string;
use physis::layer::LayerEntryData::{
    Aetheryte, BG, ChairMarker, EventNPC, EventObject, EventRange, ExitRange, MapRange, PopRange,
    PrefetchRange, SharedGroup,
};
use physis::layer::*;

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
pub struct physis_SharedGroupInstanceObject {
    asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_AetheryteInstanceObject {
    parent_data: physis_GameInstanceObject,
    bound_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ExitRangeInstanceObject {
    parent_data: physis_TriggerBoxInstanceObject,
    exit_type: ExitType,
    zone_id: u16,
    territory_type: u16,
    destination_instance_id: u32,
    return_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventRangeInstanceObject {
    parent_data: physis_TriggerBoxInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ChairMarkerInstanceObject {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PrefetchRangeInstanceObject {
    parent_data: physis_TriggerBoxInstanceObject,
    bound_instance_id: u32,
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
    SharedGroup(physis_SharedGroupInstanceObject),
    Aetheryte(physis_AetheryteInstanceObject),
    ExitRange(physis_ExitRangeInstanceObject),
    EventRange(physis_EventRangeInstanceObject),
    ChairMarker(physis_ChairMarkerInstanceObject),
    PrefetchRange(physis_PrefetchRangeInstanceObject),
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_InstanceObject {
    pub instance_id: u32,
    pub name: *const c_char,
    pub transform: Transformation,
    pub data: physis_LayerEntry,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Layer {
    pub objects: *const physis_InstanceObject,
    pub num_objects: u32,
    pub name: *const c_char,
    pub id: u32,
    pub festival_id: u16,
    pub festival_phase_id: u16,
}

fn convert_gameinstanceobject(obj: &GameInstanceObject) -> physis_GameInstanceObject {
    physis_GameInstanceObject {
        base_id: obj.base_id,
    }
}

fn convert_triggerboxinstanceobject(
    obj: &TriggerBoxInstanceObject,
) -> physis_TriggerBoxInstanceObject {
    physis_TriggerBoxInstanceObject {
        trigger_box_shape: obj.trigger_box_shape,
        priority: obj.priority,
        enabled: obj.enabled,
    }
}

pub(crate) fn convert_data(data: &LayerEntryData) -> physis_LayerEntry {
    match data {
        BG(bg) => physis_LayerEntry::BG(physis_BGInstanceObject {
            asset_path: ffi_to_c_string(&bg.asset_path.value),
            collision_asset_path: ffi_to_c_string(&bg.collision_asset_path.value),
        }),
        EventObject(eobj) => physis_LayerEntry::EventObject(physis_EventInstanceObject {
            parent_data: convert_gameinstanceobject(&eobj.parent_data),
            bound_instance_id: eobj.bound_instance_id,
            linked_instance_id: eobj.linked_instance_id,
        }),
        PopRange(pop) => physis_LayerEntry::PopRange(physis_PopRangeInstanceObject {
            pop_type: pop.pop_type,
            index: pop.index,
        }),
        EventNPC(enpc) => physis_LayerEntry::EventNPC(physis_ENPCInstanceObject {
            parent_data: physis_NPCInstanceObject {
                parent_data: convert_gameinstanceobject(&enpc.parent_data.parent_data),
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
            parent_data: convert_triggerboxinstanceobject(&map_range.parent_data),
            place_name_block: map_range.place_name_block,
            place_name_spot: map_range.place_name_spot,
            rest_bonus_effective: map_range.rest_bonus_effective,
            discovery_id: map_range.discovery_id,
            place_name_enabled: map_range.place_name_enabled,
            discovery_enabled: map_range.discovery_enabled,
            rest_bonus_enabled: map_range.rest_bonus_enabled,
        }),
        SharedGroup(shared_group) => {
            physis_LayerEntry::SharedGroup(physis_SharedGroupInstanceObject {
                asset_path: ffi_to_c_string(&shared_group.asset_path.value),
            })
        }
        Aetheryte(aetheryte) => physis_LayerEntry::Aetheryte(physis_AetheryteInstanceObject {
            parent_data: convert_gameinstanceobject(&aetheryte.parent_data),
            bound_instance_id: aetheryte.bound_instance_id,
        }),
        ExitRange(exit_range) => physis_LayerEntry::ExitRange(physis_ExitRangeInstanceObject {
            parent_data: convert_triggerboxinstanceobject(&exit_range.parent_data),
            exit_type: exit_range.exit_type,
            zone_id: exit_range.zone_id,
            territory_type: exit_range.territory_type,
            destination_instance_id: exit_range.destination_instance_id,
            return_instance_id: exit_range.return_instance_id,
        }),
        EventRange(event_range) => physis_LayerEntry::EventRange(physis_EventRangeInstanceObject {
            parent_data: convert_triggerboxinstanceobject(&event_range.parent_data),
        }),
        ChairMarker(_) => physis_LayerEntry::ChairMarker(physis_ChairMarkerInstanceObject {}),
        PrefetchRange(prefetch_range) => {
            physis_LayerEntry::PrefetchRange(physis_PrefetchRangeInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&prefetch_range.parent_data),
                bound_instance_id: prefetch_range.bound_instance_id,
            })
        }
        _ => physis_LayerEntry::None,
    }
}
