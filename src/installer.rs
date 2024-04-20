use crate::ffi_from_c_string;
use physis::installer::install_game;
use std::os::raw::c_char;

#[cfg(feature = "game_install")]
#[no_mangle]
pub extern "C" fn physis_install_game(
    installer_path: *const c_char,
    game_directory: *const c_char,
) -> bool {
    install_game(
        &ffi_from_c_string(installer_path),
        &ffi_from_c_string(game_directory),
    )
    .is_ok()
}
