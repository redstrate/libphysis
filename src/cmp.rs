use crate::physis_Buffer;
use physis::cmp::{RacialScalingParameters, CMP};
use physis::race::{Race, Subrace};
use std::ptr::null_mut;
use std::slice;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct physis_CMP {
    p_ptr: *mut CMP,
}

impl Default for physis_CMP {
    fn default() -> Self {
        Self {
            p_ptr: null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn physis_cmp_parse(buffer: physis_Buffer) -> physis_CMP {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(cmp) = CMP::from_existing(data) {
        let cmp = physis_CMP {
            p_ptr: Box::leak(Box::new(cmp)),
        };

        cmp
    } else {
        physis_CMP::default()
    }
}

// adapted from https://github.com/xivdev/Penumbra/blob/master/Penumbra/Meta/Files/CmpFile.cs#L50
fn get_rsp_index(subrace: Subrace) -> i32 {
    match subrace {
        Subrace::Midlander => 0,
        Subrace::Highlander => 1,
        Subrace::Wildwood => 10,
        Subrace::Duskwight => 11,
        Subrace::Plainsfolk => 20,
        Subrace::Dunesfolk => 21,
        Subrace::Seeker => 30,
        Subrace::Keeper => 31,
        Subrace::SeaWolf => 40,
        Subrace::Hellsguard => 41,
        Subrace::Raen => 50,
        Subrace::Xaela => 51,
        Subrace::Hellion => 60,
        Subrace::Lost => 61,
        Subrace::Rava => 70,
        Subrace::Veena => 71,
    }
}

#[no_mangle]
pub unsafe extern "C" fn physis_cmp_get_racial_scaling_parameters(
    cmp: physis_CMP,
    _: Race,
    subrace: Subrace,
) -> RacialScalingParameters {
    (*cmp.p_ptr).parameters[get_rsp_index(subrace) as usize]
}
