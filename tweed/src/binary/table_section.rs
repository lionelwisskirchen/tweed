use crate::core::error::ValidationError;
use crate::core::value_types::RefType;
use crate::core::wasm_reader::WasmReader;
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
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_tables = reader.next_leb128_u32()?;
    let mut tables = Vec::new();
    for _ in 0..number_of_tables {
        let ref_type = reader.next_byte()?;
        let flags = reader.next_leb128_u32()?;
        let min = reader.next_leb128_u32()?;
        let max = if flags != 0 {
            let max_val = reader.next_leb128_u32()?;
            if max_val < min {
                return Err(ValidationError::SizeMinimumMustNotBeGreaterThanMaximum);
            }
            Some(max_val)
        } else {
            None
        };

        match ref_type {
            0x70 => tables.push(Table {
                ref_type: RefType::FuncRef,
                limit: Limits { min, max },
            }),
            0x6F => tables.push(Table {
                ref_type: RefType::ExternRef,
                limit: Limits { min, max },
            }),
            _ => return Err(ValidationError::UnknownTable),
        }
    }
    Ok(tables)
}
