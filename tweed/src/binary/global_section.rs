use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;
use alloc::vec::Vec;

use super::function_section::FuncDecl;
use super::import_section::GlobalImport;

#[derive(Debug, Clone)]
pub struct Global {
    pub global_type: GlobalType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GlobalType {
    pub value_type: ValType,
    pub is_mutable: bool,
}

pub fn process_global_section(
    reader: &mut WasmReader,
    global_imports: &[GlobalImport],
    functions: &[FuncDecl],
) -> Result<Vec<Global>> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_globals = reader.next_leb128_u32()?;

    let mut globals = Vec::new();
    for _ in 0..number_of_globals {
        let value_type = ValType::lookup(reader.next_byte().ok_or(Error::InvalidSectionBytecode)?)?;

        let is_mutable = match reader
            .next_leb128_u32()
            .map_err(|_| Error::MalformedMutability)?
        {
            0x00 => false,
            0x01 => true,
            _ => return Err(Error::MalformedMutability),
        };
        let start = reader.current_index();
        /* We need to validate the constant expression here, as the size of the expression body is implicit.
        We can't seek to the end (0x0b) byte because there may be a 0x0b byte before it.
        For example, i32.const(900b). */
        validate_const_expr(reader, global_imports, &[value_type], functions)?;
        let end = reader.current_index();
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
