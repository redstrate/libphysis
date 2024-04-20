// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::{mem, slice};

use physis::model::{SubMesh, Vertex, MDL};

use crate::{ffi_to_c_string, physis_Buffer};

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_Part {
    num_vertices: u32,
    vertices: *const Vertex,

    num_indices: u32,
    indices: *const u16,

    material_index: u16,

    num_submeshes: u32,
    submeshes: *const SubMesh,
}

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_LOD {
    num_parts: u32,
    parts: *const physis_Part,
}

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_MDL {
    p_ptr: *mut MDL,
    num_lod: u32,
    lods: *const physis_LOD,
    num_affected_bones: u32,
    affected_bone_names: *mut *const c_char,
    num_material_names: u32,
    material_names: *mut *const c_char,
}

impl Default for physis_MDL {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            num_lod: 0,
            lods: null(),
            num_affected_bones: 0,
            affected_bone_names: null_mut(),
            num_material_names: 0,
            material_names: null_mut(),
        }
    }
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_mdl_parse(buffer: physis_Buffer) -> physis_MDL {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    let Some(mdl_d) = MDL::from_existing(data) else {
        return physis_MDL::default();
    };

    let mdl = Box::new(mdl_d);

    let mut c_lods: Vec<physis_LOD> = physis_mdl_update_vertices(&mdl);

    let mut c_bone_names = vec![];

    for bone_name in &mdl.affected_bone_names {
        c_bone_names.push(ffi_to_c_string(bone_name));
    }

    let mut c_material_names = vec![];

    for bone_name in &mdl.material_names {
        c_material_names.push(ffi_to_c_string(bone_name));
    }

    let mdl = physis_MDL {
        p_ptr: Box::leak(mdl),
        num_lod: c_lods.len() as u32,
        lods: c_lods.as_mut_ptr(),
        num_affected_bones: c_bone_names.len() as u32,
        affected_bone_names: c_bone_names.as_mut_ptr(),
        num_material_names: c_material_names.len() as u32,
        material_names: c_material_names.as_mut_ptr(),
    };

    mem::forget(c_bone_names);
    mem::forget(c_material_names);
    mem::forget(c_lods);

    mdl
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_mdl_write(mdl: &physis_MDL) -> physis_Buffer {
    unsafe {
        let mut buffer = (*mdl.p_ptr).write_to_buffer().unwrap();

        let leak_buffer = physis_Buffer {
            size: buffer.len() as u32,
            data: buffer.as_mut_ptr(),
        };

        mem::forget(buffer);

        leak_buffer
    }
}

#[cfg(feature = "visual_data")]
fn physis_mdl_update_vertices(mdl: &MDL) -> Vec<physis_LOD> {
    let mut c_lods: Vec<physis_LOD> = Vec::new();

    for lod in &mdl.lods {
        let mut c_parts: Vec<physis_Part> = Vec::new();

        for part in &lod.parts {
            let mut c_vertices: Vec<Vertex> = part.vertices.clone();
            let mut c_indices: Vec<u16> = part.indices.clone();
            let mut c_submeshes: Vec<SubMesh> = part.submeshes.clone();

            c_parts.push(physis_Part {
                num_vertices: c_vertices.len() as u32,
                vertices: c_vertices.as_mut_ptr(),
                num_indices: c_indices.len() as u32,
                indices: c_indices.as_mut_ptr(),
                material_index: part.material_index,
                num_submeshes: c_submeshes.len() as u32,
                submeshes: c_submeshes.as_mut_ptr(),
            });

            mem::forget(c_vertices);
            mem::forget(c_indices);
            mem::forget(c_submeshes);
        }

        c_lods.push(physis_LOD {
            num_parts: c_parts.len() as u32,
            parts: c_parts.as_mut_ptr(),
        });

        mem::forget(c_parts);
    }

    c_lods
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_mdl_replace_vertices(
    mdl: *mut physis_MDL,
    lod_index: u32,
    part_index: u32,
    num_vertices: u32,
    vertices_ptr: *const Vertex,
    num_indices: u32,
    indices_ptr: *const u16,
    num_submeshes: u32,
    submeshes_ptr: *const SubMesh,
) {
    unsafe {
        (*(*mdl).p_ptr).replace_vertices(
            lod_index as usize,
            part_index as usize,
            &*std::ptr::slice_from_raw_parts(vertices_ptr, num_vertices as usize),
            &*std::ptr::slice_from_raw_parts(indices_ptr, num_indices as usize),
            &*std::ptr::slice_from_raw_parts(submeshes_ptr, num_submeshes as usize),
        );

        // We need to update the C version of these LODs as well
        let mut new_lods = physis_mdl_update_vertices((*mdl).p_ptr.as_ref().unwrap());

        (*mdl).lods = new_lods.as_mut_ptr();

        mem::forget(new_lods);
    }
}
