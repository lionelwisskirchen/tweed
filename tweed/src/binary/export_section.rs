extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet};

use crate::core::error::ValidationError;
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;

use super::module_environment::ModuleEnvironment;
pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExportContext {
    FunctionExport(u32),
    TableExport(u32),
    MemoryExport(u32),
    GlobalExport(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Export {
    pub name: String,
    pub context: ExportContext,
}

pub fn process_export_section(
    reader: &mut WasmReader,
    module: &ModuleEnvironment,
    refs: &mut BTreeSet<usize>,
) -> Result<Vec<Export>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();
    let number_of_exports = reader.next_leb128_u32()?;
    let mut aggregated_exports = BTreeMap::new();
    for _ in 0..number_of_exports {
        let length_of_export_name = reader.next_leb128_u32()?;
        let export_name = reader
            .next_chunk_as_string(length_of_export_name as usize)
            .map_err(|e| match e {
                ValidationError::ReadingOutOfBounds => ValidationError::LengthOutOfBounds,
                _ => e,
            })?;
        let export_kind = reader.next_byte()?;

        let index = reader.next_leb128_u32()?;
        let context = match export_kind {
            0x00 => {
                // The func section holds the signature to code mapping for imported and local functions.
                // That's why the number of imported functions is not needed addtionally.
                if (index as usize) >= module.funcsec.len() {
                    return Err(ValidationError::UnknownFunction(None));
                }
                refs.insert(index as usize);
                ExportContext::FunctionExport(index)
            }
            0x01 => {
                if (index as usize) >= (module.tablesec.len() + module.importsec.tables.len()) {
                    return Err(ValidationError::UnknownTable);
                }
                ExportContext::TableExport(index)
            }
            0x02 => {
                if (index as usize) >= (module.memorysec.len() + module.importsec.memories.len()) {
                    return Err(ValidationError::UnknownMemory);
                }
                ExportContext::MemoryExport(index)
            }
            0x03 => {
                if (index as usize) >= (module.globalsec.len() + module.importsec.globals.len()) {
                    return Err(ValidationError::UnknownGlobal(None));
                }
                ExportContext::GlobalExport(index)
            }
            _ => return Err(ValidationError::UnsupportedExportKind),
        };
        // Check for duplicate export names
        if aggregated_exports
            .insert(
                export_name.clone(),
                Export {
                    name: export_name,
                    context,
                },
            )
            .is_some()
        {
            return Err(ValidationError::DuplicateExportName);
        }
    }
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(aggregated_exports.into_values().collect::<Vec<_>>())
}
