pub mod function_registry_entry;
pub mod memory_registry_entry;
pub mod table_registry_entry;

use crate::alloc::string::ToString;
use crate::binary::data_section::SegmentMode;
use crate::binary::element_section::{ElementItems, Mode};
use crate::binary::export_section::ExportContext;
use crate::binary::global_section::GlobalType;
use crate::binary::memory_section::WASM_PAGE_SIZE;
use crate::binary::table_section::Table;
use crate::binary::type_section::FuncType;
use crate::core::error::ExecutionError;
use crate::core::signature_lookup::lookup_signature;
use crate::core::value_types::ValType;
use crate::core::values::Value;
use crate::execution::linker::function_registry_entry::FunctionRegistryEntry;
use crate::execution::linker::memory_registry_entry::MemoryRegistryEntry;
use crate::execution::linker::table_registry_entry::TableRegistryEntry;
use crate::validation::module::Module;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;
use core::hash::Hash;
use std::collections::HashMap;

use alloc::vec::Vec;
use core::iter::zip;

use super::constant_expression_executor::execute_const_expr;
use super::executor::Executor;
use super::instance::Instance;
use super::store::{
    DataInstance, ElementInstance, FuncInstance, GlobalInstance, HostEnv, HostFunc, Internal,
    MemoryInstance, Store, TableInstance,
};
pub const DEFAULT_MODULE: &str = "__module_instance_default__";

pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Default)]
pub struct Linker {
    imported_globals: BTreeMap<ImportName, ImportSource<GlobalInstance>>,
    imported_funcs: BTreeMap<ImportName, ImportSource<HostFunc>>,
    imported_tables: BTreeMap<ImportName, ImportSource<TableInstance>>,
    imported_memories: BTreeMap<ImportName, ImportSource<MemoryInstance>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ImportName {
    pub module: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ImportSource<T> {
    Extern(usize),
    Host(T),
}

impl Linker {
    pub fn instance(&mut self, instance: &Instance) -> Result<()> {
        for (name, context) in &instance.exports {
            let key = ImportName {
                module: instance.name.clone(),
                name: name.to_string(),
            };
            match context {
                ExportContext::GlobalExport(idx) => {
                    self.imported_globals.insert(
                        key,
                        ImportSource::Extern(*instance.globaladdr.get(*idx as usize).unwrap()),
                    );
                }
                ExportContext::TableExport(idx) => {
                    self.imported_tables.insert(
                        key,
                        ImportSource::Extern(*instance.tableaddr.get(*idx as usize).unwrap()),
                    );
                }
                ExportContext::FunctionExport(idx) => {
                    self.imported_funcs.insert(
                        key,
                        ImportSource::Extern(*instance.funcaddr.get(*idx as usize).unwrap()),
                    );
                }
                ExportContext::MemoryExport(idx) => {
                    self.imported_memories.insert(
                        key,
                        ImportSource::Extern(*instance.memaddr.get(*idx as usize).unwrap()),
                    );
                }
            }
        }
        Ok(())
    }

    /// Register a new imported function
    ///
    /// This function takes a function and inserts it into the BTreeMap of the linker.
    /// It uses a combination of module and name as key
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the function is imported.
    /// * `name`: the name of the function.
    /// * `value`: the signature of the function.
    /// * `func`: The actual logic to execute.
    pub fn add_function_import(
        &mut self,
        module: String,
        name: String,
        ty: FuncType,
        func: impl FnMut(&mut HostEnv, Vec<Value>) -> Result<Vec<Value>> + 'static,
    ) -> Result<()> {
        let key = ImportName { module, name };
        let func = HostFunc {
            signature: ty,
            func: Rc::new(RefCell::new(func)),
        };
        self.imported_funcs.insert(key, ImportSource::Host(func));
        Ok(())
    }

    /// Register a new imported global
    ///
    /// This function takes a global and inserts it into the BTreeMap of the linker.
    /// It uses a combination of module and name as key
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
        let key = ImportName {
            module: module.clone(),
            name: name.clone(),
        };
        let global_type = GlobalType {
            value_type: ValType::from(value),
            is_mutable,
        };
        let instance = GlobalInstance { global_type, value };
        self.imported_globals
            .insert(key, ImportSource::Host(instance));
        Ok(())
    }

    /// Register a new imported table
    ///
    /// This function takes a table and inserts it into the BTreeMap of the linker.
    /// It uses a combination of module and name as key
    ///
    /// # Arguments
    /// * `module`: the identifier of the module from which the table is imported.
    /// * `name`: the name of the table.
    /// * `table`: a reference to the table.
    pub fn add_table_import(&mut self, module: String, name: String, table: Table) -> Result<()> {
        let key = ImportName { module, name };
        let table_instance = TableInstance::new(table.clone());

        self.imported_tables
            .insert(key, ImportSource::Host(table_instance));

        Ok(())
    }

    pub fn instantiate(
        &mut self,
        module: &mut Module,
        store: &mut Store,
        instance_name: Option<&str>,
    ) -> Result<String> {
        let name = match instance_name {
            Some(n) if !store.instance_exists(n) => n.to_string(),
            Some(_) => return Err(ExecutionError::InstanceNameTaken),
            None => format!("{}#{}", DEFAULT_MODULE, store.get_instance_count()),
        };

        /*---------------------------------
         * FUNCTIONS
         *---------------------------------*/
        let imported_func_idxs = self.instantiate_function_imports(module, store)?;
        let local_functions =
            self.instantiate_functions(module, &name, imported_func_idxs.len())?;

        let local_func_idxs = store.add_functions(local_functions);
        let all_function_idxs = [imported_func_idxs, local_func_idxs].concat();

        let mut function_registry = HashMap::new();

        for (index_in_module, index_in_store) in all_function_idxs.clone().into_iter().enumerate() {
            let instance = store.get_function_instance_by_idx(index_in_store)?;
            let registry_entry = FunctionRegistryEntry {
                instance,
                index_in_store,
            };

            function_registry.insert(index_in_module, registry_entry);
        }

        /*---------------------------------
         * Globals
         *---------------------------------*/
        let imported_global_idxs = self.instantiate_global_imports(module, store)?;

        let mut imported_globals = vec![];
        for store_addr in &imported_global_idxs {
            imported_globals.push(store.get_global_instance_by_idx(*store_addr)?);
        }

        let local_globals =
            self.instantiate_globals(module, &imported_globals, &function_registry)?;
        let local_global_idxs = store.add_globals(local_globals);
        let all_global_idxs = [imported_global_idxs, local_global_idxs].concat();

        /*---------------------------------
         * MEMORY
         *---------------------------------*/
        let imported_memory_idxs = self.instantiate_memory_imports(store, module)?;
        let local_memories = self.instantiate_memories(module)?;

        let local_memory_idxs = store.add_memories(local_memories);
        let all_memory_idxs = [imported_memory_idxs, local_memory_idxs].concat();

        // this vec stores the memory instance with its respective index on the store
        let mut memory_registry: Vec<MemoryRegistryEntry> = Vec::new();

        for (module_index, store_addr) in all_memory_idxs.clone().into_iter().enumerate() {
            let memory_instance = store.get_memory_instance_by_idx(store_addr)?;
            let registry_entry = MemoryRegistryEntry {
                instance: memory_instance,
                index_in_store: store_addr,
                index_in_module: (module_index as u32),
            };
            memory_registry.push(registry_entry);
        }

        /*---------------------------------
         * TABLES
         *---------------------------------*/
        let imported_table_idxs = self.instantiate_table_imports(store, module)?;
        let local_tables = self.instantiate_tables(module)?;
        let local_table_idxs = store.add_tables(local_tables);
        let all_tables_idxs = [imported_table_idxs, local_table_idxs].concat();

        // this vec stores the table instance with its respective index on the store
        let mut table_registry: Vec<TableRegistryEntry> = Vec::new();

        for (module_index, store_addr) in all_tables_idxs.clone().into_iter().enumerate() {
            let table_instance = store.get_table_instance_by_idx(store_addr)?;
            let registry_entry = TableRegistryEntry {
                instance: table_instance,
                index_in_store: store_addr,
                index_in_module: (module_index as u32),
            };
            table_registry.push(registry_entry);
        }

        /*---------------------------------
         * DATA
         *---------------------------------*/
        let (data, caught_error_data) = self.instantiate_datas(
            module,
            &mut memory_registry,
            &imported_globals,
            &function_registry,
        )?;

        let dataddr = store.add_datas(data);

        for entry in memory_registry {
            store.data.memories[entry.index_in_store] = entry.instance;
        }

        /*---------------------------------
         * ELEMENTS
         *---------------------------------*/
        let (elements, caught_error) = self.instantiate_elements(
            module,
            &imported_globals,
            &mut table_registry,
            &function_registry,
            &all_function_idxs,
        )?;

        let elemaddr = store.add_elements(elements);

        for entry in table_registry {
            store.data.tables[entry.index_in_store] = entry.instance;
        }

        let exports = self.instantiate_export(module)?;

        trace!("Module: {:?}", module);

        let module_instance = Instance {
            name: name.clone(),
            types: module.typesec.clone(),
            funcaddr: all_function_idxs,
            tableaddr: all_tables_idxs,
            memaddr: all_memory_idxs,
            globaladdr: all_global_idxs,
            elemaddr,
            dataddr,
            exports,
        };

        store.add_instance(module_instance, &name, &module.reader)?;

        // check if we had an error during element or data instantiation and return it if this is the case
        if let Some(err) = caught_error.or(caught_error_data) {
            return Err(err);
        }

        if let Some(id) = module.startsec {
            info!("Invoke start function with id {id}");
            let mut exec = Executor::new(store);
            exec.invoke_start(&name.to_string(), id)?;
        }
        Ok(name.to_string())
    }

    /// Instantiate the imported tables
    ///
    /// This function iterates over all imported tables (modules.importsec.tables). If the table
    /// is already in the store, the reftype is confirmed and its address is pushed to an array
    /// which is then returned.
    ///
    /// #Arguments
    /// * `store`: a mutable reference to the store instance
    /// * `module`: a reference to the current module
    ///
    /// # Returns
    /// An array containing the indices of the imported tables **on the store**
    ///
    /// # Errors
    /// * [`ExecutionError::UnknownImport`] if the table is not known to the linker
    /// * [`ExecutionError::IncompatibleImportType`] if the reftype of the table found on the store
    ///   does not match the reftype of the table from the import section
    fn instantiate_table_imports(
        &mut self,
        store: &mut Store,
        module: &Module,
    ) -> Result<Vec<usize>> {
        let mut tableaddr = vec![];
        for table_import in module.importsec.tables.iter() {
            let key = ImportName {
                module: table_import.module.clone(),
                name: table_import.name.clone(),
            };
            let import_entity = self
                .imported_tables
                .get(&key)
                .ok_or(ExecutionError::UnknownImport)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    let inst = store.get_table_instance_by_idx(*addr)?;

                    inst.ensure_ref_type_is_correct(table_import)?;

                    tableaddr.push(*addr);
                }
                ImportSource::Host(inst) => {
                    inst.ensure_ref_type_is_correct(table_import)?;

                    let store_indices = store.add_table(inst.clone());
                    tableaddr.push(store_indices);
                }
            };
        }
        trace!("Table imports {:?}", tableaddr);

        Ok(tableaddr)
    }

    /// Instantiate the imported memories
    ///
    /// This function iterates over all imported memories (module.importsec.memories). If the memory
    /// is already in the store, its address is collected.
    ///
    /// # Arguments
    /// * `store`: a mutable reference to the store instance
    /// * `module`: a reference to the current module
    ///
    /// # Returns
    /// An array containing the indices of the imported memories **on the store**
    ///
    /// # Errors
    /// * [`ExecutionError::UnknownImport`] if the memory is not known to the linker
    fn instantiate_memory_imports(
        &mut self,
        store: &mut Store,
        module: &Module,
    ) -> Result<Vec<usize>> {
        let mut memaddr = vec![];
        for memory_import in module.importsec.memories.iter() {
            let key = ImportName {
                module: memory_import.module.clone(),
                name: memory_import.name.clone(),
            };
            let import_entity = self
                .imported_memories
                .get(&key)
                .ok_or(ExecutionError::UnknownImport)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    memaddr.push(*addr);
                }
                ImportSource::Host(inst) => {
                    memaddr.push(store.add_memory(inst.clone()));
                }
            }
        }

        Ok(memaddr)
    }

    /// Instantiate the imported functions
    ///
    /// This function iterates over all imported functions (module.importsec.functions). It
    /// collects the signature of the function and pushed its store addr into the funcaddr array.
    ///
    /// # Arguments
    /// * `store`: a mutable reference to the store instance
    /// * `module`: a reference to the current module
    ///
    /// # Returns
    /// An array containing the indices of the imported functions **on the store**
    ///
    /// # Errors
    /// * [`ExecutionError::MissingFunctionImport`] if the function is not known to the linker
    /// * [`ExecutionError::InvalidSignatureIndex`] if the signature is not known to the linker
    /// * [`ExecutionError::IncompatibleImportType`] if the imported signature does not match the
    ///   already registered signature on the store
    fn instantiate_function_imports(
        &mut self,
        module: &mut Module,
        store: &mut Store,
    ) -> Result<Vec<usize>> {
        let mut funcaddr = Vec::new();
        for func_import in module.importsec.functions.iter() {
            let key = ImportName {
                module: func_import.module.clone(),
                name: func_import.name.clone(),
            };
            let import_entity = self
                .imported_funcs
                .get(&key)
                .ok_or(ExecutionError::MissingFunctionImport)?;

            let sig = module
                .typesec
                .get(func_import.context.sig_index as usize)
                .ok_or(ExecutionError::InvalidSignatureIndex)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    let target_function = store.get_function_instance_by_idx(*addr)?;

                    if target_function.get_signature() != sig {
                        return Err(ExecutionError::IncompatibleImportType);
                    };
                    funcaddr.push(*addr);
                }
                ImportSource::Host(inst) => {
                    funcaddr.push(store.add_function(FuncInstance::External(inst.clone())));
                }
            };
        }
        Ok(funcaddr)
    }

    /// Instantiate the imported globals
    ///
    /// This function iterates over all imported globals (module.importsec.globals). If the memory
    /// is already in the store, its address is collected.
    ///
    /// # Arguments
    /// * `store`: a mutable reference to the store instance
    /// * `module`: a reference to the current module
    ///
    /// # Returns
    /// An array containing the indices of the imported globals **on the store**
    ///
    /// # Errors
    /// * [`ExecutionError::MissingGlobalImport`] if the global is not known to the linker
    /// * [`ExecutionError::IncompatibleImportType`] if the imported global does not match the
    ///   already registered global on the store
    fn instantiate_global_imports(
        &mut self,
        module: &mut Module,
        store: &mut Store,
    ) -> Result<Vec<usize>> {
        let mut globaladdr = vec![];
        for import in module.importsec.globals.iter() {
            let key = ImportName {
                module: import.module.clone(),
                name: import.name.clone(),
            };
            let import_entity = self
                .imported_globals
                .get(&key)
                .ok_or(ExecutionError::MissingGlobalImport)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    let target_global = store
                        .data
                        .globals
                        .get(*addr)
                        .ok_or(ExecutionError::MissingGlobalImport)?;
                    if target_global.global_type != import.context {
                        return Err(ExecutionError::IncompatibleImportType);
                    }
                    globaladdr.push(*addr);
                }
                ImportSource::Host(inst) => {
                    globaladdr.push(store.add_global(inst.clone()));
                }
            };
        }
        trace!("Global imports {:?}", globaladdr);

        Ok(globaladdr)
    }

    fn instantiate_globals(
        &mut self,
        module: &mut Module,
        imported_globals: &[GlobalInstance],
        function_registry: &HashMap<usize, FunctionRegistryEntry>,
    ) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();

        for global in module.globalsec.iter() {
            let expr = {
                module.reader.seek(global.start + 1);
                let return_value = execute_const_expr(
                    &mut module.reader,
                    imported_globals,
                    &[global.global_type.value_type],
                    function_registry,
                )?;
                *return_value.first().ok_or(ExecutionError::TypeMismatch)?
            };
            globals.push(GlobalInstance {
                global_type: global.global_type.clone(),
                value: expr,
            });
        }
        Ok(globals)
    }
    fn instantiate_export(
        &mut self,
        module: &mut Module,
    ) -> Result<BTreeMap<String, ExportContext>> {
        let mut exports = BTreeMap::new();
        for export in &module.exportsec {
            exports.insert(export.name.clone(), export.context.clone());
        }
        Ok(exports)
    }
    fn instantiate_functions(
        &mut self,
        module: &mut Module,
        name: &str,
        offset: usize,
    ) -> Result<Vec<FuncInstance>> {
        let function_to_sidetable = zip(module.codesec.iter(), module.sidetables.iter());
        let mut functions = Vec::new();
        for (index, (function, side_table)) in function_to_sidetable.enumerate() {
            let locals: Vec<Value> = function
                .locals
                .iter()
                .map(|val_type| (*val_type).into())
                .collect::<Vec<Value>>();

            let signature_index = module
                .funcsec
                .get(index + offset)
                .ok_or(ExecutionError::InvalidFunctionIndex)?
                .sig_index;
            let signature = lookup_signature(signature_index, &module.typesec);

            functions.push(FuncInstance::Internal(Internal {
                instance: name.to_string(),
                signature: signature.clone(),
                locals,
                start: function.start,
                end: function.end,
                side_table: side_table.clone(),
            }));
        }
        Ok(functions)
    }

    fn instantiate_memories(&mut self, module: &mut Module) -> Result<Vec<MemoryInstance>> {
        let mut memories = Vec::new();

        for memory in module.memorysec.iter() {
            let data = vec![0; (memory.limits.min * WASM_PAGE_SIZE) as usize];
            memories.push(MemoryInstance::new(data, memory.limits.clone()));
        }
        Ok(memories)
    }

    /// Instantiate all tables that are stored in the module
    ///
    /// This function simply iterates over the tables in the module, creates a new [`TableInstance`]
    /// and returns this instances
    ///
    /// # Arguments
    /// * `module`: a reference the module in which the tables are stored in.
    ///
    /// # Returns
    /// A vector containing the created [`TableInstance`]
    fn instantiate_tables(&mut self, module: &Module) -> Result<Vec<TableInstance>> {
        let mut tables = Vec::new();
        for table in module.tablesec.iter() {
            tables.push(TableInstance::new(table.clone()));
        }
        Ok(tables)
    }

    /// Instantiate the elements of the module
    ///
    /// For every element two steps are done. First, a new element instance is created.
    /// This element is then inserted into the table.
    ///
    /// # Arguments
    /// * `module`: a mutable reference to the current module
    /// * `imported_globals`: a reference to the imported globals (used for const_expr execution)
    /// * `tables`: a hashmap of all table instances (v) with their idx on the store (k)
    /// * `functions`: a reference to all fuctions of the module
    /// * `func_store_idxs`: a reference to a vector holding all indices of functions on the store
    ///
    /// # Returns
    /// * A vector with all Element Instances that can now be put on the store.
    /// * A variable storing a caught error when inserting the element does not succeed. This is
    ///   needed in order to persist active elements to be stored before an out-of-bounds access
    ///   occurs
    ///
    /// # Errors
    /// [`ExecutionError::FunctionNotFound`] - the functions the element references can't be found
    /// [`ExecutionError::TypeMismatch`] - the result of the const expression can't be resolved
    ///
    /// # Note
    /// This function does **NOT** add the elements to the store.
    /// Please refer to [`store.add_elements()`]
    /// To update all element instances (through imports), the tables on the store must be updated.
    fn instantiate_elements(
        &mut self,
        module: &mut Module,
        imported_globals: &[GlobalInstance],
        tables: &mut [TableRegistryEntry],
        function_registry: &HashMap<usize, FunctionRegistryEntry>,
        func_store_idxs: &[usize],
    ) -> Result<(Vec<ElementInstance>, Option<ExecutionError>)> {
        let mut elements = Vec::new();
        let mut caught_error = None;

        for element in module.elementsec.iter() {
            /*
             * Step 1: Create an Element Instance
             * Here the element is either created by using the respective reference to a function on
             * the store or by resolving a const expression
             */
            let instance = match &element.items {
                ElementItems::RefFuncs(refs) => {
                    let mut resolved_refs: Vec<Value> = Vec::new();
                    for idx in refs {
                        let addr_in_store = func_store_idxs
                            .get(*idx as usize)
                            .ok_or(ExecutionError::FunctionNotFound)?;

                        resolved_refs.push(Value::FuncRef(*addr_in_store));
                    }
                    ElementInstance::new(&element.ref_type, resolved_refs)
                }
                ElementItems::Exprs(ty, spans) => {
                    let mut resolved_refs = vec![];
                    for span in spans {
                        module.reader.seek(span.start + 1);
                        let local_addr_value = *execute_const_expr(
                            &mut module.reader,
                            imported_globals,
                            &[*ty],
                            function_registry,
                        )?
                        .first()
                        .ok_or(ExecutionError::TypeMismatch)?;

                        match local_addr_value {
                            Value::FuncRef(local_addr) => {
                                let function_registry_entry = function_registry
                                    .get(&local_addr)
                                    .ok_or(ExecutionError::FunctionNotFound)?;

                                resolved_refs
                                    .push(Value::FuncRef(function_registry_entry.index_in_store));
                            }
                            Value::RefNull(v) => {
                                resolved_refs.push(Value::RefNull(v));
                            }
                            _ => {}
                        };
                    }
                    ElementInstance::new(&element.ref_type, resolved_refs)
                }
            };
            /*
             * Step 2: Store the element in the table
             * Here the element is either created by using the respective reference to a function on
             * the store or by resolving a const expression
             */
            match element.mode {
                Mode::Active {
                    table,
                    offset_start,
                    offset_end: _,
                } => {
                    // calculate the offset (position in table) of the element instance
                    module.reader.seek(offset_start + 1);
                    let result = *execute_const_expr(
                        &mut module.reader,
                        imported_globals,
                        &[ValType::I32],
                        function_registry,
                    )?
                    .first()
                    .ok_or(ExecutionError::TypeMismatch)?;
                    let Value::I32(offset) = result else {
                        return Err(ExecutionError::TypeMismatch);
                    };

                    let table_entry = tables
                        .iter_mut()
                        .find(|entry| entry.index_in_module == table);
                    match table_entry {
                        Some(entry) => {
                            let result =
                                entry.instance.insert(offset as usize, &instance.references);
                            match result {
                                Ok(_) => elements.push(instance),
                                Err(err) => {
                                    caught_error = Some(err);
                                    break;
                                }
                            }
                        }
                        None => return Err(ExecutionError::TypeMismatch),
                    }
                }
                Mode::Passive => elements.push(instance),
                Mode::Declarative => {
                    // do nothing
                }
            }
        }
        Ok((elements, caught_error))
    }

    /// Instantiate the data of the module
    ///
    /// For every data the address in the memory is read. The data is then written to this place in
    /// the memory.
    ///
    /// # Arguments
    /// * `module`: a mutable reference to the current module
    /// * `memory_registry`: a reference to the currently available memory instances with their store index
    /// * `imported_globals`: a reference to the imported globals (used for const_expr execution)
    /// * `functions`: a reference to all fuctions of the module
    ///
    /// # Returns
    /// * A vector with all Data Instances that can now be put on the store.
    /// * A variable storing a caught error when inserting the data does not succeed. This is
    ///   needed in order to persist active data to be stored before an out-of-bounds access occurs
    ///
    /// # Errors
    /// [`ExecutionError::InvalidMemoryIndex`] - the index to the memory is invalid
    /// [`ExecutionError::TypeMismatch`] - the const expressions can't be resolved
    ///
    /// # Note
    /// This function does **NOT** add the data to the store.
    /// Please refer to [`store.add_data()`]
    /// To update all element instances (trough imports), the tables on the store must be updated.
    fn instantiate_datas(
        &mut self,
        module: &mut Module,
        memory_registry: &mut [MemoryRegistryEntry],
        imported_globals: &[GlobalInstance],
        function_registry: &HashMap<usize, FunctionRegistryEntry>,
    ) -> Result<(Vec<DataInstance>, Option<ExecutionError>)> {
        let mut data = Vec::new();
        let mut caught_error = None;

        for data_segment in module.datasec.iter() {
            if let SegmentMode::Active {
                memory,
                offset_start,
                offset_end: _,
            } = data_segment.mode
            {
                // find the address of the data in the memory
                let memory_address = {
                    module.reader.seek(offset_start + 1);
                    let return_values = execute_const_expr(
                        &mut module.reader,
                        imported_globals,
                        &[ValType::I32],
                        function_registry,
                    )?;
                    let Value::I32(value) =
                        return_values.first().ok_or(ExecutionError::TypeMismatch)?
                    else {
                        return Err(ExecutionError::TypeMismatch);
                    };
                    *value
                };

                let mem_registry_entry = memory_registry
                    .iter_mut()
                    .find(|entry| entry.index_in_module == memory)
                    .ok_or(ExecutionError::InvalidMemoryIndex)?;

                // store the memory in the instance
                match mem_registry_entry.instance.store(
                    memory_address as usize,
                    0,
                    &data_segment.init,
                ) {
                    Ok(_) => {}
                    Err(err) => {
                        caught_error = Some(err);
                        break;
                    }
                }
            };
            data.push(DataInstance {
                data: data_segment.init.clone(),
            });
        }

        Ok((data, caught_error))
    }
}
