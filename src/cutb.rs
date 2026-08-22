// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::cutb::{Cutscene, NodeData};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::slice;

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum physis_CutsceneNode {
    Ctds(physis_CTDS),
    Unknown,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_CTDS {
    level_name: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_Cutscene {
    num_nodes: u32,
    nodes: *mut physis_CutsceneNode,
}

impl Default for physis_Cutscene {
    fn default() -> Self {
        Self {
            num_nodes: 0,
            nodes: null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_cutb_parse(platform: Platform, buffer: physis_Buffer) -> physis_Cutscene {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(cutb) = Cutscene::from_existing(platform, data) {
        let mut c_nodes = Vec::new();
        for node in cutb.nodes {
            c_nodes.push(match node.node_data {
                NodeData::CTDS(ctds) => physis_CutsceneNode::Ctds(physis_CTDS {
                    level_name: ffi_to_c_string(&ctds.level_name),
                }),
                _ => physis_CutsceneNode::Unknown,
            })
        }

        let c_cutb = physis_Cutscene {
            num_nodes: c_nodes.len() as u32,
            nodes: c_nodes.as_mut_ptr(),
        };

        std::mem::forget(c_nodes);

        c_cutb
    } else {
        physis_Cutscene::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_cutb_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    match Cutscene::from_existing(platform, data) {
        Ok(cutb) => ffi_to_c_string(&format!("{cutb:#?}")),
        Err(err) => ffi_to_c_string(&format!("{err:#?}")),
    }
}
