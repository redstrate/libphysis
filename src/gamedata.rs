// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::exd::{physis_ColumnData, physis_EXD, physis_ExcelRow, physis_ExcelRows};
use crate::exh::physis_EXH;
use crate::{ffi_free_string, ffi_from_c_string, ffi_to_c_string, ffi_to_vec, physis_Buffer};
use physis::common::{Language, Platform};
use physis::exd::{ColumnData, EXD, ExcelRowKind};
use physis::repository::RepositoryType;
use physis::resource::{Resource, SqPackResource, get_all_sheet_names, read_excel_sheet};
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};

/// Checks if the file at `path` exists.
#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_exists(
    game_data: &mut SqPackResource,
    path: *const c_char,
) -> bool {
    if let Some(r_path) = ffi_from_c_string(path) {
        game_data.exists(&r_path)
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_free_repositories(repositories: physis_Repositories) {
    let data = ffi_to_vec(repositories.repositories, repositories.repositories_count);

    for repository in &data {
        ffi_free_string(repository.version);
        ffi_free_string(repository.name);
    }

    drop(data)
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_extract_file(
    game_data: &mut SqPackResource,
    path: *const c_char,
) -> physis_Buffer {
    unsafe {
        if let Some(mut d) = game_data.read(CStr::from_ptr(path).to_string_lossy().as_ref()) {
            let b = physis_Buffer {
                size: d.len() as u32,
                data: d.as_mut_ptr(),
            };

            mem::forget(d);

            b
        } else {
            physis_Buffer::default()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_find_offset(
    game_data: &mut SqPackResource,
    path: *const c_char,
) -> u64 {
    unsafe {
        game_data
            .find_offset(CStr::from_ptr(path).to_string_lossy().as_ref())
            .unwrap_or_default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_free_sheet_header(_: *mut physis_EXH) {
    /*unsafe {
        drop(Box::from_raw(exh));
    }*/
}

/// Initializes a new SqPackResource structure.
#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_initialize(path: *const c_char) -> *mut SqPackResource {
    let Some(r_path) = ffi_from_c_string(path) else {
        return null_mut();
    };

    let game_data = SqPackResource::from_existing(Platform::Win32, &r_path);
    let boxed = Box::new(game_data);

    Box::leak(boxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_free(game_data: *mut SqPackResource) {
    unsafe {
        drop(Box::from_raw(game_data));
    }
}

#[repr(C)]
pub struct physis_Repository {
    name: *const c_char,
    repository_type: RepositoryType,
    version: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Repositories {
    repositories_count: u32,
    repositories: *mut physis_Repository,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_get_repositories(
    game_data: &SqPackResource,
) -> physis_Repositories {
    let mut c_repositories: Vec<physis_Repository> = Vec::new();

    for repository in &game_data.repositories {
        let ver = match &repository.version {
            Some(x) => ffi_to_c_string(x),
            None => null(),
        };

        c_repositories.push(physis_Repository {
            name: ffi_to_c_string(&repository.name),
            repository_type: repository.repo_type,
            version: ver,
        });
    }

    let repositories = physis_Repositories {
        repositories_count: c_repositories.len() as u32,
        repositories: c_repositories.as_mut_ptr(),
    };

    mem::forget(c_repositories);

    repositories
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_gamedata_read_excel_sheet(
    game_data: &mut SqPackResource,
    name: *const c_char,
    exh: &physis_EXH,
    language: Language,
    page: c_uint,
) -> physis_EXD {
    unsafe {
        let Some(r_name) = ffi_from_c_string(name) else {
            return physis_EXD::default();
        };

        if let Some(exd) =
            read_excel_sheet(game_data, &r_name, &*exh.p_ptr, language, page as usize)
        {
            let row_count = exd.rows.len() as c_uint;
            let exd = Box::new(exd);

            let mut c_rows = Vec::new();

            for row in &exd.rows {
                let reduced_rows = match &row.kind {
                    ExcelRowKind::SingleRow(val) => &vec![(0u16, val.clone())],
                    ExcelRowKind::SubRows(rows) => rows,
                };

                let mut c_subrows: Vec<physis_ExcelRow> = Vec::new();

                for (subrow_id, row) in reduced_rows {
                    let mut c_col_data: Vec<physis_ColumnData> = Vec::new();

                    for col_data in &row.columns {
                        match &col_data {
                            ColumnData::String(s) => {
                                c_col_data.push(physis_ColumnData::String(ffi_to_c_string(s)))
                            }
                            ColumnData::Bool(b) => c_col_data.push(physis_ColumnData::Bool(*b)),
                            ColumnData::Int8(i) => c_col_data.push(physis_ColumnData::Int8(*i)),
                            ColumnData::UInt8(i) => c_col_data.push(physis_ColumnData::UInt8(*i)),
                            ColumnData::Int16(i) => c_col_data.push(physis_ColumnData::Int16(*i)),
                            ColumnData::UInt16(i) => c_col_data.push(physis_ColumnData::UInt16(*i)),
                            ColumnData::Int32(i) => c_col_data.push(physis_ColumnData::Int32(*i)),
                            ColumnData::UInt32(i) => c_col_data.push(physis_ColumnData::UInt32(*i)),
                            ColumnData::Float32(i) => {
                                c_col_data.push(physis_ColumnData::Float32(*i))
                            }
                            ColumnData::Int64(i) => c_col_data.push(physis_ColumnData::Int64(*i)),
                            ColumnData::UInt64(i) => c_col_data.push(physis_ColumnData::UInt64(*i)),
                        }
                    }

                    c_subrows.push(physis_ExcelRow {
                        subrow_id: *subrow_id,
                        column_data: c_col_data.as_mut_ptr(),
                    });

                    mem::forget(c_col_data);
                }

                c_rows.push(physis_ExcelRows {
                    row_id: row.row_id,
                    row_count: c_subrows.len() as u32,
                    row_data: c_subrows.as_mut_ptr(),
                });

                mem::forget(c_subrows);
            }

            let exd = physis_EXD {
                p_ptr: Box::leak(exd),
                column_count: (*exh.p_ptr).column_definitions.len() as c_uint,
                row_count,
                rows: c_rows.as_ptr(),
            };

            mem::forget(c_rows);

            exd
        } else {
            physis_EXD::default()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_gamedata_get_exd_filename(
    name: *const c_char,
    exh: &physis_EXH,
    language: Language,
    page: c_uint,
) -> *const c_char {
    unsafe {
        let Some(r_name) = ffi_from_c_string(name) else {
            return null();
        };

        ffi_to_c_string(&EXD::calculate_filename(
            &r_name,
            language,
            &(*exh.p_ptr).pages[page as usize],
        ))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_free_sheet(exd: physis_EXD) {
    unsafe {
        drop(Box::from_raw(exd.p_ptr));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_apply_patch(
    gamedata: &SqPackResource,
    path: *const c_char,
) -> bool {
    if let Some(r_path) = ffi_from_c_string(path) {
        gamedata.apply_patch(&r_path).is_ok()
    } else {
        false
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SheetNames {
    name_count: u32,
    names: *mut *const c_char,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_gamedata_get_all_sheet_names(
    game_data: &mut SqPackResource,
) -> physis_SheetNames {
    let mut c_repo_names = vec![];

    for name in get_all_sheet_names(game_data).unwrap() {
        c_repo_names.push(ffi_to_c_string(&name));
    }

    let repositories = physis_SheetNames {
        name_count: c_repo_names.len() as u32,
        names: c_repo_names.as_mut_ptr(),
    };

    mem::forget(c_repo_names);

    repositories
}
