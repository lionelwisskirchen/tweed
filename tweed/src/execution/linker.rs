use crate::binary::data_section::SegmentMode;
use crate::binary::element_section::{ElementItems, Mode};
use crate::binary::export_section::ExportContext;
use crate::core::signature_lookup::lookup_signature;
use crate::core::values::Value;
use crate::core::{
    error::{Error, Result},
    value_types::ValType,
};
use crate::validation::module::Module;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::zip;

use super::constant_expression_executor::execute_const_expr;
use super::instance::Instance;
use super::store::{
    DataInstance, ElementInstance, FunctionInstance, GlobalInstance, MemoryInstance, Store,
    TableInstance,
};
pub const PAGE_SIZE: u32 = 65536;
pub const DEFAULT_MODULE: &str = "__module_instance_default__";

pub struct Linker<'module> {
    module: &'module mut Module,
    imported_globals: BTreeMap<ImportName, GlobalInstance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ImportName {
    pub module: String,
    pub name: String,
}

impl<'module> Linker<'module> {
    pub fn new(module: &'module mut Module) -> Self {
        Linker {
            module,
            imported_globals: BTreeMap::new(),
        }
    }

    pub fn auto_link_spectest_globals(&mut self) -> Result<()> {
        for import in &self.module.importsec.globals.clone() {
            if import.module == "spectest" {
                self.add_global_import(
                    import.module.clone(),
                    import.name.clone(),
                    match import.context.value_type {
                        ValType::I32 => Value::I32(42),
                        ValType::I64 => Value::I64(42),
                        _ => panic!("Unknown value type cant be converted to value."),
                    },
                )?;
            }
        }
        Ok(())
    }
    pub fn add_global_import(&mut self, module: String, name: String, value: Value) -> Result<()> {
        let key = ImportName {
            module: module.clone(),
            name: name.clone(),
        };
        if let Some(import) = self
            .module
            .importsec
            .globals
            .iter()
            .find(|g| g.module == module && g.name == name)
        {
            let global_type = import.context.clone();
            if !global_type.value_type.is_type(value) {
                return Err(Error::TypeMismatch);
            }
            let instance = GlobalInstance { global_type, value };
            self.imported_globals.insert(key, instance);
            Ok(())
        } else {
            Err(Error::UnknownGlobal)
        }
    }
    pub fn instantiate(&mut self, store: &mut Store, name: Option<&str>) -> Result<String> {
        // let name = match name {
        //     Some(n) => n,
        //     None => DEFAULT_MODULE,
        // };
        let name = match name {
            Some(n) if !store.instance_exists(n) => n.to_string(),
            Some(_) => return Err(Error::InstanceNameTaken),
            None => format!("{}#{}", DEFAULT_MODULE, store.get_instance_count()),
        };

        let mut globals = self.instantiate_global_imports()?;
        let exports = self.instantiate_export()?;
        let functions = self.instantiate_functions(&name)?;
        let mut memories = self.instantiate_memories()?;
        let data = self.instantiate_datas(&mut memories, &globals, &functions)?;
        let mut tables = self.instantiate_tables()?;
        let elements = self.instantiate_elements(&globals, &mut tables, &functions)?;

        globals.extend(self.instantiate_globals(&globals, &functions)?);
        let globaladdr = store.add_globals(globals);
        let funcaddr = store.add_functions(functions);
        let memaddr = store.add_memories(memories);
        let dataddr = store.add_datas(data);
        let tableaddr = store.add_tables(tables);

        let elemaddr = store.add_elements(elements);

        let instance = Instance {
            name: name.clone(),
            funcaddr,
            tableaddr,
            memaddr,
            globaladdr,
            elemaddr,
            dataddr,
            exports,
        };
        store.add_instance(instance, &name, &self.module.reader)?;
        Ok(name.to_string())
    }
    fn instantiate_global_imports(&mut self) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();
        for import in self.module.importsec.globals.iter() {
            let key = ImportName {
                module: import.module.clone(),
                name: import.name.clone(),
            };
            let global = self
                .imported_globals
                .get(&key)
                .ok_or(Error::MissingGlobalImport)?;
            globals.push(global.clone())
        }
        Ok(globals)
    }
    fn instantiate_globals(
        &mut self,
        imported_globals: &[GlobalInstance],
        functions: &[FunctionInstance],
    ) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();

        for global in self.module.globalsec.iter() {
            let expr = {
                self.module.reader.seek(global.start + 1);
                let return_value = execute_const_expr(
                    &mut self.module.reader,
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
    fn instantiate_export(&mut self) -> Result<BTreeMap<String, ExportContext>> {
        let mut exports = BTreeMap::new();
        for export in &self.module.exportsec {
            exports.insert(export.name.clone(), export.context.clone());
        }
        Ok(exports)
    }
    fn instantiate_functions(&mut self, name: &str) -> Result<Vec<FunctionInstance>> {
        let function_to_sidetable = zip(self.module.codesec.iter(), self.module.sidetables.iter());
        let mut functions = Vec::new();
        for (index, (function, side_table)) in function_to_sidetable.enumerate() {
            let locals: Vec<Value> = function
                .locals
                .iter()
                .map(|val_type| (*val_type).into())
                .collect::<Vec<Value>>();

            let signature_index = self
                .module
                .funcsec
                .get(index)
                .ok_or(Error::InvalidFunctionIndex)?
                .sig_index;
            let signature = lookup_signature(signature_index, &self.module.typesec);

            functions.push(FunctionInstance {
                instance: name.to_string(),
                signature: signature.clone(),
                locals,
                start: function.start,
                end: function.end,
                side_table: side_table.clone(),
            });
        }
        Ok(functions)
    }

    fn instantiate_memories(&mut self) -> Result<Vec<MemoryInstance>> {
        let mut memories = Vec::new();
        for memory in self.module.memorysec.iter() {
            let data = vec![0; (memory.min * PAGE_SIZE) as usize];
            memories.push(MemoryInstance::new(data, memory.max));
        }
        Ok(memories)
    }
    fn instantiate_elements(
        &mut self,
        imported_globals: &[GlobalInstance],
        tables: &mut [TableInstance],
        functions: &[FunctionInstance],
    ) -> Result<Vec<ElementInstance>> {
        let mut elements = Vec::new();

        for element in self.module.elementsec.iter() {
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
                        self.module.reader.seek(span.start + 1);
                        resolved_refs.push(
                            *execute_const_expr(
                                &mut self.module.reader,
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
                    self.module.reader.seek(offset_start + 1);
                    let result = *execute_const_expr(
                        &mut self.module.reader,
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
    fn instantiate_tables(&mut self) -> Result<Vec<TableInstance>> {
        let mut tables = Vec::new();
        for table in self.module.tablesec.iter() {
            tables.push(TableInstance::new(table));
        }
        Ok(tables)
    }
    fn instantiate_datas(
        &mut self,
        memories: &mut [MemoryInstance],
        imported_globals: &[GlobalInstance],
        functions: &[FunctionInstance],
    ) -> Result<Vec<DataInstance>> {
        let mut data = Vec::new();
        for data_segment in self.module.datasec.iter() {
            if let SegmentMode::Active {
                memory,
                offset_start,
                offset_end: _,
            } = data_segment.mode
            {
                let address = {
                    self.module.reader.seek(offset_start + 1);
                    let return_values = execute_const_expr(
                        &mut self.module.reader,
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

#[cfg(test)]
mod tests {

    // use crate::{
    //     binary::{code_section::FuncCode, function_section::FuncDecl, type_section::FuncType},
    //     validation::side_table::SideTable,
    //     WasmReader,
    // };

    // use super::*;

    // #[test]
    // fn test_simple_add() {
    //     let bytes = wat::parse_str(
    //         "(module
    //            (func (result i32)
    // 	    	   i32.const 10
    // 	    	   i32.const 12
    //            	   i32.add
    //            )
    //          )",
    //     )
    //     .unwrap();
    //     let mut module = Module {
    //         reader: WasmReader::new(bytes.to_vec()),
    //         typesec: vec![FuncType {
    //             parameter: vec![],
    //             result: vec![ValType::I32],
    //         }],
    //         funcsec: vec![FuncDecl { sig_index: 0 }],
    //         codesec: vec![FuncCode {
    //             locals: vec![],
    //             start: 24,
    //             end: 29,
    //         }],
    //         exportsec: vec![],
    //         sidetables: vec![SideTable { entries: vec![] }],
    //         ..Default::default()
    //     };
    //     let mut store = Store::default();
    //     let instance = Linker::new(&mut module)
    //         .instantiate(&mut store, None)
    //         .unwrap();
    //     let expected_result = vec![Value::I32(22)];

    //     assert_eq!(
    //         instance
    //             .call(0, vec![], &mut store)
    //             .unwrap(),
    //         expected_result
    //     );
    // }

    //     #[should_panic]
    //     #[test]
    //     fn test_invalid_instruction() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                (func (result i32)
    // 	    	   i32.const 10
    // 	    	   i32.const 12
    //                	   i32.add
    //                )
    //              )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 1,
    //                 end: 29,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable { entries: vec![] }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(22)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_no_end_instruction() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                (func (result i32)
    // 	    	   i32.const 10
    // 	    	   i32.const 12
    //                	   i32.add
    //                )
    //              )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 24,
    //                 end: 28,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable { entries: vec![] }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(22)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_return() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                (func (result i32)
    // 	    	   i32.const 10
    // 	    	   i32.const 12
    //                	   return
    //                    i32.const 12
    //                )
    //              )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 24,
    //                 end: 31,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable { entries: vec![] }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let  instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(12)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_blocks() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                (func (result i32)
    //                  (block $label1 (result i32)
    // 	    	   i32.const 10
    // 		   (block $label2 (param i32) (result i32)
    // 		   	  i32.const 2
    // 			  i32.add
    // 		   )
    // 	         )
    //                )
    //              )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![
    //                 FuncType {
    //                     parameter: vec![],
    //                     result: vec![ValType::I32],
    //                 },
    //                 FuncType {
    //                     parameter: vec![ValType::I32],
    //                     result: vec![ValType::I32],
    //                 },
    //             ],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 29,
    //                 end: 40,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable { entries: vec![] }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(12)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_blocks_with_branches() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                 (func (result i32)
    //                   (block $label1 (result i32)
    //                      i32.const 4
    //                      i32.const 5
    //                      i32.add
    //                      br $label1
    // 	          )
    //                   (block $label3 (param i32)
    //                         i32.const 7
    //                      (block $label2 (param i32 i32)
    //                         i32.add
    //                         br_if $label3
    //                      )
    //                   )
    // 		  i32.const 3
    //                 )
    //              )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![
    //                 FuncType {
    //                     parameter: vec![],
    //                     result: vec![ValType::I32],
    //                 },
    //                 FuncType {
    //                     parameter: vec![ValType::I32],
    //                     result: vec![],
    //                 },
    //                 FuncType {
    //                     parameter: vec![ValType::I32, ValType::I32],
    //                     result: vec![],
    //                 },
    //             ],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 33,
    //                 end: 56,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable {
    //                 entries: vec![
    //                     SideTableEntry {
    //                         ip: 2,
    //                         stp: 1,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 3,
    //                         stp: 1,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                 ],
    //             }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let  instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(3)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_for_loop() {
    //         let bytes = wat::parse_str(
    //             "(module
    //                  (func (result i32)
    //                    (local $i i32)
    //                    (loop $my_loop
    //                        local.get $i
    //                        i32.const 1
    //                        i32.add
    //                        local.set $i
    //                        local.get $i
    //                        i32.const 10
    //                        i32.lt_s
    //                        br_if $my_loop
    //                    )
    //                     local.get $i
    //                 )
    //              )
    // ",
    //         )
    //         .unwrap();

    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![ValType::I32],
    //                 start: 26,
    //                 end: 45,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable {
    //                 entries: vec![SideTableEntry {
    //                     ip: -14,
    //                     stp: 0,
    //                     vals: 0,
    //                     pop: 0,
    //                 }],
    //             }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let  instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         let expected_result = vec![Value::I32(10)];
    //         assert_eq!(
    //             instance.call(0, vec![], &mut store, &mut module.reader).unwrap(),
    //             expected_result
    //         );
    //     }

    //     #[test]
    //     fn test_if_else() {
    //         let bytes = wat::parse_str(
    //             "
    // (module
    //   (func (param i32) (result i32)
    //     (block $label1 (result i32)
    //       local.get 0
    //       (if $if (result i32)
    //         (then
    //           i32.const 1
    //           br $label1
    //         )
    //         (else
    // 	i32.const 3
    // 	br $if
    //         )
    //       )
    //       br $label1
    //     )
    //   )
    // )",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![ValType::I32],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 25,
    //                 end: 44,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable {
    //                 entries: vec![
    //                     SideTableEntry {
    //                         ip: 7,
    //                         stp: 3,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 10,
    //                         stp: 4,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 5,
    //                         stp: 2,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 2,
    //                         stp: 1,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 2,
    //                         stp: 1,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                 ],
    //             }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(1)], &mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(1)]
    //         );
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(0)], &mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(3)]
    //         );
    //     }

    //     #[test]
    //     fn test_br_table() {
    //         let bytes = wat::parse_str(
    //             "
    // (func (param $p i32) (result i32)
    //   (block
    //     (block
    //       (block
    //         (block (local.get $p)
    //                (br_table
    //                          2   ;; p == 0 => (br 2)
    //                          1   ;; p == 1 => (br 1)
    //                          0   ;; p == 2 => (br 0)
    //                          3)) ;; else => (br 3)
    //         ;; Target for (br 0)
    //         (i32.const 100)
    //         (return))
    //       ;; Target for (br 1)
    //       (i32.const 101)
    //       (return))
    //     ;; Target for (br 2)
    //     (i32.const 102)
    //     (return))
    //   ;; Target for (br 3)
    //   (i32.const 103)
    //   (return))",
    //         )
    //         .unwrap();
    //         let mut module = Module {
    //             reader: WasmReader::new(bytes.to_vec()),
    //             typesec: vec![FuncType {
    //                 parameter: vec![ValType::I32],
    //                 result: vec![ValType::I32],
    //             }],
    //             funcsec: vec![FuncDecl { sig_index: 0 }],
    //             codesec: vec![FuncCode {
    //                 locals: vec![],
    //                 start: 25,
    //                 end: 61,
    //             }],
    //             exportsec: vec![],
    //             sidetables: vec![SideTable {
    //                 entries: vec![
    //                     SideTableEntry {
    //                         ip: 15,
    //                         stp: 4,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 10,
    //                         stp: 3,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 5,
    //                         stp: 2,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                     SideTableEntry {
    //                         ip: 20,
    //                         stp: 1,
    //                         vals: 0,
    //                         pop: 0,
    //                     },
    //                 ],
    //             }],
    //             ..Default::default()
    //         };
    //         let mut store = Store::default();

    //         let instance = Linker::new(&mut module).instantiate(&mut store).unwrap();
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(0)], &mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(102)]
    //         );
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(1)], &mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(101)]
    //         );
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(2)], &mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(100)]
    //         );
    //         assert_eq!(
    //             instance.call(0, vec![Value::I32(10)],&mut store, &mut module.reader).unwrap(),
    //             vec![Value::I32(103)]
    //         );
    //     }
}
