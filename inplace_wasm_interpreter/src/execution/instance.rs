use alloc::vec::Vec;
use core::iter::zip;

use crate::{
    binary::{module::Module, type_section::FuncType},
    core::{values::ValType, wasm_reader::WasmReader},
    validation::side_table::SideTable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl core::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left + right),
            (Value::I64(left), Value::I64(right)) => Value::I64(left + right),
            _ => panic!("type mismatch"),
        }
    }
}

#[derive(Debug)]
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
pub struct Instance<'a> {
    reader: &'a mut WasmReader,
    functions: Vec<FunctionInstance>,
    types: Vec<FuncType>,
    call_stack: Vec<Frame>,
    stack: Vec<Value>,
}

impl<'a> Instance<'a> {
    pub fn call(&mut self, index: usize, args: Vec<Value>) -> Option<&[Value]> {
        self.stack.clear(); // Reset stack. Maybe not the perfect location
        for arg in args {
            self.stack.push(arg);
        }
        self.invoke(index)
    }
    fn invoke(&mut self, index: usize) -> Option<&[Value]> {
        let func = self
            .functions
            .get(index)
            .expect("Invoked function does not exist");
        let bottom = self
            .stack
            .len()
            .saturating_sub(func.signature.parameter.len());

        let mut locals = self.stack.split_off(bottom);
        locals.append(&mut func.locals.clone());

        let frame = Frame {
            program_counter: -1,
            stack_pointer: self.stack.len(),
            start: func.start,
            end: func.end,
            return_value: func.signature.result.clone(),
            locals,
            side_table: func.side_table.clone(),
            side_table_pointer: 0,
        };
        self.call_stack.push(frame);
        self.execute()
    }
    fn execute(&mut self) -> Option<&[Value]> {
        loop {
            let Some(mut current_frame) = self.call_stack.pop() else {
                return Some(&self.stack);
            };

            self.execute_frame(&mut current_frame);
        }
    }
    fn execute_frame(&mut self, current_frame: &mut Frame) {
        self.reader.seek(current_frame.start);

        while self.reader.current_index() < current_frame.end {
            let instruction = self.reader.next_byte().expect("No next byte for frame");
            #[cfg(test)]
            println!("Instruction: {:#01x}", instruction);
            #[cfg(test)]
            println!("Value Stack: {:?}", self.stack);
            match instruction {
                0x41 => {
                    let value = Value::I32(self.reader.next_leb128_i32().unwrap());
                    self.stack.push(value);
                } // I32Const
                0x02 => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing block func idx");

                    match idx {
                        0x7f | 0x40 => {}
                        _ => {
                            self.types
                                .get(idx as usize)
                                .expect("Can't get func sig by index");
                        }
                    };
                } // Block
                0x03 => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing block func idx");
                    match idx {
                        0x7f | 0x40 => {}
                        _ => {
                            self.types
                                .get(idx as usize)
                                .expect("Can't get func sig by index");
                        }
                    };
                } //loop
                0x04 => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing block func idx");

                    match idx {
                        0x7f | 0x40 => {}
                        _ => {
                            self.types
                                .get(idx as usize)
                                .expect("Can't get func sig by index");
                        }
                    };

                    let bool_val = self.stack.pop().unwrap(); // Unwrap is safe due to validation
                    if bool_val == Value::I32(0) {
                        let side_table_entry = current_frame
                            .side_table
                            .entries
                            .get(current_frame.side_table_pointer)
                            .expect("Expected entry in sidetable for if instruction");
                        let pos = (self.reader.current_index() - 1) as i32 + side_table_entry.ip; // -1 because of the func index
                        current_frame.side_table_pointer += side_table_entry.stp as usize;
                        self.reader.seek(pos as usize);
                    } else {
                        current_frame.side_table_pointer += 1;
                    }
                } //if
                0x05 => {
                    let side_table_entry = current_frame
                        .side_table
                        .entries
                        .get(current_frame.side_table_pointer)
                        .expect("Expected entry in sidetable for else instruction");
                    let pos = self.reader.current_index() as i32 + side_table_entry.ip;
                    current_frame.side_table_pointer += side_table_entry.stp as usize;
                    self.reader.seek(pos as usize);
                } //else
                0x20 => {
                    let local = current_frame
                        .locals
                        .get(self.reader.next_leb128_u32().unwrap() as usize)
                        .expect("Local get on invalid index");
                    self.stack.push(*local);
                } // LocalGet
                0x21 => {
                    let local = current_frame
                        .locals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .expect("Local get on invalid index");
                    let new_value = self.stack.pop().expect("LocalSet needs value on stack");
                    *local = new_value;
                } // LocalSet
                0x6a => {
                    let (right, left) = (self.stack.pop().unwrap(), self.stack.pop().unwrap()); // Unwrap is safe due to validation

                    self.stack.push(right + left);
                } // I32Add
                0x48 => {
                    let (Value::I32(right), Value::I32(left)) =
                        (self.stack.pop().unwrap(), self.stack.pop().unwrap())
                    else {
                        panic!("Invalid Type for i32 lt_s");
                    }; // Unwrap is safe due to validation

                    if left < right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                } // lt_s
                0x0c => {
                    self.reader
                        .next_leb128_u32()
                        .expect("Error parsing break depth");
                    let side_table_entry = current_frame
                        .side_table
                        .entries
                        .get(current_frame.side_table_pointer)
                        .expect("Expected entry in sidetable for BR instruction");
                    let pos = (self.reader.current_index() - 1) as i32 + side_table_entry.ip; // -1 because of the depth
                    self.reader.seek(pos as usize);
                    current_frame.side_table_pointer += side_table_entry.stp as usize;
                } // BR
                0x0d => {
                    self.reader
                        .next_leb128_u32()
                        .expect("Error parsing break depth");
                    let bool_val = self.stack.pop().unwrap(); // Unwrap is safe due to validation
                    if bool_val == Value::I32(0) {
                        current_frame.side_table_pointer += 1;
                    } else {
                        let side_table_entry = current_frame
                            .side_table
                            .entries
                            .get(current_frame.side_table_pointer)
                            .expect("Expected entry in sidetable for BR_if instruction");
                        let stp = current_frame.side_table_pointer as i32 + side_table_entry.stp;
                        if stp < 0 {
                            current_frame.side_table_pointer = 0;
                        } else {
                            current_frame.side_table_pointer = stp as usize;
                        }
                        let pos = (self.reader.current_index() - 1) as i32 + side_table_entry.ip; // -1 because of the depth
                        self.reader.seek(pos as usize);
                    }
                } //BR_if

                0x0b => {
                    if self.reader.current_index() == current_frame.end {
                        break;
                    }
                } //End
                _ => panic!("Invalid instruction"),
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
            .map(|valtype| match valtype {
                ValType::I32 => Value::I32(0),
                ValType::I64 => Value::I64(0),
		ValType::Unknown => {panic!("Unknown value type. This should never happen!")}
            })
            .collect();

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
        stack: vec![],
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
        assert_eq!(instance.call(0, vec![]).unwrap(), &expected_result);
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
        assert_eq!(instance.call(0, vec![]).unwrap(), &expected_result);
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
        assert_eq!(instance.call(0, vec![]).unwrap(), &expected_result);
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
                    stp: -1,
                    vals: 0,
                    pop: 0,
                }],
            }],
        };
        let mut reader = WasmReader::new(bytes.to_vec());
        let mut instance = instantiate(module, &mut reader);
        let expected_result = vec![Value::I32(10)];
        assert_eq!(instance.call(0, vec![]).unwrap(), &expected_result);
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

        assert_eq!(
            instance.call(0, vec![Value::I32(1)]).unwrap(),
            &vec![Value::I32(1)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(0)]).unwrap(),
            &vec![Value::I32(3)]
        );
    }
}
