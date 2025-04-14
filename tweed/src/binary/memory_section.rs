use crate::core::error::{Error, Result};
use crate::core::wasm_reader::WasmReader;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

pub fn process_memory_section(reader: &mut WasmReader) -> Result<Vec<Limits>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let mut memories = Vec::new();
    let number_of_memories = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    for _ in 0..number_of_memories {
        let flags = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;
        let min = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;
        let mut max = None;
        if flags != 0 {
            max = Some(
                reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?,
            );
        }
        memories.push(Limits { min, max });
    }
    Ok(memories)
}
