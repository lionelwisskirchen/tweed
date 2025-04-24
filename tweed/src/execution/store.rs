extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
    binary::{export_section::ExportContext, global_section::GlobalType, type_section::FuncType},
    validation::side_table::SideTable,
    Value,
};

#[derive(Debug, Default)]
pub struct Store {
    pub functions: Vec<FunctionInstance>,
    pub exports: BTreeMap<String, ExportContext>,
    pub memories: Vec<MemoryInstance>,
    pub data: Vec<DataInstance>,
    pub globals: Vec<GlobalInstance>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MemoryInstance {
    pub data: Vec<u8>,
    pub max: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataInstance {
    pub data: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct FunctionInstance {
    pub signature: FuncType,
    pub locals: Vec<Value>,
    pub start: usize,
    pub end: usize,
    pub side_table: SideTable,
}

#[derive(Debug, Clone)]
pub struct GlobalInstance {
    pub global_type: GlobalType,
    pub value: Value,
}
