use crate::binary::{
    code_section::FuncCode, export_section::Export, function_section::FuncDecl,
    type_section::FuncType,
};

use super::side_table::SideTable;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Module {
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub exportsec: Vec<Export>,
    pub sidetables: Vec<SideTable>,
}
