use crate::{
    binary::{
        data_count_section::process_data_count_section, element_section::process_element_section,
        start_section::process_start_section,
    },
    core::{error::ValidationError, wasm_reader::WasmReader},
    validation::side_table::SideTable,
};
use alloc::vec::Vec;

use super::{
    code_section::{process_code_section, FuncCode},
    data_section::{process_data_section, DataSegment},
    element_section::ElementSegment,
    export_section::{process_export_section, Export},
    function_section::{process_function_section, FuncDecl},
    global_section::{process_global_section, Global},
    import_section::{process_import_section, Imports},
    memory_section::{process_memory_section, Limits},
    table_section::{process_table_section, Table},
    type_section::{process_type_section, FuncType},
};
pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, Default)]
pub struct ModuleEnvironment {
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub exportsec: Vec<Export>,
    pub memorysec: Vec<Limits>,
    pub datasec: Vec<DataSegment>,
    pub globalsec: Vec<Global>,
    pub tablesec: Vec<Table>,
    pub elementsec: Vec<ElementSegment>,
    pub importsec: Imports,
    pub datacountsec: u32,
    pub startsec: Option<u32>,
    pub sidetables: Vec<SideTable>,
}

pub fn create_module_environment(reader: &mut WasmReader) -> Result<ModuleEnvironment> {
    let mut module = ModuleEnvironment {
        ..Default::default()
    };
    reader
        .compare_chunk(&[0x00, 0x61, 0x73, 0x6D])
        .map_err(|_| ValidationError::InvalidMagic)?;
    reader
        .compare_chunk(&[0x01, 0x00, 0x00, 0x00])
        .map_err(|_| ValidationError::InvalidVersion)?;

    while let Ok(section_code) = reader.next_byte() {
        match section_code {
            0x00 => {
                let size = reader.next_leb128_u32()?;
                let end = reader.current_index() + size as usize + 1;
                let length_name = reader.next_leb128_u32()?;
                reader.next_chunk_as_string(length_name as usize)?;
                reader.seek(end);
            }
            0x01 => {
                info!("Process Type Section");
                module.typesec = process_type_section(reader)?;
            }

            0x02 => {
                info!("Process Import Section");
                let imports = process_import_section(reader)?;
                let signatures = imports
                    .functions
                    .clone()
                    .into_iter()
                    .map(|f| f.context)
                    .collect();
                module.funcsec = signatures;
                module.importsec = imports;
            }
            0x03 => {
                info!("Process Function Section");
                module.funcsec.extend(process_function_section(reader)?);
            }
            0x04 => {
                info!("Process Table Section");
                module.tablesec = process_table_section(reader)?;
            }
            0x05 => {
                info!("Process Memory Section");
                module.memorysec = process_memory_section(reader)?;
            }

            0x06 => {
                info!("Process Global Section");
                module.globalsec =
                    process_global_section(reader, &module.importsec.globals, &module.funcsec)?;
            }
            0x07 => {
                info!("Process Export Section");
                module.exportsec = process_export_section(reader)?;
            }
            0x08 => {
                info!("Process Start Section");
                module.startsec = Some(process_start_section(reader, &module)?);
            }
            0x09 => {
                info!("Process Element Section");

                module.elementsec = process_element_section(
                    reader,
                    &module.importsec.globals,
                    &module.funcsec,
                    &module.tablesec,
                    &module.importsec.tables,
                )?;
            }
            0x0A => {
                info!("Process Code Section");
                let (functions, sidetables) = process_code_section(reader, &module)?;
                module.codesec = functions;
                module.sidetables = sidetables;
            }
            0x0B => {
                info!("Process Data Section");
                module.datasec =
                    process_data_section(reader, &module.importsec.globals, &module.funcsec)?;
            }
            0x0C => {
                info!("Process Data Count Section");
                module.datacountsec = process_data_count_section(reader)?;
            }
            _ => {
                return Err(ValidationError::UnknownSectionCode(section_code));
            }
        }
    }
    Ok(module)
}
