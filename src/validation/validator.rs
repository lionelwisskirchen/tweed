use self::binary::{code_section::FuncCode, type_section::FuncType};
use self::core::values::*;
use crate::validation::control_types::ControlType;
use crate::validation::side_table::SideTableEntry;
use crate::*;

use super::side_table::SideTable;

fn shorthand_lookup(val: u32) -> Option<FuncType> {
    let lookup_map = HashMap::from([(
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
        println!("\n\n-----Starting Validation Algorithm-----\n\n");

        let mut abstract_control_stack = Vec::new();
        let mut abstract_value_stack = Vec::new();
        let mut side_table = SideTable::new();
        let mut locals = Vec::new();

        locals.append(&mut function.signature.parameter.clone());
        locals.append(&mut function.locals.clone());

        self.reader.seek(function.start);

        let signature = &function.signature;

        abstract_control_stack.push(ControlType::Func {
            instruction_pointer: self.reader.current_index(),
            func_type: signature.clone(),
        });
        loop {
            println!(
                "\n---- Abstract Value Stack----\n\n {:?}",
                abstract_value_stack
            );
            println!(
                "\n---- Abstract Control Stack----\n\n {:?} \n\n\n\n",
                abstract_control_stack
            );
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
                    let new_value = abstract_value_stack
                        .pop()
                        .expect("LocalSet needs value on stack");
                    let value = locals.get(idx as usize).expect("Invalid Local Index");
                    if *value != new_value {
                        panic!("Invalid type for localset");
                    }
                } // LocalSet
                0x6a => {
                    let first = abstract_value_stack.pop().expect("No left value on stack");
                    let second = abstract_value_stack.pop().expect("No right value on stack");
                    if first != ValType::I32 || second != ValType::I32 {
                        panic!("Invalid Type for i32add");
                    };
                    abstract_value_stack.push(ValType::I32);
                } // I32Add
                0x04 => validate_if(
                    self.reader,
                    &mut abstract_value_stack,
                    &mut abstract_control_stack,
                    self.typesec,
                    &mut side_table,
                ),
                0x05 => validate_else(
                    self.reader,
                    &mut abstract_control_stack,
                    &mut abstract_value_stack,
                    &mut side_table,
                ),
                0x48 => {
                    let first = abstract_value_stack.pop().expect("No value on stack");
                    let second = abstract_value_stack.pop().expect("No value on stack");
                    if first != ValType::I32 || second != ValType::I32 {
                        panic!("Invalid Type for i32 lt_s");
                    };
                    abstract_value_stack.push(ValType::I32);
                } // lt_s
                0x0f => {
                    let return_scope = abstract_control_stack
                        .last()
                        .expect("Return is not in valid scope");
                    match return_scope {
                        ControlType::For { func_type, .. }
                        | ControlType::Block { func_type, .. }
                        | ControlType::If { func_type, .. } => {
                            if let Some(func_sig) = func_type {
                                values_are_on_top_of_stack(
                                    &mut abstract_value_stack,
                                    &func_sig.result,
                                );
                                abstract_value_stack
                                    .drain(0..abstract_value_stack.len() - func_sig.result.len());
                            } else {
                                abstract_value_stack.drain(..);
                            }
                        }
                        ControlType::Func { func_type, .. } => {
                            values_are_on_top_of_stack(
                                &mut abstract_value_stack,
                                &func_type.result,
                            );
                            abstract_value_stack
                                .drain(0..abstract_value_stack.len() - func_type.result.len());
                        }
                    }
                } // return
                0x02 => validate_block(
                    self.reader,
                    &mut abstract_value_stack,
                    &mut abstract_control_stack,
                    self.typesec,
                ), // Block
                0x03 => validate_for(
                    self.reader,
                    &mut abstract_value_stack,
                    &mut abstract_control_stack,
                    self.typesec,
                    &mut side_table,
                ),
                0x0d => validate_br_if(
                    self.reader,
                    &mut abstract_value_stack,
                    &mut abstract_control_stack,
                    &mut side_table,
                ), // br_if
                0x0c => validate_br(
                    self.reader,
                    &mut abstract_value_stack,
                    &mut abstract_control_stack,
                    &mut side_table,
                ), // br
                0x0B => {
                    let control_type = abstract_control_stack
                        .pop()
                        .expect("End needs start control value");
                    match control_type {
                        ControlType::Func { func_type, .. } => {
                            let result_stack_types = zip(&func_type.result, &abstract_value_stack);
                            for (stack_type, result_type) in result_stack_types {
                                if stack_type != result_type {
                                    panic!("Invalid return type");
                                }
                            }
                            abstract_value_stack
                                .drain(0..abstract_value_stack.len() - func_type.result.len());
                            break;
                        }
                        ControlType::Block {
                            func_type,
                            sidetable_idxs,
                            ..
                        } => end_block(
                            self.reader,
                            &mut abstract_value_stack,
                            &mut side_table,
                            &sidetable_idxs,
                            func_type,
                        ),
                        ControlType::If {
                            func_type,
                            sidetable_idxs,
                            ..
                        } => end_if(
                            self.reader,
                            &mut abstract_value_stack,
                            &mut side_table,
                            &sidetable_idxs,
                            func_type,
                        ),
                        ControlType::For { func_type, .. } => {
                            end_for(&mut abstract_value_stack, func_type)
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
                    for expected_param in &sig.parameter {
                        let param_on_stack = abstract_value_stack
                            .pop()
                            .expect("Param expected but nothing on stack");
                        if *expected_param != param_on_stack {
                            panic!("Expected param does not match value on stack");
                        }
                    }
                    for ret_val in sig.result.iter() {
                        abstract_value_stack.push(*ret_val);
                    }
                } // Call
                _ => panic!("Undefined opcode {}", byte),
            };
        }
        println!(
            "\n---- Abstract Value Stack----\n\n {:?}",
            abstract_value_stack
        );
        println!(
            "\n---- Abstract Control Stack----\n\n {:?} \n\n\n\n",
            abstract_control_stack
        );
        println!("\n\n-----Finished Validation Algorithm-----\n\n");

        side_table
    }
}
fn end_if(
    reader: &mut WasmReader,
    abstract_value_stack: &mut [ValType],
    side_table: &mut SideTable,
    side_table_indices: &[usize],
    func_type: Option<FuncType>,
) {
    if let Some(func_type) = func_type {
        if func_type.result != *abstract_value_stack {
            panic!("Invalid return type");
        }
    } else if !abstract_value_stack.is_empty() {
        panic!("Too many return types");
    }

    let abs_target_pos = reader.current_index();
    for entry_index in side_table_indices.iter() {
        let sidetable_offset = side_table.entries.len() as i32 - *entry_index as i32;
        let entry = side_table
            .entries
            .get_mut(*entry_index)
            .expect("Sidetable entry does not exist.");
        let abs_start_pos = entry.ip;
        let rel_offset = abs_target_pos as i32 - abs_start_pos;

        entry.stp = sidetable_offset;
        entry.ip = rel_offset;
    }
}

fn end_for(abstract_value_stack: &mut [ValType], func_type: Option<FuncType>) {
    if let Some(func_type) = func_type {
        if func_type.result != *abstract_value_stack {
            panic!("Invalid return type");
        }
    } else if !abstract_value_stack.is_empty() {
        panic!("Too many return types");
    }
}

fn end_block(
    reader: &mut WasmReader,
    abstract_value_stack: &mut [ValType],
    side_table: &mut SideTable,
    side_table_indices: &[usize],
    func_type: Option<FuncType>,
) {
    if let Some(func_type) = func_type {
        if func_type.result != *abstract_value_stack {
            panic!("Invalid return type");
        }
    } else if !abstract_value_stack.is_empty() {
        panic!("Too many return types");
    }

    let abs_target_pos = reader.current_index();
    for entry_index in side_table_indices.iter() {
        let sidetable_offset = side_table.entries.len() as i32 - *entry_index as i32;
        let entry = side_table
            .entries
            .get_mut(*entry_index)
            .expect("Sidetable entry does not exist.");
        let abs_start_pos = entry.ip;
        let rel_offset = abs_target_pos as i32 - abs_start_pos;

        entry.stp = sidetable_offset;
        entry.ip = rel_offset;
    }
}
fn validate_else(
    reader: &mut WasmReader,
    abstract_control_stack: &mut [ControlType],
    abstract_value_stack: &mut Vec<ValType>,
    side_table: &mut SideTable,
) {
    let control_type = abstract_control_stack
        .last_mut()
        .expect("Else needs start control value");

    let abs_target_pos = reader.current_index() + 1; // We dont want to jump to else !

    let entry_idx = side_table.entries.len();

    side_table.entries.push(SideTableEntry {
        ip: reader.current_index() as i32,
        stp: 0,
        vals: 0,
        pop: 0,
    });
    match control_type {
        ControlType::If {
            func_type,
            sidetable_idxs,
            side_table_entry,
            ..
        } => {
            sidetable_idxs.push(entry_idx);
            let side_table_len = side_table.entries.len();

            if let Some(function_type) = func_type {
                // Reset like we didnt validated the if block
                abstract_value_stack.clone_from(&function_type.parameter)
            } else {
                *abstract_value_stack = vec![];
            }

            let entry = side_table
                .entries
                .get_mut(*side_table_entry)
                .expect("Sidetable entry for if not found");

            entry.ip = abs_target_pos as i32 - entry.ip;
            entry.stp = side_table_len as i32 - *side_table_entry as i32;
        }
        _ => {
            panic!("Else needs if as start control")
        }
    }
}

fn validate_if(
    reader: &mut WasmReader,
    abstract_value_stack: &mut Vec<ValType>,
    abstract_control_stack: &mut Vec<ControlType>,
    type_section: &[FuncType],
    side_table: &mut SideTable,
) {
    let start_index = reader.current_index();
    let func_idx = reader
        .next_leb128_u32()
        .expect("Error parsing block func idx");
    let signature = lookup_signature(func_idx, type_section);

    abstract_value_stack.pop(); // True False Value

    if let Some(sig) = &signature {
        if sig.parameter != *abstract_value_stack {
            panic!("Unexpected values on stack")
        }
    }
    side_table.entries.push(SideTableEntry {
        ip: start_index as i32,
        stp: 0,
        vals: 0,
        pop: 0,
    });

    let index_of_sidetable_entry = side_table.entries.len() - 1;

    abstract_control_stack.push(ControlType::If {
        instruction_pointer: reader.current_index(),
        func_type: signature,
        sidetable_idxs: Vec::new(),
        side_table_entry: index_of_sidetable_entry,
    });
}

fn validate_br_if(
    reader: &mut WasmReader,
    abstract_value_stack: &mut Vec<ValType>,
    abstract_control_stack: &mut [ControlType],
    side_table: &mut SideTable,
) {
    let bool_val = abstract_value_stack
        .pop()
        .expect("Br_if requires val on stack");
    if bool_val != ValType::I32 {
        panic!("Br_if requires I32 on top of stack")
    }
    validate_br(
        reader,
        abstract_value_stack,
        abstract_control_stack,
        side_table,
    );
}

fn validate_br(
    reader: &mut WasmReader,
    abstract_value_stack: &mut [ValType],
    abstract_control_stack: &mut [ControlType],
    side_table: &mut SideTable,
) {
    let instruction_index = reader.current_index() as i32;
    let target_control = get_target_control_structure(reader, abstract_control_stack); // Manipulates reader index!!
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
            side_table.entries.push(SideTableEntry {
                ip: instruction_index,
                stp: 0,
                vals: 0,
                pop: 0,
            });

            let index_of_sidetable_entry = side_table.entries.len().saturating_sub(1);

            if let Some(func_sig) = func_type {
                values_are_on_top_of_stack(abstract_value_stack, &func_sig.result);
            }
            sidetable_idxs.push(index_of_sidetable_entry);
        }
        ControlType::For {
            side_table_entry,
            func_type,
            instruction_pointer,
        } => {
            branch_to_for(
                func_type,
                abstract_value_stack,
                side_table,
                side_table_entry,
                instruction_pointer,
                reader,
            );
        }
        _ => {
            panic!("Branching target not allowed")
        }
    }
}

fn branch_to_for(
    func_type: &Option<FuncType>,
    abstract_value_stack: &mut [ValType],
    side_table: &mut SideTable,
    side_table_entry: &mut usize,
    instruction_pointer: &mut usize,
    reader: &mut WasmReader,
) {
    if let Some(func_sig) = func_type {
        values_are_on_top_of_stack(abstract_value_stack, &func_sig.result);
    }
    let number_of_side_table_entries = side_table.entries.len() as i32;
    let sidetable_offset = *side_table_entry as i32 - number_of_side_table_entries;
    let instruction_offset = *instruction_pointer as i32 - reader.current_index() as i32;
    println!(
        "Sidetable Offset: {}\n Instruction Offset: {}",
        sidetable_offset, instruction_offset
    );

    let entry = SideTableEntry {
        stp: sidetable_offset - 1,
        ip: instruction_offset,
        vals: 0,
        pop: 0,
    };
    side_table.entries.push(entry);
}

fn get_target_control_structure<'a>(
    reader: &mut WasmReader,
    abstract_control_stack: &'a mut [ControlType],
) -> &'a mut ControlType {
    let depth = reader.next_leb128_u32().expect("Error parsing break depth");

    if depth as usize > abstract_control_stack.len() - 1 {
        panic!("Depth exceeds control depth.")
    }
    let control_stack_index_size = abstract_control_stack.len() - 1;
    let start_control = abstract_control_stack
        .get_mut(control_stack_index_size - depth as usize)
        .expect("Can't index control start");
    start_control
}

fn validate_for(
    reader: &mut WasmReader,
    abstract_value_stack: &mut Vec<ValType>,
    abstract_control_stack: &mut Vec<ControlType>,
    type_section: &[FuncType],
    side_table: &mut SideTable,
) {
    let func_idx = reader
        .next_leb128_u32()
        .expect("Error parsing block func idx");
    let signature = lookup_signature(func_idx, type_section);
    if let Some(sig) = &signature {
        if sig.parameter != *abstract_value_stack {
            panic!("Unexpected values on stack")
        }
    }
    abstract_control_stack.push(ControlType::For {
        instruction_pointer: reader.current_index(),
        func_type: signature,
        side_table_entry: side_table.entries.len(),
    });
}

fn validate_block(
    reader: &mut WasmReader,
    abstract_value_stack: &mut Vec<ValType>,
    abstract_control_stack: &mut Vec<ControlType>,
    type_section: &[FuncType],
) {
    let func_idx = reader
        .next_leb128_u32()
        .expect("Error parsing block func idx");
    let signature = lookup_signature(func_idx, type_section);
    if let Some(sig) = &signature {
        if sig.parameter != *abstract_value_stack {
            panic!("Unexpected values on stack")
        }
    }
    abstract_control_stack.push(ControlType::Block {
        instruction_pointer: reader.current_index(),
        func_type: signature,
        sidetable_idxs: Vec::new(),
    });
}
fn values_are_on_top_of_stack(abstract_value_stack: &mut [ValType], values: &[ValType]) {
    let actual_values = abstract_value_stack
        .get(abstract_value_stack.len() - values.len()..abstract_value_stack.len())
        .expect("Params are not compatible to if signature");
    if values != actual_values {
        panic!("Params are not compatible to if signature");
    }
}
fn lookup_signature(func_idx: u32, type_section: &[FuncType]) -> Option<FuncType> {
    if func_idx == 0x40 {
        return None;
    }
    if let Some(signature) = shorthand_lookup(func_idx) {
        return Some(signature);
    };
    Some(
        type_section
            .get(func_idx as usize)
            .expect("No func idx for sig")
            .clone(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;
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
