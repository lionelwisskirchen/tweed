use crate::binary::memory_section::Limits;
use crate::binary::memory_section::Memory;
use crate::binary::table_section::Table;
use crate::core::error::ValidationError;
use crate::core::value_types::RefType;
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

#[derive(Debug, Clone)]
pub struct TableImport {
    pub module: String,
    pub name: String,
    pub context: Table,
}

#[derive(Debug, Clone)]
pub struct MemoryImport {
    pub module: String,
    pub name: String,
    pub context: Memory,
}

#[derive(Debug, Clone, Default)]
pub struct Imports {
    pub globals: Vec<GlobalImport>,
    pub tables: Vec<TableImport>,
    pub functions: Vec<FunctionImport>,
    pub memories: Vec<MemoryImport>,
}
pub type Result<T> = core::result::Result<T, ValidationError>;

/// Processes the **import section** according to
/// [WebAssembly specification: Import Section](https://webassembly.github.io/spec/core/binary/modules.html#import-section).
///
/// This function reads the imports and differs between Functions, Tables, Memories and Globals.
/// The Imports are stored in a [`Imports`] Struct which is returned at the end.
///
/// # Arguments
/// * `reader`: a mutable reference to the WasmReader to read more bytes.
///
/// # Return
/// Ok(Imports) if everything is fine.
///
/// # Error
/// * `ValidationError::MissingValue`: The reftype can't be determined correctly
/// * `ValidationError::MalformedMutability`: The imported global is either declared as mutable or
///   its mutability cannot be determined
/// * `ValidationError::UnsupportedImportKind`: The kind of the import cannot be parsed
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
            // Function Import
            0x00 => {
                let index = reader.next_leb128_u32()?;
                imports.functions.push(FunctionImport {
                    module: module_name,
                    name: import_name,
                    context: FuncDecl { sig_index: index },
                });
            }
            // Table Import
            0x01 => {
                let reftype = RefType::from_byte(reader.next_leb128_u32()? as u8)
                    .ok_or(ValidationError::MissingValue)?;
                let mut limits = Limits::default();
                let flags = reader.next_leb128_u32()?;
                limits.min = reader.next_leb128_u32()?;
                limits.max = if flags != 0 {
                    let max_val = reader.next_leb128_u32()?;
                    Some(max_val)
                } else {
                    None
                };

                let table = Table {
                    ref_type: reftype,
                    limit: limits,
                };
                let table_import = TableImport {
                    module: module_name,
                    name: import_name,
                    context: table,
                };
                imports.tables.push(table_import);
            }
            // Memory Import
            0x02 => {
                let flags = reader.next_leb128_u32()?;
                let min = reader.next_leb128_u32()?;
                let max = if flags != 0 {
                    let max_val = reader.next_leb128_u32()?;
                    Some(max_val)
                } else {
                    None
                };

                let limits = Limits { min, max };
                let memory = Memory { limits };

                let memory_import = MemoryImport {
                    module: module_name,
                    name: import_name,
                    context: memory,
                };

                imports.memories.push(memory_import);
            }
            // Global Import
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
            _ => return Err(ValidationError::UnsupportedImportKind),
        }
    }
    Ok(imports)
}
