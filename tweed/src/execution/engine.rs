extern crate alloc;

use super::adresses::InstanceAddr;
use super::instances::function_instance::HostFunc;
use super::instances::{FuncInstance, GlobalInstance, MemoryInstance, TableInstance};
use super::store::{HostEnv, Store};

use alloc::rc::Rc;
use core::cell::RefCell;

use super::linker::Linker;
use crate::binary::global_section::GlobalType;
use crate::binary::memory_section::Limits;
use crate::binary::table_section::Table;
use crate::binary::type_section::FuncType;
use crate::core::error::ExecutionError;
use crate::core::value_types::ValType;

use crate::execution::executor::Executor;
use crate::execution::instantiator::ModuleInstantiator;
use crate::validation::module::Module;

use crate::Value;

use alloc::string::String;
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;
#[derive(Debug)]
pub enum InstanceRef {
    Name(String),
    Addr(InstanceAddr),
}

/// The core WebAssembly interpreter engine.
///
/// Manages module instantiation, linking, execution, and runtime storage.
#[derive(Debug, Default)]
pub struct Engine {
    executor: Executor,
    linker: Linker,
    store: Store,
}
impl Engine {
    /// Instantiates a module, resolves its imports, and invokes the start function if present.
    /// The imports are automatically wired into other modules via the linker.
    ///
    /// # Returns
    /// The store address of the instantiated module instance.
    pub fn instantiate(&mut self, module: Module) -> Result<InstanceRef> {
        let mut instantiator = ModuleInstantiator {};
        let imports = &module.importsec;

        let external = self.linker.resolve(imports)?;

        let store_addr = instantiator.module_instantiate(&mut self.store, module, &external)?;
        let start = self.store.instances.get(store_addr)?.start;
        if let Some(func_addr) = start {
            info!("Invoke start function with store address {func_addr}");
            self.executor.invoke(func_addr, &[], &mut self.store)?;
        }
        Ok(InstanceRef::Addr(store_addr))
    }
    /// Registers an instance under the given name in the linker.
    /// The engine itself does not support name-based invocation.
    ///
    /// # Important:
    /// Registering an Instance doesn't automatically exposes their exports.
    ///
    /// # Parameters
    /// - `name`: The name under which the instance will be registered in the linker.
    /// - `instance`: A reference identifying the instance to register (either by name or address).
    ///
    /// # Returns
    /// The registered module name.
    pub fn register(&mut self, name: &str, instance: InstanceRef) -> Result<InstanceRef> {
        let target_instance_addr = self.linker.resolve_instance(&instance)?;
        let name = self.linker.register_instance(name, target_instance_addr)?;
        Ok(InstanceRef::Name(name))
    }

    /// Publishes all exports of an instance, making them available for import by other instances.
    ///
    /// Export registration is performed via the linker, which operates on named instances only.
    /// Therefore, only instances identified by a module name can expose their exports.
    ///
    /// # Errors
    /// Returns [`ExecutionError::UnknownInstance`] if the provided
    /// `name` is an address.
    pub fn register_exports(&mut self, name: InstanceRef) -> Result<()> {
        let target_instance_addr = self.linker.resolve_instance(&name)?;
        let InstanceRef::Name(instance_name) = name else {
            return Err(ExecutionError::UnknownInstance);
        };
        let instance = self.store.instances.get(target_instance_addr)?;
        self.linker.register_exports(instance_name, instance)
    }
    /// Invokes a function exported by an instance.
    ///
    /// The instance can be referenced either by name or by direct address.
    /// If a name is provided, it is resolved via the linker to an instance address.
    /// The function is then resolved within the instance and executed by the executor.
    ///
    /// # Parameters
    /// - `instance_name`: Reference to the instance (by name or address) that exports the function.
    /// - `function_name`: Name of the function to invoke within the instance.
    /// - `params`: Arguments passed to the function.
    ///
    /// # Returns
    /// The result values returned by the executed function.
    pub fn invoke(
        &mut self,
        instance: InstanceRef,
        function_name: String,
        params: Vec<Value>,
    ) -> Result<Vec<Value>> {
        let instance_addr = self.linker.resolve_instance(&instance)?;
        let func_addr = self
            .store
            .instances
            .get(instance_addr)?
            .get_func_by_name(&function_name)?;
        self.executor.invoke(func_addr, &params, &mut self.store)
    }
    /// Register a new global, that can be imported by other modules
    ///
    /// This function takes a global and inserts it into the Store
    /// It also exposes it under the name to other instances in the Store.
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the global is imported.
    /// * `name`: the name of the global.
    /// * `value`: the value of the global.
    /// * `is_mutable`: indicates whether the global is mutable or not.
    pub fn add_global_import(
        &mut self,
        module: String,
        name: String,
        value: Value,
        is_mutable: bool,
    ) -> Result<()> {
        let global_type = GlobalType {
            value_type: ValType::from(value),
            is_mutable,
        };
        let global_instance = GlobalInstance { global_type, value };
        let global_addr = self.store.globals.add_one(global_instance);
        self.linker.register_global(module, name, global_addr)
    }
    /// Register a new function, that can be imported by other modules
    ///
    /// This function takes a host function and inserts it into the Store
    /// It also exposes it under the name to other instances in the Store.
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the table is imported.
    /// * `name`: the name of the table.
    /// * `ty`: the function type.
    /// * `func`: the actual function.
    pub fn add_function_import(
        &mut self,
        module: String,
        name: String,
        ty: FuncType,
        func: impl FnMut(&mut HostEnv, Vec<Value>) -> Result<Vec<Value>> + 'static,
    ) -> Result<()> {
        let func_instance = HostFunc {
            signature: ty,
            func: Rc::new(RefCell::new(func)),
        };
        let func_addr = self
            .store
            .functions
            .add_one(FuncInstance::External(func_instance));
        self.linker.register_function(module, name, func_addr)
    }
    /// Register a new table, that can be imported by other modules
    ///
    /// This function takes a table and inserts it into the Store
    /// It also exposes it under the name to other instances in the Store.
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the table is imported.
    /// * `name`: the name of the table.
    /// * `table`: the actual table.
    pub fn add_table_import(&mut self, module: String, name: String, table: Table) -> Result<()> {
        let table_instance = TableInstance::new(table.clone());
        let table_addr = self.store.tables.add_one(table_instance);
        self.linker.register_table(module, name, table_addr)
    }
    /// Register a new memory, that can be imported by other modules
    ///
    /// This function takes a memory and inserts it into the Store
    /// It also exposes it under the name to other instances in the Store.
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the table is imported.
    /// * `name`: the name of the table.
    /// * `memory`: the actual memory.
    pub fn add_memory_import(
        &mut self,
        module: String,
        name: String,
        min: u32,
        max: Option<u32>,
        mempagesize: u32,
    ) -> Result<()> {
        let data = vec![0; (min * mempagesize) as usize];
        let memory_instance = MemoryInstance::new(data, Limits { min, max }, mempagesize);
        let memory_addr = self.store.memories.add_one(memory_instance);
        self.linker.register_memory(module, name, memory_addr)
    }
    /// Returns a global instance by name from the given instance.
    ///
    /// # Parameters
    /// - `instance`: Reference to the instance (name or address) that owns the global.
    /// - `name`: Name of the global to retrieve.
    pub fn get_global(&self, instance: InstanceRef, name: String) -> Result<&GlobalInstance> {
        let instance_addr = self.linker.resolve_instance(&instance)?;
        let instance = self.store.instances.get(instance_addr)?;
        let global_addr = instance.get_global_by_name(&name)?;
        self.store.globals.get(global_addr)
    }
}
