// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::os::raw::c_char;

use physis::installer::install_game;

use crate::ffi_from_c_string;

#[cfg(feature = "game_install")]
#[no_mangle]
pub extern "C" fn physis_install_game(
    installer_path: *const c_char,
    game_directory: *const c_char,
) -> bool {
    let Some(r_installer_path) = ffi_from_c_string(installer_path) else {
        return false;
    };

    let Some(r_game_directory) = ffi_from_c_string(game_directory) else {
        return false;
    };

    install_game(&r_installer_path, &r_game_directory).is_ok()
}
