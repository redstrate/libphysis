extern crate core;

use std::ffi::{CStr, CString};
use std::{mem, slice};
use std::fs::read;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};
use physis::gamedata::GameData;
use physis::blowfish::Blowfish;
use physis::bootdata::BootData;
use physis::common::Language;
use physis::equipment::{build_equipment_path, get_slot_abbreviation, get_slot_from_id, Slot};
use physis::exh::EXH;
use physis::exd::{ColumnData, EXD};
use physis::installer::install_game;
use physis::model::{MDL, Vertex};
use physis::mtrl::Material;
use physis::race::{Gender, Race, Subrace};
use physis::repository::RepositoryType;
use physis::skeleton::Skeleton;
use physis::sqpack::calculate_hash;
use physis::tex::Texture;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


fn ffi_from_c_string(ptr : *const c_char) -> String {
    unsafe {
        CStr::from_ptr(ptr as *mut i8).to_str().unwrap().to_string()
    }
}

fn ffi_to_c_string(s : &String) -> *const c_char {
    let s = CString::new(s.as_bytes());

    match s {
        Ok(x) => {
            x.into_raw()
        }
        Err(_) => {
            null()
        }
    }
}

fn ffi_to_vec<T>(ptr : *mut T, count : u32) -> Vec<T> {
    unsafe {
        Vec::from_raw_parts(ptr, count as usize, count as usize)
    }
}

fn ffi_free_string(ptr : *const c_char) {
    unsafe {
        let str = CString::from_raw(ptr as *mut i8);
        drop(str);
    }
}

/// Initializes a new BootData structure. Path must be a valid boot path, or else it will return NULL.
#[no_mangle] pub extern "C" fn physis_bootdata_initialize(path : *const c_char) -> *mut BootData {
    if let Some(boot_data) = BootData::from_existing(&ffi_from_c_string(path)) {
        let boxed = Box::new(boot_data);

        Box::leak(boxed)
    } else {
        null_mut()
    }
}

#[no_mangle] pub extern "C" fn physis_bootdata_free(boot_data : *mut BootData) {
    unsafe {
        drop(Box::from_raw(boot_data));
    }
}

#[no_mangle] pub extern "C" fn physis_initialize_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

/// Initializes a new GameData structure. Path must be a valid game path, or else it will return NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_initialize(path : *const c_char) -> *mut GameData {
    if let Some(mut game_data) = GameData::from_existing(&ffi_from_c_string(path)) {
        game_data.reload_repositories();

        let boxed = Box::new(game_data);

        Box::leak(boxed)
    } else {
        null_mut()
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_free(game_data : *mut GameData) {
    unsafe {
        drop(Box::from_raw(game_data));
    }
}

#[repr(C)]
pub struct physis_Repository {
    name : *const c_char,
    repository_type : RepositoryType,
    version : *const c_char
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Repositories {
    repositories_count: u32,
    repositories: *mut physis_Repository
}

#[no_mangle] pub extern "C" fn physis_gamedata_get_repositories(game_data : &GameData) -> physis_Repositories {
    let mut c_repositories : Vec<physis_Repository> = Vec::new();

    for repository in &game_data.repositories {
        let ver = match &repository.version {
            Some(x) => {
                ffi_to_c_string(x)
            }
            None => {
                null()
            }
        };

        c_repositories.push(physis_Repository {
            name: ffi_to_c_string(&repository.name),
            repository_type: repository.repo_type,
            version: ver
        });
    }

    let repositories = physis_Repositories {
        repositories_count: c_repositories.len() as u32,
        repositories: c_repositories.as_mut_ptr()
    };

    mem::forget(c_repositories);

    repositories
}

#[no_mangle] pub extern "C" fn physis_bootdata_get_version(boot_data : &BootData) -> *const c_char {
    ffi_to_c_string(&boot_data.version)
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_repositories(repositories : physis_Repositories) {
    let data = ffi_to_vec(repositories.repositories, repositories.repositories_count);

    for repository in &data {
        ffi_free_string(repository.version);
        ffi_free_string(repository.name);
    }

    drop(data)
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_extract_file(game_data : &GameData, path : *const c_char) -> physis_Buffer {
    unsafe {
        if let Some(mut d) = game_data.extract(CStr::from_ptr(path).to_string_lossy().as_ref()) {
            let b = physis_Buffer {
                size: d.len() as u32,
                data: d.as_mut_ptr()
            };

            mem::forget(d);

            b
        } else {
            physis_Buffer {
                size: 0,
                data: null_mut()
            }
        }
    }
}

/// Checks if the file at `path` exists.
#[no_mangle] pub extern "C" fn physis_gamedata_exists(game_data : &GameData, path : *const c_char) -> bool {
    game_data.exists(&ffi_from_c_string(path))
}

#[no_mangle] pub extern "C" fn physis_gamedata_read_excel_sheet_header(game_data : &GameData, name : *const c_char) -> *mut EXH {
    if let Some(header) = game_data.read_excel_sheet_header(&ffi_from_c_string(name)) {
        let exh = Box::new(header);

        Box::leak(exh)
    } else {
        null_mut()
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_sheet_header(exh: *mut EXH) {
    unsafe {
        drop(Box::from_raw(exh));
    }
}

#[repr(C)]
pub enum physis_ColumnData {
    String(*const c_char),
    Bool(bool),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Float32(f32),
    Int64(i64),
    UInt64(u64)
}

#[repr(C)]
pub struct physis_ExcelRow {
    column_data: *mut physis_ColumnData
}

#[repr(C)]
pub struct physis_EXD {
    p_ptr : *mut EXD,
    column_count: c_uint,

    row_data : *mut physis_ExcelRow,
    row_count : c_uint
}

#[no_mangle] pub extern "C" fn physis_gamedata_read_excel_sheet(game_data : &GameData, name : *const c_char, exh : &EXH, language : Language, page : c_uint) -> physis_EXD {
    let exd = Box::new(game_data.read_excel_sheet(&ffi_from_c_string(name), exh, language, page as usize).unwrap());

    let mut c_rows : Vec<physis_ExcelRow> = Vec::new();

    for row in &exd.rows {
        let mut c_col_data : Vec<physis_ColumnData> = Vec::new();

        for col_data in &row.data {
            match col_data {
                ColumnData::String(s) => {
                    c_col_data.push(physis_ColumnData::String(ffi_to_c_string(s)))
                }
                ColumnData::Bool(b) => { c_col_data.push(physis_ColumnData::Bool(*b)) }
                ColumnData::Int8(i) => { c_col_data.push(physis_ColumnData::Int8(*i)) }
                ColumnData::UInt8(i) => { c_col_data.push(physis_ColumnData::UInt8(*i)) }
                ColumnData::Int16(i) => { c_col_data.push(physis_ColumnData::Int16(*i)) }
                ColumnData::UInt16(i) => { c_col_data.push(physis_ColumnData::UInt16(*i)) }
                ColumnData::Int32(i) => { c_col_data.push(physis_ColumnData::Int32(*i)) }
                ColumnData::UInt32(i) => { c_col_data.push(physis_ColumnData::UInt32(*i)) }
                ColumnData::Float32(i) => { c_col_data.push(physis_ColumnData::Float32(*i)) }
                ColumnData::Int64(i) => { c_col_data.push(physis_ColumnData::Int64(*i)) }
                ColumnData::UInt64(i) => { c_col_data.push(physis_ColumnData::UInt64(*i)) }
            }
        }

        c_rows.push(physis_ExcelRow {
            column_data: c_col_data.as_mut_ptr()
        });

        mem::forget(c_col_data);
    }

    let exd = physis_EXD {
        p_ptr: Box::leak(exd),
        column_count: exh.column_definitions.len() as c_uint,
        row_data: c_rows.as_mut_ptr(),
        row_count: c_rows.len() as c_uint
    };

    mem::forget(c_rows);

    exd
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_sheet(exd : physis_EXD)  {
    unsafe {
        let data = Vec::from_raw_parts(exd.row_data, exd.row_count as usize, exd.row_count as usize);

        for i in 0..exd.row_count {
            let col_data = Vec::from_raw_parts(data[i as usize].column_data, exd.column_count as usize, exd.column_count as usize);

            for col in &col_data {
                match col {
                    physis_ColumnData::String(s) => {
                        ffi_free_string(*s);
                    }
                    _ => {}
                }
            }

            drop(col_data);
        }

        drop(data);

        drop(Box::from_raw(exd.p_ptr));
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_initialize(key : *mut u8, key_size : c_uint) -> *mut Blowfish {
    let data = unsafe { slice::from_raw_parts(key, key_size as usize) };
    Box::into_raw(Box::new(Blowfish::new(&data)))
}

#[no_mangle] pub extern "C" fn physis_blowfish_free(blowfish : *mut Blowfish) {
    unsafe {
        drop(Box::from_raw(blowfish));
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_encrypt(blowfish : &Blowfish, in_data : *mut u8, in_data_size : c_uint, out_data : &mut *mut u8, out_data_size : *mut u32) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.encrypt(&*in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_decrypt(blowfish : &Blowfish, in_data : *mut u8, in_data_size : c_uint, out_data : &mut *mut u8, out_data_size : *mut u32) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.decrypt(&*in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_apply_patch(gamedata : &GameData, path : *const c_char) -> bool {
    gamedata.apply_patch(&ffi_from_c_string(path)).is_ok()
}

#[no_mangle] pub extern "C" fn physis_bootdata_apply_patch(bootdata : &BootData, path : *const c_char) -> bool {
    bootdata.apply_patch(&ffi_from_c_string(path)).is_ok()
}

#[no_mangle] pub extern "C" fn physis_install_game(installer_path : *const c_char, game_directory : *const c_char) -> bool {
    install_game(&ffi_from_c_string(installer_path), &ffi_from_c_string(game_directory)).is_ok()
}

#[repr(C)]
pub struct physis_Part {
    num_vertices : u32,
    vertices : *const Vertex,

    num_indices : u32,
    indices : *const u16
}

#[repr(C)]
pub struct physis_LOD {
    num_parts : u32,
    parts : *const physis_Part
}

#[repr(C)]
pub struct physis_MDL {
    num_lod : u32,
    lods : *const physis_LOD,
    num_affected_bones : u32,
    affected_bone_names: *mut *const c_char
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Buffer {
    size : u32,
    data: *mut u8
}

#[no_mangle] pub extern "C" fn physis_mdl_parse(size : u32, data : *mut u8) -> physis_MDL {
    let data = unsafe { slice::from_raw_parts(data, size as usize) };

    let mdl = MDL::from_existing(&data.to_vec()).unwrap();

    let mut c_lods : Vec<physis_LOD> = Vec::new();

    for lod in mdl.lods {
        let mut c_parts : Vec<physis_Part> = Vec::new();

        for part in lod.parts {
            let mut c_vertices : Vec<Vertex> = part.vertices;
            let mut c_indices : Vec<u16> = part.indices;

            c_parts.push(physis_Part {
                num_vertices: c_vertices.len() as u32,
                vertices: c_vertices.as_mut_ptr(),
                num_indices: c_indices.len() as u32,
                indices: c_indices.as_mut_ptr()
            });

            mem::forget(c_vertices);
            mem::forget(c_indices);
        }

        c_lods.push(physis_LOD {
            num_parts: c_parts.len() as u32,
            parts: c_parts.as_mut_ptr()
        });

        mem::forget(c_parts);
    }

    let mut c_bone_names = vec![];

    for bone_name in mdl.affected_bone_names {
        c_bone_names.push(ffi_to_c_string(&bone_name));
    }

    let mdl = physis_MDL {
        num_lod: c_lods.len() as u32,
        lods: c_lods.as_mut_ptr(),
        num_affected_bones : c_bone_names.len() as u32,
        affected_bone_names: c_bone_names.as_mut_ptr()
    };

    mem::forget(c_bone_names);
    mem::forget(c_lods);

    mdl
}

#[no_mangle] pub extern "C" fn physis_read_file(path : *const c_char) -> physis_Buffer {
    let mut f = read(ffi_from_c_string(path)).unwrap();
    let buf = physis_Buffer {
        size: f.len() as u32,
        data: f.as_mut_ptr()
    };

    mem::forget(f);

    buf
}

#[no_mangle] pub extern "C" fn physis_calculate_hash(path : *const c_char) -> u64 {
    calculate_hash(&ffi_from_c_string(path))
}

#[no_mangle] pub extern "C" fn physis_build_equipment_path(model_id : i32, race : Race, subrace : Subrace, gender : Gender, slot: Slot) -> *const c_char {
    ffi_to_c_string(&build_equipment_path(model_id, race, subrace, gender, slot))
}

#[no_mangle] pub extern "C" fn physis_slot_from_id(slot_id : i32) -> Slot {
    match get_slot_from_id(slot_id) {
        None => Slot::Head, // FIXME: this is currently used to cover-up the few missing slots. PLEASE DO NOT SHIP
        Some(x) => x
    }
}

#[repr(C)]
pub struct physis_Bone {
    pub index: u32,
    pub name: *const c_char,
    pub parent_bone: *mut physis_Bone,
    pub parent_index: u32,

    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3]
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Skeleton {
    num_bones : u32,
    bones: *mut physis_Bone,
    root_bone: *mut physis_Bone
}

fn convert_skeleton(skeleton: &Skeleton) -> physis_Skeleton {
    let mut c_bones = vec![];

    for (i, bone) in skeleton.bones.iter().enumerate() {
        c_bones.push(physis_Bone {
            index: i as u32,
            name: ffi_to_c_string(&bone.name),
            parent_bone: null_mut(),
            parent_index: bone.parent_index as u32,
            position: bone.position,
            rotation: bone.rotation,
            scale: bone.scale
        })
    }

    for (i, bone) in skeleton.bones.iter().enumerate() {
        if bone.parent_index != -1 {
            c_bones[i].parent_bone = &mut c_bones[bone.parent_index as usize] as *mut physis_Bone;
        }
    }

    let skel = physis_Skeleton {
        num_bones: c_bones.len() as u32,
        bones: c_bones.as_mut_ptr(),
        root_bone: &mut c_bones[0] as *mut physis_Bone
    };

    mem::forget(c_bones);

    skel
}

#[no_mangle] pub extern "C" fn physis_skeleton_from_packfile(buffer : physis_Buffer) -> physis_Skeleton {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(skeleton) = Skeleton::from_packfile(&data.to_vec()) {
        convert_skeleton(&skeleton)
    } else {
        physis_Skeleton {
            num_bones: 0,
            bones: null_mut(),
            root_bone: null_mut()
        }
    }
}

#[no_mangle] pub extern "C" fn physis_skeleton_from_skel(buffer : physis_Buffer) -> physis_Skeleton {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(skeleton) = Skeleton::from_skel(&data.to_vec()) {
        convert_skeleton(&skeleton)
    } else {
        physis_Skeleton {
            num_bones: 0,
            bones: null_mut(),
            root_bone: null_mut()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Texture {
    width: u32,
    height: u32,
    rgba_size: u32,
    rgba: *mut u8
}

#[no_mangle] pub extern "C" fn physis_texture_parse(buffer : physis_Buffer) -> physis_Texture {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(mut texture) = Texture::from_existing(&data.to_vec()) {
        let tex = physis_Texture {
            width: texture.width,
            height: texture.height,
            rgba_size: texture.rgba.len() as u32,
            rgba: texture.rgba.as_mut_ptr()
        };

        mem::forget(texture.rgba);

        tex
    } else {
        physis_Texture {
            width: 0,
            height: 0,
            rgba_size: 0,
            rgba: null_mut()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Material {
    num_textures: u32,
    textures: *mut *const c_char
}

#[no_mangle] pub extern "C" fn physis_material_parse(buffer : physis_Buffer) -> physis_Material {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(material) = Material::from_existing(&data.to_vec()) {
        let mut c_strings = vec![];

        for tex in &material.texture_paths {
            c_strings.push(ffi_to_c_string(tex));
        }

        let mat = physis_Material {
            num_textures: c_strings.len() as u32,
            textures: c_strings.as_mut_ptr()
        };

        mem::forget(c_strings);

        mat
    } else {
        physis_Material {
            num_textures: 0,
            textures: null_mut()
        }
    }
}

#[no_mangle] pub extern "C" fn physis_get_slot_name(slot: Slot) -> *const c_char {
    // TODO: no need to dynamically allocate a new string
    ffi_to_c_string(&get_slot_abbreviation(slot).to_string())
}
