// SPDX-FileCopyrightText: 2024 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use physis::race::{get_race_id, get_supported_subraces, Gender, Race, Subrace};

#[no_mangle]
pub extern "C" fn physis_get_race_code(race: Race, subrace: Subrace, gender: Gender) -> i32 {
    get_race_id(race, subrace, gender).unwrap()
}

#[repr(C)]
pub struct physis_SupportedSubraces {
    subraces: [Subrace; 2],
}

#[no_mangle]
pub extern "C" fn physis_get_supported_subraces(race: Race) -> physis_SupportedSubraces {
    physis_SupportedSubraces {
        subraces: get_supported_subraces(race),
    }
}
