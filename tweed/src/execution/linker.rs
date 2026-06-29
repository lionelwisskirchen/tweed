use crate::alloc::string::ToString;
use crate::binary::custom_section::FunctionName;
use crate::binary::custom_section::LinkingInformation;
use crate::binary::export_section::ExportContext;
use crate::binary::import_section::Import;
use crate::binary::import_section::Imports;
use crate::core::error::ExecutionError;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::hash::Hash;

use super::adresses::ExternAddr;
use super::adresses::{FuncAddr, GlobalAddr, InstanceAddr, MemoryAddr, TableAddr};

use super::dependency_graph::DependencyGraph;
use super::dependency_graph::InterfaceBinding;
use super::engine::InstanceRef;
use super::instances::module_instance::ModuleInstance;
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Maintains the linking state between instantiated modules.
///
/// The linker is responsible for:
/// - tracking registered module instances,
/// - exposing exported entities,
/// - resolving imports of instances,
/// - and mapping symbolic import names [`ImportName`] to store addresses.
#[derive(Default, Debug)]
pub struct Linker {
    instances: BTreeMap<String, InstanceAddr>,
    dependency_graph: DependencyGraph,
    imported_globals: BTreeMap<ImportName, GlobalAddr>,
    imported_funcs: BTreeMap<ImportName, FuncAddr>,
    imported_tables: BTreeMap<ImportName, TableAddr>,
    imported_memories: BTreeMap<ImportName, MemoryAddr>,
}

/// Each import is labeled by a two-level name space, consisting of
/// a module name and an item name for an entity within that module.
/// [WebAssembly specification: Imports](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-import).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ImportName {
    pub module: String,
    pub name: String,
}

impl Linker {
    /// Registers a function so it can be imported by other instances.
    ///
    /// The function is exposed under the given module and function name
    /// and mapped to the provided function address.
    ///
    /// # Parameters
    /// - `module`: Name of the module under which the function is exposed.
    /// - `name`: Name of the exported function.
    /// - `addr`: Address of the function in the store.
    pub fn register_function(
        &mut self,
        module: String,
        name: String,
        addr: FuncAddr,
    ) -> Result<()> {
        let key = ImportName { module, name };
        self.imported_funcs.insert(key, addr);
        Ok(())
    }
    /// Registers a global so it can be imported by other instances.
    ///
    /// The global is exposed under the given module and global name
    /// and mapped to the provided global address.
    ///
    /// # Parameters
    /// - `module`: Name of the module under which the global is exposed.
    /// - `name`: Name of the exported global.
    /// - `addr`: Address of the global in the store.
    pub fn register_global(
        &mut self,
        module: String,
        name: String,
        addr: GlobalAddr,
    ) -> Result<()> {
        let key = ImportName { module, name };
        self.imported_globals.insert(key, addr);
        Ok(())
    }
    /// Registers a table so it can be imported by other instances.
    ///
    /// The table is exposed under the given module and table name
    /// and mapped to the provided table address.
    ///
    /// # Parameters
    /// - `module`: Name of the module under which the table is exposed.
    /// - `name`: Name of the exported table.
    /// - `addr`: Address of the table in the store.
    pub fn register_table(&mut self, module: String, name: String, addr: TableAddr) -> Result<()> {
        let key = ImportName { module, name };
        self.imported_tables.insert(key, addr);
        Ok(())
    }
    /// Registers a memory so it can be imported by other instances.
    ///
    /// The memory is exposed under the given module and memory name
    /// and mapped to the provided memory address.
    ///
    /// # Parameters
    /// - `module`: Name of the module under which the memory is exposed.
    /// - `name`: Name of the exported memory.
    /// - `addr`: Address of the memory in the store.
    pub fn register_memory(
        &mut self,
        module: String,
        name: String,
        addr: MemoryAddr,
    ) -> Result<()> {
        let key = ImportName { module, name };
        self.imported_memories.insert(key, addr);
        Ok(())
    }
    pub fn resolve_instance(&self, instance: &InstanceRef) -> Result<InstanceAddr> {
        match instance {
            InstanceRef::Addr(addr) => Ok(*addr),
            InstanceRef::Name(name) => self
                .instances
                .get(name)
                .cloned()
                .ok_or(ExecutionError::UnknownInstance),
        }
    }

    /// Checks if an import name exists in the linking context.
    pub fn import_exists(&self, key: &ImportName) -> bool {
        self.imported_funcs.contains_key(key)
            | self.imported_tables.contains_key(key)
            | self.imported_globals.contains_key(key)
            | self.imported_memories.contains_key(key)
    }

    /// Resolves all imports declared by an instance into store addresses.
    ///
    /// Each import is looked up by its `(module, name)` pair and translated
    /// into the corresponding [`ExternAddr`] stored in the runtime.
    ///
    /// # Parameters
    /// - `imports`: Collection of imports declared by the instance.
    ///
    /// # Returns
    /// A vector containing the resolved external addresses in declaration order.
    ///
    /// # Errors
    /// Returns an error if any import cannot be resolved:
    /// - [`ExecutionError::MissingFunctionImport`] for missing function imports.
    /// - [`ExecutionError::MissingGlobalImport`] for missing global imports
    /// - [`ExecutionError::MissingMemoryimport`] for missing memory imports.
    /// - [`ExecutionError::MissingTableimport`] for missing table imports.
    pub fn resolve(&mut self, imports: &Imports) -> Result<Vec<ExternAddr>> {
        let mut resolved_import = vec![];

        for import in imports.imports() {
            match import {
                Import::Function(imp) => {
                    let key = ImportName {
                        module: imp.module.clone(),
                        name: imp.name.clone(),
                    };
                    if let Ok(interface) = imp.module.clone().try_into() {
                        let store_address = self
                            .dependency_graph
                            .find(&interface, FunctionName(imp.name.clone()))
                            .or(self
                                .imported_funcs
                                .get(&key)
                                .ok_or(ExecutionError::MissingFunctionImport));

                        // 1. Check if key exists in general
                        if store_address.is_err() {
                            if !self.import_exists(&key) {
                                return Err(ExecutionError::MissingFunctionImport);
                            } else {
                                return Err(ExecutionError::IncompatibleImportType);
                            }
                        };
                        resolved_import.push(ExternAddr::Func(*store_address.unwrap()));
                    } else {
                        // 1. Check if key exists in general
                        if !self.import_exists(&key) {
                            return Err(ExecutionError::MissingFunctionImport);
                        }
                        // 2. Check if key exists with the correct type
                        let store_address = self
                            .imported_funcs
                            .get(&key)
                            .ok_or(ExecutionError::IncompatibleImportType)?;
                        resolved_import.push(ExternAddr::Func(*store_address));
                    }
                }
                Import::Global(imp) => {
                    let key = ImportName {
                        module: imp.module.clone(),
                        name: imp.name.clone(),
                    };
                    // 1. Check if key exists in general
                    if !self.import_exists(&key) {
                        return Err(ExecutionError::MissingGlobalImport);
                    };
                    // 2. Check if key exists with the correct type
                    let store_address = self
                        .imported_globals
                        .get(&key)
                        .ok_or(ExecutionError::IncompatibleImportType)?;

                    resolved_import.push(ExternAddr::Global(*store_address));
                }
                Import::Memory(imp) => {
                    let key = ImportName {
                        module: imp.module.clone(),
                        name: imp.name.clone(),
                    };
                    // 1. Check if key exists in general
                    if !self.import_exists(&key) {
                        return Err(ExecutionError::MissingMemoryImport);
                    };
                    // 2. Check if key exists with the correct type
                    let store_address = self
                        .imported_memories
                        .get(&key)
                        .ok_or(ExecutionError::IncompatibleImportType)?;
                    resolved_import.push(ExternAddr::Memory(*store_address));
                }
                Import::Table(imp) => {
                    let key = ImportName {
                        module: imp.module.clone(),
                        name: imp.name.clone(),
                    };
                    // 1. Check if key exists in general
                    if !self.import_exists(&key) {
                        return Err(ExecutionError::MissingTableImport);
                    };
                    // 2. Check if key exists with the correct type
                    let store_address = self
                        .imported_tables
                        .get(&key)
                        .ok_or(ExecutionError::IncompatibleImportType)?;
                    resolved_import.push(ExternAddr::Table(*store_address));
                }
            }
        }
        Ok(resolved_import)
    }
    /// Registers an instance under the given name.
    ///
    /// Registered instances can later be looked up and referenced
    /// by their assigned identifier.
    ///
    /// # Parameters
    /// - `instance_name`: Name under which the instance is registered.
    /// - `instance`: Address of the instance in the store.
    ///
    /// # Returns
    /// The registered instance name.
    pub fn register_instance(
        &mut self,
        instance_name: &str,
        instance: InstanceAddr,
    ) -> Result<String> {
        self.instances.insert(instance_name.to_string(), instance);
        Ok(instance_name.to_string())
    }

    pub fn register_interface(&mut self, linking_information: LinkingInformation) -> Result<()> {
        self.dependency_graph.add_interface(linking_information)
    }
    pub fn register_interface_bindings(&mut self, bindings: Vec<InterfaceBinding>) -> Result<()> {
        for binding in bindings {
            self.dependency_graph.add_binding(binding)?;
        }
        Ok(())
    }
    /// Publishes all exports of an instance so they can be imported
    /// by other instances.
    ///
    /// Each exported entity is registered under the instance name
    /// and its export name, making it available for later import
    /// resolution.
    ///
    /// # Parameters
    /// - `name`: Name for the instance.
    /// - `instance`: The instantiated module containing store addresses.
    pub fn register_exports(&mut self, name: String, instance: &ModuleInstance) -> Result<()> {
        let has_interface_definitions = !instance.implementations.is_empty();
        if has_interface_definitions {
            self.register_interface_bindings(instance.implementations.clone())?;
        }

        let exports = &instance.exports;
        for (ent_name, entity) in exports {
            let import_name = ImportName {
                module: name.clone(),
                name: ent_name.to_string(),
            };
            match entity {
                ExportContext::FunctionExport(id) => {
                    if !has_interface_definitions {
                        let store_addr = instance.funcaddr.get(*id as usize).unwrap();
                        self.imported_funcs.insert(import_name, *store_addr);
                    }
                }
                ExportContext::GlobalExport(id) => {
                    let store_addr = instance.globaladdr.get(*id as usize).unwrap();
                    self.imported_globals.insert(import_name, *store_addr);
                }
                ExportContext::MemoryExport(id) => {
                    let store_addr = instance.memaddr.get(*id as usize).unwrap();
                    self.imported_memories.insert(import_name, *store_addr);
                }
                ExportContext::TableExport(id) => {
                    let store_addr = instance.tableaddr.get(*id as usize).unwrap();
                    self.imported_tables.insert(import_name, *store_addr);
                }
            }
        }

        Ok(())
    }
}
