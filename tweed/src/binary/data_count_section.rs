use crate::core::error::{Error, Result};
use crate::core::wasm_reader::WasmReader;

pub fn process_data_count_section(reader: &mut WasmReader) -> Result<u32> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let number_of_data_segments = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    Ok(number_of_data_segments)
}
