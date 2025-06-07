extern crate alloc;

use std::collections::BTreeMap;

use alloc::vec::Vec;

use crate::binary::table_section::Table;
use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::{
    binary::{global_section::GlobalType, type_section::FuncType},
    core::values::FromLeBytes,
    validation::side_table::SideTable,
    Value,
};
use alloc::string::String;

use super::execution_stack::ExecutionStack;
use super::executor::Frame;
use super::instance::Instance;

pub const DEFAULT_MODULE: &str = "__interpreter_default__";

#[derive(Debug, Default)]
pub struct Store {
    pub functions: Vec<FunctionInstance>,
    pub memories: Vec<MemoryInstance>,
    pub datas: Vec<DataInstance>,
    pub globals: Vec<GlobalInstance>,
    pub tables: Vec<TableInstance>,
    pub elements: Vec<ElementInstance>,
    pub instances: BTreeMap<String, Instance>,
    pub call_stack: Vec<Frame>,
    pub stack: ExecutionStack,
}
pub fn add_instances<T>(target: &mut Vec<T>, mut new: Vec<T>) -> Vec<usize> {
    let start_index = target.len().saturating_sub(1);
    target.append(&mut new);
    let end_index = target.len().saturating_sub(1);
    (start_index..=end_index).collect()
}
impl Store {
    pub fn add_instance(&mut self, instance: Instance, name: Option<&str>) -> Result<Instance> {
        let na = match name {
            Some(n) => n,
            None => DEFAULT_MODULE,
        };
        self.instances.insert(na.to_string(), instance.clone());
        Ok(instance.clone())
    }
    pub fn get_instance(&self, name: Option<&str>) -> Result<Instance> {
        let res = match name {
            Some(n) => self.instances.get(n),
            None => self.instances.get(DEFAULT_MODULE),
        };
        Ok(res.ok_or(Error::InvalidFunctionIndex)?.clone())
    }
    pub fn add_globals(&mut self, instances: Vec<GlobalInstance>) -> Vec<usize> {
        add_instances(&mut self.globals, instances)
    }
    pub fn add_functions(&mut self, instances: Vec<FunctionInstance>) -> Vec<usize> {
        add_instances(&mut self.functions, instances)
    }
    pub fn add_memories(&mut self, instances: Vec<MemoryInstance>) -> Vec<usize> {
        add_instances(&mut self.memories, instances)
    }
    pub fn add_datas(&mut self, instances: Vec<DataInstance>) -> Vec<usize> {
        add_instances(&mut self.datas, instances)
    }
    pub fn add_tables(&mut self, instances: Vec<TableInstance>) -> Vec<usize> {
        add_instances(&mut self.tables, instances)
    }
    pub fn add_elements(&mut self, instances: Vec<ElementInstance>) -> Vec<usize> {
        add_instances(&mut self.elements, instances)
    }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableInstance {
    table_type: Table,
    reference_values: Vec<Value>,
}
impl TableInstance {
    pub fn new(table: &Table) -> Self {
        let data = vec![Value::RefNull(table.ref_type); table.limit.min as usize];
        TableInstance {
            table_type: table.clone(),
            reference_values: data,
        }
    }
    pub fn get(&self, index: usize) -> Result<Value> {
        let res = self
            .reference_values
            .get(index)
            .ok_or(Error::MemoryAccessOutOfBounds)?;
        Ok(*res)
    }
    pub fn insert(&mut self, offset: usize, vals: &[Value]) -> Result<()> {
        let end = offset
            .checked_add(vals.len())
            .ok_or(Error::MemoryAccessOutOfBounds)?;
        if end > self.reference_values.len() {
            return Err(Error::MemoryAccessOutOfBounds);
        }
        for (i, val) in vals.iter().enumerate() {
            self.reference_values[offset + i] = *val;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementInstance {
    reference_type: ValType,
    pub references: Vec<Value>,
}

impl ElementInstance {
    pub fn new(ref_type: &ValType, values: Vec<Value>) -> Self {
        ElementInstance {
            reference_type: *ref_type,
            references: values,
        }
    }
}
