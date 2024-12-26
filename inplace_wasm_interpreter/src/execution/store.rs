extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::binary::export_section::ExportContext;

use super::instance::FunctionInstance;

#[derive(Debug, Default)]
pub struct Store {
    pub functions: Vec<FunctionInstance>,
    pub exports: BTreeMap<String, ExportContext>,
    pub memories: Vec<MemoryInstance>,
    pub data: Vec<DataInstance>,
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
