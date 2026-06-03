use crate::core::error::ValidationError;
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;

use super::module_environment::ModuleEnvironment;
pub type Result<T> = core::result::Result<T, ValidationError>;

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
    module: &mut ModuleEnvironment,
) -> Result<()> {
    let section_size = reader.next_leb128_u32()?;
    let section_end = reader.current_index() + section_size as usize;
    let number_of_globals = reader.next_leb128_u32()?;
    for _ in 0..number_of_globals {
        let value_type = ValType::lookup(&reader.next_byte()?)?;

        let is_mutable = match reader
            .next_leb128_u32()
            .map_err(|_| ValidationError::MalformedMutability)?
        {
            0x00 => false,
            0x01 => true,
            _ => return Err(ValidationError::MalformedMutability),
        };
        let start = reader.current_index();
        /* We need to validate the constant expression here, as the size of the expression body is implicit.
        We can't seek to the end (0x0b) byte because there may be a 0x0b byte before it.
        For example, i32.const(900b). */
        validate_const_expr(
            reader,
            &[value_type],
            section_end,
            module,
            number_of_globals as usize,
        )?;

        let end = reader.current_index();
        let global_type = GlobalType {
            value_type,
            is_mutable,
        };
        module.globalsec.push(Global {
            global_type,
            start,
            end,
        });
    }
    if reader.current_index() != section_end {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(())
}
