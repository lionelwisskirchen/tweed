use crate::binary::export_section::ExportContext;
use crate::core::signature_lookup::lookup_signature;
use crate::core::values::Value;
use crate::validation::module::Module;
use crate::{
    binary::type_section::FuncType,
    core::{
        error::{Error, Result},
        opcode::*,
        value_types::ValType,
        wasm_reader::WasmReader,
    },
    validation::side_table::SideTable,
};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::iter::zip;

use super::execution_stack::ExecutionStack;
use super::store::Store;

#[derive(Debug, Clone)]
pub struct FunctionInstance {
    pub signature: FuncType,
    pub locals: Vec<Value>,
    pub start: usize,
    pub end: usize,
    pub side_table: SideTable,
}

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub start: usize,
    pub end: usize,
    pub return_value: Vec<ValType>,
    pub locals: Vec<Value>,
    pub side_table: SideTable,
    pub side_table_pointer: usize,
}

// An instantiated Module
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Instance {
    store: Store,
    reader: WasmReader,
    call_stack: Vec<Frame>,
    stack: ExecutionStack,
}

impl Instance {
    pub fn call_by_name(&mut self, name: &str, args: Vec<Value>) -> Result<Vec<Value>> {
        let Some(export_instance) = self.store.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };
        match export_instance {
            ExportContext::FunctionExport(index) => self.call(*index as usize, args),
        }
    }
    pub fn call(&mut self, index: usize, args: Vec<Value>) -> Result<Vec<Value>> {
        self.stack.reset(); // Reset stack. Maybe not the perfect location
        self.stack.push_vec(&args);
        self.invoke(index)
    }
    fn build_frame(&mut self, function: FunctionInstance) -> Result<Frame> {
        let mut locals = self.stack.pop_n(function.signature.parameter.len())?;
        locals.append(&mut function.locals.clone()); // Locals = Params + Locals
        Ok(Frame {
            program_counter: 0,
            stack_pointer: self.stack.len(),
            start: function.start,
            end: function.end,
            return_value: function.signature.result.clone(),
            locals,
            side_table: function.side_table.clone(),
            side_table_pointer: 0,
        })
    }
    fn invoke(&mut self, index: usize) -> Result<Vec<Value>> {
        let function = self
            .store
            .functions
            .get(index)
            .ok_or(Error::InvalidFunctionIndex)?;
        let frame = self.build_frame(function.clone())?;
        self.call_stack.push(frame);
        self.execute()
    }
    fn execute(&mut self) -> Result<Vec<Value>> {
        loop {
            #[cfg(test)]
            println!("{:?}", self.call_stack);
            let Some(mut current_frame) = self.call_stack.pop() else {
                return self.stack.pop_n(self.stack.len());
            };

            self.execute_frame(&mut current_frame)?;
        }
    }
    fn skip_block_type(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }
    fn skip_depth(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }
    fn do_goto(&mut self, current_frame: &mut Frame) -> Result<()> {
        let side_table_entry = current_frame
            .side_table
            .entries
            .get(current_frame.side_table_pointer)
            .ok_or(Error::InvalidSidetableIndex)?;
        let pos = self.reader.current_index() as i32 + side_table_entry.ip;
        current_frame.side_table_pointer = current_frame
            .side_table_pointer
            .saturating_add_signed(side_table_entry.stp as isize);
        self.reader.seek(pos as usize);
        let saved_vals = self.stack.pop_n(side_table_entry.vals)?;
        self.stack.pop_n(side_table_entry.pop)?;
        self.stack.push_vec(&saved_vals);
        Ok(())
    }

    fn execute_frame(&mut self, current_frame: &mut Frame) -> Result<()> {
        self.reader
            .seek(current_frame.start + current_frame.program_counter);
        while self.reader.current_index() < current_frame.end {
            let instruction = self
                .reader
                .next_byte()
                .ok_or(Error::InvalidSectionBytecode)?;
            match *instruction {
                I32_CONST => {
                    let value = Value::I32(self.reader.next_leb128_i32().unwrap());
                    self.stack.push(value);
                }
                I64_CONST => {
                    let value = Value::I64(self.reader.next_leb128_i64().unwrap());
                    self.stack.push(value);
                }
                BLOCK | LOOP => {
                    self.skip_block_type()?;
                }
                IF => {
                    let bool_val = self.stack.pop()?;
                    if bool_val == Value::I32(0) {
                        self.do_goto(current_frame)?;
                    } else {
                        self.skip_block_type()?;
                        current_frame.side_table_pointer += 1;
                    }
                }
                ELSE => {
                    self.do_goto(current_frame)?;
                }
                LOCAL_GET => {
                    let local = current_frame
                        .locals
                        .get(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::InvalidLocalIndex)?;
                    self.stack.push(*local);
                }
                LOCAL_SET => {
                    let local = current_frame
                        .locals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::InvalidLocalIndex)?;
                    let new_value = self.stack.pop()?;
                    *local = new_value;
                }
                DROP => {
                    self.stack.pop()?;
                }
                NOP => {}
                I32_CTZ => {
                    let val = self.stack.pop_i32()?;
                    let result = val.trailing_zeros() as i32;
                    self.stack.push(result.into());
                }
                I64_CTZ => {
                    let val = self.stack.pop_i64()?;
                    let result = val.trailing_zeros() as i64;
                    self.stack.push(result.into());
                }
                I64_ADD => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = right + left;
                    self.stack.push(result.into());
                }
                I32_ADD => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    let result = right + left;
                    self.stack.push(result.into());
                }
                I32_SUB => {
                    let (right, left) = (self.stack.pop()?, self.stack.pop()?);

                    self.stack.push(left - right);
                }
                I32_EQZ => {
                    let val = self.stack.pop_i32()?;
                    if val == 0 {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_LT_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if left < right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GT_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    if left > right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GE_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    if left == right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_AND => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    let result = right & left;
                    self.stack.push(Value::I32(result));
                }
                SELECT => {
                    let (cond, b, a) =
                        (self.stack.pop_i32()?, self.stack.pop()?, self.stack.pop()?);
                    if cond == 0 {
                        self.stack.push(b);
                    } else {
                        self.stack.push(a);
                    }
                }
                BR => {
                    self.do_goto(current_frame)?;
                }
                BR_IF => {
                    let bool_val = self.stack.pop_i32()?;
                    if bool_val == 0 {
                        self.skip_depth()?;
                        current_frame.side_table_pointer += 1;
                    } else {
                        self.do_goto(current_frame)?;
                    }
                }
                BR_TABLE => {
                    let branch_index = self.stack.pop_i32()?;
                    let number_of_branches =
                        self.reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)? as usize;
                    if branch_index as usize >= number_of_branches {
                        current_frame.side_table_pointer += number_of_branches;
                    } else {
                        current_frame.side_table_pointer += branch_index as usize;
                    }
                    self.do_goto(current_frame)?;
                    #[cfg(test)]
                    println!("{:?}", self.reader.rest());
                }
                RETURN => {
                    let count = current_frame.return_value.len();
                    let results = self.stack.pop_n(count)?;
                    self.stack.unwind_to(current_frame.stack_pointer);
                    self.stack.push_vec(&results);
                    break;
                }
                END => {
                    if self.reader.current_index() == current_frame.end {
                        break;
                    }
                }
                CALL => {
                    let index = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidFunctionIndex)?;
                    let function = self
                        .store
                        .functions
                        .get(index as usize)
                        .ok_or(Error::InvalidFunctionIndex)?;

                    // TODO External/internal functions
                    let frame = self.build_frame(function.clone())?;
                    current_frame.program_counter =
                        self.reader.current_index() - current_frame.start + 1;
                    self.call_stack.push(current_frame.clone());
                    self.call_stack.push(frame);
                    break;
                }
                _ => return Err(Error::UndefinedInstruction(*instruction)),
            };
        }
        current_frame.program_counter = self.reader.current_index() - current_frame.start + 1;
        Ok(())
    }
}

pub fn instantiate(module: Module) -> Result<Instance> {
    let function_to_sidetable = zip(module.codesec, module.sidetables);
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

        functions.push(FunctionInstance {
            signature: signature.clone(),
            locals,
            start: function.start,
            end: function.end,
            side_table,
        });
    }
    let mut exports = BTreeMap::new();

    for export in module.exportsec {
        exports.insert(export.name.clone(), export.context.clone());
    }

    let store = Store { functions, exports };
    Ok(Instance {
        reader: module.reader,
        store,
        call_stack: vec![],
        stack: ExecutionStack::default(),
    })
}

#[cfg(test)]
mod tests {

    use crate::{
        binary::{code_section::FuncCode, function_section::FuncDecl},
        validation::side_table::SideTableEntry,
    };

    use super::*;
    #[test]
    fn test_simple_add() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 29,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
        };

        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }
    #[should_panic]
    #[test]
    fn test_invalid_instruction() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 1,
                end: 29,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
        };

        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_no_end_instruction() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 28,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
        };

        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_return() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 31,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
        };

        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_blocks() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
                 (block $label1 (result i32)
	    	   i32.const 10
		   (block $label2 (param i32) (result i32)
		   	  i32.const 2
			  i32.add
		   )
	         )
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![
                FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![ValType::I32],
                },
            ],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 29,
                end: 40,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_blocks_with_branches() {
        let bytes = wat::parse_str(
            "(module
                (func (result i32)
                  (block $label1 (result i32)
                     i32.const 4
                     i32.const 5
                     i32.add
                     br $label1
	          )
                  (block $label3 (param i32)
                        i32.const 7                        
                     (block $label2 (param i32 i32)
                        i32.add
                        br_if $label3
                     )
                  )
		  i32.const 3
                )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![
                FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![],
                },
                FuncType {
                    parameter: vec![ValType::I32, ValType::I32],
                    result: vec![],
                },
            ],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 33,
                end: 56,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 3,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
        };
        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(3)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_for_loop() {
        let bytes = wat::parse_str(
            "(module
                 (func (result i32)
                   (local $i i32)
                   (loop $my_loop
                       local.get $i
                       i32.const 1
                       i32.add
                       local.set $i
                       local.get $i
                       i32.const 10
                       i32.lt_s
                       br_if $my_loop
                   )
                    local.get $i
                )
             )
",
        )
        .unwrap();

        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![ValType::I32],
                start: 26,
                end: 45,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![SideTableEntry {
                    ip: -14,
                    stp: 0,
                    vals: 0,
                    pop: 0,
                }],
            }],
        };
        let mut instance = instantiate(module).unwrap();
        let expected_result = vec![Value::I32(10)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_if_else() {
        let bytes = wat::parse_str(
            "
(module
  (func (param i32) (result i32)
    (block $label1 (result i32)
      local.get 0
      (if $if (result i32)
        (then
          i32.const 1
          br $label1
        )
        (else
	i32.const 3
	br $if
        )
      )
      br $label1
    )
  )
)",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 25,
                end: 44,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 7,
                        stp: 3,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 10,
                        stp: 4,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 5,
                        stp: 2,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
        };
        let mut instance = instantiate(module).unwrap();

        assert_eq!(
            instance.call(0, vec![Value::I32(1)]).unwrap(),
            vec![Value::I32(1)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(0)]).unwrap(),
            vec![Value::I32(3)]
        );
    }

    #[test]
    fn test_br_table() {
        let bytes = wat::parse_str(
            "
(func (param $p i32) (result i32)
  (block
    (block
      (block
        (block (local.get $p)
               (br_table
                         2   ;; p == 0 => (br 2)
                         1   ;; p == 1 => (br 1)
                         0   ;; p == 2 => (br 0)
                         3)) ;; else => (br 3)
        ;; Target for (br 0)
        (i32.const 100)
        (return))
      ;; Target for (br 1)
      (i32.const 101)
      (return))
    ;; Target for (br 2)
    (i32.const 102)
    (return))
  ;; Target for (br 3)
  (i32.const 103)
  (return))",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 25,
                end: 61,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 15,
                        stp: 4,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 10,
                        stp: 3,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 5,
                        stp: 2,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 20,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
        };
        let mut instance = instantiate(module).unwrap();
        assert_eq!(
            instance.call(0, vec![Value::I32(0)]).unwrap(),
            vec![Value::I32(102)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(1)]).unwrap(),
            vec![Value::I32(101)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(2)]).unwrap(),
            vec![Value::I32(100)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(10)]).unwrap(),
            vec![Value::I32(103)]
        );
    }
}
