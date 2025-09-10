use crate::core::error::{Error, Result};
use crate::core::{value_types::ValType, wasm_reader::WasmReader};
use alloc::vec::Vec;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FuncType {
    pub parameter: Vec<ValType>,
    pub result: Vec<ValType>,
}

pub fn process_type_section(reader: &mut WasmReader) -> Result<Vec<FuncType>> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let mut function_types = Vec::new();
    let number_of_types = reader.next_leb128_u32()?;

    for _ in 0..number_of_types {
        reader.next_leb128_u32()?; // Can be ignored
        let number_of_params = reader.next_leb128_u32()?;
        let params = reader
            .next_chunk(number_of_params as usize)
            .ok_or(Error::InvalidSectionBytecode)?
            .iter()
            .map(ValType::lookup)
            .collect::<Result<Vec<_>>>()?;
        let number_of_results = reader.next_leb128_u32()?;
        let results = reader
            .next_chunk(number_of_results as usize)
            .ok_or(Error::InvalidSectionBytecode)?
            .iter()
            .map(ValType::lookup)
            .collect::<Result<Vec<_>>>()?;
        function_types.push(FuncType {
            parameter: params,
            result: results,
        });
    }
    Ok(function_types)
}
