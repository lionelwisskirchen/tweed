use crate::binary::data_section::SegmentMode;
use crate::binary::element_section::{ElementItems, Mode};
use crate::binary::export_section::ExportContext;
use crate::binary::global_section::GlobalType;
use crate::binary::type_section::FuncType;
use crate::core::signature_lookup::lookup_signature;
use crate::core::values::Value;
use crate::core::{
    error::{Error, Result},
    value_types::ValType,
};
use crate::validation::module::Module;
use alloc::rc::Rc;
use core::cell::RefCell;

use alloc::collections::BTreeMap;
use alloc::string::String;

use alloc::vec::Vec;
use core::iter::zip;

use super::constant_expression_executor::execute_const_expr;
use super::instance::Instance;
use super::store::{
    DataInstance, ElementInstance, FuncInstance, GlobalInstance, HostEnv, HostFunc, Internal,
    MemoryInstance, Store, TableInstance,
};
pub const PAGE_SIZE: u32 = 65536;
pub const MAX_MEM_PAGES: u32 = 65536;
pub const DEFAULT_MODULE: &str = "__module_instance_default__";

#[derive(Default)]
pub struct Linker {
    imported_globals: BTreeMap<ImportName, ImportSource<GlobalInstance>>,
    imported_funcs: BTreeMap<ImportName, ImportSource<HostFunc>>,
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
                ExportContext::FunctionExport(idx) => {
                    self.imported_funcs.insert(
                        key,
                        ImportSource::Extern(*instance.funcaddr.get(*idx as usize).unwrap()),
                    );
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
    pub fn add_function(
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
    pub fn instantiate(
        &mut self,
        module: &mut Module,
        store: &mut Store,
        name: Option<&str>,
    ) -> Result<String> {
        let name = match name {
            Some(n) if !store.instance_exists(n) => n.to_string(),
            Some(_) => return Err(Error::InstanceNameTaken),
            None => format!("{}#{}", DEFAULT_MODULE, store.get_instance_count()),
        };

        let mut globaladdr = self.instantiate_global_imports(module, store)?;
        let mut imported_globals = vec![];
        // TODO extract to helper func
        for addr in &globaladdr {
            imported_globals.push(
                store
                    .data
                    .globals
                    .get(*addr)
                    .ok_or(Error::UnknownGlobal)?
                    .clone(),
            );
        }

        let mut funcaddr = self.instantiate_function_imports(module, store)?;
        let mut memories = self.instantiate_memory_imports(module, store)?;
        let mut tables = self.instantiate_table_imports(module, store)?;

        let local_functions = self.instantiate_functions(module, &name)?;
        funcaddr.extend(store.add_functions(local_functions));

        let mut functions = vec![];
        // TODO extract to helper func
        for addr in &funcaddr {
            functions.push(
                store
                    .data
                    .functions
                    .get(*addr)
                    .ok_or(Error::UnknownInstance)?
                    .clone(),
            );
        }

        let globals = self.instantiate_globals(module, &imported_globals, &functions)?;
        globaladdr.extend(store.add_globals(globals));

        memories.extend(self.instantiate_memories(module)?);
        tables.extend(self.instantiate_tables(module)?);
        let data = self.instantiate_datas(module, &mut memories, &imported_globals, &functions)?;
        let elements =
            self.instantiate_elements(module, &imported_globals, &mut tables, &functions)?;

        let memaddr = store.add_memories(memories);
        let dataddr = store.add_datas(data);
        let tableaddr = store.add_tables(tables);
        let elemaddr = store.add_elements(elements);

        let exports = self.instantiate_export(module)?;
        let instance = Instance {
            name: name.clone(),
            types: module.typesec.clone(),
            funcaddr,
            tableaddr,
            memaddr,
            globaladdr,
            elemaddr,
            dataddr,
            exports,
        };
        store.add_instance(instance, &name, &module.reader)?;
        Ok(name.to_string())
    }
    fn instantiate_table_imports(
        &mut self,
        _module: &mut Module,
        _store: &mut Store,
    ) -> Result<Vec<TableInstance>> {
        // TODO implement
        Ok(Vec::new())
    }
    fn instantiate_memory_imports(
        &mut self,
        _module: &mut Module,
        _store: &mut Store,
    ) -> Result<Vec<MemoryInstance>> {
        // TODO implement
        Ok(Vec::new())
    }
    fn instantiate_function_imports(
        &mut self,
        module: &mut Module,
        store: &mut Store,
    ) -> Result<Vec<usize>> {
        let mut funcaddr = Vec::new();
        for import in module.importsec.functions.iter() {
            let key = ImportName {
                module: import.module.clone(),
                name: import.name.clone(),
            };
            let import_entity = self
                .imported_funcs
                .get(&key)
                .ok_or(Error::MissingFunctionImport)?;

            let sig = module
                .typesec
                .get(import.context.sig_index as usize)
                .ok_or(Error::InvalidSignatureIndex)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    let target_function = store
                        .data
                        .functions
                        .get(*addr)
                        .ok_or(Error::MissingFunctionImport)?;

                    if target_function.get_signature() != sig {
                        return Err(Error::IncompatibleImportType);
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
                .ok_or(Error::MissingGlobalImport)?;

            match import_entity {
                ImportSource::Extern(addr) => {
                    let target_global = store
                        .data
                        .globals
                        .get(*addr)
                        .ok_or(Error::MissingGlobalImport)?;
                    if target_global.global_type != import.context {
                        return Err(Error::IncompatibleImportType);
                    }
                    globaladdr.push(*addr);
                }
                ImportSource::Host(inst) => {
                    globaladdr.push(store.add_global(inst.clone()));
                }
            };
        }
        info!("Global imports {:?}", globaladdr);

        Ok(globaladdr)
    }
    fn instantiate_globals(
        &mut self,
        module: &mut Module,
        imported_globals: &[GlobalInstance],
        functions: &[FuncInstance],
    ) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();

        for global in module.globalsec.iter() {
            let expr = {
                module.reader.seek(global.start + 1);
                let return_value = execute_const_expr(
                    &mut module.reader,
                    imported_globals,
                    &[global.global_type.value_type],
                    functions,
                )?;
                *return_value.first().ok_or(Error::TypeMismatch)?
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
                .get(index)
                .ok_or(Error::InvalidFunctionIndex)?
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
            let data = vec![0; (memory.min * PAGE_SIZE) as usize];
            memories.push(MemoryInstance::new(data, memory.max));
        }
        Ok(memories)
    }
    fn instantiate_elements(
        &mut self,
        module: &mut Module,
        imported_globals: &[GlobalInstance],
        tables: &mut [TableInstance],
        functions: &[FuncInstance],
    ) -> Result<Vec<ElementInstance>> {
        let mut elements = Vec::new();
        for element in module.elementsec.iter() {
            let instance = match &element.items {
                ElementItems::RefFuncs(refs) => {
                    let resolved_refs: Vec<Value> = refs
                        .iter()
                        .map(|reference| Value::FuncRef(*reference as usize))
                        .collect();
                    ElementInstance::new(&element.ref_type, resolved_refs)
                }
                ElementItems::Exprs(ty, spans) => {
                    let mut resolved_refs = vec![];
                    for span in spans {
                        module.reader.seek(span.start + 1);
                        resolved_refs.push(
                            *execute_const_expr(
                                &mut module.reader,
                                imported_globals,
                                &[*ty],
                                functions,
                            )?
                            .first()
                            .ok_or(Error::TypeMismatch)?,
                        );
                    }
                    ElementInstance::new(&element.ref_type, resolved_refs)
                }
            };
            match element.mode {
                Mode::Active {
                    table,
                    offset_start,
                    offset_end: _,
                } => {
                    module.reader.seek(offset_start + 1);
                    let result = *execute_const_expr(
                        &mut module.reader,
                        imported_globals,
                        &[ValType::I32],
                        functions,
                    )?
                    .first()
                    .ok_or(Error::TypeMismatch)?;
                    let Value::I32(offset) = result else {
                        return Err(Error::TypeMismatch);
                    };
                    let target_table = tables.get_mut(table as usize).ok_or(Error::TypeMismatch)?;
                    target_table.insert(offset as usize, &instance.references)?;
                    elements.push(instance)
                }
                Mode::Passive => elements.push(instance),
                Mode::Declarative => {
                    // do nothing
                }
            }
        }
        Ok(elements)
    }
    fn instantiate_tables(&mut self, module: &mut Module) -> Result<Vec<TableInstance>> {
        let mut tables = Vec::new();
        for table in module.tablesec.iter() {
            tables.push(TableInstance::new(table));
        }
        Ok(tables)
    }
    fn instantiate_datas(
        &mut self,
        module: &mut Module,
        memories: &mut [MemoryInstance],
        imported_globals: &[GlobalInstance],
        functions: &[FuncInstance],
    ) -> Result<Vec<DataInstance>> {
        let mut data = Vec::new();
        for data_segment in module.datasec.iter() {
            if let SegmentMode::Active {
                memory,
                offset_start,
                offset_end: _,
            } = data_segment.mode
            {
                let address = {
                    module.reader.seek(offset_start + 1);
                    let return_values = execute_const_expr(
                        &mut module.reader,
                        imported_globals,
                        &[ValType::I32],
                        functions,
                    )?;
                    let Value::I32(value) = return_values.first().ok_or(Error::TypeMismatch)?
                    else {
                        return Err(Error::TypeMismatch);
                    };
                    *value
                };

                let mem_inst = memories
                    .get_mut(memory as usize)
                    .ok_or(Error::InvalidMemoryIndex)?;
                mem_inst.store(address as usize, 0, &data_segment.init)?;
            };
            data.push(DataInstance {
                data: data_segment.init.clone(),
            });
        }
        Ok(data)
    }
}
