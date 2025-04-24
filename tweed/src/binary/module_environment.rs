use crate::core::{
    error::{Error, Result},
    wasm_reader::WasmReader,
};
use alloc::vec::Vec;

use super::{
    code_section::{process_code_section, FuncCode},
    data_section::{process_data_section, DataSegment},
    export_section::{process_export_section, Export},
    function_section::{process_function_section, FuncDecl},
    global_section::{process_global_section, Global},
    import_section::{process_import_section, Imports},
    memory_section::{process_memory_section, Limits},
    type_section::{process_type_section, FuncType},
};

#[derive(Debug, Default)]
pub struct ModuleEnvironment {
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub exportsec: Vec<Export>,
    pub memorysec: Vec<Limits>,
    pub datasec: Vec<DataSegment>,
    pub globalsec: Vec<Global>,
    pub importsec: Imports,
}

pub fn create_module_environment(reader: &mut WasmReader) -> Result<ModuleEnvironment> {
    let mut module = ModuleEnvironment {
        ..Default::default()
    };
    reader
        .compare_chunk(&[0x00, 0x61, 0x73, 0x6D])
        .ok_or(Error::InvalidMagic)?;
    reader
        .compare_chunk(&[0x01, 0x00, 0x00, 0x00])
        .ok_or(Error::InvalidVersion)?;

    while let Some(section_code) = reader.next_byte() {
        match section_code {
            0x00 => break,
            0x01 => {
                module.typesec = process_type_section(reader)?;
            }

            0x02 => {
                module.importsec = process_import_section(reader)?;
            }
            0x03 => {
                module.funcsec = process_function_section(reader)?;
            }
            0x05 => {
                module.memorysec = process_memory_section(reader)?;
            }
            0x0a => {
                module.codesec = process_code_section(reader)?;
            }
            0x06 => {
                module.globalsec = process_global_section(reader)?;
            }
            0x07 => {
                module.exportsec = process_export_section(reader)?;
            }
            0x0B => {
                module.datasec = process_data_section(reader)?;
            }
            _ => {
                return Err(Error::UnknownSectionCode(*section_code));
            }
        }
    }
    Ok(module)
}
