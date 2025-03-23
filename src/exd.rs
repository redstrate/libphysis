// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{exh::physis_EXH, ffi_free_string, ffi_to_c_string};
use physis::exd::{ColumnData, EXD};
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::ptr::null_mut;

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
    UInt64(u64),
}

#[repr(C)]
pub struct physis_ExcelRow {
    pub column_data: *mut physis_ColumnData,
}

#[repr(C)]
pub struct physis_EXD {
    pub p_ptr: *mut EXD,
    pub column_count: c_uint,
}

impl Default for physis_EXD {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
            column_count: 0,
        }
    }
}

#[repr(C)]
pub struct physis_ExcelRows {
    pub row_data: *mut physis_ExcelRow,
    pub row_count: c_uint,
}

impl Default for physis_ExcelRows {
    fn default() -> Self {
        Self {
            row_data: null_mut(),
            row_count: 0,
        }
    }
}

pub fn physis_exd_read_row(exd: &physis_EXD, exh: &physis_EXH, id: u32) -> physis_ExcelRows {
    unsafe {
        if let Some(rows) = (*exd.p_ptr).read_row(&*exh.p_ptr, id) {
            let mut c_rows: Vec<physis_ExcelRow> = Vec::new();
            for row in &rows {
                let mut c_col_data: Vec<physis_ColumnData> = Vec::new();

                for col_data in &row.data {
                    match col_data {
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

                c_rows.push(physis_ExcelRow {
                    column_data: c_col_data.as_mut_ptr(),
                });

                mem::forget(c_col_data);
            }

            let rows = physis_ExcelRows {
                row_data: c_rows.as_mut_ptr(),
                row_count: c_rows.len() as u32,
            };

            mem::forget(c_rows);

            return rows;
        }

        physis_ExcelRows::default()
    }
}

pub fn physis_exd_free_rows(exd: &physis_EXD, rows: &physis_ExcelRows) {
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
