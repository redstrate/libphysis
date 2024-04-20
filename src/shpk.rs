use crate::physis_Buffer;
use physis::shpk::ShaderPackage;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Shader {
    len: i32,
    bytecode: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_SHPK {
    num_vertex_shaders: i32,
    vertex_shaders: *mut physis_Shader,
    num_pixel_shaders: i32,
    pixel_shaders: *mut physis_Shader,
}

impl Default for physis_SHPK {
    fn default() -> Self {
        Self {
            num_vertex_shaders: 0,
            vertex_shaders: null_mut(),
            num_pixel_shaders: 0,
            pixel_shaders: null_mut(),
        }
    }
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_parse_shpk(buffer: physis_Buffer) -> physis_SHPK {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(shpk) = ShaderPackage::from_existing(&data.to_vec()) {
        let mut c_vertex_shaders = vec![];
        let mut c_fragment_shaders = vec![];

        for shader in shpk.vertex_shaders {
            let mut bytecode = shader.bytecode.clone();

            let shader = physis_Shader {
                len: bytecode.len() as i32,
                bytecode: bytecode.as_mut_ptr(),
            };

            c_vertex_shaders.push(shader);

            mem::forget(bytecode);
        }

        for shader in shpk.pixel_shaders {
            let mut bytecode = shader.bytecode.clone();

            let shader = physis_Shader {
                len: bytecode.len() as i32,
                bytecode: bytecode.as_mut_ptr(),
            };

            c_fragment_shaders.push(shader);

            mem::forget(bytecode);
        }

        let mat = physis_SHPK {
            num_vertex_shaders: c_vertex_shaders.len() as i32,
            vertex_shaders: c_vertex_shaders.as_mut_ptr(),
            num_pixel_shaders: c_fragment_shaders.len() as i32,
            pixel_shaders: c_fragment_shaders.as_mut_ptr(),
        };

        mem::forget(c_vertex_shaders);
        mem::forget(c_fragment_shaders);

        mat
    } else {
        physis_SHPK::default()
    }
}
