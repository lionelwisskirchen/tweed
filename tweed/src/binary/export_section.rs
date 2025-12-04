extern crate alloc;
use crate::core::error::{Result, ValidationError};
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExportContext {
    FunctionExport(u32),
    MemoryExport(u32),
    GlobalExport(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Export {
    pub name: String,
    pub context: ExportContext,
}

pub fn process_export_section(reader: &mut WasmReader) -> Result<Vec<Export>> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_exports = reader.next_leb128_u32()?;
    let mut aggregated_exports = Vec::new();
    for _ in 0..number_of_exports {
        let length_of_export_name = reader.next_leb128_u32()?;
        let export_name = reader.next_chunk_as_string(length_of_export_name as usize)?;

        let export_kind = reader.next_byte()?;
        let index = reader.next_leb128_u32()?;
        match export_kind {
            0x00 => {
                let context = ExportContext::FunctionExport(index);
                aggregated_exports.push(Export {
                    name: export_name,
                    context,
                });
            }
            0x02 => {
                let context = ExportContext::MemoryExport(index);
                aggregated_exports.push(Export {
                    name: export_name,
                    context,
                });
            }
            0x03 => {
                let context = ExportContext::GlobalExport(index);
                aggregated_exports.push(Export {
                    name: export_name,
                    context,
                });
            }
            _ => return Err(ValidationError::UnsupportedExportKind),
        }
    }
    Ok(aggregated_exports)
}
