use crate::{ffi_to_c_string, physis_Buffer};
use physis::pbd::PreBoneDeformer;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PBD {
    p_ptr: *mut PreBoneDeformer,
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_parse_pbd(buffer: physis_Buffer) -> physis_PBD {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(pbd) = PreBoneDeformer::from_existing(&data.to_vec()) {
        physis_PBD {
            p_ptr: Box::leak(Box::new(pbd)),
        }
    } else {
        physis_PBD { p_ptr: null_mut() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PreBoneDeformBone {
    name: *const c_char,
    deform: [f32; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_PreBoneDeformMatrices {
    num_bones: i32,
    bones: *mut physis_PreBoneDeformBone,
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_pbd_get_deform_matrix(
    pbd: physis_PBD,
    from_body_id: u16,
    to_body_id: u16,
) -> physis_PreBoneDeformMatrices {
    unsafe {
        if let Some(prebd) = (*pbd.p_ptr).get_deform_matrices(from_body_id, to_body_id) {
            let mut c_bones = vec![];

            for bone in &prebd.bones {
                c_bones.push(physis_PreBoneDeformBone {
                    name: ffi_to_c_string(&bone.name),
                    deform: bone.deform,
                });
            }

            let mat = physis_PreBoneDeformMatrices {
                num_bones: c_bones.len() as i32,
                bones: c_bones.as_mut_ptr(),
            };

            mem::forget(c_bones);

            mat
        } else {
            physis_PreBoneDeformMatrices {
                num_bones: 0,
                bones: null_mut(),
            }
        }
    }
}
