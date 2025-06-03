extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::core::error::{Error, Result};
use crate::{
    binary::{export_section::ExportContext, global_section::GlobalType, type_section::FuncType},
    core::values::FromLeBytes,
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
    data: Vec<u8>,
    max: Option<u32>,
}

impl MemoryInstance {
    pub fn new(data: Vec<u8>, max: Option<u32>) -> Self {
        MemoryInstance { data, max }
    }
    pub fn load_slice(&mut self, address: usize, bytes: usize) -> Result<&[u8]> {
        self.data
            .get(address..address + bytes)
            .ok_or(Error::MemoryAccessOutOfBounds)
    }
    pub fn load<T>(&mut self, address: usize, bytes: usize) -> Result<T>
    where
        T: FromLeBytes,
    {
        let data_bytes = self.load_slice(address, bytes)?;
        let data = T::from_le_bytes(data_bytes);
        Ok(data)
    }
    pub fn store(&mut self, address: usize, data: &[u8]) -> Result<()> {
        let end = address + data.len();
        if end > self.data.len() {
            return Err(Error::MemoryAccessOutOfBounds);
        }
        self.data[address..end].copy_from_slice(data);
        Ok(())
    }
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
