extern crate alloc;
use crate::core::error::ValidationError;
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;
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

pub fn process_export_section(reader: &mut WasmReader) -> Result<Vec<Export>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();
    let number_of_exports = reader.next_leb128_u32()?;
    let mut aggregated_exports = Vec::new();
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
        match export_kind {
            0x00 => {
                let context = ExportContext::FunctionExport(index);
                aggregated_exports.push(Export {
                    name: export_name,
                    context,
                });
            }
            0x01 => {
                let context = ExportContext::TableExport(index);
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
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(aggregated_exports)
}
