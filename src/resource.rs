// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::exd::{physis_ExcelEntry, physis_ExcelRow, physis_Field};
use crate::exh::physis_EXH;
use crate::{ffi_free_string, ffi_from_c_string, ffi_to_c_string, ffi_to_vec, physis_Buffer};
use physis::excel::Field;
use physis::excel::Row;
use physis::excel::{Entry, Sheet};
use physis::repository::RepositoryType;
use physis::resource::{
    RepairAction, Resource, SqPackRelease, SqPackResource, generic_read_excel_sheet,
};
use physis::sqpack::Hash;
use physis::{Language, Platform};
use std::ffi::{CStr, c_void};
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::path::Path;
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
        if let Ok(mut d) = (*resource.p_ptr).read(CStr::from_ptr(path).to_string_lossy().as_ref()) {
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
    p_ptr: *mut Sheet,
    page_index: u32,
    pub entry_count: c_uint,
    pub entries: *mut physis_ExcelEntry,
    pub column_count: c_uint,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_excel_page_free_rows(
    page: &physis_ExcelSheetPage,
    rows: &physis_ExcelEntry,
) {
    unsafe {
        let data = Vec::from_raw_parts(
            rows.subrows,
            rows.subrow_count as usize,
            rows.subrow_count as usize,
        );

        for i in 0..rows.subrow_count {
            let col_data = Vec::from_raw_parts(
                data[i as usize].columns,
                page.column_count as usize,
                page.column_count as usize,
            );

            for col in &col_data {
                if let physis_Field::String(s) = col {
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
    p_ptr: *mut Sheet,
    page_count: u32,
    pages: *mut physis_ExcelSheetPage,
}

impl Default for physis_ExcelSheet {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            page_count: 0,
            pages: null_mut(),
        }
    }
}

fn to_c_entry(row_id: u32, entry: &Entry) -> physis_ExcelEntry {
    let mut c_subrows: Vec<physis_ExcelRow> = Vec::new();

    for (subrow_id, row) in &entry.subrows {
        c_subrows.push(to_c_row(*subrow_id, row));
    }

    let rows = physis_ExcelEntry {
        row_id,
        subrow_count: c_subrows.len() as u32,
        subrows: c_subrows.as_mut_ptr(),
    };

    mem::forget(c_subrows);

    rows
}

fn to_c_row(subrow_id: u16, row: &Row) -> physis_ExcelRow {
    let mut c_col_data: Vec<physis_Field> = Vec::new();

    for col_data in &row.columns {
        match &col_data {
            Field::String(s) => c_col_data.push(physis_Field::String(ffi_to_c_string(s))),
            Field::Bool(b) => c_col_data.push(physis_Field::Bool(*b)),
            Field::Int8(i) => c_col_data.push(physis_Field::Int8(*i)),
            Field::UInt8(i) => c_col_data.push(physis_Field::UInt8(*i)),
            Field::Int16(i) => c_col_data.push(physis_Field::Int16(*i)),
            Field::UInt16(i) => c_col_data.push(physis_Field::UInt16(*i)),
            Field::Int32(i) => c_col_data.push(physis_Field::Int32(*i)),
            Field::UInt32(i) => c_col_data.push(physis_Field::UInt32(*i)),
            Field::Float32(i) => c_col_data.push(physis_Field::Float32(*i)),
            Field::Int64(i) => c_col_data.push(physis_Field::Int64(*i)),
            Field::UInt64(i) => c_col_data.push(physis_Field::UInt64(*i)),
        }
    }

    let row = physis_ExcelRow {
        subrow_id,
        columns: c_col_data.as_mut_ptr(),
    };

    std::mem::forget(c_col_data);

    row
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

        if let Ok(exd) = (*resource.p_ptr).read_excel_sheet(&*exh.p_ptr, &r_name, language) {
            let exd = Box::new(exd);
            let pages = exd.pages.clone();
            let p_ptr = Box::leak(exd);

            let mut c_pages = Vec::new();
            for (i, page) in pages.iter().enumerate() {
                let mut c_entries = Vec::new();

                for row in &page.entries {
                    c_entries.push(to_c_entry(row.id, row));
                }

                let page = physis_ExcelSheetPage {
                    p_ptr,
                    page_index: i as u32,
                    column_count: (*exh.p_ptr).column_definitions.len() as c_uint,
                    entry_count: page.entries.len() as u32,
                    entries: c_entries.as_mut_ptr(),
                };

                mem::forget(c_entries);

                c_pages.push(page);
            }

            let exd = physis_ExcelSheet {
                p_ptr,
                page_count: c_pages.len() as u32,
                pages: c_pages.as_mut_ptr(),
            };

            mem::forget(c_pages);

            exd
        } else {
            physis_ExcelSheet::default()
        }
    }
}

// TODO: is there some way to de-duplicate these functions?
#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_custom_read_excel_sheet(
    resource: &physis_CustomResource,
    name: *const c_char,
    exh: &physis_EXH,
    language: Language,
) -> physis_ExcelSheet {
    unsafe {
        let Some(r_name) = ffi_from_c_string(name) else {
            return physis_ExcelSheet::default();
        };

        if let Ok(exd) =
            generic_read_excel_sheet(&mut *resource.p_ptr, &*exh.p_ptr, &r_name, language)
        {
            let exd = Box::new(exd);
            let pages = exd.pages.clone();
            let p_ptr = Box::leak(exd);

            let mut c_pages = Vec::new();
            for (i, page) in pages.iter().enumerate() {
                let mut c_entries = Vec::new();

                for row in &page.entries {
                    c_entries.push(to_c_entry(row.id, row));
                }

                let page = physis_ExcelSheetPage {
                    p_ptr,
                    page_index: i as u32,
                    column_count: (*exh.p_ptr).column_definitions.len() as c_uint,
                    entry_count: page.entries.len() as u32,
                    entries: c_entries.as_mut_ptr(),
                };

                mem::forget(c_entries);

                c_pages.push(page);
            }

            let exd = physis_ExcelSheet {
                p_ptr,
                page_count: c_pages.len() as u32,
                pages: c_pages.as_mut_ptr(),
            };

            mem::forget(c_pages);

            exd
        } else {
            physis_ExcelSheet::default()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_sqpack_update_excel_sheet_page(
    page: &mut physis_ExcelSheetPage,
    row_id: u32,
    subrow_id: u16,
    column_index: usize,
    new_field: &physis_Field,
) {
    unsafe {
        for i in 0..page.entry_count {
            let entry = page.entries.add(i as usize);
            if (*entry).row_id == row_id {
                for j in 0..(*entry).subrow_count {
                    let subrow = (*entry).subrows.add(j as usize);
                    if (*subrow).subrow_id == subrow_id {
                        // Update the C++ model
                        *(*subrow).columns.add(column_index) = (*new_field).clone();
                        // Then update the Rust model
                        if let Some(entry) = (*page.p_ptr).entry_mut(row_id) {
                            for (id, subrow) in &mut entry.subrows {
                                if *id == subrow_id {
                                    let old_field = &mut subrow.columns[column_index];
                                    *old_field = match new_field {
                                        physis_Field::String(val) => {
                                            Field::String(ffi_from_c_string(*val).unwrap())
                                        }
                                        physis_Field::Bool(val) => Field::Bool(*val),
                                        physis_Field::Int8(val) => Field::Int8(*val),
                                        physis_Field::UInt8(val) => Field::UInt8(*val),
                                        physis_Field::Int16(val) => Field::Int16(*val),
                                        physis_Field::UInt16(val) => Field::UInt16(*val),
                                        physis_Field::Int32(val) => Field::Int32(*val),
                                        physis_Field::UInt32(val) => Field::UInt32(*val),
                                        physis_Field::Float32(val) => Field::Float32(*val),
                                        physis_Field::Int64(val) => Field::Int64(*val),
                                        physis_Field::UInt64(val) => Field::UInt64(*val),
                                    };
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_sqpack_write_sheet_page_to_buffer(
    page: &mut physis_ExcelSheetPage,
    exh: &physis_EXH,
) -> physis_Buffer {
    unsafe {
        if let Some(mut d) =
            (&(*page.p_ptr)).pages[page.page_index as usize].write_to_buffer(&*exh.p_ptr)
        {
            let b = physis_Buffer {
                size: d.len() as u32,
                data: d.as_mut_ptr(),
            };

            mem::forget(d);

            return b;
        }

        physis_Buffer::default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_sqpack_free_excel_sheet(sheet: &physis_ExcelSheet) {
    if sheet.p_ptr.is_null() {
        return;
    }

    unsafe {
        let data = ffi_to_vec(sheet.pages, sheet.page_count);
        for page in &data {
            let data = ffi_to_vec(page.entries, page.entry_count);
            for entry in &data {
                let data = ffi_to_vec(entry.subrows, entry.subrow_count);
                for subrow in &data {
                    let data = ffi_to_vec(subrow.columns, page.column_count);
                    for column in &data {
                        if let physis_Field::String(string) = &column {
                            ffi_free_string(*string)
                        }
                    }
                    drop(data);
                }
                drop(data);
            }
            drop(data);
        }
        drop(data);

        drop(Box::from_raw(sheet.p_ptr));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_excel_get_row(
    sheet: &physis_ExcelSheet,
    row_id: u32,
) -> physis_ExcelRow {
    unsafe {
        if let Some(row) = (*sheet.p_ptr).row(row_id) {
            return to_c_row(0, row);
        }
    }

    physis_ExcelRow::default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_excel_get_subrow(
    sheet: &physis_ExcelSheet,
    row_id: u32,
    subrow_id: u16,
) -> physis_ExcelRow {
    unsafe {
        if let Some(row) = (*sheet.p_ptr).subrow(row_id, subrow_id) {
            return to_c_row(subrow_id, row);
        }
    }

    physis_ExcelRow::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_free_row(row: &physis_ExcelRow, column_count: u32) {
    if row.columns.is_null() {
        return;
    }

    let data = ffi_to_vec(row.columns, column_count);

    for field in &data {
        if let physis_Field::String(string) = *field {
            ffi_free_string(string)
        }
    }

    drop(data)
}

// TODO: not final API, this sucks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_excel_get_subrow_count(
    sheet: &physis_ExcelSheet,
    row_id: u32,
) -> usize {
    unsafe {
        if let Some(row) = (*sheet.p_ptr).entry(row_id) {
            return row.subrows.len();
        }
    }

    0
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
pub extern "C" fn physis_sqpack_free_all_sheet_names(names: physis_SheetNames) {
    let data = ffi_to_vec(names.names, names.name_count);

    for name in &data {
        ffi_free_string(*name);
    }

    drop(data)
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
        if let Ok(mut d) = (*resource.p_ptr).read_from_hash(
            Path::new(CStr::from_ptr(index_path).to_string_lossy().as_ref()),
            hash,
        ) {
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

#[derive(Clone)]
struct CustomResource {
    user_data: *mut c_void,
    read_func: extern "C" fn(*mut c_void, *const c_char) -> physis_Buffer,
    exists_func: extern "C" fn(*mut c_void, *const c_char) -> bool,
}

// Erase safety woohoo
unsafe impl Sync for CustomResource {}
unsafe impl Send for CustomResource {}

impl Resource for CustomResource {
    fn read(&mut self, path: &str) -> physis::Result<physis::ByteBuffer> {
        // TODO: free string
        let c_path = ffi_to_c_string(&path.to_string());
        let data = (self.read_func)(self.user_data, c_path);
        if data.size == 0 {
            return Err(physis::Error::FileNotFound {
                path: path.to_string(),
            });
        }

        // This ensures the callee still owns the memory and we don't accidentally free it
        let original = ffi_to_vec(data.data, data.size);
        let clone = original.clone();
        std::mem::forget(original);

        Ok(clone)
    }

    fn exists(&mut self, path: &str) -> bool {
        // TODO: free string
        (self.exists_func)(self.user_data, ffi_to_c_string(&path.to_string()))
    }
}

#[repr(C)]
pub struct physis_CustomResource {
    p_ptr: *mut CustomResource,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_custom_initialize(
    user_data: *mut c_void,
    read_func: extern "C" fn(*mut c_void, *const c_char) -> physis_Buffer,
    exists_func: extern "C" fn(*mut c_void, *const c_char) -> bool,
) -> physis_CustomResource {
    let resource = CustomResource {
        user_data,
        read_func,
        exists_func,
    };

    let boxed = Box::new(resource);

    physis_CustomResource {
        p_ptr: Box::leak(boxed),
    }
}
