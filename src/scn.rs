// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::layer::{free_layer, physis_Layer, to_c_layer};
use crate::tmb::{physis_Tmb, physis_tmb_free, to_c_tmb};
use crate::{ffi_free_string, ffi_to_c_string, ffi_to_vec};
use physis::scn::ScnSGActionControllerDescriptor;
use physis::scn::{ScnLayerGroup, ScnSection, ScnTimeline, ScnTimelineInstance};
use std::ffi::c_char;

#[repr(C)]
pub struct physis_ScnSection {
    num_layer_groups: u32,
    layer_groups: *mut physis_ScnLayerGroup,

    general: physis_ScnGeneralSection,
    timelines: physis_ScnTimelinesSection,

    num_lgb_paths: u32,
    lgb_paths: *mut *const c_char,

    action_descriptors: physis_ScnSGActionDescriptors,
}

#[repr(C)]
pub struct physis_ScnGeneralSection {
    bg_path: *const c_char,
}

#[repr(C)]
pub struct physis_ScnTimelinesSection {
    timeline_count: u32,
    timelines: *mut physis_ScnTimeline,
}

#[repr(C)]
pub struct physis_ScnTimeline {
    sub_id: u32,
    animation_type: *const c_char,
    tmb: physis_Tmb,
    instance_count: u32,
    instances: *mut ScnTimelineInstance,
}

#[repr(C)]
pub struct physis_ScnLayerGroup {
    layer_group_id: u32,
    name: *const c_char,
    layer_count: u32,
    layers: *mut physis_Layer,
}

#[repr(C)]
pub struct physis_ScnSGActionDescriptors {
    descriptor_count: u32,
    descriptors: *mut ScnSGActionControllerDescriptor,
}

pub fn to_c_section(section: &ScnSection) -> physis_ScnSection {
    let mut c_layer_groups = Vec::new();
    for layer_group in &section.layer_groups {
        c_layer_groups.push(to_c_layer_group(layer_group));
    }

    let mut c_lgb_paths = Vec::new();
    for lgb_path in &section.lgb_paths {
        c_lgb_paths.push(ffi_to_c_string(lgb_path));
    }

    let general = physis_ScnGeneralSection {
        bg_path: ffi_to_c_string(&section.general.bg_path.value),
    };

    let mut c_timelines = Vec::new();
    for timeline in &section.timelines.timelines {
        c_timelines.push(to_c_timeline(timeline));
    }

    let timelines = physis_ScnTimelinesSection {
        timeline_count: c_timelines.len() as u32,
        timelines: c_timelines.as_mut_ptr(),
    };

    let mut c_descriptors = section.action_descriptors.descriptors.clone();

    let action_descriptors = physis_ScnSGActionDescriptors {
        descriptor_count: c_descriptors.len() as u32,
        descriptors: c_descriptors.as_mut_ptr(),
    };

    let scn = physis_ScnSection {
        num_layer_groups: c_layer_groups.len() as u32,
        layer_groups: c_layer_groups.as_mut_ptr(),
        general,
        timelines,
        num_lgb_paths: c_lgb_paths.len() as u32,
        lgb_paths: c_lgb_paths.as_mut_ptr(),
        action_descriptors,
    };

    std::mem::forget(c_layer_groups);
    std::mem::forget(c_lgb_paths);
    std::mem::forget(c_timelines);
    std::mem::forget(c_descriptors);

    scn
}

pub fn to_c_layer_group(section: &ScnLayerGroup) -> physis_ScnLayerGroup {
    let mut c_layers = Vec::new();

    for layer in &section.layers {
        c_layers.push(to_c_layer(layer));
    }

    let layer_group = physis_ScnLayerGroup {
        layer_group_id: section.layer_group_id,
        name: ffi_to_c_string(&section.name.value),
        layer_count: c_layers.len() as u32,
        layers: c_layers.as_mut_ptr(),
    };

    std::mem::forget(c_layers);

    layer_group
}

pub fn to_c_timeline(timeline: &ScnTimeline) -> physis_ScnTimeline {
    let mut c_instances = timeline.instances.clone();

    let c_timeline = physis_ScnTimeline {
        sub_id: timeline.sub_id,
        animation_type: ffi_to_c_string(&timeline.animation_type.value),
        tmb: to_c_tmb(&timeline.tmb),
        instance_count: c_instances.len() as u32,
        instances: c_instances.as_mut_ptr(),
    };

    std::mem::forget(c_instances);

    c_timeline
}

pub(crate) fn drop_section(section: &physis_ScnSection) {
    let data = ffi_to_vec(section.layer_groups, section.num_layer_groups);
    for group in &data {
        ffi_free_string(group.name);

        let data = ffi_to_vec(group.layers, group.layer_count);
        for layer in &data {
            free_layer(layer);
        }
        drop(data);
    }
    drop(data);

    ffi_free_string(section.general.bg_path);

    let data = ffi_to_vec(
        section.timelines.timelines,
        section.timelines.timeline_count,
    );
    for timeline in &data {
        ffi_free_string(timeline.animation_type);

        physis_tmb_free(&timeline.tmb);

        let data = ffi_to_vec(timeline.instances, timeline.instance_count);
        drop(data);
    }
    drop(data);

    let data = ffi_to_vec(section.lgb_paths, section.num_lgb_paths);
    for path in &data {
        ffi_free_string(*path);
    }
    drop(data);

    let data = ffi_to_vec(
        section.action_descriptors.descriptors,
        section.action_descriptors.descriptor_count,
    );
    drop(data);
}
