extern crate alloc;

use alloc::collections::BTreeMap;

use super::instance::Instance;
use super::linker::{MAX_MEM_PAGES, PAGE_SIZE};
use crate::alloc::string::ToString;
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
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Debug, Default)]
pub struct StoreData {
    pub functions: Vec<FuncInstance>,
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

#[derive(Debug)]
pub struct HostEnv<'store> {
    pub memories: &'store Vec<MemoryInstance>,
    pub globals: &'store Vec<GlobalInstance>,
}

pub fn add_instances<T>(target: &mut Vec<T>, mut new: Vec<T>) -> Vec<usize> {
    if new.is_empty() {
        return vec![];
    }
    let start_index = target.len();
    target.append(&mut new);
    let end_index = target.len().saturating_sub(1);

    (start_index..=end_index).collect()
}
impl<'a> From<&'a StoreData> for HostEnv<'a> {
    fn from(store: &'a StoreData) -> Self {
        HostEnv {
            memories: &store.memories,
            globals: &store.globals,
        }
    }
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
    pub fn get_func(&mut self, instance_name: &str, func_name: &str) -> Result<&mut FuncInstance> {
        let instance = self.get_instance(instance_name)?;
        let func_addr = instance.get_func_by_name(func_name)?;
        self.data
            .functions
            .get_mut(func_addr)
            .ok_or(Error::UnknownExportCall)
    }
    pub fn rename_instance(&mut self, name: &str, new_name: &str) -> Result<()> {
        let mut instance = self.instances.remove(name).ok_or(Error::UnknownInstance)?;
        let reader = self.reader.remove(name).ok_or(Error::UnknownInstance)?;
        instance.name = new_name.to_string();
        self.add_instance(instance, new_name, &reader)?;
        Ok(())
        // TODO refactor and move to linker?
    }
    pub fn get_instance(&self, name: &str) -> Result<&Instance> {
        let instance = self.instances.get(name).ok_or(Error::UnknownInstance)?;
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
    pub fn add_global(&mut self, instances: GlobalInstance) -> usize {
        *add_instances(&mut self.data.globals, vec![instances])
            .first()
            .unwrap()
        // TODO Refactor
    }
    pub fn add_function(&mut self, instances: FuncInstance) -> usize {
        *add_instances(&mut self.data.functions, vec![instances])
            .first()
            .unwrap()
        // TODO Refactor
    }
    pub fn add_functions(&mut self, instances: Vec<FuncInstance>) -> Vec<usize> {
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
    max: Option<u32>, // TODO Store Limit to be closer to spec!
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
    pub fn size(&self) -> i32 {
        (self.data.len() as u32 / PAGE_SIZE) as i32
    }
    pub fn pages(&self) -> u32 {
        (self.data.len() / PAGE_SIZE as usize) as u32
    }
    pub fn grow(&mut self, amount: u32) -> Result<()> {
        let new_len = amount + self.pages();
        if new_len > MAX_MEM_PAGES {
            return Err(Error::MemoryAccessOutOfBounds);
        }
        if let Some(max) = self.max {
            if new_len > max {
                return Err(Error::MemoryAccessOutOfBounds);
            }
        }
        self.data.resize((new_len * PAGE_SIZE) as usize, 0);
        Ok(())
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
impl DataInstance {
    pub fn clear(&mut self) {
        self.data = Vec::new();
    }

    pub fn load_slice(&self, address: usize, bytes: usize) -> Result<Vec<u8>> {
        self.data
            .get(address..address + bytes)
            .ok_or(Error::MemoryAccessOutOfBounds)
            .map(|s| s.to_vec())
    }
}

#[derive(Debug, Clone)]
pub enum FuncInstance {
    Internal(Internal),
    External(HostFunc),
}
impl FuncInstance {
    pub fn get_signature(&self) -> &FuncType {
        match self {
            FuncInstance::Internal(internal) => &internal.signature,
            FuncInstance::External(host_func) => &host_func.signature,
        }
    }
}

pub type Func = Rc<RefCell<dyn FnMut(&mut HostEnv, Vec<Value>) -> Result<Vec<Value>>>>;

#[derive(Clone)]
pub struct HostFunc {
    pub signature: FuncType,
    pub func: Func,
}
impl core::fmt::Debug for HostFunc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HostFunc")
            .field("signature", &self.signature)
            .field("func", &"<host func>")
            .finish()
    }
}
#[derive(Debug, Clone)]
pub struct Internal {
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
