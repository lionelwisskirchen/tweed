use crate::core::error::{Error, Result};
use crate::core::value_types::RefType;
use crate::core::wasm_reader::WasmReader;
use alloc::vec::Vec;

use super::memory_section::Limits;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Table {
    pub ref_type: RefType,
    pub limit: Limits,
}

pub fn process_table_section(reader: &mut WasmReader) -> Result<Vec<Table>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let number_of_tables = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    let mut tables = Vec::new();
    for _ in 0..number_of_tables {
        let ref_type = *reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
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
        match ref_type {
            0x70 => tables.push(Table {
                ref_type: RefType::FuncRef,
                limit: Limits { min, max },
            }),
            0x6F => tables.push(Table {
                ref_type: RefType::ExternRef,
                limit: Limits { min, max },
            }),
            _ => unreachable!(),
        }
    }
    Ok(tables)
}
