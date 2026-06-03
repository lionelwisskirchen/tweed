use crate::core::error::ValidationError;

use crate::core::wasm_reader::WasmReader;
pub type Result<T> = core::result::Result<T, ValidationError>;

pub fn process_custom_section(reader: &mut WasmReader) -> Result<()> {
    let section_size = reader.next_leb128_u32()? as usize;
    let start_of_section = reader.current_index();

    // Ensure the declared section size does not exceed remaining input.
    if reader.rest().len() < section_size {
        return Err(ValidationError::LengthOutOfBounds);
    }

    // We require at least 1 byte to begin parsing the custom section payload,
    // because the first field is a LEB128-encoded string length.
    if section_size == 0 {
        return Err(ValidationError::UnexpectedEnd);
    }

    let length_name = reader.next_leb128_u32()?;
    reader.next_chunk_as_string(length_name as usize)?;

    if reader.current_index() > start_of_section + section_size {
        return Err(ValidationError::SectionSizeMismatch);
    }

    // Skip any remaining bytes in the section.
    // We intentionally ignore custom section payload contents.
    reader.seek(start_of_section + section_size + 1);

    Ok(())
}
