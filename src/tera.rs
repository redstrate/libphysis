use crate::{ffi_to_c_string, physis_Buffer};
use physis::tera::Terrain;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PlateModel {
    position: [f32; 2],
    filename: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Terrain {
    num_plates: i32,
    plates: *mut physis_PlateModel,
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_parse_tera(buffer: physis_Buffer) -> physis_Terrain {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(tera) = Terrain::from_existing(data) {
        let mut c_plates = vec![];

        for plate in &tera.plates {
            c_plates.push(physis_PlateModel {
                position: plate.position.into(),
                filename: ffi_to_c_string(&plate.filename),
            });
        }

        let mat = physis_Terrain {
            num_plates: c_plates.len() as i32,
            plates: c_plates.as_mut_ptr(),
        };

        mem::forget(c_plates);

        mat
    } else {
        physis_Terrain {
            num_plates: 0,
            plates: null_mut(),
        }
    }
}
