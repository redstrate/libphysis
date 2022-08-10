extern crate core;

use core::ffi;
use std::ffi::{CStr, CString};
use std::{mem, slice};
use std::fs::read;
use std::os::raw::{c_char, c_uint, c_uchar};
use std::ptr::{null, null_mut};
use physis::gamedata::GameData;
use physis::blowfish::Blowfish;
use physis::bootdata::BootData;
use physis::common::Language;
use physis::equipment::{build_equipment_path, get_slot_from_id, Slot};
use physis::exh::EXH;
use physis::exd::{ColumnData, ExcelRow, EXD};
use physis::installer::install_game;
use physis::model::{MDL, Vertex};
use physis::race::{Gender, Race, Subrace};

use physis::repository::RepositoryType;
/// Initializes a new BootData structure. Path must be a valid boot path, or else it will return NULL.
#[no_mangle] pub extern "C" fn physis_bootdata_initialize(path : *const c_char) -> *mut BootData {
    unsafe {
        let mut boot_data = Box::new(BootData::from_existing(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap());

        Box::leak(boot_data)
    }
}

#[no_mangle] pub extern "C" fn physis_bootdata_free(boot_data : *mut BootData) {
    unsafe {
        drop(Box::from_raw(boot_data));
    }
}

/// Initializes a new GameData structure. Path must be a valid game path, or else it will return NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_initialize(path : *const c_char) -> *mut GameData {
    unsafe {
        let mut game_data = Box::new(GameData::from_existing(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap());

        game_data.reload_repositories();

        Box::leak(game_data)
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
    repositories_count : u32,
    repositories : *mut physis_Repository
}

fn ffi_to_c_string(s : &String) -> *const c_char {
    let s = CString::new(s.as_bytes()).unwrap();
    let ptr = s.as_ptr();

    mem::forget(s);

    ptr
}

fn ffi_to_vec<T>(ptr : *mut T, count : u32) -> Vec<T> {
    unsafe {
        Vec::from_raw_parts(ptr, count as usize, count as usize)
    }
}

fn ffi_free_string(ptr : *const c_char) {
    unsafe {
        let str = CString::from_raw(*ptr as *mut i8);
        drop(str);
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_get_repositories(game_data : &GameData) -> physis_Repositories {
    let mut c_repositories : Vec<physis_Repository> = Vec::new();

    for repository in &game_data.repositories {
        c_repositories.push(physis_Repository {
            name: ffi_to_c_string(&repository.name),
            repository_type: repository.repo_type,
            version: ffi_to_c_string(&repository.version)
        });
    }

    let repositories = physis_Repositories {
        repositories_count: c_repositories.len() as u32,
        repositories: c_repositories.as_mut_ptr()
    };

    mem::forget(repositories);

    repositories
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_repositories(repositories : physis_Repositories) {
    let data = ffi_to_vec(repositories.repositories, repositories.repositories_count);

    for repository in data {
        ffi_free_string(repository.version);
        ffi_free_string(repository.name);
    }

    drop(data)
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_extract_file(game_data : &GameData, path : *const c_char) -> physis_Buffer {
    unsafe {
        let mut d = game_data.extract(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap();

        let b = physis_Buffer {
            size: d.len() as u32,
            data: d.as_mut_ptr()
        };

        mem::forget(d);

        b
    }
}

/// Checks if the file at `path` exists.
#[no_mangle] pub extern "C" fn physis_gamedata_exists(game_data : &GameData, path : *const c_char) -> bool {
    unsafe {
        game_data.exists(CStr::from_ptr(path).to_string_lossy().as_ref())
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_read_excel_sheet_header(game_data : &GameData, name : *const c_char) -> *mut EXH {
    unsafe {
        let mut exh = Box::new(game_data.read_excel_sheet_header(CStr::from_ptr(name).to_string_lossy().as_ref()).unwrap());

        Box::leak(exh)
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
    unsafe {
        let mut exd = Box::new(game_data.read_excel_sheet(CStr::from_ptr(name).to_string_lossy().as_ref(), exh, language, page as usize).unwrap());

        let mut c_rows : Vec<physis_ExcelRow> = Vec::new();

        for mut row in &exd.rows {
            let mut c_col_data : Vec<physis_ColumnData> = Vec::new();

            for col_data in &row.data {
                match col_data {
                    ColumnData::String(s) => {
                        let s = CString::new(s.as_bytes()).unwrap();
                        let ptr = s.as_ptr();

                        mem::forget(s);

                        c_col_data.push(physis_ColumnData::String(ptr))
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
}

#[no_mangle] pub extern "C" fn physis_gamedata_free_sheet(exd : physis_EXD)  {
    unsafe {
        let data = Vec::from_raw_parts(exd.row_data, exd.row_count as usize, exd.row_count as usize);

        for i in 0..exd.row_count {
            let col_data = Vec::from_raw_parts(data[i as usize].column_data, exd.column_count as usize, exd.column_count as usize);

            for col in &col_data {
                match col {
                    physis_ColumnData::String(s) => {
                        let str = CString::from_raw(*s as *mut i8);
                        drop(str);
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
    unsafe {
        gamedata.apply_patch(CStr::from_ptr(path).to_str().unwrap()).is_ok()
    }
}

#[no_mangle] pub extern "C" fn physis_bootdata_apply_patch(bootdata : &BootData, path : *const c_char) -> bool {
    unsafe {
        bootdata.apply_patch(CStr::from_ptr(path).to_str().unwrap()).is_ok()
    }
}

#[no_mangle] pub extern "C" fn physis_install_game(installer_path : *const c_char, game_directory : *const c_char) {
    unsafe {
        install_game(CStr::from_ptr(installer_path).to_str().unwrap(), CStr::from_ptr(game_directory).to_str().unwrap());
    }
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
    lods : *const physis_LOD
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

    let mdl = physis_MDL {
        num_lod: c_lods.len() as u32,
        lods: c_lods.as_mut_ptr()
    };

    mem::forget(c_lods);

    mdl
}

#[no_mangle] pub extern "C" fn physis_read_file(path : *const c_char) -> physis_Buffer {
    let mut f = unsafe { read(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap() };
    let buf = physis_Buffer {
        size: f.len() as u32,
        data: f.as_mut_ptr()
    };

    mem::forget(buf);

    buf
}

#[no_mangle] pub extern "C" fn physis_build_equipment_path(model_id : i32, race : Race, subrace : Subrace, gender : Gender, slot: Slot) -> *const c_char {
    CString::new(build_equipment_path(model_id, race, subrace, gender, slot)).unwrap().into_raw()
}

#[no_mangle] pub extern "C" fn physis_slot_from_id(slot_id : i32) -> Slot {
    match get_slot_from_id(slot_id) {
        None => Slot::Head, // FIXME: this is currently used to cover-up the few missing slots. PLEASE DO NOT SHIP
        Some(x) => x
    }
}