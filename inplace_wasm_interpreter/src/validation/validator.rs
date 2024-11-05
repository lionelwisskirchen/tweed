use self::binary::{code_section::FuncCode, type_section::FuncType};
use self::core::values::*;
use crate::validation::control_types::ControlType;
use crate::*;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use validation::control_stack::ControlStack;
use validation::validation_stack::{self, ValidationStack};

use super::side_table::SideTable;

fn shorthand_lookup(val: u32) -> Option<FuncType> {
    let lookup_map = BTreeMap::from([(
        0x7f,
        FuncType {
            parameter: Vec::new(),
            result: vec![ValType::I32],
        },
    )]);
    lookup_map.get(&val).cloned()
}

pub struct Validator<'a, 'b> {
    reader: &'a mut WasmReader,
    typesec: &'b Vec<FuncType>,
}

impl<'a, 'b> Validator<'a, 'b> {
    pub fn new(reader: &'a mut WasmReader, typesec: &'b Vec<FuncType>) -> Self {
        Validator { reader, typesec }
    }

    pub fn validate(&mut self, function: &FuncCode) -> SideTable {
        #[cfg(test)]
        println!("\n\n-----Starting Validation Algorithm-----\n\n");

        let mut abstract_control_stack = ControlStack::new();
        let mut abstract_value_stack = ValidationStack::new();
        let mut side_table = SideTable::new();
        let mut locals = Vec::new();

        locals.append(&mut function.signature.parameter.clone());
        locals.append(&mut function.locals.clone());

        self.reader.seek(function.start);

        let signature = &function.signature;

        abstract_control_stack.push(ControlType::Func {
            instruction_pointer: self.reader.current_index(),
            func_type: signature.clone(),
            reachable: true,
        });
        loop {
            let byte = self.reader.next_byte().expect("Missing End statement");
            match byte {
                0x41 => {
                    // I32Const
                    self.reader
                        .next_leb128_i32()
                        .expect("Could not read i32 const");
                    abstract_value_stack.push(ValType::I32);
                }
                0x20 => {
                    let idx = self.reader.next_leb128_u32().expect("Error parsing indx");
                    let value = locals.get(idx as usize).expect("Invalid Local Index");
                    abstract_value_stack.push(*value);
                } // LocalGet
                0x21 => {
                    let idx = self.reader.next_leb128_u32().expect("Error parsing indx");
                    let value = locals.get(idx as usize).expect("Invalid Local Index");

                    abstract_value_stack
                        .pop_expected(*value, abstract_control_stack.top_is_reachable());
                } // LocalSet
                0x6a => {
                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable());
                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable());
                    abstract_value_stack.push(ValType::I32);
                } // I32Add
                0x04 => {
                    let index_of_if_bytecode = self.reader.current_index() as i32;
                    let signature = read_block_type(self.reader, self.typesec);
                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable()); // bool value
                    abstract_value_stack.pop_expected_vec(
                        &signature.parameter,
                        abstract_control_stack.top_is_reachable(),
                    );

                    let index_of_if_sidetable_entry =
                        side_table.create_entry(index_of_if_bytecode, 0, 0, 0);

                    abstract_control_stack.push(ControlType::If {
                        instruction_pointer: self.reader.current_index(),
                        func_type: signature.clone(),
                        sidetable_idxs: Vec::new(),
                        side_table_entry: index_of_if_sidetable_entry,
                        reachable: true,
                        height_of_value_stack: abstract_value_stack.height(),
                    });

                    abstract_value_stack.push_vec(&signature.parameter);
                } // If,
                0x05 => {
                    let control_type_is_reachable = abstract_control_stack.top_is_reachable();
                    let control_type = abstract_control_stack.top();
                    let index_of_else_sidetable_entry =
                        side_table.create_entry(self.reader.current_index() as i32, 0, 0, 0);

                    match control_type {
                        ControlType::If {
                            func_type,
                            sidetable_idxs,
                            side_table_entry,
                            height_of_value_stack,
                            reachable,
                            ..
                        } => {
                            sidetable_idxs.push(index_of_else_sidetable_entry);
                            let expected_results = func_type.result.clone();
                            let count = abstract_value_stack.height() - *height_of_value_stack;

                            #[cfg(test)]
                            println!("Reachable? {:?}", reachable);
                            // It's allowed to have too less result values on the stack
                            // if the block is marked as unreachable
                            if (count != expected_results.len() as u32 && *reachable) {
                                panic!("Arity Mismatch.");
                            }

                            abstract_value_stack
                                .pop_expected_vec(&expected_results, control_type_is_reachable);
                            let side_table_len = side_table.len();
                            let entry = side_table.get_entry(*side_table_entry);
                            // branch from "if" will go to instruction after else
                            entry.ip = (self.reader.current_index() + 1) as i32 - entry.ip;
                            // branch if will skip all sidetable entries from if to else
                            entry.stp = side_table_len as i32 - *side_table_entry as i32;

                            // Reset value stack
                            abstract_value_stack.set_top(*height_of_value_stack);
                            abstract_value_stack.push_vec(&func_type.parameter);
                        }
                        _ => {
                            panic!("Else needs if as start control")
                        }
                    }
                } // Else,
                0x48 => {
                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable());
                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable());
                    abstract_value_stack.push(ValType::I32);
                } // lt_s
                0x0f => {
                    let control_type_is_reachable = abstract_control_stack.top_is_reachable();
                    let control_type = abstract_control_stack.top();
                    match control_type {
                        ControlType::For {
                            func_type,
                            reachable,
                            ..
                        }
                        | ControlType::Block {
                            func_type,
                            reachable,
                            ..
                        }
                        | ControlType::If {
                            func_type,
                            reachable,
                            ..
                        }
                        | ControlType::Func {
                            func_type,
                            reachable,
                            ..
                        } => {
                            abstract_value_stack
                                .pop_expected_vec(&func_type.result, control_type_is_reachable);
                            *reachable = false;
                        }
                    }
                } // return
                0x02 => {
                    let signature = read_block_type(self.reader, self.typesec);
                    abstract_value_stack.pop_expected_vec(
                        &signature.parameter,
                        abstract_control_stack.top_is_reachable(),
                    );

                    abstract_control_stack.push(ControlType::Block {
                        instruction_pointer: self.reader.current_index(),
                        func_type: signature.clone(),
                        sidetable_idxs: Vec::new(),
                        reachable: true,
                        height_of_value_stack: abstract_value_stack.height(),
                    });
                    abstract_value_stack.push_vec(&signature.parameter);
                } // Block
                0x03 => {
                    let signature = read_block_type(self.reader, self.typesec);
                    abstract_value_stack.pop_expected_vec(
                        &signature.parameter,
                        abstract_control_stack.top_is_reachable(),
                    );

                    abstract_control_stack.push(ControlType::For {
                        instruction_pointer: self.reader.current_index(),
                        func_type: signature.clone(),
                        side_table_entry: side_table.len(),
                        reachable: true,
                        height_of_value_stack: abstract_value_stack.height(),
                    });
                    abstract_value_stack.push_vec(&signature.parameter);
                } // Loop
                0x0d => {
                    let index_of_br_if_bytecode = self.reader.current_index() as i32;

                    abstract_value_stack
                        .pop_expected(ValType::I32, abstract_control_stack.top_is_reachable()); // bool
                    let depth = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing break depth");

                    let control_type_is_reachable = abstract_control_stack.top_is_reachable();
                    let target_control = abstract_control_stack.get_control_type(depth);
                    let index_of_br_sidetable_entry =
                        side_table.create_entry(index_of_br_if_bytecode, 0, 0, 0);
                    match target_control {
                        ControlType::Block {
                            sidetable_idxs,
                            func_type,
                            ..
                        }
                        | ControlType::If {
                            sidetable_idxs,
                            func_type,
                            ..
                        } => {
                            sidetable_idxs.push(index_of_br_sidetable_entry);
                            abstract_value_stack
                                .pop_expected_vec(&func_type.result, control_type_is_reachable);
                            abstract_value_stack.push_vec(&func_type.result);
                        }
                        ControlType::For {
                            func_type,
                            side_table_entry,
                            instruction_pointer,
                            ..
                        } => {
                            abstract_value_stack
                                .pop_expected_vec(&func_type.parameter, control_type_is_reachable);
                            abstract_value_stack.push_vec(&func_type.parameter); // Really parameter?
                            let stp = *side_table_entry as i32 - side_table.len() as i32;
                            let ip =
                                *instruction_pointer as i32 - self.reader.current_index() as i32;
                            let entry = side_table.get_entry(index_of_br_sidetable_entry);
                            entry.ip = ip;
                            entry.stp = stp;
                        }
                        _ => {
                            panic!("Branching target not allowed")
                        }
                    };
                } // br_if
                0x0c => {
                    let index_of_br_bytecode = self.reader.current_index() as i32;
                    let depth = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing break depth");
                    let target_control = abstract_control_stack.get_control_type(depth);
                    let index_of_br_sidetable_entry =
                        side_table.create_entry(index_of_br_bytecode, 0, 0, 0);
                    match target_control {
                        ControlType::Block {
                            sidetable_idxs,
                            func_type,
                            reachable,
                            height_of_value_stack,
                            ..
                        }
                        | ControlType::If {
                            sidetable_idxs,
                            func_type,
                            reachable,
                            height_of_value_stack,
                            ..
                        } => {
                            sidetable_idxs.push(index_of_br_sidetable_entry);
                            abstract_value_stack.pop_expected_vec(&func_type.result, *reachable);

			    let entry = side_table.get_entry(index_of_br_sidetable_entry);
                            entry.pop = (abstract_value_stack.height() - *height_of_value_stack) as usize;
                            entry.vals = func_type.result.len();
			    
                            abstract_value_stack.set_top(*height_of_value_stack);
                            abstract_control_stack.set_unreachable();
                        }
                        ControlType::For {
                            func_type,
                            reachable,
                            side_table_entry,
                            instruction_pointer,
                            height_of_value_stack,
                        } => {
                            abstract_value_stack.pop_expected_vec(&func_type.parameter, *reachable);

                            let stp = *side_table_entry as i32 - side_table.len() as i32;
                            let ip =
                                *instruction_pointer as i32 - self.reader.current_index() as i32;
                            let entry = side_table.get_entry(*side_table_entry);
                            entry.ip = ip;
                            entry.stp = stp;
                            abstract_value_stack.set_top(*height_of_value_stack);
                            abstract_control_stack.set_unreachable();
                        }
                        _ => {
                            panic!("Branching target not allowed")
                        }
                    };
                } // br
                0x0B => {
                    let control_type = abstract_control_stack.pop();

                    match control_type {
                        ControlType::Func {
                            func_type,
                            reachable,
                            ..
                        } => {
                            let expected_results = func_type.result;
                            let count = abstract_value_stack.height();

                            // It's allowed to have too less result values on the stack
                            // if the block is marked as unreachable
                            if count != expected_results.len() as u32 && reachable {
                                panic!("Arity Mismatch.");
                            }

                            abstract_value_stack.pop_expected_vec(&expected_results, reachable);
                            abstract_value_stack.push_vec(&expected_results);
                            break;
                        }
                        ControlType::Block {
                            func_type,
                            sidetable_idxs,
                            height_of_value_stack,
                            reachable,
                            ..
                        } => {
                            let expected_results = func_type.result;
                            let count = abstract_value_stack
                                .height()
                                .saturating_sub(height_of_value_stack);

                            // It's allowed to have too less result values on the stack
                            // if the block is marked as unreachable
                            if count != expected_results.len() as u32 && reachable {
                                panic!("Arity Mismatch.");
                            }
			    	
                            abstract_value_stack.pop_expected_vec(&expected_results, reachable);
			    
                            let abs_target_pos = self.reader.current_index();
                            for entry_index in sidetable_idxs.iter() {
                                let sidetable_offset =
                                    side_table.len() as i32 - *entry_index as i32;
                                let entry = side_table.get_entry(*entry_index);
                                let abs_start_pos = entry.ip;
                                let rel_offset = abs_target_pos as i32 - abs_start_pos;
			
			
                                entry.stp = sidetable_offset;
                                entry.ip = rel_offset;
                            }

                            // Reset value stack
                            abstract_value_stack.set_top(height_of_value_stack);
                            abstract_value_stack.push_vec(&expected_results);
                        }
                        ControlType::If {
                            func_type,
                            sidetable_idxs,
                            reachable,
                            height_of_value_stack,
                            ..
                        } => {
                            let expected_results = func_type.result;
                            let count = abstract_value_stack.height() - height_of_value_stack;

                            // It's allowed to have too less result values on the stack
                            // if the block is marked as unreachable
                            if count != expected_results.len() as u32 && reachable {
                                panic!("Arity Mismatch.");
                            }

                            abstract_value_stack.pop_expected_vec(&expected_results, reachable);

                            let abs_target_pos = self.reader.current_index();
                            for entry_index in sidetable_idxs.iter() {
                                let sidetable_offset =
                                    side_table.len() as i32 - *entry_index as i32;
                                let entry = side_table.get_entry(*entry_index);
                                let abs_start_pos = entry.ip;
                                let rel_offset = abs_target_pos as i32 - abs_start_pos;
                                entry.stp = sidetable_offset;
                                entry.ip = rel_offset;
                            }

                            // Reset value stack
                            abstract_value_stack.set_top(height_of_value_stack);
                            abstract_value_stack.push_vec(&expected_results);
                        }
                        ControlType::For {
                            func_type,
                            reachable,
                            height_of_value_stack,
                            ..
                        } => {
                            let expected_results = func_type.result;
                            let count = abstract_value_stack.height() - height_of_value_stack;

                            // It's allowed to have too less result values on the stack
                            // if the block is marked as unreachable
                            if count != expected_results.len() as u32 && reachable {
                                panic!("Arity Mismatch.");
                            }

                            abstract_value_stack.pop_expected_vec(
                                &expected_results,
                                abstract_control_stack.top_is_reachable(),
                            );

                            // Reset value stack
                            abstract_value_stack.set_top(height_of_value_stack);
                            abstract_value_stack.push_vec(&expected_results);
                        }
                    }
                } // END
                0x10 => {
                    let func_index = self
                        .reader
                        .next_leb128_u32()
                        .expect("Called func index is invalid");
                    let sig = self
                        .typesec
                        .get(func_index as usize)
                        .expect("Called func sig does not exist");
                    abstract_value_stack.pop_expected_vec(
                        &sig.parameter,
                        abstract_control_stack.top_is_reachable(),
                    );
                    abstract_value_stack.push_vec(&sig.result);
                } // Call
                _ => panic!("Undefined opcode {}", byte),
            };
        }
        // #[cfg(test)]
        // println!(
        //     "\n---- Abstract Value Stack----\n\n {:?}",
        //     abstract_value_stack
        // );
        // #[cfg(test)]
        // println!(
        //     "\n---- Abstract Control Stack----\n\n {:?} \n\n\n\n",
        //     abstract_control_stack
        // );
        // #[cfg(test)]
        // println!("\n\n-----Finished Validation Algorithm-----\n\n");

        side_table
    }
}

fn read_block_type(reader: &mut WasmReader, type_section: &[FuncType]) -> FuncType {
    let func_idx = reader
        .next_leb128_u32()
        .expect("Error parsing block func idx");
    lookup_signature(func_idx, type_section)
}
fn lookup_signature(func_idx: u32, type_section: &[FuncType]) -> FuncType {
    if func_idx == 0x40 {
        return FuncType {
            ..Default::default()
        };
    };
    if let Some(signature) = shorthand_lookup(func_idx) {
        return signature;
    };
    type_section
        .get(func_idx as usize)
        .expect("No func idx for sig")
        .clone()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::validation::side_table::SideTableEntry;
    #[test]
    fn test_validation_without_control_structs() {
        let bytes = [
            0x12, 0x01, 0x10, // Number of Function Bodies (1), Body Size (16),
            0x01, 0x01, 0x7f, // Number of local declarations (1), 1 x I32
            0x20, 0x00, 0x20, 0x01, // Local.get(Index 0), Local.get(Index 1)
            0x6a, // I32Add
            0x21, 0x00, // Local.set(Index 0)
            0x20, 0x00, 0x20, 0x01, // Local.get(Index 0), Local.get(Index 1)
            0x6a, // I32Add
            0x0b, // Terminal
        ];
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![ValType::I32, ValType::I32],
            result: vec![ValType::I32],
        }];
        let code = FuncCode {
            signature: FuncType {
                parameter: vec![ValType::I32, ValType::I32],
                result: vec![ValType::I32],
            },
            locals: vec![ValType::I32],
            start: 6,
            end: 18,
        };

        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable::new();
        assert_eq!(actual_sidetable, expected_sidetable);
    }

    #[test]
    fn test_validation_with_blocks() {
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
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![
            FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
        ];
        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            locals: vec![],
            start: 29,
            end: 40,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable::new();
        assert_eq!(actual_sidetable, expected_sidetable);
    }

    #[test]
    fn test_validation_with_branches() {
        let bytes = wat::parse_str(
            "(module
                (func
                  (block $label1
                     br $label1
 	            (block $label2
		       (block $label3
		           br $label1
		        )
      		       (block $label4
		           br $label2
		        )
		    )
	          )
                )
             )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];
        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![],
            },
            locals: vec![],
            start: 23,
            end: 41,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 15,
                    stp: 3,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 9,
                    stp: 2,
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
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }
    #[test]
    fn test_validation_with_if_else() {
        let bytes = wat::parse_str(
            "(module
              (func (result i32)
                i32.const 0
                (if (result i32)
                 (then
                  i32.const 1
                 )
                (else
                 i32.const 0
                )
              )
             )
            )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];
        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            locals: vec![],
            start: 24,
            end: 34,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 5,
                    stp: 2,
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
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }
    #[test]
    fn test_validation_with_if_else_and_branches() {
        let bytes = wat::parse_str(
            "
(module
  (func (result i32)
    (block $label1 (result i32)
      i32.const 0
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
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];
        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            locals: vec![],
            start: 24,
            end: 34,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
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
                    vals: 1,
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
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }

    #[test]
    fn test_validation_with_if_else_and_conditional_branches() {
        let bytes = wat::parse_str(
            "
(module
  (func)
  (func (param i32) (result i32)
    (block $label1 (result i32)
      (local.get 0)
      (if (result i32)
        (then
          (call 0)
          i32.const 1
        )
        (else
          (call 0)
          i32.const 0
        )
      )
      i32.const 2
      (br_if 0)
      i32.const 3
      return
    )
  )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![
            FuncType {
                parameter: vec![],
                result: vec![],
            },
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
        ];

        let code = FuncCode {
            signature: FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
            locals: vec![],
            start: 32,
            end: 49,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 7,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }

    #[test]
    fn test_validation_with_for_loop() {
        let bytes = wat::parse_str(
            "
(module

(func
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
 )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];

        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![],
            },
            locals: vec![ValType::I32],
            start: 25,
            end: 42,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
            entries: vec![SideTableEntry {
                ip: -14,
                stp: -1,
                vals: 0,
                pop: 0,
            }],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }

    #[test]
    fn test_validation_with_for_loop_and_blocks_with_branches() {
        let bytes = wat::parse_str(
            "
(module
(func
      (local $i i32)
      (loop $my_loop
      local.get $i
      i32.const 1
      i32.add
      local.set $i
       (block $label1 (result i32)
      i32.const 0
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
      i32.const 10
      i32.lt_s
      br_if $my_loop
    )
 )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];

        let code = FuncCode {
            signature: FuncType {
                parameter: vec![],
                result: vec![],
            },
            locals: vec![ValType::I32],
            start: 25,
            end: 42,
        };
        let mut validator = Validator::new(&mut reader, &typesec);
        let actual_sidetable = validator.validate(&code);
        let expected_sidetable = SideTable {
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
                    vals: 1,
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
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: -31,
                    stp: -6,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
    }
}
