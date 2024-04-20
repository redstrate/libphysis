use physis::exd::EXD;
use std::os::raw::{c_char, c_uint};

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

    pub row_data: *mut physis_ExcelRow,
    pub row_count: c_uint,
}
