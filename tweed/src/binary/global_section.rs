use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Global {
    pub global_type: GlobalType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub value_type: ValType,
    pub is_mutable: bool,
}

pub fn process_global_section(reader: &mut WasmReader) -> Result<Vec<Global>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let number_of_globals = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;

    let mut globals = Vec::new();
    for _ in 0..number_of_globals {
        let value_type = ValType::lookup(reader.next_byte().ok_or(Error::InvalidSectionBytecode)?)?;
        let is_mutable = match reader.next_leb128_u32().ok_or(Error::MalformedMutability)? {
            0x00 => false,
            0x01 => true,
            _ => return Err(Error::MalformedMutability),
        };
        let start = reader.current_index();
        let end = reader
            .read_until(0xb)
            .ok_or(Error::InvalidSectionBytecode)?;
        let global_type = GlobalType {
            value_type,
            is_mutable,
        };
        globals.push(Global {
            global_type,
            start,
            end,
        });
    }
    Ok(globals)
}
