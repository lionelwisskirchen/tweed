extern crate alloc;

use super::adresses::{
    Address, DataAddr, ElementAddr, FuncAddr, GlobalAddr, InstanceAddr, MemoryAddr, ReaderAddr,
    TableAddr,
};
use super::instances::{
    DataInstance, ElementInstance, FuncInstance, GlobalInstance, MemoryInstance, ModuleInstance,
    TableInstance,
};
use crate::core::error::ExecutionError;
use crate::core::validated_wasm_reader::ValidatedWasmReader;

use alloc::vec::Vec;
use core::marker::PhantomData;
pub type Result<T> = core::result::Result<T, ExecutionError>;

/// The runtime Store of the WebAssembly engine.
///
/// The store represents all global state that can be manipulated by WebAssembly programs.
/// It consists of the runtime representation of all instances of functions, tables,
/// memories, globals, tags, element segments and data segments that have been
/// allocated during the life time of the abstract machine.
///
/// <https://webassembly.github.io/spec/core/exec/runtime.html#store>
#[derive(Debug, Default)]
pub struct Store {
    pub functions: FunctionStore,
    pub memories: MemoryStore,
    pub datas: DataStore,
    pub globals: GlobalStore,
    pub tables: TableStore,
    pub elements: ElementStore,
    /// The in-place specific bytecode reader
    pub reader: ReaderStore,
    /// The WebAssembly instances sharing the Store
    pub instances: InstanceStore,
}

/// The sub stores for the runtime representation of all instances.
pub type GlobalStore = TypedStore<GlobalInstance, GlobalAddr>;
pub type FunctionStore = TypedStore<FuncInstance, FuncAddr>;
pub type MemoryStore = TypedStore<MemoryInstance, MemoryAddr>;
pub type DataStore = TypedStore<DataInstance, DataAddr>;
pub type TableStore = TypedStore<TableInstance, TableAddr>;
pub type ElementStore = TypedStore<ElementInstance, ElementAddr>;
type ReaderStore = TypedStore<ValidatedWasmReader, ReaderAddr>;
type InstanceStore = TypedStore<ModuleInstance, InstanceAddr>;

/// A strongly typed storage container used by the runtime to manage
/// all internal WebAssembly store entities (functions, globals, memories, tables, etc.).
/// `TypedStore` provides a type-safe abstraction over a simple contiguous
/// `Vec<T>` and associates each stored value with a typed address (`Addr`).
///
/// # Type Parameters
/// - `T`: The stored runtime representation (e.g. `FuncInstance`)
/// - `Addr`: A typed address implementing [`Address`], used to index into the store
#[derive(Debug)]
pub struct TypedStore<T, Addr> {
    items: Vec<T>,
    _marker: PhantomData<Addr>,
}
impl<T, Addr> TypedStore<T, Addr>
where
    Addr: Address,
{
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn get(&self, addr: Addr) -> Result<&T> {
        self.items
            .get(addr.as_usize())
            .ok_or(ExecutionError::UnknownInstance)
    }
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn get_mut(&mut self, addr: Addr) -> Result<&mut T> {
        self.items
            .get_mut(addr.as_usize())
            .ok_or(ExecutionError::UnknownInstance)
    }
    pub fn add(&mut self, mut instances: Vec<T>) -> Vec<Addr> {
        let start = self.items.len();
        let len = instances.len();
        self.items.append(&mut instances);
        (start..start + len).map(|i| Addr::from_usize(i)).collect()
    }
    pub fn add_one(&mut self, instance: T) -> Addr {
        self.items.push(instance);
        Addr::from_usize(self.items.len() - 1)
    }
}

impl<T, Addr> Default for TypedStore<T, Addr> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl InstanceStore {
    pub fn reserve_instance_addr(&mut self) -> InstanceAddr {
        let addr = InstanceAddr(self.items.len());
        self.items.push(ModuleInstance::default());
        addr
    }
    pub fn insert_instance_at_addr(
        &mut self,
        addr: InstanceAddr,
        instance: ModuleInstance,
    ) -> Result<InstanceAddr> {
        let InstanceAddr(id) = addr;
        let slot = self
            .items
            .get_mut(id)
            .ok_or(ExecutionError::InstanceNameTaken)?;

        *slot = instance;
        Ok(addr)
    }
}

/// Defines the Environment, when a host function is executed.
#[derive(Debug)]
pub struct HostEnv<'store> {
    pub memories: &'store Vec<MemoryInstance>,
    pub globals: &'store Vec<GlobalInstance>,
}

impl<'a> From<&'a mut Store> for HostEnv<'a> {
    fn from(store: &'a mut Store) -> Self {
        HostEnv {
            memories: &store.memories.items,
            globals: &store.globals.items,
        }
    }
}

pub fn map_memory<'a>(
    index: usize,
    instance: &ModuleInstance,
    memories: &'a mut MemoryStore,
) -> Result<&'a mut MemoryInstance> {
    let mem_addr = instance
        .memaddr
        .get(index) // only one memory allowed
        .ok_or(ExecutionError::InvalidMemoryIndex)?;
    memories.get_mut(*mem_addr)
}
