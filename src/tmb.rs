// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::physis_Buffer;
use physis::ReadableFile;
use physis::common::Platform;
use physis::tmb::Tmb;
use std::ptr::null;
use std::slice;

#[repr(C)]
#[derive(Clone)]
pub struct physis_Tmb {
    node_count: u32,
    nodes: *const physis_TimelineNode,
}

impl Default for physis_Tmb {
    fn default() -> Self {
        Self {
            node_count: 0,
            nodes: null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_TimelineNode {}

pub fn to_c_tmb(tmb: &Tmb) -> physis_Tmb {
    let mut c_nodes = Vec::new();
    for _node in &tmb.nodes {
        c_nodes.push(physis_TimelineNode {});
    }

    let tmb = physis_Tmb {
        node_count: c_nodes.len() as u32,
        nodes: c_nodes.as_ptr(),
    };

    std::mem::forget(c_nodes);

    tmb
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_tmb_parse(platform: Platform, buffer: physis_Buffer) -> physis_Tmb {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(tmb) = Tmb::from_existing(platform, data) {
        to_c_tmb(&tmb)
    } else {
        physis_Tmb::default()
    }
}
