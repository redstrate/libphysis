use crate::physis_Buffer;
use physis::chardat::CharacterData;
use std::slice;

#[no_mangle]
pub extern "C" fn physis_chardat_parse(buffer: physis_Buffer) -> CharacterData {
    let data = unsafe { slice::from_raw_parts(buffer.data, buffer.size as usize) };

    CharacterData::from_existing(&data.to_vec()).unwrap()
}
