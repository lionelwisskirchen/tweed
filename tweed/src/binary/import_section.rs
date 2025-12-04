use crate::core::error::{Result, ValidationError};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;

use super::function_section::FuncDecl;
use super::global_section::GlobalType;

#[derive(Debug, Clone)]
pub struct GlobalImport {
    pub module: String,
    pub name: String,
    pub context: GlobalType,
}
#[derive(Debug, Clone)]
pub struct FunctionImport {
    pub module: String,
    pub name: String,
    pub context: FuncDecl,
}
#[derive(Debug, Clone, Default)]
pub struct Imports {
    pub globals: Vec<GlobalImport>,
    pub functions: Vec<FunctionImport>,
}

pub fn process_import_section(reader: &mut WasmReader) -> Result<Imports> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_imports = reader.next_leb128_u32()?;

    let mut imports = Imports::default();
    for _ in 0..number_of_imports {
        let length_of_module_name = reader.next_leb128_u32()?;
        let module_name = reader.next_chunk_as_string(length_of_module_name as usize)?;
        let length_of_import_name = reader.next_leb128_u32()?;
        let import_name = reader.next_chunk_as_string(length_of_import_name as usize)?;
        let import_kind = reader.next_byte()?;
        match import_kind {
            0x00 => {
                let index = reader.next_leb128_u32()?;
                imports.functions.push(FunctionImport {
                    module: module_name,
                    name: import_name,
                    context: FuncDecl { sig_index: index },
                });
            }
            0x03 => {
                let value_type = ValType::lookup(&reader.next_byte()?)?;
                let is_mutable = match reader
                    .next_leb128_u32()
                    .map_err(|_| ValidationError::MalformedMutability)?
                {
                    0x00 => false,
                    0x01 => true,
                    _ => return Err(ValidationError::MalformedMutability),
                };

                imports.globals.push(GlobalImport {
                    module: module_name,
                    name: import_name,
                    context: GlobalType {
                        value_type,
                        is_mutable,
                    },
                });
            }
            _ => return Err(ValidationError::UnsupportedExportKind),
        }
    }
    Ok(imports)
}
