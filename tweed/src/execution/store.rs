extern crate alloc;

use std::collections::BTreeMap;

use alloc::vec::Vec;

use crate::binary::table_section::Table;
use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::{
    binary::{global_section::GlobalType, type_section::FuncType},
    core::values::FromLeBytes,
    validation::side_table::SideTable,
    Value,
};
use alloc::string::String;

use super::instance::Instance;

#[derive(Debug, Default)]
pub struct StoreData {
    pub functions: Vec<FunctionInstance>,
    pub memories: Vec<MemoryInstance>,
    pub datas: Vec<DataInstance>,
    pub globals: Vec<GlobalInstance>,
    pub tables: Vec<TableInstance>,
    pub elements: Vec<ElementInstance>,
}

#[derive(Debug, Default)]
pub struct Store {
    pub data: StoreData,
    pub instances: BTreeMap<String, Instance>,
    pub reader: BTreeMap<String, WasmReader>,
}
pub fn add_instances<T>(target: &mut Vec<T>, mut new: Vec<T>) -> Vec<usize> {
    let start_index = target.len().saturating_sub(1);
    target.append(&mut new);
    let end_index = target.len().saturating_sub(1);
    (start_index..=end_index).collect()
}

impl Store {
    pub fn add_instance(
        &mut self,
        instance: Instance,
        name: &str,
        reader: &WasmReader,
    ) -> Result<()> {
        self.instances.insert(name.to_string(), instance.clone());
        self.reader.insert(name.to_string(), reader.clone());
        Ok(())
    }
    pub fn get_func(&self, instance_name: &str, func_name: &str) -> Result<&FunctionInstance> {
        let instance = self.get_instance(instance_name)?;
        let func_addr = instance.get_func_by_name(func_name)?;
        self.data
            .functions
            .get(func_addr)
            .ok_or(Error::UnknownExportCall)
    }
    pub fn get_instance(&self, name: &str) -> Result<&Instance> {
        let instance = self
            .instances
            .get(name)
            .ok_or(Error::InvalidFunctionIndex)?;
        Ok(instance)
    }
    pub fn get_instance_count(&self) -> usize {
        self.instances.len()
    }
    pub fn instance_exists(&self, name: &str) -> bool {
        self.instances.contains_key(name)
    }
    pub fn add_globals(&mut self, instances: Vec<GlobalInstance>) -> Vec<usize> {
        add_instances(&mut self.data.globals, instances)
    }
    pub fn add_functions(&mut self, instances: Vec<FunctionInstance>) -> Vec<usize> {
        add_instances(&mut self.data.functions, instances)
    }
    pub fn add_memories(&mut self, instances: Vec<MemoryInstance>) -> Vec<usize> {
        add_instances(&mut self.data.memories, instances)
    }
    pub fn add_datas(&mut self, instances: Vec<DataInstance>) -> Vec<usize> {
        add_instances(&mut self.data.datas, instances)
    }
    pub fn add_tables(&mut self, instances: Vec<TableInstance>) -> Vec<usize> {
        add_instances(&mut self.data.tables, instances)
    }
    pub fn add_elements(&mut self, instances: Vec<ElementInstance>) -> Vec<usize> {
        add_instances(&mut self.data.elements, instances)
    }
}

pub fn map_memory<'a>(
    index: usize,
    instance: &Instance,
    store: &'a mut StoreData,
) -> Result<&'a mut MemoryInstance> {
    let mem_addr = instance
        .memaddr
        .get(index) // only one memory allowed
        .ok_or(Error::InvalidMemoryIndex)?;
    store
        .memories
        .get_mut(*mem_addr)
        .ok_or(Error::InvalidMemoryIndex)
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
    pub fn store(&mut self, address: usize, offset: usize, data: &[u8]) -> Result<()> {
        let abs_address = offset
            .checked_add(address)
            .ok_or(Error::MemoryAccessOutOfBounds)?;
        let end = abs_address + data.len();
        if end > self.data.len() {
            return Err(Error::MemoryAccessOutOfBounds);
        }
        self.data[abs_address..end].copy_from_slice(data);
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
    pub instance: String,
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
            .ok_or(Error::TableAccessOutOfBounds)?;
        Ok(*res)
    }
    pub fn insert(&mut self, offset: usize, vals: &[Value]) -> Result<()> {
        let end = offset
            .checked_add(vals.len())
            .ok_or(Error::TableAccessOutOfBounds)?;
        if end > self.reference_values.len() {
            return Err(Error::TableAccessOutOfBounds);
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
