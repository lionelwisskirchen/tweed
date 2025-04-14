extern crate alloc;
use crate::core::error::{Error, Result};
use crate::core::{value_types::ValType, wasm_reader::WasmReader};
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncCode {
    pub locals: Vec<ValType>,
    pub start: usize,
    pub end: usize,
}

pub fn process_code_section(reader: &mut WasmReader) -> Result<Vec<FuncCode>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let number_of_codes = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;

    let mut aggregated_functions = Vec::new();
    for _ in 0..number_of_codes {
        let func_body_size = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;

        let position_of_last_instruction = reader.calculate_offset(func_body_size as usize);

        let number_of_locals = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;
        let mut locals = Vec::new();
        for _ in 0..number_of_locals {
            let count = reader
                .next_leb128_u32()
                .ok_or(Error::InvalidSectionBytecode)?;
            let local_type =
                ValType::lookup(reader.next_byte().ok_or(Error::InvalidSectionBytecode)?)?;
            for _ in 0..count {
                locals.push(local_type);
            }
        }
        let function = FuncCode {
            locals,
            start: reader.current_index() + 1,
            end: position_of_last_instruction,
        };

        aggregated_functions.push(function);
        reader.seek(position_of_last_instruction + 1);
    }
    Ok(aggregated_functions)
}
