// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::exd::{physis_ColumnData, physis_ExcelRow, physis_ExcelRows};
use crate::exh::physis_EXH;
use crate::{ffi_free_string, ffi_from_c_string, ffi_to_c_string, ffi_to_vec, physis_Buffer};
use physis::common::{Language, Platform};
use physis::excel::{ColumnData, ExcelRowKind, ExcelSheet};
use physis::repository::RepositoryType;
use physis::resource::{RepairAction, Resource, SqPackRelease, SqPackResource};
use physis::sqpack::Hash;
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};

#[repr(C)]
pub struct physis_SqPackResource {
    p_ptr: *mut SqPackResource,
    pub platform: Platform,
    pub release: SqPackRelease,
}

impl Default for physis_SqPackResource {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            platform: Platform::Win32,
            release: SqPackRelease::Retail,
        }
    }
}

/// Initializes a new SqPackResource structure.
#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_initialize(path: *const c_char) -> physis_SqPackResource {
    let Some(r_path) = ffi_from_c_string(path) else {
        return physis_SqPackResource::default();
    };

    let resource = SqPackResource::from_existing(&r_path);

    let platform = resource.platform();
    let release = resource.release;

    let boxed = Box::new(resource);

    physis_SqPackResource {
        p_ptr: Box::leak(boxed),
        platform,
        release,
    }
}

/// Frees this SqPackResource.
#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_free(resource: &physis_SqPackResource) {
    unsafe {
        drop(Box::from_raw(resource.p_ptr));
    }
}

/// Checks if the file at `path` exists.
#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_exists(
    resource: &physis_SqPackResource,
    path: *const c_char,
) -> bool {
    if let Some(r_path) = ffi_from_c_string(path) {
        unsafe { (*resource.p_ptr).exists(&r_path) }
    } else {
        false
    }
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_read(
    resource: &physis_SqPackResource,
    path: *const c_char,
) -> physis_Buffer {
    unsafe {
        if let Some(mut d) = (*resource.p_ptr).read(CStr::from_ptr(path).to_string_lossy().as_ref())
        {
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

#[repr(C)]
pub struct physis_ExcelSheetPage {
    pub row_count: c_uint,
    pub rows: *const physis_ExcelRows,
    pub column_count: c_uint,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_excel_page_free_rows(
    page: &physis_ExcelSheetPage,
    rows: &physis_ExcelRows,
) {
    unsafe {
        let data = Vec::from_raw_parts(
            rows.row_data,
            rows.row_count as usize,
            rows.row_count as usize,
        );

        for i in 0..rows.row_count {
            let col_data = Vec::from_raw_parts(
                data[i as usize].column_data,
                page.column_count as usize,
                page.column_count as usize,
            );

            for col in &col_data {
                if let physis_ColumnData::String(s) = col {
                    ffi_free_string(*s);
                }
            }

            drop(col_data);
        }

        drop(data);
    }
}

#[repr(C)]
pub struct physis_ExcelSheet {
    p_ptr: *mut ExcelSheet,
    page_count: u32,
    pages: *const physis_ExcelSheetPage,
}

impl Default for physis_ExcelSheet {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            page_count: 0,
            pages: null(),
        }
    }
}

fn to_c_row(row_id: u32, row_kind: &ExcelRowKind) -> physis_ExcelRows {
    let reduced_rows = match row_kind {
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
                ColumnData::Float32(i) => c_col_data.push(physis_ColumnData::Float32(*i)),
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

    let rows = physis_ExcelRows {
        row_id,
        row_count: c_subrows.len() as u32,
        row_data: c_subrows.as_mut_ptr(),
    };

    mem::forget(c_subrows);

    rows
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_sqpack_read_excel_sheet(
    resource: &physis_SqPackResource,
    name: *const c_char,
    exh: &physis_EXH,
    language: Language,
) -> physis_ExcelSheet {
    unsafe {
        let Some(r_name) = ffi_from_c_string(name) else {
            return physis_ExcelSheet::default();
        };

        if let Ok(exd) = (*resource.p_ptr).read_excel_sheet((*exh.p_ptr).clone(), &r_name, language)
        {
            let exd = Box::new(exd);

            let mut c_pages = Vec::new();
            for page in &exd.pages {
                let mut c_rows = Vec::new();

                for row in &page.rows {
                    c_rows.push(to_c_row(row.row_id, &row.kind));
                }

                let page = physis_ExcelSheetPage {
                    column_count: (*exh.p_ptr).column_definitions.len() as c_uint,
                    row_count: page.rows.len() as u32,
                    rows: c_rows.as_ptr(),
                };

                mem::forget(c_rows);

                c_pages.push(page);
            }

            let exd = physis_ExcelSheet {
                p_ptr: Box::leak(exd),
                page_count: c_pages.len() as u32,
                pages: c_pages.as_ptr(),
            };

            mem::forget(c_pages);

            exd
        } else {
            physis_ExcelSheet::default()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_free_excel_sheet(sheet: &physis_ExcelSheet) {
    unsafe {
        drop(Box::from_raw(sheet.p_ptr)); // TODO: free other things
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_excel_get_row(
    sheet: &physis_ExcelSheet,
    row_id: u32,
) -> physis_ExcelRows {
    unsafe {
        if let Some(row_kind) = (*sheet.p_ptr).get_row(row_id) {
            return to_c_row(row_id, &row_kind);
        }
    }

    physis_ExcelRows::default()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_SheetNames {
    name_count: u32,
    names: *mut *const c_char,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_get_all_sheet_names(
    resource: &physis_SqPackResource,
) -> physis_SheetNames {
    let mut c_repo_names = vec![];

    unsafe {
        for name in (*resource.p_ptr).get_all_sheet_names().unwrap() {
            c_repo_names.push(ffi_to_c_string(&name));
        }
    }

    let repositories = physis_SheetNames {
        name_count: c_repo_names.len() as u32,
        names: c_repo_names.as_mut_ptr(),
    };

    mem::forget(c_repo_names);

    repositories
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_free_repositories(repositories: physis_Repositories) {
    let data = ffi_to_vec(repositories.repositories, repositories.repositories_count);

    for repository in &data {
        ffi_free_string(repository.version);
        ffi_free_string(repository.name);
    }

    drop(data)
}

/// Extracts the raw game file from `hash` and `index_path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_read_from_hash(
    resource: &physis_SqPackResource,
    index_path: *const c_char,
    hash: Hash,
) -> physis_Buffer {
    unsafe {
        if let Some(mut d) = (*resource.p_ptr)
            .read_from_hash(CStr::from_ptr(index_path).to_string_lossy().as_ref(), hash)
        {
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
pub extern "C" fn physis_sqpack_find_offset(
    resource: &physis_SqPackResource,
    path: *const c_char,
) -> u64 {
    unsafe {
        (*resource.p_ptr)
            .find_offset(CStr::from_ptr(path).to_string_lossy().as_ref())
            .unwrap_or_default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_free_sheet_header(exh: *mut physis_EXH) {
    unsafe {
        drop(Box::from_raw(exh));
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
pub extern "C" fn physis_sqpack_get_repositories(
    resource: &physis_SqPackResource,
) -> physis_Repositories {
    let mut c_repositories: Vec<physis_Repository> = Vec::new();

    unsafe {
        for repository in &(*resource.p_ptr).repositories {
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
    }

    let repositories = physis_Repositories {
        repositories_count: c_repositories.len() as u32,
        repositories: c_repositories.as_mut_ptr(),
    };

    mem::forget(c_repositories);

    repositories
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_RepairActions {
    action_count: u32,
    repositories: *const *const c_char,
    actions: *const RepairAction,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_needs_repair(
    resource: &physis_SqPackResource,
) -> physis_RepairActions {
    unsafe {
        if let Some(repairs) = (*resource.p_ptr).needs_repair() {
            let mut c_repositories = vec![];
            let mut c_actions = vec![];

            for (repository, action) in repairs {
                c_repositories.push(ffi_to_c_string(&repository.name));
                c_actions.push(action);
            }

            let actions = physis_RepairActions {
                action_count: c_actions.len() as u32,
                repositories: c_repositories.as_ptr(),
                actions: c_actions.as_ptr(),
            };

            mem::forget(c_repositories);
            mem::forget(c_actions);

            actions
        } else {
            physis_RepairActions {
                action_count: 0,
                repositories: null(),
                actions: null(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_repair(resource: &physis_SqPackResource) -> bool {
    unsafe {
        if let Some(repairs) = (*resource.p_ptr).needs_repair() {
            (*resource.p_ptr).perform_repair(&repairs).is_ok()
        } else {
            true
        }
    }
}
