use crate::core::error::ValidationError;
use crate::core::wasm_reader::WasmReader;

pub type Result<T> = core::result::Result<T, ValidationError>;

pub fn process_start_section(reader: &mut WasmReader) -> Result<u32> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let start_function_idx = reader.next_leb128_u32()?;
    Ok(start_function_idx)
}
