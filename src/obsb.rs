// SPDX-FileCopyrightText: 2026 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(unused)] // cbindgen madness

use crate::{ffi_to_c_string, physis_Buffer};
use physis::Platform;
use physis::ReadableFile;
use physis::envs::EnvTimelineElement;
use physis::obsb::Obsb;
use std::ffi::c_char;
use std::ptr::{null, null_mut};
use std::slice;

#[repr(C)]
pub struct physis_Obsb {
    envs_count: u32,
    envs: *mut physis_Envs,
}

impl Default for physis_Obsb {
    fn default() -> Self {
        Self {
            envs_count: 0,
            envs: null_mut(),
        }
    }
}

#[repr(C)]
pub struct physis_Envs {
    section_count: u32,
    sections: *mut physis_EnvChildSection,
}

#[repr(C)]
pub struct physis_EnvChildSection {
    weather_id: u32,
    timeline_count: u32,
    timelines: *mut physis_EnvTimelineElement,
}

#[repr(C)]
pub enum physis_EnvTimelineElement {
    ChangeVisibility {
        point_count: u32,
        points: *mut physis_EnvChangeVisibility,
    },
    Unknown,
}

#[repr(C)]
pub struct physis_EnvChangeVisibility {
    time: f32,
    visible: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn physis_obsb_parse(platform: Platform, buffer: physis_Buffer) -> physis_Obsb {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(obsb) = Obsb::from_existing(platform, data) {
        let mut c_envss = Vec::new();
        for env in &obsb.envs {
            let mut c_sections = Vec::new();
            for section in &env.sections {
                let mut c_timelines = Vec::new();
                for timeline in &section.timelines {
                    let c_timeline = match &timeline.data {
                        EnvTimelineElement::ChangeVisibility(elements) => {
                            let mut c_points = Vec::new();
                            for element in elements {
                                c_points.push(physis_EnvChangeVisibility {
                                    time: element.time,
                                    visible: element.visible,
                                });
                            }

                            let c_element = physis_EnvTimelineElement::ChangeVisibility {
                                point_count: c_points.len() as u32,
                                points: c_points.as_mut_ptr(),
                            };

                            std::mem::forget(c_points);

                            c_element
                        }
                        _ => physis_EnvTimelineElement::Unknown,
                    };

                    c_timelines.push(c_timeline);
                }

                let c_section = physis_EnvChildSection {
                    weather_id: section.weather_id,
                    timeline_count: c_timelines.len() as u32,
                    timelines: c_timelines.as_mut_ptr(),
                };

                std::mem::forget(c_timelines);

                c_sections.push(c_section);
            }

            let c_envs = physis_Envs {
                section_count: c_sections.len() as u32,
                sections: c_sections.as_mut_ptr(),
            };

            std::mem::forget(c_sections);

            c_envss.push(c_envs);
        }

        let c_obsb = physis_Obsb {
            envs_count: c_envss.len() as u32,
            envs: c_envss.as_mut_ptr(),
        };

        std::mem::forget(c_envss);

        return c_obsb;
    }

    physis_Obsb::default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn physis_obsb_debug(
    platform: Platform,
    buffer: physis_Buffer,
) -> *const c_char {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Ok(obsb) = Obsb::from_existing(platform, data) {
        ffi_to_c_string(&format!("{obsb:#?}"))
    } else {
        null()
    }
}
