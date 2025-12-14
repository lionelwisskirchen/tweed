extern crate alloc;
use crate::core::error::ValidationError;
use crate::core::{value_types::ValType, wasm_reader::WasmReader};
use crate::validation::side_table::SideTable;
use crate::validation::validator::CodeValidator;
use alloc::vec::Vec;

use super::module_environment::ModuleEnvironment;
pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncCode {
    pub locals: Vec<ValType>,
    pub start: usize,
    pub end: usize,
}

pub fn process_code_section(
    reader: &mut WasmReader,
    environment: &ModuleEnvironment,
) -> Result<(Vec<FuncCode>, Vec<SideTable>)> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_codes = reader.next_leb128_u32()?;

    let mut aggregated_functions = Vec::new();
    let mut aggregated_sidetables = Vec::new();
    for code_index in 0..number_of_codes {
        let func_body_size = reader.next_leb128_u32()?;
        let number_of_locals = reader.next_leb128_u32()?;
        let mut locals = Vec::new();
        for _ in 0..number_of_locals {
            let count = reader.next_leb128_u32()?;
            let local_type = ValType::lookup(&reader.next_byte()?)?;
            for _ in 0..count {
                locals.push(local_type);
            }
        }
        let start_position = reader.current_index() + 1;
        let mut validator = CodeValidator::new(reader, environment);
        let side_table = validator.validate_code(&locals, code_index as usize)?;
        if reader.current_index() > start_position + func_body_size as usize {
            return Err(ValidationError::ReadingOutOfBounds);
        }
        let function = FuncCode {
            locals,
            start: start_position,
            end: reader.current_index(),
        };

        aggregated_functions.push(function);
        aggregated_sidetables.push(side_table);
    }
    Ok((aggregated_functions, aggregated_sidetables))
}
