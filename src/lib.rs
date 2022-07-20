use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::ptr::null_mut;
use physis::gamedata::GameData;

/// Initializes a new GameData structure. Path must be a valid game path, or else it will return NULL.
#[no_mangle] pub extern fn gamedata_initialize(path : *const c_char) -> *mut GameData {
    unsafe {
        let mut game_data = Box::new(GameData::from_existing(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap());

        game_data.reload_repositories();

        Box::leak(game_data)
    }
}

/// Extracts the raw game file from `path`, and puts it in `data` with `size` length. If the path was not found,
/// `size` is 0 and `data` is NULL.
#[no_mangle] pub extern fn gamedata_extract_file(game_data : &GameData, path : *const c_char, size : *mut u32, data : &mut *mut u8) {
    unsafe {
        let mut d = game_data.extract(CStr::from_ptr(path).to_string_lossy().as_ref()).unwrap();

        *data = d.as_mut_ptr();
        *size = d.len() as u32;

        std::mem::forget(d);
    }
}

/// Checks if the file at `path` exists.
#[no_mangle] pub extern fn gamedata_exists(game_data : &GameData, path : *const c_char) -> bool {
    unsafe {
        game_data.exists(CStr::from_ptr(path).to_string_lossy().as_ref())
    }
}

