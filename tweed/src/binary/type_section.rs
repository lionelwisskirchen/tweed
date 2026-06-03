use crate::core::error::ValidationError;
use crate::core::{value_types::ValType, wasm_reader::WasmReader};
use alloc::vec::Vec;

pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FuncType {
    pub parameter: Vec<ValType>,
    pub result: Vec<ValType>,
}

pub fn process_type_section(reader: &mut WasmReader) -> Result<Vec<FuncType>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();
    let mut function_types = Vec::new();
    let number_of_types = reader.next_leb128_u32()?;

    for _ in 0..number_of_types {
        match reader
            .next_byte()
            .map_err(|_| ValidationError::LengthOutOfBounds)?
        {
            0x60 => {} // https://www.w3.org/TR/2025/CRD-wasm-core-2-20250616/#types%E2%91%A6
            _ => return Err(ValidationError::InvalidIntegerRepresentation),
        };
        let number_of_params = reader.next_leb128_u32()?;
        let params = reader
            .next_chunk(number_of_params as usize)?
            .iter()
            .map(ValType::lookup)
            .collect::<Result<Vec<_>>>()?;
        let number_of_results = reader.next_leb128_u32()?;
        let results = reader
            .next_chunk(number_of_results as usize)?
            .iter()
            .map(ValType::lookup)
            .collect::<Result<Vec<_>>>()?;
        function_types.push(FuncType {
            parameter: params,
            result: results,
        });
    }
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    }
    Ok(function_types)
}
