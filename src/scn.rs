use crate::ffi_to_c_string;
use crate::layer::{physis_Layer, to_c_layer};
use physis::layer::{ScnLayerGroup, ScnSection};
use std::ffi::c_char;

#[repr(C)]
pub struct physis_ScnSection {
    num_layer_groups: u32,
    layer_groups: *const physis_ScnLayerGroup,

    general: physis_ScnGeneralSection,

    num_lgb_paths: u32,
    lgb_paths: *const *const c_char,
}

#[repr(C)]
pub struct physis_ScnGeneralSection {
    bg_path: *const c_char,
}

#[repr(C)]
pub struct physis_ScnLayerGroup {
    layer_group_id: u32,
    name: *const c_char,
    layer_count: u32,
    layers: *const physis_Layer,
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

    let scn = physis_ScnSection {
        num_layer_groups: c_layer_groups.len() as u32,
        layer_groups: c_layer_groups.as_ptr(),
        general,
        num_lgb_paths: c_lgb_paths.len() as u32,
        lgb_paths: c_lgb_paths.as_ptr(),
    };

    std::mem::forget(c_layer_groups);
    std::mem::forget(c_lgb_paths);

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
        layers: c_layers.as_ptr(),
    };

    std::mem::forget(c_layers);

    layer_group
}
