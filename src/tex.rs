use crate::physis_Buffer;
use physis::tex::Texture;
use std::ptr::null_mut;
use std::{mem, slice};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg(feature = "visual_data")]
pub struct physis_Texture {
    width: u32,
    height: u32,
    rgba_size: u32,
    rgba: *mut u8,
}

#[cfg(feature = "visual_data")]
#[no_mangle]
pub extern "C" fn physis_texture_parse(buffer: physis_Buffer) -> physis_Texture {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    if let Some(mut texture) = Texture::from_existing(&data.to_vec()) {
        let tex = physis_Texture {
            width: texture.width,
            height: texture.height,
            rgba_size: texture.rgba.len() as u32,
            rgba: texture.rgba.as_mut_ptr(),
        };

        mem::forget(texture.rgba);

        tex
    } else {
        physis_Texture {
            width: 0,
            height: 0,
            rgba_size: 0,
            rgba: null_mut(),
        }
    }
}
