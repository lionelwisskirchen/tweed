extern crate alloc;

use core::iter::zip;

use alloc::collections::BTreeMap;

use super::store::Store;

use super::adresses::ExternAddr;
use super::constant_expression_executor::execute_const_expr;
use crate::binary::data_section::SegmentMode;
use crate::binary::element_section::{ElementItems, Mode};
use crate::binary::import_section::Import;
use crate::binary::memory_section::WASM_PAGE_SIZE;
use crate::core::error::ExecutionError;
use crate::core::signature_lookup::lookup_signature;
use crate::core::value_types::ValType;

use crate::execution::instances::function_instance::Internal;
use crate::execution::instances::{
    DataInstance, ElementInstance, FuncInstance, GlobalInstance, MemoryInstance, ModuleInstance,
    TableInstance,
};

use crate::validation::module::Module;

use crate::execution::adresses::{FuncAddr, GlobalAddr, InstanceAddr, MemoryAddr, TableAddr};

use crate::Value;

use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, Default)]
pub struct ModuleInstantiator {}

impl ModuleInstantiator {
    /// Instantiates a WebAssembly module into the given store and produces a
    /// runtime module instance.
    ///
    /// # Important
    ///
    /// - Exports are *not globally visible by default*; they are only stored within
    ///   the resulting module instance and must be explicitly wired into other
    ///   modules via the linker.
    /// - Imports must already be resolvable at instantiation time via the linker
    ///   or host environment.
    /// - All runtime objects are allocated in the shared `Store`, and module
    ///   instances only contain references (addresses) to these objects.
    /// - The order of the external Adresses must match the order of the
    ///   expected imports of the Module.
    ///
    /// # Returns
    /// The address of the instantiated Module inside of the store.
    pub fn module_instantiate(
        &mut self,
        store: &mut Store,
        mut module: Module,
        external: &[ExternAddr],
    ) -> Result<InstanceAddr> {
        let module_imports = module.importsec.imports();
        // Step 4: https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation
        if module_imports.len() != external.len() {
            Err(ExecutionError::IncompatibleNumberOfImports)?
        }
        // Step 5: https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation
        for (external, import) in external.iter().zip(module_imports) {
            match external {
                ExternAddr::Func(func_addr) => {
                    let function_inst_sig = store.functions.get(*func_addr)?.get_signature();
                    let import_inst_sig = match import {
                        Import::Function(imp) => module
                            .typesec
                            .get(imp.context.sig_index as usize)
                            .ok_or(ExecutionError::InvalidSignatureIndex)?,
                        _ => return Err(ExecutionError::IncompatibleImportType),
                    };
                    if function_inst_sig != import_inst_sig {
                        return Err(ExecutionError::IncompatibleImportType);
                    }
                }
                ExternAddr::Table(table_addr) => {
                    let table_inst = store.tables.get(*table_addr)?;
                    let import_inst = match import {
                        Import::Table(imp) => imp,
                        _ => return Err(ExecutionError::IncompatibleImportType),
                    };
                    table_inst.ensure_ref_type_is_correct(&import_inst)?;
                    table_inst.ensure_limits_are_correct(&import_inst)?
                }
                ExternAddr::Global(global_addr) => {
                    let global_inst = store.globals.get(*global_addr)?;
                    let import_inst = match import {
                        Import::Global(imp) => imp,
                        _ => return Err(ExecutionError::IncompatibleImportType),
                    };
                    if import_inst.context != global_inst.global_type {
                        return Err(ExecutionError::IncompatibleImportType);
                    }
                }
                ExternAddr::Memory(memory_addr) => {
                    let memory_inst = store.memories.get(*memory_addr)?;
                    let import_inst = match import {
                        Import::Memory(imp) => imp,
                        _ => return Err(ExecutionError::IncompatibleImportType),
                    };
                    memory_inst.ensure_limits_are_correct(&import_inst)?;
                }
            }
        }
        // We need the instance_addr in the functions as backref
        let instance_addr = store.instances.reserve_instance_addr();
        /*---------------------------------
         * FUNCTIONS
         *---------------------------------*/
        let imported_func_addr: Vec<FuncAddr> = external
            .iter()
            .filter_map(|a| FuncAddr::try_from(*a).ok())
            .collect();
        let mut local_functions = Vec::new();
        let function_to_sidetable = zip(module.codesec.iter(), module.sidetables.iter());
        for (index, (function, side_table)) in function_to_sidetable.enumerate() {
            let locals: Vec<Value> = function
                .locals
                .iter()
                .map(|val_type| (*val_type).into())
                .collect::<Vec<Value>>();

            let signature_index = module
                .funcsec
                .get(index + imported_func_addr.len())
                .ok_or(ExecutionError::InvalidFunctionIndex)?
                .sig_index;
            let signature = lookup_signature(signature_index, &module.typesec);

            local_functions.push(FuncInstance::Internal(Internal {
                instance: instance_addr,
                signature: signature.clone(),
                locals,
                start: function.start,
                end: function.end,
                side_table: side_table.clone(),
            }));
        }
        let local_func_idxs = store.functions.add(local_functions);
        let all_function_idxs = [imported_func_addr, local_func_idxs].concat();

        /*---------------------------------
         * Globals
         *---------------------------------*/
        let imported_global_addr: Vec<GlobalAddr> = external
            .iter()
            .filter_map(|a| GlobalAddr::try_from(*a).ok())
            .collect();
        let mut local_globals = Vec::new();
        for global in module.globalsec.iter() {
            let expr = {
                module.reader.seek(global.start + 1);
                let return_value = execute_const_expr(
                    &mut module.reader,
                    &imported_global_addr,
                    &all_function_idxs,
                    &[global.global_type.value_type],
                    store,
                )?;
                *return_value.first().ok_or(ExecutionError::TypeMismatch)?
            };
            local_globals.push(GlobalInstance {
                global_type: global.global_type.clone(),
                value: expr,
            });
        }
        let local_global_idxs = store.globals.add(local_globals);
        let all_global_idxs = [imported_global_addr.clone(), local_global_idxs].concat();

        /*---------------------------------
         * MEMORY
         *---------------------------------*/
        let imported_memory_addr: Vec<MemoryAddr> = external
            .iter()
            .filter_map(|a| MemoryAddr::try_from(*a).ok())
            .collect();
        let mut local_memories = vec![];
        for memory in module.memorysec.iter() {
            let data = vec![0; (memory.limits.min * WASM_PAGE_SIZE) as usize];
            local_memories.push(MemoryInstance::new(data, memory.limits.clone()));
        }
        let local_memory_addr = store.memories.add(local_memories);
        let all_memory_idxs = [imported_memory_addr, local_memory_addr].concat();

        /*---------------------------------
         * TABLES
         *---------------------------------*/
        let imported_table_addr: Vec<TableAddr> = external
            .iter()
            .filter_map(|a| TableAddr::try_from(*a).ok())
            .collect();
        let mut local_tables = vec![];
        for table in module.tablesec.iter() {
            local_tables.push(TableInstance::new(table.clone()));
        }
        let local_table_addr = store.tables.add(local_tables);
        let all_table_idxs = [imported_table_addr, local_table_addr].concat();

        /*---------------------------------
         * DATA
         *---------------------------------*/
        let mut data = Vec::new();
        let mut error_in_data_instantiation = None;
        for data_segment in module.datasec.iter() {
            if let SegmentMode::Active {
                memory_idx,
                offset_start,
                offset_end: _,
            } = data_segment.mode
            {
                // find the address of the data in the memory
                let data_address = {
                    module.reader.seek(offset_start + 1);
                    let return_values = execute_const_expr(
                        &mut module.reader,
                        &imported_global_addr,
                        &all_function_idxs,
                        &[ValType::I32],
                        store,
                    )?;
                    let Value::I32(value) =
                        return_values.first().ok_or(ExecutionError::TypeMismatch)?
                    else {
                        return Err(ExecutionError::TypeMismatch);
                    };
                    *value
                };

                let memory_instance_addr = all_memory_idxs
                    .get(memory_idx as usize)
                    .ok_or(ExecutionError::InvalidMemoryIndex)?;
                let instance = store.memories.get_mut(*memory_instance_addr)?;
                if let Err(err) = instance.store(data_address as usize, 0, &data_segment.init) {
                    error_in_data_instantiation = Some(err);
                    break;
                }
            }
            data.push(DataInstance {
                data: data_segment.init.clone(),
            });
        }
        let all_data_idxs = store.datas.add(data);

        /*---------------------------------
         * ELEMENTS
         *---------------------------------*/
        let mut elements = Vec::new();
        let mut error_in_element_instantiation = None;
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
                        let addr_in_store = all_function_idxs
                            .get(*idx as usize)
                            .ok_or(ExecutionError::FunctionNotFound)?;

                        resolved_refs.push(Value::FuncRef(addr_in_store.0));
                    }
                    ElementInstance::new(&element.ref_type, resolved_refs)
                }
                ElementItems::Exprs(ty, spans) => {
                    let mut resolved_refs = vec![];
                    for span in spans {
                        module.reader.seek(span.start + 1);
                        let local_addr_value = *execute_const_expr(
                            &mut module.reader,
                            &imported_global_addr,
                            &all_function_idxs,
                            &[*ty],
                            store,
                        )?
                        .first()
                        .ok_or(ExecutionError::TypeMismatch)?;

                        match local_addr_value {
                            Value::FuncRef(local_addr) => {
                                let store_addr = all_function_idxs
                                    .get(local_addr)
                                    .ok_or(ExecutionError::FunctionNotFound)?;
                                resolved_refs.push(Value::FuncRef(store_addr.0));
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
                        &imported_global_addr,
                        &all_function_idxs,
                        &[ValType::I32],
                        store,
                    )?
                    .first()
                    .ok_or(ExecutionError::TypeMismatch)?;
                    let Value::I32(offset) = result else {
                        return Err(ExecutionError::TypeMismatch);
                    };
                    let table_address = all_table_idxs
                        .get(table as usize)
                        .ok_or(ExecutionError::TypeMismatch)?;
                    let table_instance = store.tables.get_mut(*table_address)?;

                    if let Err(err) = table_instance.insert(offset as usize, &instance.references) {
                        error_in_element_instantiation = Some(err);
                        break;
                    }
                    elements.push(instance);
                }
                Mode::Passive => elements.push(instance),
                Mode::Declarative => {
                    // do nothing
                }
            }
        }
        let all_elem_idxs = store.elements.add(elements);
        let mut exports = BTreeMap::new();
        for export in &module.exportsec {
            exports.insert(export.name.clone(), export.context.clone());
        }
        let mut start_address = None;
        if let Some(id) = module.startsec {
            start_address = all_function_idxs.get(id as usize).cloned()
        }

        let module_instance = ModuleInstance {
            types: module.typesec.clone(),
            funcaddr: all_function_idxs,
            tableaddr: all_table_idxs,
            memaddr: all_memory_idxs,
            globaladdr: all_global_idxs,
            elemaddr: all_elem_idxs,
            dataddr: all_data_idxs,
            exports,
            reader: *store.reader.add(vec![module.reader]).first().unwrap(),
            start: start_address,
        };

        store
            .instances
            .insert_instance_at_addr(instance_addr, module_instance)?;

        // check if we had an error during element or data instantiation and return it if this is the case
        if let Some(err) = error_in_data_instantiation.or(error_in_element_instantiation) {
            return Err(err);
        }
        Ok(instance_addr)
    }
}
