extern crate core;

use core::ffi;
use std::ffi::{CStr, CString};
use std::{mem, slice};
use std::os::raw::{c_char, c_uint, c_uchar};
use std::ptr::null_mut;
use physis::gamedata::GameData;
use physis::blowfish::Blowfish;
use physis::common::Language;
use physis::exh::EXH;
use physis::exd::{ColumnData, ExcelRow, EXD};
use physis::installer::install_game;
use physis::patch::process_patch;

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

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_extract_file(game_data : &GameData, path : *const c_char, size : *mut u32, data : &mut *mut u8) {
    unsafe {
        let mut d = game_data.extract(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap();

        *data = d.as_mut_ptr();
        *size = d.len() as u32;

        std::mem::forget(d);
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
    column_data: *const physis_ColumnData
}

#[repr(C)]
pub struct physis_EXD {
    ptr : *mut EXD,
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
                        c_col_data.push(physis_ColumnData::String(CString::new(s.as_bytes()).unwrap().as_ptr()))
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

            std::mem::forget(&c_col_data);

            c_rows.push(physis_ExcelRow {
                column_data: c_col_data.as_ptr()
            });
        }

        std::mem::forget(&c_rows);

        physis_EXD {
            ptr: Box::leak(exd),
            column_count: exh.column_definitions.len() as c_uint,
            row_data: c_rows.as_mut_ptr(),
            row_count: c_rows.len() as c_uint
        }
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

#[no_mangle] pub extern "C" fn physis_patch_process(data_path : *const c_char, path : *const c_char) {
    unsafe {
        process_patch(CStr::from_ptr(data_path).to_str().unwrap(), CStr::from_ptr(path).to_str().unwrap())
    }
}

#[no_mangle] pub extern "C" fn physis_install_game(installer_path : *const c_char, game_directory : *const c_char) {
    unsafe {
        install_game(CStr::from_ptr(installer_path).to_str().unwrap(), CStr::from_ptr(game_directory).to_str().unwrap());
    }
}