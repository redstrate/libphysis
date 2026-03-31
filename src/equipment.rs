// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_from_c_string, ffi_to_c_string};
use physis::equipment::{CharacterCategory, EquipSlotCategory};
use physis::model::MDL;
use physis::mtrl::Material;
use physis::race::{Gender, Race, Tribe};
use std::os::raw::c_char;
use std::ptr::null;

#[unsafe(no_mangle)]
pub extern "C" fn physis_slot_from_id(slot_id: i32) -> EquipSlotCategory {
    EquipSlotCategory::from_repr(slot_id as u8).unwrap_or(EquipSlotCategory::Invalid)
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_get_slot_name(slot: EquipSlotCategory) -> *const c_char {
    // TODO: no need to dynamically allocate a new string
    if let Some(abbr) = slot.abbreviation() {
        ffi_to_c_string(&abbr.to_string())
    } else {
        null()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_equipment_path(
    model_id: i32,
    race: Race,
    tribe: Tribe,
    gender: Gender,
    slot: EquipSlotCategory,
) -> *const c_char {
    ffi_to_c_string(&MDL::equipment_path(model_id, race, tribe, gender, slot))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_character_path(
    category: CharacterCategory,
    body_ver: i32,
    race: Race,
    tribe: Tribe,
    gender: Gender,
) -> *const c_char {
    ffi_to_c_string(&MDL::character_path(
        category, body_ver, race, tribe, gender,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_skin_material_path(
    race_code: i32,
    body_code: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::skin_material_path(
        race_code,
        body_code,
        &r_material_name,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_gear_material_path(
    gear_id: i32,
    gear_version: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::gear_material_path(
        gear_id,
        gear_version,
        &r_material_name,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_face_material_path(
    race_code: i32,
    face_code: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::face_material_path(
        race_code,
        face_code,
        &r_material_name,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_hair_material_path(
    race_code: i32,
    hair_code: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::hair_material_path(
        race_code,
        hair_code,
        &r_material_name,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_ear_material_path(
    race_code: i32,
    ear_code: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::ear_material_path(
        race_code,
        ear_code,
        &r_material_name,
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_build_tail_material_path(
    race_code: i32,
    tail_code: i32,
    material_name: *const c_char,
) -> *const c_char {
    let Some(r_material_name) = ffi_from_c_string(material_name) else {
        return null();
    };

    ffi_to_c_string(&Material::tail_material_path(
        race_code,
        tail_code,
        &r_material_name,
    ))
}
