use crate::{
    binary::{
        code_section::FuncCode, data_section::DataSegment, export_section::Export,
        function_section::FuncDecl, global_section::Global, import_section::Imports,
        memory_section::Limits, type_section::FuncType,
    },
    core::wasm_reader::WasmReader,
};

use super::side_table::SideTable;
use alloc::vec::Vec;

#[derive(Debug, Default, Clone)]
pub struct Module {
    pub reader: WasmReader,
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub memorysec: Vec<Limits>,
    pub exportsec: Vec<Export>,
    pub datasec: Vec<DataSegment>,
    pub globalsec: Vec<Global>,
    pub importsec: Imports,
    pub sidetables: Vec<SideTable>,
}
