use crate::{
    binary::{
        code_section::FuncCode, data_section::DataSegment, element_section::ElementSegment,
        export_section::Export, function_section::FuncDecl, global_section::Global,
        import_section::Imports, memory_section::Memory, table_section::Table,
        type_section::FuncType,
    },
    core::validated_wasm_reader::ValidatedWasmReader,
};

use super::side_table::SideTable;
use alloc::vec::Vec;

#[derive(Debug, Default, Clone)]
pub struct Module {
    pub reader: ValidatedWasmReader,
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub memorysec: Vec<Memory>,
    pub exportsec: Vec<Export>,
    pub datasec: Vec<DataSegment>,
    pub globalsec: Vec<Global>,
    pub importsec: Imports,
    pub tablesec: Vec<Table>,
    pub elementsec: Vec<ElementSegment>,
    pub startsec: Option<u32>,
    pub sidetables: Vec<SideTable>,
}
