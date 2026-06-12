use crate::core::error::ValidationError;
use crate::core::value_types::RefType;
use crate::core::wasm_reader::{TableTypeFlags, WasmReader};
use alloc::vec::Vec;

use super::memory_section::Limits;
pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Table {
    pub ref_type: RefType,
    pub limit: Limits,
}

/// Processes the **table section** according to
/// [WebAssembly specification: Table Section](https://webassembly.github.io/spec/core/binary/modules.html#table-section).
///
/// This function parses the table section into the [`Table`] structure. For this it determines the
/// size (limits) and the ref_type of the table.
///
/// # Arguments
/// * `reader`: a mutable reference to the WasmReader to read more bytes.
///
/// # Return
/// Ok(Vec<Table>) if everything is fine.
///
/// # Error
/// * `ValidationError::SizeMinimumMustNotBeGreaterThanMaximum`: The min value of the table size
///   is larger than the max value.
pub fn process_table_section(reader: &mut WasmReader) -> Result<Vec<Table>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();
    let number_of_tables = reader.next_leb128_u32()?;
    let mut tables = Vec::new();
    for _ in 0..number_of_tables {
        tables.push(process_table(reader)?)
    }
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(tables)
}
fn process_table(reader: &mut WasmReader) -> Result<Table> {
    let ref_type = reader.next_byte()?;
    let flags = TableTypeFlags(reader.next_flag()?);
    let min = reader.next_leb128_u32()?;
    let max = if flags.has_max() {
        let max_val = reader.next_leb128_u32().map_err(|e| match e {
            ValidationError::ReadingOutOfBounds => ValidationError::IntegerTooLarge,
            _ => e,
        })?;
        if max_val < min {
            return Err(ValidationError::SizeMinimumMustNotBeGreaterThanMaximum);
        }
        Some(max_val)
    } else {
        None
    };

    match ref_type {
        0x70 => Ok(Table {
            ref_type: RefType::FuncRef,
            limit: Limits { min, max },
        }),
        0x6F => Ok(Table {
            ref_type: RefType::ExternRef,
            limit: Limits { min, max },
        }),
        _ => Err(ValidationError::UnknownTable),
    }
}
