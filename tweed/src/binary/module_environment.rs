use crate::{
    binary::{
        data_count_section::process_data_count_section, element_section::process_element_section,
        start_section::process_start_section,
    },
    core::{error::ValidationError, signature_lookup::lookup_signature, wasm_reader::WasmReader},
    validation::side_table::SideTable,
};
use alloc::vec::Vec;

use super::{
    code_section::{process_code_section, FuncCode},
    custom_section::process_custom_section,
    data_section::{process_data_section, DataSegment},
    element_section::ElementSegment,
    export_section::{process_export_section, Export},
    function_section::{process_function_section, FuncDecl},
    global_section::{process_global_section, Global},
    import_section::{process_import_section, Imports},
    memory_section::{process_memory_section, Memory},
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
    pub memorysec: Vec<Memory>,
    pub datasec: Vec<DataSegment>,
    pub globalsec: Vec<Global>,
    pub tablesec: Vec<Table>,
    pub elementsec: Vec<ElementSegment>,
    pub importsec: Imports,
    pub datacountsec: Option<u32>,
    pub startsec: Option<u32>,
    pub sidetables: Vec<SideTable>,
}

/// Defines the Order of the sections:
/// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    DataCount = 10,
    Code = 11,
    Data = 12,
}
impl TryFrom<u8> for Section {
    type Error = ValidationError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Custom),
            1 => Ok(Self::Type),
            2 => Ok(Self::Import),
            3 => Ok(Self::Function),
            4 => Ok(Self::Table),
            5 => Ok(Self::Memory),
            6 => Ok(Self::Global),
            7 => Ok(Self::Export),
            8 => Ok(Self::Start),
            9 => Ok(Self::Element),
            10 => Ok(Self::Code),
            11 => Ok(Self::Data),
            12 => Ok(Self::DataCount),
            _ => Err(ValidationError::MalformedSectionId),
        }
    }
}

pub fn create_module_environment(reader: &mut WasmReader) -> Result<ModuleEnvironment> {
    let mut module = ModuleEnvironment {
        ..Default::default()
    };
    let mut state = Section::Custom; // Custom can be everywhere, so this is fine
    reader
        .compare_chunk(&[0x00, 0x61, 0x73, 0x6D])
        .map_err(|e| match e {
            ValidationError::ReadingOutOfBounds => ValidationError::UnexpectedEnd,
            _ => ValidationError::InvalidMagic,
        })?;

    reader
        .compare_chunk(&[0x01, 0x00, 0x00, 0x00])
        .map_err(|e| match e {
            ValidationError::ReadingOutOfBounds => ValidationError::UnexpectedEnd,
            _ => ValidationError::InvalidVersion,
        })?;
    let mut number_of_internal_funcs = 0;
    while let Ok(section_code) = reader.next_byte() {
        let next_section = Section::try_from(section_code)?;
        // Custom Section can be at any place
        if next_section != Section::Custom && next_section <= state {
            return Err(ValidationError::UnexpectedContentAfterLastSection);
        }
        state = next_section;

        match next_section {
            Section::Custom => {
                info!("Process Custom Section");

                process_custom_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => ValidationError::UnexpectedEnd,
                    _ => e,
                })?;
            }
            Section::Type => {
                info!("Process Type Section");
                module.typesec = process_type_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
            }

            Section::Import => {
                info!("Process Import Section");
                let imports = process_import_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
                let signatures = imports
                    .functions
                    .clone()
                    .into_iter()
                    .map(|f| f.context)
                    .collect();
                module.funcsec = signatures;
                module.importsec = imports;
            }
            Section::Function => {
                info!("Process Function Section");
                let internal_funcs = process_function_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
                number_of_internal_funcs = internal_funcs.len();
                module.funcsec.extend(internal_funcs);
            }
            Section::Table => {
                info!("Process Table Section");
                module.tablesec = process_table_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
            }
            Section::Memory => {
                info!("Process Memory Section");

                // currently (WebAssembly V2) there is only one memory allowed at once
                if !module.importsec.memories.is_empty() {
                    return Err(ValidationError::MultipleMemories);
                }
                module.memorysec = process_memory_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
            }

            Section::Global => {
                info!("Process Global Section");
                process_global_section(reader, &mut module).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
            }
            Section::Export => {
                info!("Process Export Section");
                module.exportsec =
                    process_export_section(reader, &module).map_err(|e| match e {
                        ValidationError::ReadingOutOfBounds => {
                            ValidationError::UnexpectedEndOfFunctionOrSection
                        }
                        _ => e,
                    })?;
            }
            Section::Start => {
                info!("Process Start Section");
                let start_function_idx = process_start_section(reader).map_err(|e| match e {
                    ValidationError::ReadingOutOfBounds => {
                        ValidationError::UnexpectedEndOfFunctionOrSection
                    }
                    _ => e,
                })?;
                // Check if start section signature is valid [] -> []
                // https://webassembly.github.io/spec/core/valid/modules.html#start-function
                let signature_index = module
                    .funcsec
                    .get(start_function_idx as usize)
                    .ok_or(ValidationError::UnknownFunction)?;
                let signature = lookup_signature(signature_index.sig_index, &module.typesec);
                if !signature.parameter.is_empty() || !signature.result.is_empty() {
                    return Err(ValidationError::InvalidStartFunctionSignature);
                }
                module.startsec = Some(start_function_idx);
            }
            Section::Element => {
                info!("Process Element Section");
                module.elementsec =
                    process_element_section(reader, &mut module).map_err(|e| match e {
                        ValidationError::ReadingOutOfBounds => ValidationError::UnexpectedEnd,
                        _ => e,
                    })?;
            }
            Section::Code => {
                info!("Process Code Section");

                let (functions, sidetables) =
                    process_code_section(reader, &module).map_err(|e| match e {
                        ValidationError::ReadingOutOfBounds => {
                            ValidationError::UnexpectedEndOfFunctionOrSection
                        }
                        _ => e,
                    })?;

                module.codesec = functions;
                module.sidetables = sidetables;
            }
            Section::Data => {
                info!("Process Data Section");
                module.datasec =
                    process_data_section(reader, &mut module).map_err(|e| match e {
                        ValidationError::ReadingOutOfBounds => {
                            ValidationError::UnexpectedEndOfFunctionOrSection
                        }
                        _ => e,
                    })?;
            }
            Section::DataCount => {
                info!("Process Data Count Section");
                module.datacountsec =
                    Some(process_data_count_section(reader).map_err(|e| match e {
                        ValidationError::ReadingOutOfBounds => {
                            ValidationError::UnexpectedEndOfFunctionOrSection
                        }
                        _ => e,
                    })?);
            }
        }
    }

    if number_of_internal_funcs != module.codesec.len() {
        return Err(ValidationError::InconsistentFunctionCodeSectionLengths);
    }

    if module
        .datacountsec
        .is_some_and(|x| x as usize != module.datasec.len())
    {
        return Err(ValidationError::InconsistentDataDataCountSectionLengths);
    }
    info!("Finished Module parsing and validation");
    Ok(module)
}
