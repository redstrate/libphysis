// SPDX-FileCopyrightText: 2025 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_from_c_string, ffi_to_c_string};
use physis::patch::{ChunkType, SqpkOperation, SqpkTargetInfo, ZiPatch};
use std::ffi::c_char;
use std::ptr::null_mut;

#[unsafe(no_mangle)]
pub extern "C" fn physis_patch_apply(data_dir: *const c_char, patch_path: *const c_char) -> bool {
    let Some(data_dir) = ffi_from_c_string(data_dir) else {
        return false;
    };

    let Some(patch_path) = ffi_from_c_string(patch_path) else {
        return false;
    };

    ZiPatch::apply(&data_dir, &patch_path).is_ok()
}

#[repr(C)]
pub struct physis_ZiPatchSqpkAddData {
    main_id: u16,
    sub_id: u16,
    file_id: u32,

    block_offset: u64,

    block_data_size: u32,
    block_data: *mut u8,
}

#[repr(C)]
pub struct physis_SqpkFileOperationData {
    path: *const c_char,
}

#[repr(C)]
#[allow(dead_code)]
pub enum physis_ZiPatchSqpkOperation {
    AddData(physis_ZiPatchSqpkAddData),
    FileOperation(physis_SqpkFileOperationData),
    TargetInfo(SqpkTargetInfo),
    Unknown,
}

#[repr(C)]
pub struct physis_ZiPatchSqpkChunk {
    operation: physis_ZiPatchSqpkOperation,
}

#[repr(C)]
#[allow(dead_code)]
pub enum physis_ZiPatchChunkType {
    Sqpk(physis_ZiPatchSqpkChunk),
    Unknown,
}

#[repr(C)]
pub struct physis_ZiPatchChunk {
    chunk_type: physis_ZiPatchChunkType,
}

#[repr(C)]
pub struct physis_ZiPatch {
    num_chunks: u32,
    chunks: *mut physis_ZiPatchChunk,
}

impl Default for physis_ZiPatch {
    fn default() -> Self {
        Self {
            num_chunks: 0,
            chunks: null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_patch_parse(patch_path: *const c_char) -> physis_ZiPatch {
    let Some(patch_path) = ffi_from_c_string(patch_path) else {
        return physis_ZiPatch::default();
    };

    let Ok(patch) = ZiPatch::list_operations(&patch_path) else {
        return physis_ZiPatch::default();
    };

    let mut c_chunks = Vec::new();
    for chunk in &patch.chunks {
        let chunk_type = match &chunk.chunk_type {
            ChunkType::Sqpk(sqpk) => physis_ZiPatchChunkType::Sqpk(physis_ZiPatchSqpkChunk {
                operation: match &sqpk.operation {
                    SqpkOperation::AddData(add_data) => {
                        let mut c_data = add_data.block_data.clone();

                        let c_add_data =
                            physis_ZiPatchSqpkOperation::AddData(physis_ZiPatchSqpkAddData {
                                main_id: add_data.main_id,
                                sub_id: add_data.sub_id,
                                file_id: add_data.file_id,
                                block_offset: add_data.block_offset,
                                block_data_size: c_data.len() as u32,
                                block_data: c_data.as_mut_ptr(),
                            });

                        std::mem::forget(c_data);

                        c_add_data
                    }
                    SqpkOperation::FileOperation(fop) => {
                        physis_ZiPatchSqpkOperation::FileOperation(physis_SqpkFileOperationData {
                            path: ffi_to_c_string(&fop.path),
                        })
                    }
                    SqpkOperation::TargetInfo(target_info) => {
                        physis_ZiPatchSqpkOperation::TargetInfo(target_info.clone())
                    }
                    _ => physis_ZiPatchSqpkOperation::Unknown,
                },
            }),
            _ => physis_ZiPatchChunkType::Unknown,
        };

        c_chunks.push(physis_ZiPatchChunk { chunk_type });
    }

    let c_patch = physis_ZiPatch {
        num_chunks: c_chunks.len() as u32,
        chunks: c_chunks.as_mut_ptr(),
    };

    std::mem::forget(c_chunks);

    c_patch
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_patch_index_path(
    sqpk_target_info: SqpkTargetInfo,
    main_id: u16,
    sub_id: u16,
    file_id: u32,
) -> *const c_char {
    ffi_to_c_string(
        &ZiPatch::index_path(&sqpk_target_info, main_id, sub_id, file_id)
            .to_str()
            .unwrap()
            .to_string(),
    )
}
