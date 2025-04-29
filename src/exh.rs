// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::physis_Buffer;
use physis::common::Language;
use physis::exh::{ColumnDataType, EXH};
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_EXH {
    pub(crate) p_ptr: *mut EXH,
    page_count: u32,
    language_count: u32,
    languages: *mut Language,
    column_count: u32,
    row_count: u32,
    column_types: *mut ColumnDataType,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_parse_excel_sheet_header(buffer: physis_Buffer) -> *mut physis_EXH {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    let exh = EXH::from_existing(data);
    if exh.is_none() {
        return null_mut();
    }

    let exh = Box::new(exh.unwrap());

    let mut c_languages: Vec<Language> = vec![];

    for lang in &exh.languages {
        c_languages.push(*lang);
    }

    let mut c_column_types: Vec<ColumnDataType> = vec![];

    for column in &exh.column_definitions {
        c_column_types.push(column.data_type.clone());
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
        row_count,
        column_types: c_column_types.as_mut_ptr(),
    };

    mem::forget(c_languages);
    mem::forget(c_column_types);

    Box::leak(Box::new(repositories))
}
