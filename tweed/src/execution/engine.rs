extern crate alloc;

use super::adresses::InstanceAddr;
use super::executor::ExecutionState;
use super::instances::function_instance::HostFunc;
use super::instances::{FuncInstance, GlobalInstance, MemoryInstance, TableInstance};
use super::store::{HostEnv, Store};

use alloc::rc::Rc;
use core::cell::RefCell;

use super::linker::Linker;
use crate::binary::custom_section::InterfaceId;
use crate::binary::global_section::GlobalType;
use crate::binary::memory_section::Limits;
use crate::binary::table_section::Table;
use crate::binary::type_section::FuncType;
use crate::core::error::ExecutionError;
use crate::core::value_types::ValType;

use crate::execution::adresses::FuncAddr;
use crate::execution::executor::Executor;
use crate::execution::instantiator::ModuleInstantiator;
use crate::validation::module::Module;

use crate::alloc::string::ToString;
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
    pub fn new(max_steps: usize) -> Self {
        Self {
            executor: Executor::new(max_steps),
            ..Default::default()
        }
    }

    /// Instantiates a module, resolves its imports, and invokes the start function if present.
    /// The imports are automatically wired into other modules via the linker.
    /// When the Module contains dependency information it automatically registers itself,
    /// under the given name.
    /// When a Module contains a start section it automatically invokes the
    /// referenced function.
    ///
    /// # Returns
    /// The store address of the instantiated module instance.
    pub fn instantiate(&mut self, module: Module) -> Result<InstanceRef> {
        let mut instantiator = ModuleInstantiator {};
        let imports = &module.importsec;

        // TODO: Modules without linking infos aren't allowed to be linked with interfaces
        let external = self.linker.resolve(imports)?;

        // Register new interface definitions to the linker
        if let Some(ref linking_infos) = module.linkingsec {
            self.linker.register_interface(linking_infos.clone())?;
        }

        let linking_infos = &module.linkingsec.clone();
        let store_addr = instantiator.module_instantiate(&mut self.store, module, &external)?;

        let instance = self.store.instances.get(store_addr)?;
        let start = instance.start;

        // When dependency infos are available automatically register the instance
        // under the name stored in the linking infos.
        if let Some(linking_info) = linking_infos {
            self.register(&linking_info.package_name, InstanceRef::Addr(store_addr))?;
        }

        if let Some(func_addr) = start {
            info!("Invoke start function with store address {func_addr}");
            self.invoke_by_addr(func_addr, &[])?;
        }
        Ok(InstanceRef::Addr(store_addr))
    }

    fn invoke_by_addr(&mut self, func_addr: FuncAddr, args: &[Value]) -> Result<ExecutionState> {
        if self.executor.is_running {
            return Err(ExecutionError::FunctionIsRunning);
        };
        match self.executor.invoke(func_addr, args, &mut self.store)? {
            ExecutionState::Finished(values) => Ok(ExecutionState::Finished(values)),
            _ => self.executor.step(&mut self.store),
        }
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

    /// Tries to bind all the interface with a concrete implementation based on the
    /// Information specified in the instance.
    pub fn bind_interfaces(&mut self, name: &InstanceRef) -> Result<Vec<InterfaceId>> {
        let target_instance_addr = self.linker.resolve_instance(name)?;
        let instance = self.store.instances.get(target_instance_addr)?;
        if !&instance.implementations.is_empty() {
            return self
                .linker
                .register_interface_bindings(instance.implementations.clone());
        }
        Err(ExecutionError::UnknownInstance)
    }

    /// Publishes all exports of an instance, making them available for import by other instances.
    ///
    /// Export registration is performed via the linker, which operates on named instances only.
    /// Therefore, only instances identified by a module name can expose their exports.
    ///
    /// # Errors
    /// Returns [`ExecutionError::UnknownInstance`] if the provided
    /// `name` is an address.
    pub fn register_exports(&mut self, name: &InstanceRef) -> Result<()> {
        let target_instance_addr = self.linker.resolve_instance(name)?;
        let InstanceRef::Name(instance_name) = name else {
            return Err(ExecutionError::UnknownInstance);
        };
        let instance = self.store.instances.get(target_instance_addr)?;
        self.linker
            .register_exports(instance_name.to_string(), instance)
    }

    /// Invokes the implementation of an interface function.
    ///
    /// # Parameters
    /// - `interface`: The identifier of an wit interface.
    /// - `function_name`: Name of the function to invoke within the instance.
    /// - `params`: Arguments passed to the function.
    ///
    /// # Returns
    /// The result values returned by the executed function.
    pub fn invoke_interface(
        &mut self,
        interface: InterfaceId,
        function_name: String,
        params: Vec<Value>,
    ) -> Result<ExecutionState> {
        let instance = &InstanceRef::Addr(
            self.linker
                .get_binding(&interface)
                .ok_or(ExecutionError::InterfaceNotFound)?
                .instance_addr,
        );
        self.invoke(instance, function_name, params)
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
        instance: &InstanceRef,
        function_name: String,
        params: Vec<Value>,
    ) -> Result<ExecutionState> {
        let instance_addr = self.linker.resolve_instance(instance)?;
        let func_addr = self
            .store
            .instances
            .get(instance_addr)?
            .get_func_by_name(&function_name)?;
        self.invoke_by_addr(func_addr, &params)
    }

    pub fn step(&mut self) -> Result<ExecutionState> {
        let result = self.executor.step(&mut self.store);
        for upgrade_info in &self.executor.applied_upgrades {
            // TODO how to handle a linker upgrade failure?
            self.linker.apply_upgrade(upgrade_info)?;
        }
        self.executor.applied_upgrades.clear();
        result
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
    ) -> Result<FuncAddr> {
        let func_instance = HostFunc {
            signature: ty,
            func: Rc::new(RefCell::new(func)),
        };
        let func_addr = self
            .store
            .functions
            .add_one(FuncInstance::External(func_instance));
        self.linker.register_function(module, name, func_addr)?;
        Ok(func_addr)
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

    /// Returns a memory instance by index from the given instance.
    ///
    /// # Parameters
    /// - `instance`: Reference to the instance (name or address) that owns the memory.
    /// - `memory_idx`: Index of the memory to retrieve.
    pub fn get_memory(&self, instance: &InstanceRef, memory_idx: usize) -> Result<&MemoryInstance> {
        if memory_idx > 0 {
            return Err(ExecutionError::MultipleMemories);
        }

        let instance_addr = self.linker.resolve_instance(instance)?;
        let instance = self.store.instances.get(instance_addr)?;

        let mem_addr = instance.memaddr[memory_idx];

        self.store.memories.get(mem_addr)
    }

    /// Upgrades an interface for the selection.
    /// It ignores if the binding is currently executed.
    pub fn force_upgrade_interface(
        &mut self,
        interface: &InterfaceId,
        selection: Vec<InterfaceId>,
    ) -> Result<()> {
        let upgrade_info = self.linker.prepare_upgrade(interface, selection)?;
        for dependent in &upgrade_info.selection {
            if let Some(binding) = self.linker.get_binding(dependent) {
                let dependent_instance = self.store.instances.get_mut(binding.instance_addr)?;
                dependent_instance.prepare_upgrade(&upgrade_info)?;
                for upgrade in dependent_instance.apply_upgrade() {
                    self.linker.apply_upgrade(&upgrade)?;
                }
            }
        }
        Ok(())
    }
    /// Upgrades an interface for the selection.
    pub fn upgrade_interface(
        &mut self,
        interface: &InterfaceId,
        selection: Vec<InterfaceId>,
    ) -> Result<()> {
        let upgrade_info = self.linker.prepare_upgrade(interface, selection)?;
        let active_instances: Vec<InstanceAddr> = self
            .executor
            .call_stack
            .list()
            .iter()
            .map(|inst| inst.instance_addr)
            .collect();
        for dependent in &upgrade_info.selection {
            if let Some(binding) = self.linker.get_binding(dependent) {
                let dependent_instance = self.store.instances.get_mut(binding.instance_addr)?;
                dependent_instance.prepare_upgrade(&upgrade_info)?;
                if !active_instances.contains(&binding.instance_addr) {
                    for upgrade in dependent_instance.apply_upgrade() {
                        self.linker.apply_upgrade(&upgrade)?;
                    }
                }
            }
        }

        Ok(())
    }
    pub fn collect_garbage(&mut self) -> Result<()> {
        self.linker.collect_garbage()
    }

    pub fn to_dot(&self) -> String {
        self.linker.to_dot()
    }
}
