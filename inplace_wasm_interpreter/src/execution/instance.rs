use crate::core::values::Value;
use crate::{
    binary::{module::Module, type_section::FuncType},
    core::{opcode::*, value_types::ValType, wasm_reader::WasmReader},
    validation::side_table::SideTable,
};
use alloc::vec::Vec;
use core::iter::zip;

use super::execution_stack::ExecutionStack;

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
    pub program_counter: isize,
    pub stack_pointer: usize,
    pub start: usize,
    pub end: usize,
    pub return_value: Vec<ValType>,
    pub locals: Vec<Value>,
    pub side_table: SideTable,
    pub side_table_pointer: usize,
}

// An instantiated Module
#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance<'a> {
    reader: &'a mut WasmReader,
    functions: Vec<FunctionInstance>,
    types: Vec<FuncType>,
    call_stack: Vec<Frame>,
    stack: ExecutionStack,
}

impl<'a> Instance<'a> {
    pub fn call(&mut self, index: usize, args: Vec<Value>) -> Vec<Value> {
        self.stack.reset(); // Reset stack. Maybe not the perfect location
        self.stack.push_vec(&args);
        self.invoke(index)
    }
    fn build_frame(&mut self, function: FunctionInstance) -> Frame {
        let mut locals = self.stack.pop_n(function.signature.parameter.len());
        locals.append(&mut function.locals.clone()); // Locals = Params + Locals
        Frame {
            program_counter: -1,
            stack_pointer: self.stack.len(),
            start: function.start,
            end: function.end,
            return_value: function.signature.result.clone(),
            locals,
            side_table: function.side_table.clone(),
            side_table_pointer: 0,
        }
    }
    fn invoke(&mut self, index: usize) -> Vec<Value> {
        let function = self
            .functions
            .get(index)
            .expect("Invoked function does not exist");
        let frame = self.build_frame(function.clone());
        self.call_stack.push(frame);
        self.execute()
    }
    fn execute(&mut self) -> Vec<Value> {
        loop {
            let Some(mut current_frame) = self.call_stack.pop() else {
                return self.stack.pop_n(self.stack.len());
            };
            self.execute_frame(&mut current_frame);
        }
    }
    fn skip_block_type(&mut self) {
        self.reader
            .next_leb128_u32()
            .expect("Error parsing block func idx");
    }
    fn skip_depth(&mut self) {
        self.reader
            .next_leb128_u32()
            .expect("Error parsing break depth");
    }
    fn do_goto(&mut self, current_frame: &mut Frame) {
        let side_table_entry = current_frame
            .side_table
            .entries
            .get(current_frame.side_table_pointer)
            .expect("Expected entry in sidetable for instruction");
        let pos = self.reader.current_index() as i32 + side_table_entry.ip;
        current_frame.side_table_pointer = current_frame
            .side_table_pointer
            .saturating_add_signed(side_table_entry.stp as isize);
        self.reader.seek(pos as usize);
        let saved_vals = self.stack.pop_n(side_table_entry.vals);
        self.stack.pop_n(side_table_entry.pop);
        self.stack.push_vec(&saved_vals);
    }

    fn execute_frame(&mut self, current_frame: &mut Frame) {
        self.reader.seek(current_frame.start);

        while self.reader.current_index() < current_frame.end {
            let instruction = self.reader.next_byte().expect("No next byte for frame");
            match *instruction {
                I32_CONST => {
                    let value = Value::I32(self.reader.next_leb128_i32().unwrap());
                    self.stack.push(value);
                }
                BLOCK | LOOP => {
                    self.skip_block_type();
                }
                IF => {
                    let bool_val = self.stack.pop();
                    if bool_val == Value::I32(0) {
                        self.do_goto(current_frame);
                    } else {
                        self.skip_block_type();
                        current_frame.side_table_pointer += 1;
                    }
                }
                ELSE => {
                    self.do_goto(current_frame);
                }
                LOCAL_GET => {
                    let local = current_frame
                        .locals
                        .get(self.reader.next_leb128_u32().unwrap() as usize)
                        .expect("Local get on invalid index");
                    self.stack.push(*local);
                }
                LOCAL_SET => {
                    let local = current_frame
                        .locals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .expect("Local get on invalid index");
                    let new_value = self.stack.pop();
                    *local = new_value;
                }
                I32_ADD => {
                    let (right, left) = (self.stack.pop(), self.stack.pop());
                    self.stack.push(right + left);
                }
                I32_SUB => {
                    let (right, left) = (self.stack.pop(), self.stack.pop());

                    self.stack.push(left - right);
                }
                I32_LT_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    if left < right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GT_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());

                    if left > right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GE_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());

                    if left == right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_AND => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());

                    let result = right & left;
                    self.stack.push(Value::I32(result));
                }
                BR => {
                    self.do_goto(current_frame);
                }
                BR_IF => {
                    let bool_val = self.stack.pop_i32();
                    if bool_val == 0 {
                        self.skip_depth();
                        current_frame.side_table_pointer += 1;
                    } else {
                        self.do_goto(current_frame);
                    }
                }
                BR_TABLE => {
                    let branch_index = self.stack.pop_i32();
                    let number_of_branches = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing number of branches in BR_Table")
                        as usize;
                    if branch_index as usize >= number_of_branches {
                        current_frame.side_table_pointer += number_of_branches;
                    } else {
                        current_frame.side_table_pointer += branch_index as usize;
                    }
                    self.do_goto(current_frame);
                    #[cfg(test)]
                    println!("{:?}", self.reader.rest());
                }
                RETURN => {
                    let count = current_frame.return_value.len();
                    let results = self.stack.pop_n(count);
                    self.stack.unwind_to(current_frame.stack_pointer);
                    self.stack.push_vec(&results);
                    break;
                }
                END => {
                    if self.reader.current_index() == current_frame.end {
                        break;
                    }
                }
                _ => panic!("Invalid instruction: {}", instruction),
            };
        }
    }
}

pub fn instantiate(module: Module, reader: &mut WasmReader) -> Instance {
    let function_to_sidetable = zip(module.codesec, module.sidetables);
    let mut functions = Vec::new();

    for (function, side_table) in function_to_sidetable {
        let locals: Vec<Value> = function
            .locals
            .iter()
            .map(|val_type| (*val_type).into())
            .collect::<Vec<Value>>();

        functions.push(FunctionInstance {
            signature: function.signature,
            locals,
            start: function.start,
            end: function.end,
            side_table,
        });
    }
    Instance {
        reader,
        types: module.typesec,
        functions,
        call_stack: vec![],
        stack: ExecutionStack::default(),
    }
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
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 24,
                end: 29,
            }],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 1,
                end: 29,
            }],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 24,
                end: 28,
            }],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 24,
                end: 31,
            }],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 29,
                end: 40,
            }],
            sidetables: vec![SideTable { entries: vec![] }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 33,
                end: 56,
            }],
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
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(3)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                locals: vec![ValType::I32],
                start: 26,
                end: 45,
            }],
            sidetables: vec![SideTable {
                entries: vec![SideTableEntry {
                    ip: -14,
                    stp: 0,
                    vals: 0,
                    pop: 0,
                }],
            }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(10)];
        assert_eq!(instance.call(0, vec![]), expected_result);
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
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 25,
                end: 44,
            }],
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
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);

        assert_eq!(instance.call(0, vec![Value::I32(1)]), vec![Value::I32(1)]);
        assert_eq!(instance.call(0, vec![Value::I32(0)]), vec![Value::I32(3)]);
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
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                signature: FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![ValType::I32],
                },
                locals: vec![],
                start: 25,
                end: 61,
            }],
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
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        assert_eq!(instance.call(0, vec![Value::I32(0)]), vec![Value::I32(102)]);
        assert_eq!(instance.call(0, vec![Value::I32(1)]), vec![Value::I32(101)]);
        assert_eq!(instance.call(0, vec![Value::I32(2)]), vec![Value::I32(100)]);
        assert_eq!(
            instance.call(0, vec![Value::I32(10)]),
            vec![Value::I32(103)]
        );
    }
}
