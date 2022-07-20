extern crate core;

use core::ffi;
use std::ffi::CStr;
use std::{mem, slice};
use std::os::raw::{c_char, c_uint, c_uchar};
use std::ptr::null_mut;
use physis::gamedata::GameData;
use physis::blowfish::Blowfish;
use physis::installer::install_game;
use physis::patch::process_patch;

/// Initializes a new GameData structure. Path must be a valid game path, or else it will return NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_initialize(path : *const c_char) -> *mut GameData {
    unsafe {
        let mut game_data = Box::new(GameData::from_existing(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap());

        game_data.reload_repositories();

        Box::leak(game_data)
    }
}

#[no_mangle] pub extern "C" fn physis_gamedata_free(game_data : *mut GameData) {
    unsafe {
        drop(Box::from_raw(game_data));
    }
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[no_mangle] pub extern "C" fn physis_gamedata_extract_file(game_data : &GameData, path : *const c_char, size : *mut u32, data : &mut *mut u8) {
    unsafe {
        let mut d = game_data.extract(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap();

        *data = d.as_mut_ptr();
        *size = d.len() as u32;

        std::mem::forget(d);
    }
}

/// Checks if the file at `path` exists.
#[no_mangle] pub extern "C" fn physis_gamedata_exists(game_data : &GameData, path : *const c_char) -> bool {
    unsafe {
        game_data.exists(CStr::from_ptr(path).to_string_lossy().as_ref())
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_initialize(key : *mut u8, key_size : c_uint) -> *mut Blowfish {
    let data = unsafe { slice::from_raw_parts(key, key_size as usize) };
    Box::into_raw(Box::new(Blowfish::new(&data)))
}

#[no_mangle] pub extern "C" fn physis_blowfish_free(blowfish : *mut Blowfish) {
    unsafe {
        drop(Box::from_raw(blowfish));
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_encrypt(blowfish : &Blowfish, in_data : *mut u8, in_data_size : c_uint, out_data : &mut *mut u8, out_data_size : *mut u32) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.encrypt(&*in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false
    }
}

#[no_mangle] pub extern "C" fn physis_blowfish_decrypt(blowfish : &Blowfish, in_data : *mut u8, in_data_size : c_uint, out_data : &mut *mut u8, out_data_size : *mut u32) -> bool {
    let in_data = unsafe { slice::from_raw_parts(in_data, in_data_size as usize) };

    let result = blowfish.decrypt(&*in_data);

    match result {
        Some(mut out_data_vec) => {
            unsafe {
                *out_data = out_data_vec.as_mut_ptr();
                *out_data_size = out_data_vec.len() as u32;
            }

            mem::forget(out_data_vec);

            true
        }
        None => false
    }
}

#[no_mangle] pub extern "C" fn physis_patch_process(data_path : *const c_char, path : *const c_char) {
    unsafe {
        process_patch(CStr::from_ptr(data_path).to_str().unwrap(), CStr::from_ptr(path).to_str().unwrap())
    }
}

#[no_mangle] pub extern "C" fn physis_install_game(installer_path : *const c_char, game_directory : *const c_char) {
    unsafe {
        install_game(CStr::from_ptr(installer_path).to_str().unwrap(), CStr::from_ptr(game_directory).to_str().unwrap());
    }
}