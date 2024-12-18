use crate::{
    binary::{
        code_section::FuncCode, export_section::Export, function_section::FuncDecl,
        type_section::FuncType,
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
    pub exportsec: Vec<Export>,
    pub sidetables: Vec<SideTable>,
}
