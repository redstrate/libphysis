// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ffi_free_string;
use physis::exd::EXD;
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};
use std::slice;

#[repr(C)]
#[allow(dead_code)]
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
    UInt64(u64),
}

#[repr(C)]
pub struct physis_ExcelRow {
    pub subrow_id: u16,
    pub column_data: *mut physis_ColumnData,
}

#[repr(C)]
pub struct physis_ExcelRows {
    pub row_id: u32,
    pub row_data: *mut physis_ExcelRow,
    pub row_count: c_uint,
}

impl Default for physis_ExcelRows {
    fn default() -> Self {
        Self {
            row_id: 0,
            row_data: null_mut(),
            row_count: 0,
        }
    }
}

#[repr(C)]
pub struct physis_EXD {
    pub p_ptr: *mut EXD,
    pub column_count: c_uint,
    pub row_count: c_uint,
    pub rows: *const physis_ExcelRows,
}

impl Default for physis_EXD {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            column_count: 0,
            row_count: 0,
            rows: null(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_exd_get_row(exd: &physis_EXD, row_id: u32) -> *const physis_ExcelRow {
    unsafe {
        let rows = unsafe { slice::from_raw_parts(exd.rows, exd.row_count as usize) };
        for row in rows {
            if row.row_id == row_id {
                return row.row_data;
            }
        }

        null()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_exd_free_rows(exd: &physis_EXD, rows: &physis_ExcelRows) {
    unsafe {
        let data = Vec::from_raw_parts(
            rows.row_data,
            rows.row_count as usize,
            rows.row_count as usize,
        );

        for i in 0..rows.row_count {
            let col_data = Vec::from_raw_parts(
                data[i as usize].column_data,
                exd.column_count as usize,
                exd.column_count as usize,
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
