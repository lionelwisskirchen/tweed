use crate::core::{error::ValidationError, wasm_reader::WasmReader};
pub type Result<T> = core::result::Result<T, ValidationError>;
pub fn process_data_count_section(reader: &mut WasmReader) -> Result<u32> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_data_segments = reader.next_leb128_u32()?;
    Ok(number_of_data_segments)
}
