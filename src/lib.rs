// SPDX-FileCopyrightText: 2023 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

extern crate core;

use std::{mem, slice};
use std::ffi::{CStr, CString};
use std::fs::read;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};
use physis::common::Platform;

use physis::blowfish::Blowfish;
use physis::bootdata::BootData;
use physis::chardat::CharacterData;
use physis::cmp::{CMP, RacialScalingParameters};
use physis::common::Language;
use physis::equipment::{build_character_path, build_ear_material_path, build_equipment_path, build_face_material_path, build_gear_material_path, build_hair_material_path, build_skin_material_path, build_tail_material_path, CharacterCategory, get_slot_abbreviation, get_slot_from_id, Slot};
use physis::exd::{ColumnData, EXD};
use physis::exh::EXH;
use physis::gamedata::GameData;
#[cfg(feature = "game_install")]
use physis::installer::install_game;
#[cfg(feature = "visual_data")]
use physis::model::{MDL, Vertex};
#[cfg(feature = "visual_data")]
use physis::mtrl::Material;
use physis::race::{Gender, get_race_id, get_supported_subraces, Race, Subrace};
use physis::repository::RepositoryType;
#[cfg(feature = "visual_data")]
use physis::skeleton::Skeleton;
#[cfg(feature = "visual_data")]
use physis::tex::Texture;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::Layer;
use tracing_subscriber::prelude::*;
use std::fmt::Write;
use physis::cfg::ConfigFile;
use physis::exl::EXL;
use physis::index::IndexFile;
#[cfg(feature = "visual_data")]
use physis::model::SubMesh;
#[cfg(feature = "visual_data")]
use physis::shpk::ShaderPackage;
#[cfg(feature = "visual_data")]
use physis::pbd::PreBoneDeformer;
#[cfg(feature = "visual_data")]
use physis::tera::Terrain;
use physis::dic::Dictionary;

type LogCallback = unsafe extern "C" fn(QtMsgType, *const c_char, *const c_char, i32);

pub struct CustomLayer {
    callback: LogCallback,
}

struct PrintlnVisitor<'a> {
    string: &'a mut String,
}

/// cbindgen:ignore
#[repr(i8)]
pub enum QtMsgType {
    Debug = 0,
    Warning = 1,
    Critical = 2,
    Fatal = 3,
    Info = 4,
}

impl tracing::field::Visit for PrintlnVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(self.string, "{:#?} ", value).unwrap();
        } else {
            write!(self.string, "{} = {:?} ", field.name(), value).unwrap();
        }
    }
}

impl<S> Layer<S> for CustomLayer
    where
        S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut buffer: String = String::new();
        let mut visitor = PrintlnVisitor {
            string: &mut buffer
        };
        event.record(&mut visitor);

        let msg_type = match *event.metadata().level() {
            Level::ERROR => QtMsgType::Critical,
            Level::WARN => QtMsgType::Warning,
            Level::INFO => QtMsgType::Info,
            Level::DEBUG => QtMsgType::Debug,
            Level::TRACE => QtMsgType::Debug
        };

        unsafe {
            let file = if let Some(file) = event.metadata().file() {
                ffi_to_c_string(&file.to_string())
            } else {
                null()
            };

            let line = if let Some(line) = event.metadata().line() {
                line as i32
            } else {
                -1
            };

            (self.callback)(msg_type, ffi_to_c_string(&buffer), file, line);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_tracing_callback(callback: LogCallback) {
    tracing_subscriber::registry().with(CustomLayer {
        callback
    }).init();
}
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

#[no_mangle] pub extern "C" fn physis_get_physis_version() -> *const c_char {
    ffi_to_c_string(&physis::PHYSIS_VERSION.to_string())
}

#[no_mangle] pub extern "C" fn physis_get_libphysis_version() -> *const c_char {
    ffi_to_c_string(&env!("CARGO_PKG_VERSION").to_string())
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
    if let Some(mut game_data) = GameData::from_existing(Platform::Win32, &ffi_from_c_string(path)) {
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SheetNames {
    name_count: u32,
    names: *mut *const c_char
}

#[no_mangle] pub extern "C" fn physis_gamedata_get_all_sheet_names(game_data : &mut GameData) -> physis_SheetNames {
    let mut c_repo_names = vec![];

    for name in game_data.get_all_sheet_names().unwrap() {
        c_repo_names.push(ffi_to_c_string(&name));
    }

    let repositories = physis_SheetNames {
        name_count: c_repo_names.len() as u32,
        names: c_repo_names.as_mut_ptr()
    };

    mem::forget(c_repo_names);

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
#[no_mangle] pub extern "C" fn physis_gamedata_extract_file(game_data : &mut GameData, path : *const c_char) -> physis_Buffer {
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
#[no_mangle] pub extern "C" fn physis_gamedata_exists(game_data : &mut GameData, path : *const c_char) -> bool {
    game_data.exists(&ffi_from_c_string(path))
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EXH {
    p_ptr : *mut EXH,
    page_count: u32,
    language_count: u32,
    languages: *mut Language,
    column_count: u32,
    row_count: u32
}

#[no_mangle] pub extern "C" fn physis_parse_excel_sheet_header(buffer: physis_Buffer) -> *mut physis_EXH {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    let exh = EXH::from_existing(&data.to_vec());
    if exh.is_none() {
        return null_mut();
    }

    let exh = Box::new(exh.unwrap());

    let mut c_languages : Vec<Language> = vec![];

    for lang in &exh.languages {
        c_languages.push(*lang);
    }

    let page_len = exh.pages.len() as u32;
    let row_count = exh.header.row_count as u32;
    let column_count = exh.column_definitions.len() as u32;

    let repositories = physis_EXH {
        p_ptr: Box::leak(exh),
        page_count: page_len,
        language_count: c_languages.len() as u32,
        languages: c_languages.as_mut_ptr(),
        column_count,
        row_count
    };

    mem::forget(c_languages);

    Box::leak(Box::new(repositories))
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_sheet_header(_: *mut physis_EXH) {
    /*unsafe {
        drop(Box::from_raw(exh));
    }*/
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

#[no_mangle] pub unsafe extern "C" fn physis_gamedata_read_excel_sheet(game_data : &mut GameData, name : *const c_char, exh : &physis_EXH, language : Language, page : c_uint) -> physis_EXD {
    if let Some(exd) = game_data.read_excel_sheet(&ffi_from_c_string(name), &*exh.p_ptr, language, page as usize) {
        let exd = Box::new(exd);

        let mut c_rows: Vec<physis_ExcelRow> = Vec::new();

        for row in &exd.rows {
            let mut c_col_data: Vec<physis_ColumnData> = Vec::new();

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
            column_count: (*exh.p_ptr).column_definitions.len() as c_uint,
            row_data: c_rows.as_mut_ptr(),
            row_count: c_rows.len() as c_uint
        };

        mem::forget(c_rows);

        exd
    } else {
        physis_EXD {
            p_ptr: null_mut(),
            column_count: 0,
            row_data: null_mut(),
            row_count: 0
        }
    }
}

#[no_mangle] pub unsafe extern "C" fn physis_gamedata_get_exd_filename(name : *const c_char, exh : &physis_EXH, language : Language, page : c_uint) -> *const c_char {
    ffi_to_c_string(&EXD::calculate_filename(&*ffi_from_c_string(name), language, &(*exh.p_ptr).pages[page as usize]))
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

#[cfg(feature = "game_install")]
#[no_mangle] pub extern "C" fn physis_install_game(installer_path : *const c_char, game_directory : *const c_char) -> bool {
    install_game(&ffi_from_c_string(installer_path), &ffi_from_c_string(game_directory)).is_ok()
}

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_Part {
    num_vertices : u32,
    vertices : *const Vertex,

    num_indices : u32,
    indices : *const u16,

    material_index : u16,

    num_submeshes : u32,
    submeshes : *const SubMesh,
}

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_LOD {
    num_parts : u32,
    parts : *const physis_Part
}

#[repr(C)]
#[cfg(feature = "visual_data")]
pub struct physis_MDL {
    p_ptr: *mut MDL,
    num_lod : u32,
    lods : *const physis_LOD,
    num_affected_bones : u32,
    affected_bone_names: *mut *const c_char,
    num_material_names : u32,
    material_names: *mut *const c_char
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Buffer {
    size : u32,
    data: *mut u8
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_mdl_parse(buffer: physis_Buffer) -> physis_MDL {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    let Some(mdl_d) = MDL::from_existing(&data.to_vec()) else {
        return physis_MDL {
            p_ptr: null_mut(),
            num_lod: 0,
            lods: null_mut(),
            num_affected_bones : 0,
            affected_bone_names: null_mut(),
            num_material_names : 0,
            material_names: null_mut()
        };
    };

    let mdl = Box::new(mdl_d);

    let mut c_lods : Vec<physis_LOD> = physis_mdl_update_vertices(&mdl);

    let mut c_bone_names = vec![];

    for bone_name in &mdl.affected_bone_names {
        c_bone_names.push(ffi_to_c_string(&bone_name));
    }

    let mut c_material_names = vec![];

    for bone_name in &mdl.material_names {
        c_material_names.push(ffi_to_c_string(&bone_name));
    }

    let mdl = physis_MDL {
        p_ptr: Box::leak(mdl),
        num_lod: c_lods.len() as u32,
        lods: c_lods.as_mut_ptr(),
        num_affected_bones : c_bone_names.len() as u32,
        affected_bone_names: c_bone_names.as_mut_ptr(),
        num_material_names : c_material_names.len() as u32,
        material_names: c_material_names.as_mut_ptr()
    };

    mem::forget(c_bone_names);
    mem::forget(c_material_names);
    mem::forget(c_lods);

    mdl
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_mdl_write(mdl: &physis_MDL) -> physis_Buffer {
    unsafe {
        let mut buffer = (*mdl.p_ptr).write_to_buffer().unwrap();

        let leak_buffer = physis_Buffer {
            size: buffer.len() as u32,
            data: buffer.as_mut_ptr()
        };

        mem::forget(buffer);

        leak_buffer
    }
}

#[cfg(feature = "visual_data")]
fn physis_mdl_update_vertices(mdl: &MDL) -> Vec<physis_LOD> {
    let mut c_lods : Vec<physis_LOD> = Vec::new();

    for lod in &mdl.lods {
        let mut c_parts : Vec<physis_Part> = Vec::new();

        for part in &lod.parts {
            let mut c_vertices : Vec<Vertex> = part.vertices.clone();
            let mut c_indices : Vec<u16> = part.indices.clone();
            let mut c_submeshes : Vec<SubMesh> = part.submeshes.clone();

            c_parts.push(physis_Part {
                num_vertices: c_vertices.len() as u32,
                vertices: c_vertices.as_mut_ptr(),
                num_indices: c_indices.len() as u32,
                indices: c_indices.as_mut_ptr(),
                material_index: part.material_index,
                num_submeshes: c_submeshes.len() as u32,
                submeshes: c_submeshes.as_mut_ptr()
            });

            mem::forget(c_vertices);
            mem::forget(c_indices);
            mem::forget(c_submeshes);
        }

        c_lods.push(physis_LOD {
            num_parts: c_parts.len() as u32,
            parts: c_parts.as_mut_ptr()
        });

        mem::forget(c_parts);
    }

    c_lods
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_mdl_replace_vertices(mdl: *mut physis_MDL,
                                                           lod_index: u32,
                                                           part_index: u32,
                                                           num_vertices: u32,
                                                           vertices_ptr: *const Vertex,
                                                           num_indices: u32,
                                                           indices_ptr: *const u16,
                                                           num_submeshes: u32,
                                                           submeshes_ptr: *const SubMesh) {
    unsafe {
        (*(*mdl).p_ptr).replace_vertices(lod_index as usize, part_index as usize, &*std::ptr::slice_from_raw_parts(vertices_ptr, num_vertices as usize), &*std::ptr::slice_from_raw_parts(indices_ptr, num_indices as usize), &*std::ptr::slice_from_raw_parts(submeshes_ptr, num_submeshes as usize));

        // We need to update the C version of these LODs as well
        let mut new_lods = physis_mdl_update_vertices((*mdl).p_ptr.as_ref().unwrap());

        (*mdl).lods = new_lods.as_mut_ptr();

        mem::forget(new_lods);
    }
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
    IndexFile::calculate_hash(&ffi_from_c_string(path))
}

#[no_mangle]
pub extern "C" fn physis_build_equipment_path(model_id: i32, race: Race, subrace: Subrace, gender: Gender, slot: Slot) -> *const c_char {
    ffi_to_c_string(&build_equipment_path(model_id, race, subrace, gender, slot))
}

#[no_mangle]
pub extern "C" fn physis_build_character_path(category: CharacterCategory, body_ver: i32, race: Race, subrace: Subrace, gender: Gender) -> *const c_char {
    ffi_to_c_string(&build_character_path(category, body_ver, race, subrace, gender))
}

#[no_mangle]
pub extern "C" fn physis_build_skin_material_path(race_code: i32, body_code: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_skin_material_path(race_code, body_code, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_build_gear_material_path(gear_id: i32, gear_version: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_gear_material_path(gear_id, gear_version, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_build_face_material_path(race_code: i32, face_code: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_face_material_path(race_code, face_code, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_build_hair_material_path(race_code: i32, hair_code: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_hair_material_path(race_code, hair_code, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_build_ear_material_path(race_code: i32, ear_code: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_ear_material_path(race_code, ear_code, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_build_tail_material_path(race_code: i32, tail_code: i32, material_name: *const c_char) -> *const c_char {
    ffi_to_c_string(&build_tail_material_path(race_code, tail_code, &*ffi_from_c_string(material_name)))
}

#[no_mangle]
pub extern "C" fn physis_get_race_code(race: Race, subrace: Subrace, gender: Gender) -> i32 {
    get_race_id(race, subrace, gender).unwrap()
}

#[repr(C)]
pub struct physis_SupportedSubraces {
    subraces: [Subrace; 2],
}

#[no_mangle]
pub extern "C" fn physis_get_supported_subraces(race: Race) -> physis_SupportedSubraces {
    physis_SupportedSubraces {
        subraces: get_supported_subraces(race)
    }
}

#[no_mangle]
pub extern "C" fn physis_slot_from_id(slot_id: i32) -> Slot {
    match get_slot_from_id(slot_id) {
        None => Slot::Head, // FIXME: this is currently used to cover-up the few missing slots. PLEASE DO NOT SHIP
        Some(x) => x
    }
}

#[repr(C)]
#[cfg(feature = "visual_data")]
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
#[cfg(feature = "visual_data")]
pub struct physis_Skeleton {
    num_bones : u32,
    bones: *mut physis_Bone,
    root_bone: *mut physis_Bone
}

#[cfg(feature = "visual_data")]
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

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_parse_skeleton(buffer : physis_Buffer) -> physis_Skeleton {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(skeleton) = Skeleton::from_existing(&data.to_vec()) {
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
#[cfg(feature = "visual_data")]
pub struct physis_Texture {
    width: u32,
    height: u32,
    rgba_size: u32,
    rgba: *mut u8
}

#[cfg(feature = "visual_data")]
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
#[cfg(feature = "visual_data")]
pub struct physis_Material {
    num_textures: u32,
    textures: *mut *const c_char
}

#[cfg(feature = "visual_data")]
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_CMP {
    p_ptr: *mut CMP
}

#[no_mangle] pub extern "C" fn physis_cmp_parse(buffer : physis_Buffer) -> physis_CMP {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(cmp) = CMP::from_existing(&data.to_vec()) {
        let cmp = physis_CMP {
            p_ptr: Box::leak(Box::new(cmp))
        };

        cmp
    } else {
        physis_CMP {
            p_ptr: null_mut()
        }
    }
}

// adapted from https://github.com/xivdev/Penumbra/blob/master/Penumbra/Meta/Files/CmpFile.cs#L50
fn get_rsp_index(subrace: Subrace) -> i32 {
    match subrace {
        Subrace::Midlander => 0,
        Subrace::Highlander => 1,
        Subrace::Wildwood => 10,
        Subrace::Duskwight => 11,
        Subrace::Plainsfolk => 20,
        Subrace::Dunesfolk => 21,
        Subrace::Seeker => 30,
        Subrace::Keeper => 31,
        Subrace::SeaWolf => 40,
        Subrace::Hellsguard => 41,
        Subrace::Raen => 50,
        Subrace::Xaela => 51,
        Subrace::Hellion => 60,
        Subrace::Lost => 61,
        Subrace::Rava => 70,
        Subrace::Veena => 71
    }
}

#[no_mangle]
pub unsafe extern "C" fn physis_cmp_get_racial_scaling_parameters(cmp: physis_CMP, _: Race, subrace: Subrace) -> RacialScalingParameters {
    return (*cmp.p_ptr).parameters[get_rsp_index(subrace) as usize];
}

#[no_mangle] pub extern "C" fn physis_chardat_parse(buffer : physis_Buffer) -> CharacterData {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    CharacterData::from_existing(&data.to_vec()).unwrap()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_ConfigFile {
    p_ptr: *mut ConfigFile
}

#[no_mangle] pub extern "C" fn physis_cfg_parse(buffer : physis_Buffer) -> physis_ConfigFile {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(cfg) = ConfigFile::from_existing(&data.to_vec()) {
        let cfg_struct = physis_ConfigFile {
            p_ptr: Box::leak(Box::new(cfg))
        };

        cfg_struct
    } else {
        physis_ConfigFile {
            p_ptr: null_mut()
        }
    }
}

#[no_mangle] pub extern "C" fn physis_cfg_set_value(cfg : physis_ConfigFile, key: *const c_char, value: *const c_char) {
    unsafe {
        (*cfg.p_ptr).set_value(&ffi_from_c_string(key), &ffi_from_c_string(value));
    }
}

#[no_mangle] pub extern "C" fn physis_cfg_write(cfg : physis_ConfigFile) -> physis_Buffer {
    unsafe {
        let mut buffer = (*cfg.p_ptr).write_to_buffer().unwrap();

        let leak_buffer = physis_Buffer {
            size: buffer.len() as u32,
            data: buffer.as_mut_ptr()
        };

        mem::forget(buffer);

        leak_buffer
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_IndexEntries {
    num_entries: u32,
    dir_entries: *const u32,
    filename_entries: *const u32
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_index_parse(path: *const c_char) -> physis_IndexEntries {
    if let Some(idx_file) = IndexFile::from_existing(&ffi_from_c_string(path)) {
        let mut c_dir_entries = vec![];
        let mut c_file_entries = vec![];

        for entry in &idx_file.entries {
            c_file_entries.push(entry.hash as u32);
            c_dir_entries.push((entry.hash >> 32) as u32);
        }

        let mat = physis_IndexEntries {
            num_entries: c_dir_entries.len() as u32,
            dir_entries: c_dir_entries.as_mut_ptr(),
            filename_entries: c_file_entries.as_mut_ptr()
        };

        mem::forget(c_dir_entries);
        mem::forget(c_file_entries);

        mat
    } else {
        physis_IndexEntries {
            num_entries: 0,
            dir_entries: null_mut(),
            filename_entries: null_mut()
        }
    }
}

#[no_mangle] pub extern "C" fn physis_generate_partial_hash(name: *const c_char) -> u32 {
    IndexFile::calculate_partial_hash(&ffi_from_c_string(name))
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EXL {
    version: i32,
    entry_count: i32,
    entry_keys: *const *const c_char,
    entry_values: *const i32
}

#[no_mangle] pub extern "C" fn physis_gamedata_read_excel_list(buffer : physis_Buffer) -> physis_EXL {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(exl) = EXL::from_existing(&data.to_vec()) {
        let mut c_keys = vec![];
        let mut c_values = vec![];

        for (key, value) in &exl.entries {
            c_keys.push(ffi_to_c_string(key));
            c_values.push(*value);
        }

        let mat = physis_EXL {
            version: exl.version,
            entry_count: c_keys.len() as i32,
            entry_keys: c_keys.as_mut_ptr(),
            entry_values: c_values.as_mut_ptr()
        };

        mem::forget(c_keys);
        mem::forget(c_values);

        mat
    } else {
        physis_EXL {
            version: 0,
            entry_count: 0,
            entry_keys: null_mut(),
            entry_values: null_mut()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Shader {
    len: i32,
    bytecode: *mut u8
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_SHPK {
    num_vertex_shaders: i32,
    vertex_shaders: *mut physis_Shader,
    num_pixel_shaders: i32,
    pixel_shaders: *mut physis_Shader
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_parse_shpk(buffer : physis_Buffer) -> physis_SHPK {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(shpk) = ShaderPackage::from_existing(&data.to_vec()) {
        let mut c_vertex_shaders = vec![];
        let mut c_fragment_shaders = vec![];

        for shader in shpk.vertex_shaders {
            let mut bytecode = shader.bytecode.clone();

            let shader = physis_Shader {
                len: bytecode.len() as i32,
                bytecode: bytecode.as_mut_ptr()
            };

            c_vertex_shaders.push(shader);

            mem::forget(bytecode);
        }

        for shader in shpk.pixel_shaders {
            let mut bytecode = shader.bytecode.clone();

            let shader = physis_Shader {
                len: bytecode.len() as i32,
                bytecode: bytecode.as_mut_ptr()
            };

            c_fragment_shaders.push(shader);

            mem::forget(bytecode);
        }

        let mat = physis_SHPK {
            num_vertex_shaders: c_vertex_shaders.len() as i32,
            vertex_shaders: c_vertex_shaders.as_mut_ptr(),
            num_pixel_shaders: c_fragment_shaders.len() as i32,
            pixel_shaders: c_fragment_shaders.as_mut_ptr()
        };

        mem::forget(c_vertex_shaders);
        mem::forget(c_fragment_shaders);

        mat
    } else {
        physis_SHPK {
            num_vertex_shaders: 0,
            vertex_shaders: null_mut(),
            num_pixel_shaders: 0,
            pixel_shaders: null_mut()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PBD {
    p_ptr : *mut PreBoneDeformer
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_parse_pbd(buffer: physis_Buffer) -> physis_PBD {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(pbd) = PreBoneDeformer::from_existing(&data.to_vec()) {
        physis_PBD {
            p_ptr: Box::leak(Box::new(pbd))
        }
    } else {
        physis_PBD {
            p_ptr: null_mut()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PreBoneDeformBone {
    name: *const c_char,
    deform: [f32; 12]
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PreBoneDeformMatrices {
    num_bones: i32,
    bones: *mut physis_PreBoneDeformBone
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_pbd_get_deform_matrix(pbd: physis_PBD, from_body_id: u16, to_body_id: u16) -> physis_PreBoneDeformMatrices {
    unsafe {
        if let Some(prebd) = (*pbd.p_ptr).get_deform_matrices(from_body_id, to_body_id) {
            let mut c_bones = vec![];

            for bone in &prebd.bones {
                c_bones.push(physis_PreBoneDeformBone {
                    name: ffi_to_c_string(&bone.name),
                    deform: bone.deform
                });
            }

            let mat = physis_PreBoneDeformMatrices {
                num_bones: c_bones.len() as i32,
                bones: c_bones.as_mut_ptr()
            };

            mem::forget(c_bones);

            mat
        } else {
            physis_PreBoneDeformMatrices {
                num_bones: 0,
                bones: null_mut()
            }
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PlateModel {
    position: [f32; 2],
    filename: *const c_char
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Terrain {
    num_plates: i32,
    plates: *mut crate::physis_PlateModel
}

#[cfg(feature = "visual_data")]
#[no_mangle] pub extern "C" fn physis_parse_tera(buffer: physis_Buffer) -> physis_Terrain {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    unsafe {
        if let Some(tera) = Terrain::from_existing(data) {
            let mut c_plates = vec![];

            for plate in &tera.plates {
                c_plates.push(crate::physis_PlateModel {
                    position: plate.position.into(),
                    filename: ffi_to_c_string(&plate.filename)
                });
            }

            let mat = crate::physis_Terrain {
                num_plates: c_plates.len() as i32,
                plates: c_plates.as_mut_ptr()
            };

            mem::forget(c_plates);

            mat
        } else {
            crate::physis_Terrain {
                num_plates: 0,
                plates: null_mut()
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Dictionary {
    num_words: i32,
    words: *const *const c_char
}

#[no_mangle] pub extern "C" fn physis_parse_dictionary(buffer: physis_Buffer) -> physis_Dictionary {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    unsafe {
        if let Some(dic) = Dictionary::from_existing(data) {
            let mut c_words = vec![];

            for word in &dic.words {
                c_words.push(ffi_to_c_string(&word));
            }

            let mat = crate::physis_Dictionary {
                num_words: c_words.len() as i32,
                words: c_words.as_ptr()
            };

            mem::forget(c_words);

            mat
        } else {
            crate::physis_Dictionary {
                num_words: 0,
                words: null()
            }
        }
    }
}