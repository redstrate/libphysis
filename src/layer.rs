// SPDX-FileCopyrightText: 2025 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_free_string, ffi_to_c_string, ffi_to_vec};
use physis::layer::LayerEntryData::*;
use physis::layer::*;
use physis::{Color, ColorIntensity};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_BgPartInstanceObject {
    pub asset_path: *const c_char,
    pub collision_asset_path: *const c_char,
    pub collision_type: ModelCollisionType,
    pub collision_attributes: CollisionAttributes,
    pub visible: bool,
    pub world_light_shadow_mode: ShadowMode,
    pub object_light_shadow_mode: ShadowMode,
    pub fade_out_distance: f32,
    pub bounding_sphere_size: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LightInstanceObject {
    pub shape: LightShape,
    pub attenuation: f32,
    pub range: f32,
    pub attenuation_cone_coefficient: f32,
    pub spot_angle: f32,
    pub texture_path: *const c_char,
    pub color: ColorIntensity,
    pub enable_specular_highlights: bool,
    pub enable_bg_parts_shadows: bool,
    pub enable_character_shadows: bool,
    pub shadow_plane_near: f32,
    pub flat_light_skew_angle: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_VfxInstanceObject {
    pub asset_path: *const c_char,
    pub soft_particle_fade_range: f32,
    pub color: Color,
    pub active: bool,
    pub unk1: bool,
    pub unk2: bool,
    pub fade_near_start: f32,
    pub fade_near_end: f32,
    pub fade_far_start: f32,
    pub fade_far_end: f32,
    pub z_correct: f32,
    pub unk3: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_GameObjectInstanceObject {
    pub base_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventObjectInstanceObject {
    pub parent_data: physis_GameObjectInstanceObject,
    pub bound_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PopRangeInstanceObject {
    pub pop_type: PopType,
    pub inner_radius_ratio: f32,
    pub position_count: u32,
    pub positions: *mut [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_CharacterInstanceObject {
    pub parent_data: physis_GameObjectInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventNpcInstanceObject {
    pub parent_data: physis_CharacterInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TriggerBoxInstanceObject {
    pub trigger_box_shape: TriggerBoxShape,
    pub priority: i16,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_MapRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
    pub map: u32,
    pub place_name_block: u32,
    pub place_name_spot: u32,
    pub weather: u32,
    pub bgm: u32,
    pub unk1: u8,
    pub unk2: u8,
    pub housing_block_id: u8,
    pub rest_bonus_effective: bool,
    pub discovery_id: u8,
    pub map_enabled: bool,
    pub place_name_enabled: bool,
    pub discovery_enabled: bool,
    pub bgm_enabled: bool,
    pub weather_enabled: bool,
    pub rest_bonus_enabled: bool,
    pub bgm_play_zone_in_only: bool,
    pub lift_enabled: bool,
    pub housing_enabled: bool,
    pub log_flying_height_max_err: bool,
    pub unk4: bool,
    pub mounts_and_ornaments_disabled: bool,
    pub lalafells_only: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SharedGroupInstanceObject {
    pub asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_AetheryteInstanceObject {
    pub parent_data: physis_GameObjectInstanceObject,
    pub bound_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ExitRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
    pub exit_type: ExitType,
    pub zone_id: u16,
    pub territory_type: u16,
    pub index: i32,
    pub destination_instance_id: u32,
    pub return_instance_id: u32,
    pub player_running_direction: f32,
    pub unk9c: u16,
    pub unk_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ChairMarkerInstanceObject {
    left_enable: bool,
    right_enable: bool,
    back_enable: bool,
    chair_type: ChairType,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PrefetchRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
    pub bound_instance_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EnvSetInstanceObject {
    pub asset_path: *const c_char,
    pub bound_instance_id: u32,
    pub shape: EnvSetShape,
    pub is_env_map_shooting_point: bool,
    pub priority: u8,
    pub effective_range: f32,
    pub interpolation_time: i32,
    pub reverb: f32,
    pub filter: f32,
    pub sound_asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EnvLocationObject {
    pub ambient_light_asset_path: *const c_char,
    pub env_map_asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SoundInstanceObject {
    pub asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_CollisionBoxInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
    pub collision_attributes: CollisionAttributes,
    pub layer_mask_is_43h: bool,
    pub collision_asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_DoorRangeInstanceObject {
    pub parent_data: physis_RangeInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_RangeInstanceObject {
    pub shape: RangeShape,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LineVFXInstanceObject {
    pub line_style: LineStyle,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TreasureInstanceObject {
    pub parent_data: physis_GameObjectInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TargetMarkerInstanceObject {
    pub target_market_type: TargetMarkerType,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PathControlPoint {
    pub position: [f32; 3],
    pub point_id: u16,
    pub select: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_PathInstanceObject {
    pub control_point_count: u32,
    pub control_points: *mut physis_PathControlPoint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ClientPathInstanceObject {
    pub parent_data: physis_PathInstanceObject,
    pub unk1: bool,
    pub unk2: bool,
    pub unk3: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]

pub struct physis_CullingBoxInstanceObject {}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ClickableRangeInstanceObject {
    pub parent_data: physis_RangeInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_BattleNpcInstanceObject {
    pub parent_data: physis_CharacterInstanceObject,
    pub name_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_DecalInstanceObject {
    pub asset_path: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_VolumetricCloudInstanceObject {
    pub asset_path: *const c_char,
    pub color: ColorIntensity,
    pub active: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ShowHideRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EventEffectRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_WaterRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
    pub enabled: bool,
    pub unk2: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_GameContentsRangeInstanceObject {
    pub parent_data: physis_TriggerBoxInstanceObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_FateRangeInstanceObject {
    pub parent_data: physis_RangeInstanceObject,
    pub fate_layout_label_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum physis_LayerEntry {
    Unknown,
    BgPart(physis_BgPartInstanceObject),
    Light(physis_LightInstanceObject),
    Vfx(physis_VfxInstanceObject),
    EventObject(physis_EventObjectInstanceObject),
    PopRange(physis_PopRangeInstanceObject),
    EventNpc(physis_EventNpcInstanceObject),
    MapRange(physis_MapRangeInstanceObject),
    SharedGroup(physis_SharedGroupInstanceObject),
    Aetheryte(physis_AetheryteInstanceObject),
    ExitRange(physis_ExitRangeInstanceObject),
    EventRange(physis_EventRangeInstanceObject),
    ChairMarker(physis_ChairMarkerInstanceObject),
    PrefetchRange(physis_PrefetchRangeInstanceObject),
    EnvSet(physis_EnvSetInstanceObject),
    EnvLocation(physis_EnvLocationObject),
    Sound(physis_SoundInstanceObject),
    CollisionBox(physis_CollisionBoxInstanceObject),
    DoorRange(physis_DoorRangeInstanceObject),
    LineVFX(physis_LineVFXInstanceObject),
    Treasure(physis_TreasureInstanceObject),
    TargetMarker(physis_TargetMarkerInstanceObject),
    ClientPath(physis_ClientPathInstanceObject),
    CullingBox(physis_CullingBoxInstanceObject),
    ClickableRange(physis_ClickableRangeInstanceObject),
    BattleNpc(physis_BattleNpcInstanceObject),
    Decal(physis_DecalInstanceObject),
    VolumetricCloud(physis_VolumetricCloudInstanceObject),
    ShowHideRange(physis_ShowHideRangeInstanceObject),
    EventEffectRange(physis_EventEffectRangeInstanceObject),
    WaterRange(physis_WaterRangeInstanceObject),
    GameContentsRange(physis_GameContentsRangeInstanceObject),
    FateRange(physis_FateRangeInstanceObject),
    SphereCastRange(),
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
    pub objects: *mut physis_InstanceObject,
    pub num_objects: u32,
    pub name: *const c_char,
    pub id: u32,
    pub festival_id: u16,
    pub festival_phase_id: u16,
    pub layer_set_referenced_list: physis_LayerSetReferencedList,
    pub visible: bool,
    pub object_set_referenced_count: u32,
    pub object_set_referenced: *mut physis_ObjectSetReferenced,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_LayerSetReferencedList {
    pub referenced_type: LayerSetReferencedType,
    pub layer_set_id_count: u32,
    pub layer_set_ids: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ObjectSetReferenced {
    pub asset_type: LayerEntryType,
    pub instance_id: u32,
    pub obsb_path: *const c_char,
}

fn convert_gameinstanceobject(obj: &GameObjectInstanceObject) -> physis_GameObjectInstanceObject {
    physis_GameObjectInstanceObject {
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
        BgPart(bg) => physis_LayerEntry::BgPart(physis_BgPartInstanceObject {
            asset_path: ffi_to_c_string(&bg.asset_path.value),
            collision_asset_path: ffi_to_c_string(&bg.collision_asset_path.value),
            collision_type: bg.collision_type,
            collision_attributes: bg.collision_attributes,
            visible: bg.visible,
            world_light_shadow_mode: bg.world_light_shadow_mode,
            object_light_shadow_mode: bg.object_light_shadow_mode,
            fade_out_distance: bg.fade_out_distance,
            bounding_sphere_size: bg.bounding_sphere_size,
        }),
        Light(light) => physis_LayerEntry::Light(physis_LightInstanceObject {
            shape: light.shape,
            color: light.color,
            attenuation: light.attenuation,
            range: light.range,
            attenuation_cone_coefficient: light.attenuation_cone_coefficient,
            spot_angle: light.spot_angle,
            texture_path: ffi_to_c_string(&light.texture_path.value),
            enable_specular_highlights: light.enable_specular_highlights,
            enable_bg_parts_shadows: light.enable_bg_part_shadows,
            enable_character_shadows: light.enable_character_shadows,
            shadow_plane_near: light.shadow_plane_near,
            flat_light_skew_angle: light.flat_light_skew_angle,
        }),
        Vfx(vfx) => physis_LayerEntry::Vfx(physis_VfxInstanceObject {
            asset_path: ffi_to_c_string(&vfx.asset_path.value),
            soft_particle_fade_range: vfx.soft_particle_fade_range,
            color: vfx.color,
            active: vfx.active,
            unk1: vfx.unk1,
            unk2: vfx.unk2,
            fade_near_start: vfx.fade_near_start,
            fade_near_end: vfx.fade_near_end,
            fade_far_start: vfx.fade_far_start,
            fade_far_end: vfx.fade_far_end,
            z_correct: vfx.z_correct,
            unk3: vfx.unk3,
        }),
        EventObject(eobj) => physis_LayerEntry::EventObject(physis_EventObjectInstanceObject {
            parent_data: convert_gameinstanceobject(&eobj.parent_data),
            bound_instance_id: eobj.bound_instance_id,
        }),
        PopRange(pop) => {
            let mut c_pos = pop.positions.clone();

            let c_pop = physis_LayerEntry::PopRange(physis_PopRangeInstanceObject {
                pop_type: pop.pop_type,
                inner_radius_ratio: pop.inner_radius_ratio,
                position_count: c_pos.len() as u32,
                positions: c_pos.as_mut_ptr(),
            });

            std::mem::forget(c_pos);

            c_pop
        }
        EventNPC(enpc) => physis_LayerEntry::EventNpc(physis_EventNpcInstanceObject {
            parent_data: physis_CharacterInstanceObject {
                parent_data: convert_gameinstanceobject(&enpc.parent_data.parent_data),
            },
        }),
        MapRange(map_range) => physis_LayerEntry::MapRange(physis_MapRangeInstanceObject {
            parent_data: convert_triggerboxinstanceobject(&map_range.parent_data),
            map: map_range.map,
            place_name_block: map_range.place_name_block,
            place_name_spot: map_range.place_name_spot,
            weather: map_range.weather,
            bgm: map_range.bgm,
            unk1: map_range.unk1,
            unk2: map_range.unk2,
            housing_block_id: map_range.housing_block_id,
            rest_bonus_effective: map_range.rest_bonus_effective,
            discovery_id: map_range.discovery_id,
            map_enabled: map_range.map_enabled,
            place_name_enabled: map_range.place_name_enabled,
            discovery_enabled: map_range.discovery_enabled,
            bgm_enabled: map_range.bgm_enabled,
            weather_enabled: map_range.weather_enabled,
            rest_bonus_enabled: map_range.rest_bonus_enabled,
            bgm_play_zone_in_only: map_range.bgm_play_zone_in_only,
            lift_enabled: map_range.lift_enabled,
            housing_enabled: map_range.housing_enabled,
            log_flying_height_max_err: map_range.log_flying_height_max_err,
            unk4: map_range.unk4,
            mounts_and_ornaments_disabled: map_range.mounts_and_ornaments_disabled,
            lalafells_only: map_range.lalafells_only,
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
            index: exit_range.index,
            player_running_direction: exit_range.player_running_direction,
            unk9c: exit_range.unk9c,
            unk_instance_id: exit_range.unk_instance_id,
        }),
        EventRange(event_range) => physis_LayerEntry::EventRange(physis_EventRangeInstanceObject {
            parent_data: convert_triggerboxinstanceobject(&event_range.parent_data),
        }),
        ChairMarker(chair_marker) => {
            physis_LayerEntry::ChairMarker(physis_ChairMarkerInstanceObject {
                back_enable: chair_marker.back_enable,
                left_enable: chair_marker.left_enable,
                right_enable: chair_marker.right_enable,
                chair_type: chair_marker.chair_type,
            })
        }
        PrefetchRange(prefetch_range) => {
            physis_LayerEntry::PrefetchRange(physis_PrefetchRangeInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&prefetch_range.parent_data),
                bound_instance_id: prefetch_range.bound_instance_id,
            })
        }
        EnvSpace(env_set) => physis_LayerEntry::EnvSet(physis_EnvSetInstanceObject {
            asset_path: ffi_to_c_string(&env_set.asset_path.value),
            bound_instance_id: env_set.bound_instance_id,
            shape: env_set.shape,
            is_env_map_shooting_point: env_set.is_env_map_shooting_point,
            priority: env_set.priority,
            effective_range: env_set.effective_range,
            interpolation_time: env_set.interpolation_time,
            reverb: env_set.reverb,
            filter: env_set.filter,
            sound_asset_path: ffi_to_c_string(&env_set.sound_asset_path.value),
        }),
        EnvLocation(env_location) => physis_LayerEntry::EnvLocation(physis_EnvLocationObject {
            ambient_light_asset_path: ffi_to_c_string(&env_location.ambient_light_asset_path.value),
            env_map_asset_path: ffi_to_c_string(&env_location.env_map_asset_path.value),
        }),
        Sound(sound) => physis_LayerEntry::Sound(physis_SoundInstanceObject {
            asset_path: ffi_to_c_string(&sound.asset_path.value),
        }),
        CollisionBox(collision_box) => {
            physis_LayerEntry::CollisionBox(physis_CollisionBoxInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&collision_box.parent_data),
                collision_attributes: collision_box.collision_attributes,
                layer_mask_is_43h: collision_box.layer_mask_is_43h,
                collision_asset_path: ffi_to_c_string(&collision_box.collision_asset_path.value),
            })
        }
        DoorRange(door_range) => physis_LayerEntry::DoorRange(physis_DoorRangeInstanceObject {
            parent_data: physis_RangeInstanceObject {
                shape: door_range.parent_data.shape,
            },
        }),
        LineVFX(line_vfx) => physis_LayerEntry::LineVFX(physis_LineVFXInstanceObject {
            line_style: line_vfx.line_style,
        }),
        Treasure(treasure) => physis_LayerEntry::Treasure(physis_TreasureInstanceObject {
            parent_data: convert_gameinstanceobject(&treasure.parent_data),
        }),
        TargetMarker(target_marker) => {
            physis_LayerEntry::TargetMarker(physis_TargetMarkerInstanceObject {
                target_market_type: target_marker.target_marker_type,
            })
        }
        ClientPath(client_path) => {
            let mut c_points = Vec::new();
            for point in &client_path.parent_data.control_points {
                c_points.push(physis_PathControlPoint {
                    position: point.position,
                    point_id: point.point_id,
                    select: point.select,
                });
            }

            let object = physis_LayerEntry::ClientPath(physis_ClientPathInstanceObject {
                parent_data: physis_PathInstanceObject {
                    control_point_count: c_points.len() as u32,
                    control_points: c_points.as_mut_ptr(),
                },
                unk1: client_path.unk1,
                unk2: client_path.unk2,
                unk3: client_path.unk3,
            });

            std::mem::forget(c_points);

            object
        }
        CullingBox(_) => physis_LayerEntry::CullingBox(physis_CullingBoxInstanceObject {}),
        ClickableRange(clickable_range) => {
            physis_LayerEntry::ClickableRange(physis_ClickableRangeInstanceObject {
                parent_data: physis_RangeInstanceObject {
                    shape: clickable_range.parent_data.shape,
                },
            })
        }
        BattleNPC(bnpc) => physis_LayerEntry::BattleNpc(physis_BattleNpcInstanceObject {
            parent_data: physis_CharacterInstanceObject {
                parent_data: convert_gameinstanceobject(&bnpc.parent_data.parent_data),
            },
            name_id: bnpc.name_id,
        }),
        Decal(decal) => physis_LayerEntry::Decal(physis_DecalInstanceObject {
            asset_path: ffi_to_c_string(&decal.asset_path.value),
        }),
        VolumetricCloud(cloud) => {
            physis_LayerEntry::VolumetricCloud(physis_VolumetricCloudInstanceObject {
                asset_path: ffi_to_c_string(&cloud.asset_path.value),
                color: cloud.color,
                active: cloud.active,
            })
        }
        ShowHideRange(collider) => {
            physis_LayerEntry::ShowHideRange(physis_ShowHideRangeInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&collider.parent_data),
            })
        }
        EventEffectRange(collider) => {
            physis_LayerEntry::EventEffectRange(physis_EventEffectRangeInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&collider.parent_data),
            })
        }
        WaterRange(collider) => physis_LayerEntry::WaterRange(physis_WaterRangeInstanceObject {
            parent_data: convert_triggerboxinstanceobject(&collider.parent_data),
            enabled: collider.enabled,
            unk2: collider.unk2,
        }),
        GameContentsRange(collider) => {
            physis_LayerEntry::GameContentsRange(physis_GameContentsRangeInstanceObject {
                parent_data: convert_triggerboxinstanceobject(&collider.parent_data),
            })
        }
        FateRange(range) => physis_LayerEntry::FateRange(physis_FateRangeInstanceObject {
            parent_data: physis_RangeInstanceObject {
                shape: range.parent_data.shape,
            },
            fate_layout_label_id: range.fate_layout_label_id,
        }),
        SphereCastRange() => physis_LayerEntry::SphereCastRange(),
        _ => physis_LayerEntry::Unknown,
    }
}

pub(crate) fn to_c_layer(layer: &Layer) -> physis_Layer {
    let mut c_objects = vec![];

    for object in &layer.objects {
        c_objects.push(physis_InstanceObject {
            instance_id: object.instance_id,
            name: ffi_to_c_string(&object.name.value),
            transform: object.transform,
            data: convert_data(&object.data),
        });
    }

    let mut c_layer_sets = layer.header.layer_set_referenced_list.layer_set_ids.clone();

    let layer_set_referenced_list = physis_LayerSetReferencedList {
        referenced_type: layer.header.layer_set_referenced_list.referenced_type,
        layer_set_id_count: c_layer_sets.len() as u32,
        layer_set_ids: c_layer_sets.as_mut_ptr(),
    };

    std::mem::forget(c_layer_sets);

    let mut c_obsb = Vec::new();
    for obsb in &layer.header.object_set_referenced {
        c_obsb.push(physis_ObjectSetReferenced {
            asset_type: obsb.asset_type,
            instance_id: obsb.instance_id,
            obsb_path: ffi_to_c_string(&obsb.obsb_path.value),
        });
    }

    let layer = physis_Layer {
        objects: c_objects.as_mut_ptr(),
        num_objects: c_objects.len() as u32,
        name: ffi_to_c_string(&layer.header.name.value),
        id: layer.header.layer_id,
        festival_id: layer.header.festival_id,
        festival_phase_id: layer.header.festival_phase_id,
        layer_set_referenced_list,
        visible: layer.header.visible,
        object_set_referenced_count: c_obsb.len() as u32,
        object_set_referenced: c_obsb.as_mut_ptr(),
    };

    std::mem::forget(c_objects);
    std::mem::forget(c_obsb);

    layer
}

pub(crate) fn free_layer(layer: &physis_Layer) {
    let data = ffi_to_vec(layer.objects, layer.num_objects);
    for object in &data {
        ffi_free_string(object.name);

        match &object.data {
            physis_LayerEntry::Unknown => {}
            physis_LayerEntry::BgPart(bg) => {
                ffi_free_string(bg.asset_path);
                ffi_free_string(bg.collision_asset_path);
            }
            physis_LayerEntry::Light(_) => {}
            physis_LayerEntry::Vfx(vfx) => {
                ffi_free_string(vfx.asset_path);
            }
            physis_LayerEntry::EventObject(_) => {}
            physis_LayerEntry::PopRange(_) => {} // TODO: free relative positions
            physis_LayerEntry::EventNpc(_) => {}
            physis_LayerEntry::MapRange(_) => {}
            physis_LayerEntry::SharedGroup(sgb) => {
                ffi_free_string(sgb.asset_path);
            }
            physis_LayerEntry::Aetheryte(_) => {}
            physis_LayerEntry::ExitRange(_) => {}
            physis_LayerEntry::EventRange(_) => {}
            physis_LayerEntry::ChairMarker(_) => {}
            physis_LayerEntry::PrefetchRange(_) => {}
            physis_LayerEntry::EnvSet(env_set) => {
                ffi_free_string(env_set.asset_path);
                ffi_free_string(env_set.sound_asset_path);
            }
            physis_LayerEntry::EnvLocation(env_location) => {
                ffi_free_string(env_location.ambient_light_asset_path);
                ffi_free_string(env_location.env_map_asset_path);
            }
            physis_LayerEntry::Sound(sound) => {
                ffi_free_string(sound.asset_path);
            }
            physis_LayerEntry::CollisionBox(_) => {}
            physis_LayerEntry::DoorRange(_) => {}
            physis_LayerEntry::LineVFX(_) => {}
            physis_LayerEntry::Treasure(_) => {}
            physis_LayerEntry::TargetMarker(_) => {}
            physis_LayerEntry::ClientPath(_) => {}
            physis_LayerEntry::CullingBox(_) => {}
            physis_LayerEntry::ClickableRange(_) => {}
            physis_LayerEntry::BattleNpc(_) => {}
            physis_LayerEntry::Decal(decal) => {
                ffi_free_string(decal.asset_path);
            }
            physis_LayerEntry::VolumetricCloud(cloud) => {
                ffi_free_string(cloud.asset_path);
            }
            physis_LayerEntry::ShowHideRange(_) => {}
            physis_LayerEntry::EventEffectRange(_) => {}
            physis_LayerEntry::WaterRange(_) => {}
            physis_LayerEntry::GameContentsRange(_) => {}
            physis_LayerEntry::FateRange(_) => {}
            physis_LayerEntry::SphereCastRange() => {}
        }
    }
    drop(data);

    ffi_free_string(layer.name);
}
