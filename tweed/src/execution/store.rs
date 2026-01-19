extern crate alloc;

use alloc::collections::BTreeMap;

use super::instance::Instance;
use crate::alloc::string::ToString;
use crate::binary::import_section::TableImport;
use crate::binary::memory_section::{Limits, Memory, MAX_MEM_PAGES, WASM_PAGE_SIZE};
use crate::binary::table_section::Table;
use crate::core::error::ExecutionError;
use crate::core::validated_wasm_reader::ValidatedWasmReader;
use crate::core::value_types::{RefType, ValType};

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
use std::fmt;
pub type Result<T> = core::result::Result<T, ExecutionError>;
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
    pub reader: BTreeMap<String, ValidatedWasmReader>,
}

#[derive(Debug)]
pub struct HostEnv<'store> {
    pub memories: &'store Vec<MemoryInstance>,
    pub globals: &'store Vec<GlobalInstance>,
}

/// Appends elements from `new` to `target` and returns the indices of the added elements.
///
/// # Arguments
/// * `target`: The vector receiving the new elements.
/// * `new`: The vector of elements to be appended.
///
/// # Returns
/// A `Vec<usize>` containing the indices where the new elements are located in `target`.
fn add_instances<T>(target: &mut Vec<T>, mut new: Vec<T>) -> Vec<usize> {
    if new.is_empty() {
        return vec![];
    }
    let start_index = target.len();
    target.append(&mut new);
    let end_index = target.len().saturating_sub(1);

    (start_index..=end_index).collect()
}

/// Appends an element to `target` and returns the index of the added element.
///
/// # Arguments
/// * `target`: The vector receiving the new elements.
/// * `new`: The vector of elements to be appended.
///
/// # Returns
/// The index where the new element is located in `target`.
fn add_instance<T>(target: &mut Vec<T>, new: T) -> usize {
    target.push(new);
    target.len().saturating_sub(1)
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
        reader: &ValidatedWasmReader,
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
            .ok_or(ExecutionError::UnknownExportCall)
    }

    /// Get a function that is in the store by its index
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the function to retrieve.
    ///
    /// # Returns
    /// A cloned [`FuncInstance`]
    ///
    /// # Errors
    ///  Returns [`Error::UnknownInstance`] if `index` is out of bounds.
    pub fn get_func_by_idx(&self, index: usize) -> Result<FuncInstance> {
        let func_instance = self
            .data
            .functions
            .get(index)
            .ok_or(ExecutionError::UnknownInstance)?
            .clone();

        Ok(func_instance)
    }

    pub fn rename_instance(&mut self, name: &str, new_name: &str) -> Result<()> {
        let mut instance = self
            .instances
            .remove(name)
            .ok_or(ExecutionError::UnknownInstance)?;
        let reader = self
            .reader
            .remove(name)
            .ok_or(ExecutionError::UnknownInstance)?;
        instance.name = new_name.to_string();
        self.add_instance(instance, new_name, &reader)?;
        Ok(())
        // TODO refactor and move to linker?
    }
    pub fn get_instance(&self, name: &str) -> Result<&Instance> {
        let instance = self
            .instances
            .get(name)
            .ok_or(ExecutionError::UnknownInstance)?;
        Ok(instance)
    }
    pub fn get_instance_count(&self) -> usize {
        self.instances.len()
    }
    pub fn instance_exists(&self, name: &str) -> bool {
        self.instances.contains_key(name)
    }

    /// Add a single global instance to the store
    ///
    /// # Arguments
    /// * `instance` - A global instance that is put on the store.
    ///
    /// # Returns
    /// The index of the global on the store
    pub fn add_global(&mut self, instance: GlobalInstance) -> usize {
        add_instance(&mut self.data.globals, instance)
    }

    /// Add a vector of globals to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated globals that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the globals on the store
    pub fn add_globals(&mut self, instances: Vec<GlobalInstance>) -> Vec<usize> {
        add_instances(&mut self.data.globals, instances)
    }

    /// Add a single function instance to the store
    ///
    /// # Arguments
    /// * `instance` - A function instance that is put on the store.
    ///
    /// # Returns
    /// The index of the function on the store
    pub fn add_function(&mut self, instance: FuncInstance) -> usize {
        add_instance(&mut self.data.functions, instance)
    }

    /// Add a vector of functions to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated functions that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the functions on the store
    pub fn add_functions(&mut self, instances: Vec<FuncInstance>) -> Vec<usize> {
        add_instances(&mut self.data.functions, instances)
    }

    /// Add a single memory instance to the store
    ///
    /// # Arguments
    /// * `instance` - A memory instance that is put on the store.
    ///
    /// # Returns
    /// The index of the memory on the store
    pub fn add_memory(&mut self, instance: MemoryInstance) -> usize {
        add_instance(&mut self.data.memories, instance)
    }

    /// Add a vector of memories to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated memories that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the memories on the store
    pub fn add_memories(&mut self, instances: Vec<MemoryInstance>) -> Vec<usize> {
        add_instances(&mut self.data.memories, instances)
    }

    /// Add a vector of datas to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated datas that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the datas on the store
    pub fn add_datas(&mut self, instances: Vec<DataInstance>) -> Vec<usize> {
        add_instances(&mut self.data.datas, instances)
    }

    /// Add a single table instance to the store
    ///
    /// # Arguments
    /// * `instance` - A table instance that is put on the store.
    ///
    /// # Returns
    /// The index of the table on the store
    pub fn add_table(&mut self, instance: TableInstance) -> usize {
        add_instance(&mut self.data.tables, instance)
    }

    /// Add a vector of tables to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated tables that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the tables on the store
    pub fn add_tables(&mut self, instances: Vec<TableInstance>) -> Vec<usize> {
        add_instances(&mut self.data.tables, instances)
    }

    /// Add a single element instance to the store
    ///
    /// # Arguments
    /// * `instance` - A element instance that is put on the store.
    ///
    /// # Returns
    /// The index of the element on the store
    pub fn add_element(&mut self, instance: ElementInstance) -> usize {
        add_instance(&mut self.data.elements, instance)
    }

    /// Add a vector of elements to the store
    ///
    /// # Arguments
    /// * `instances` - A vector of instantiated elements that are put on the store.
    ///
    /// # Returns
    /// An vec containg the indices of the elements on the store
    pub fn add_elements(&mut self, instances: Vec<ElementInstance>) -> Vec<usize> {
        add_instances(&mut self.data.elements, instances)
    }

    /// Get a [´FuncInstance´] by its index from the store.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the function to retrieve
    ///
    /// # Returns
    /// The cloned function instance if found
    ///
    /// # Errors
    /// [`ExecutionError::UnknownInstance`] - The [´FuncInstance´] can't be found.
    pub fn get_function_instance_by_idx(&self, index: usize) -> Result<FuncInstance> {
        let instance = self
            .data
            .functions
            .get(index)
            .ok_or(ExecutionError::UnknownInstance)?
            .clone();

        Ok(instance)
    }

    /// Get a [´GlobalInstance´] by its index from the store.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the global to retrieve
    ///
    /// # Returns
    /// The cloned global instance if found
    ///
    /// # Errors
    /// [`ExecutionError::UnknownInstance`] - The [´GlobalInstance´] can't be found.
    pub fn get_global_instance_by_idx(&self, index: usize) -> Result<GlobalInstance> {
        let instance = self
            .data
            .globals
            .get(index)
            .ok_or(ExecutionError::UnknownInstance)?
            .clone();

        Ok(instance)
    }

    /// Get a [´TableInstance´] by its index from the store.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the table to retrieve
    ///
    /// # Returns
    /// The cloned table instance if found
    ///
    /// # Errors
    /// [`ExecutionError::UnknownInstance`] - The [´TableInstance´] can't be found.
    pub fn get_table_instance_by_idx(&self, index: usize) -> Result<TableInstance> {
        let instance = self
            .data
            .tables
            .get(index)
            .ok_or(ExecutionError::UnknownInstance)?
            .clone();

        Ok(instance)
    }

    /// Get a [´MemoryInstance´] by its index from the store.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the memory to retrieve
    ///
    /// # Returns
    /// The cloned memory instance if found
    ///
    /// # Errors
    /// [`ExecutionError::UnknownInstance`] - The [´TableInstance´] can't be found.
    pub fn get_memory_instance_by_idx(&self, index: usize) -> Result<MemoryInstance> {
        let instance = self
            .data
            .memories
            .get(index)
            .ok_or(ExecutionError::UnknownInstance)?
            .clone();

        Ok(instance)
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
        .ok_or(ExecutionError::InvalidMemoryIndex)?;
    store
        .memories
        .get_mut(*mem_addr)
        .ok_or(ExecutionError::InvalidMemoryIndex)
}

#[derive(Debug, Clone)]
pub struct MemoryInstance {
    pub memory_type: Memory,
    pub bytes: Vec<u8>,
}

impl MemoryInstance {
    pub fn new(bytes: Vec<u8>, limits: Limits) -> Self {
        MemoryInstance {
            memory_type: Memory { limits },
            bytes,
        }
    }

    pub fn load_slice(&mut self, address: usize, bytes: usize) -> Result<&[u8]> {
        self.bytes
            .get(address..address + bytes)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)
    }

    pub fn size(&self) -> i32 {
        (self.bytes.len() as u32 / WASM_PAGE_SIZE) as i32
    }

    pub fn pages(&self) -> u32 {
        (self.bytes.len() / WASM_PAGE_SIZE as usize) as u32
    }

    pub fn grow(&mut self, amount: u32) -> Result<()> {
        let new_len = amount + self.pages();
        if new_len > MAX_MEM_PAGES {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        }
        if let Some(max) = self.memory_type.limits.max {
            if new_len > max {
                return Err(ExecutionError::MemoryAccessOutOfBounds);
            }
        }
        self.bytes.resize((new_len * WASM_PAGE_SIZE) as usize, 0);
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
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
        let end = abs_address + data.len();
        if end > self.bytes.len() {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        }
        self.bytes[abs_address..end].copy_from_slice(data);
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
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)
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

#[derive(PartialEq, Clone)]
pub struct TableInstance {
    table_type: Table,
    reference_values: Vec<Value>,
}
impl TableInstance {
    pub fn new(table: Table) -> Self {
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
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        Ok(*res)
    }
    pub fn insert(&mut self, offset: usize, vals: &[Value]) -> Result<()> {
        let end = offset
            .checked_add(vals.len())
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        if end > self.reference_values.len() {
            return Err(ExecutionError::TableAccessOutOfBounds);
        }
        for (i, val) in vals.iter().enumerate() {
            self.reference_values[offset + i] = *val;
        }

        Ok(())
    }

    /// Checks if the table's reference type is `ExternRef`.
    ///
    /// Returns `false` if the type is `ExternRef`.
    fn is_func_ref(&self) -> bool {
        match self.table_type.ref_type {
            RefType::FuncRef => true,
            RefType::ExternRef => false,
        }
    }

    /// Checks if the table's reference type is `ExternRef`.
    ///
    /// Returns `false` if the type is `FuncRef`.
    fn is_extern_ref(&self) -> bool {
        match self.table_type.ref_type {
            RefType::FuncRef => false,
            RefType::ExternRef => true,
        }
    }

    /// Check if the given table has the same reftype
    ///
    /// # Arguments
    /// * `table_import`: reference to the import table.
    ///
    /// # Return
    /// Error if the import type is not compatible
    pub fn ensure_ref_type_is_correct(&self, table_import: &TableImport) -> Result<()> {
        if (table_import.context.ref_type == RefType::FuncRef && self.is_func_ref())
            || (table_import.context.ref_type == RefType::ExternRef && self.is_extern_ref())
        {
            Ok(())
        } else {
            Err(ExecutionError::IncompatibleImportType)
        }
    }
}

impl fmt::Debug for TableInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TableInstance {{")?;
        writeln!(f, "  table_type: {:?},", self.table_type)?;
        write!(f, "  reference_values: ")?;

        // Format values with pipes
        if self.reference_values.is_empty() {
            write!(f, "|empty|")?;
        } else {
            for (i, value) in self.reference_values.iter().enumerate() {
                if i == 0 {
                    write!(f, "|")?;
                }
                write!(f, "({i}) {:?} |", value)?;
                if i % 10 == 0 && i != 0 {
                    writeln!(f)?;
                }
            }
        }

        writeln!(f, "\n}}")
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
