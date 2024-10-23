use std::iter::zip;

struct WasmReader<'a> {
    bytecode: &'a [u8],
    byte_pointer: usize,
}

impl<'a> WasmReader<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            bytecode: input,
            byte_pointer: 0,
        }
    }
    pub fn next(&mut self) -> Option<&u8> {
        let curr = self.bytecode.get(self.byte_pointer);
        self.byte_pointer += 1;
        curr
    }
    pub fn seek(&mut self, position: usize) {
        self.byte_pointer = position
    }
    pub fn curr(&self) -> usize {
        self.byte_pointer.saturating_sub(1)
    }
    pub fn next_leb128_u32(&mut self) -> Option<u32> {
        let mut result: u32 = 0;
        let mut shift = 0;
        while let Some(byte) = self.next() {
            let low = (byte & 0x7f) as u32; // 0111 1111 Mask -> MSB signals next byte is also relevant

            result |= low << shift;
            if (byte & 0x80) == 0 {
                // No next byte, Algorithm is finished
                break;
            }
            shift += 7;
            if shift > 32 {
                return None;
            }
        }
        Some(result)
    }
    pub fn next_leb128_i32(&mut self) -> Option<i32> {
        let mut result: i32 = 0;
        let mut shift = 0;
        while let Some(byte) = self.next() {
            let low = (byte & 0x7f) as i32; // 0111 1111 Mask -> MSB signals next byte is also relevant

            result |= low << shift;
            if (byte & 0x80) == 0 {
                // No next byte, Algorithm is finished
                break;
            }
            shift += 7;
            if shift > 32 {
                return None;
            }
        }
        Some(result)
    }
    pub fn calculate_offset(&self, offset: usize) -> usize {
        self.byte_pointer + offset
    }
    pub fn next_chunk(&mut self, number_of_bytes: usize) -> Option<&'a [u8]> {
        let chunk = self
            .bytecode
            .get(self.byte_pointer..self.byte_pointer + number_of_bytes);
        self.byte_pointer += number_of_bytes;
        chunk
    }
    pub fn get_slice(&self, start: usize, end: usize) -> Option<&'a [u8]> {
        self.bytecode.get(start..end)
    }
    pub fn compare_chunk(&mut self, number_of_bytes: usize, expected: &[u8]) -> Option<&'a [u8]> {
        let slice = self.next_chunk(number_of_bytes)?;
        if slice == expected {
            return Some(slice);
        }
        None
    }
    pub fn rest(&self) -> &[u8] {
        &self.bytecode[self.byte_pointer..]
    }
}

struct Validator<'a, 'c> {
    abstract_value_stack: Vec<ValType>,
    abstract_control_stack: Vec<ControlType<'a>>,
    locals: Vec<ValType>,
    signature: &'a FuncType,
    reader: WasmReader<'c>,
    side_table: SideTable,
    func_signatures: &'a Vec<FuncType>,
}

impl<'a, 'c> Validator<'a, 'c> {
    pub fn new(
        locals: &Vec<ValType>,
        signature: &'a FuncType,
        instructions: &'c [u8],
        func_signatures: &'a Vec<FuncType>,
    ) -> Self {
        let params = signature.parameter.iter().cloned();
        let locals = locals.iter().cloned();

        Self {
            locals: params.chain(locals).collect::<Vec<ValType>>(),
            abstract_control_stack: Vec::new(),
            abstract_value_stack: Vec::new(),
            signature,
            reader: WasmReader::new(instructions),
            side_table: SideTable {
                entries: Vec::new(),
            },
            func_signatures,
        }
    }
    pub fn validate(&mut self) -> Option<SideTable> {
        self.abstract_control_stack.push(ControlType::Func {
            instruction_pointer: self.reader.curr(),
            func_type: self.signature
        });
        println!("Value Stack: {:?}", self.abstract_value_stack);
        println!("Control Stack: {:?}", self.abstract_control_stack);
        println!("Reader: {:?}", self.reader.bytecode);
        loop {
            let byte = self.reader.next().expect("Missing End statement");
            println!("Byte: {}", byte);
            match byte {
                0x41 => {
                    // I32Const
                    self.reader
                        .next_leb128_i32()
                        .expect("Could not read i32 const");
                    self.abstract_value_stack.push(ValType::I32);
                }
                0x42 => {} // I64Const

                // $\frac{C.locals[x] = t}{C  \vdash i32.add: [i32, i32] \rightarrow ]i32] }$
                0x20 => {
                    let idx = self.reader.next_leb128_u32().expect("Error parsing indx");
                    let value = self.locals.get(idx as usize).expect("Invalid Local Index");
                    self.abstract_value_stack.push(*value);
                } // LocalGet
                0x21 => {} // LocalSet
                0x6a => {
                    let first = self.abstract_value_stack.pop().expect("No value on stack");
                    let second = self.abstract_value_stack.pop().expect("No value on stack");
                    if first != ValType::I32 || second != ValType::I32 {
                        panic!("Invalid Type for i32add");
                    };
                    self.abstract_value_stack.push(ValType::I32);
                } // I32Add
                0x0c => {
                    let depth = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing break depth");
                    println!("Break depth: {}", depth);
                    let stack_idx_length = self.abstract_control_stack.len() - 1;
                    if depth as usize > stack_idx_length {
                        panic!("Depth exceeds control depth.")
                    }
                    let start_control = self
                        .abstract_control_stack
                        .get_mut(stack_idx_length - depth as usize)
                        .expect("Can't index control start");

                    let entry_idx = self.side_table.entries.len();
                    self.side_table.entries.push(SideTableEntry {
                        ip: self.reader.curr() as i32 - 1, // -1 Because of the depth
                        vals: 0,
                        pop: 0,
                    });

                    match start_control {
                        ControlType::Block { sidetable_idxs, .. } => {
                            sidetable_idxs.push(entry_idx);
                        }
                        ControlType::If { sidetable_idxs, .. } => {
                            sidetable_idxs.push(entry_idx);
                        }
			_ => {panic!("Branching target not allowed")}
                    }
                } // br
                0x7c => {} // I64Add

                0x02 => {
                    println!("Block start");
                    let func_idx = self
                        .reader
                        .next_leb128_u32()
                        .expect("Error parsing block func idx");
                    let signature = if func_idx == 0x40 {
                        // Signals no signature
                        None
                    } else {
                        Some(
                            self.func_signatures
                                .get(func_idx as usize)
                                .expect("No func idx for sig"),
                        )
                    };
		    
		    
                    self.abstract_control_stack.push(ControlType::Block {
                        instruction_pointer: self.reader.curr(),
                        func_type: signature,
                        sidetable_idxs: Vec::new(),
                    });
                }
                0x0B => {
                    let control_type = self
                        .abstract_control_stack
                        .pop()
                        .expect("End needs start control value");
                    match control_type {
                        ControlType::Func {
                            instruction_pointer,
                            func_type
                        } => {
                            // $\frac{C \vdash \mathit{instr}^\ast: [] \rightarrow [t'^\ast] \qquad \vdash [t'^\ast] \leq [t^\ast]}{C \vdash \mathit{instr}^\ast \mathsf{end}: [t^\ast]}$
                            let result_stack_types =
                                zip(&func_type.result, &self.abstract_value_stack);
                            for (stack_type, result_type) in result_stack_types {
                                if stack_type != result_type {
                                    panic!("Invalid return type");
                                }
                            }
                            self.abstract_value_stack
                                .drain(0..self.abstract_value_stack.len() - func_type.result.len());
                            break;
                        }
                        ControlType::Block {
                            instruction_pointer,
                            func_type,
                            sidetable_idxs,
                        } => {
                            let abs_target_pos = self.reader.curr();
                            for entry_index in sidetable_idxs {
                                let entry = self
                                    .side_table
                                    .entries
                                    .get_mut(entry_index)
                                    .expect("Sidetable entry does not exist.");
                                let abs_start_pos = entry.ip;
                                let mut rel_offset = abs_target_pos as i32 - abs_start_pos;
                                rel_offset += 1; // We don't want to index the end instr, but rather the following isntruction
                                println!("Absolute position of end: {abs_target_pos}");
                                println!("Absolute position of start: {abs_start_pos}");
                                entry.ip = rel_offset;
                            }
                        }
                        ControlType::If {
                            instruction_pointer,
                            func_type,
                            sidetable_idxs,
                        } => {}
                    }
                } // END
                0x10 => {
                    let func_index = self
                        .reader
                        .next_leb128_u32()
                        .expect("Called func index is invalid");
                    let sig = self
                        .func_signatures
                        .get(func_index as usize)
                        .expect("Called func sig does not exist");
                    for expected_param in &sig.parameter {
                        let param_on_stack = self
                            .abstract_value_stack
                            .pop()
                            .expect("Param expected but nothing on stack");
                        if *expected_param != param_on_stack {
                            panic!("Expected param does not match value on stack");
                        }
                    }
                    for ret_val in sig.result.iter() {
                        self.abstract_value_stack.push(*ret_val);
                    }
                } // Call
                _ => panic!("Undefined opcode {}", byte),
            };
            println!("Value Stack:{:?}", self.abstract_value_stack);
            println!("Control Stack: {:?}", self.abstract_control_stack);
        }
        println!("Value Stack:{:?}", self.abstract_value_stack);
        println!("Control Stack: {:?}", self.abstract_control_stack);
        Some(self.side_table.clone())
    }
}

#[derive(Debug)]
struct FuncDecl {
    index: u32,
    sig_index: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValType {
    I32,
    I64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum ControlType<'a> {
    Func {
        instruction_pointer: usize,
        func_type: &'a FuncType
    },
    Block {
        instruction_pointer: usize,
        func_type: Option<&'a FuncType>,
        sidetable_idxs: Vec<usize>,
    },
    If {
        instruction_pointer: usize,
        func_type: Option<&'a FuncType>,
        sidetable_idxs: Vec<usize>,
    },
}

impl ValType {
    pub fn lookup(val: &u8) -> ValType {
        match val {
            0x7F => ValType::I32,
            0x7E => ValType::I64,
            _ => panic!("Unknown value type {}", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FuncType {
    parameter: Vec<ValType>,
    result: Vec<ValType>,
}

#[derive(Debug, Default)]
struct Module {
    typesec: Vec<FuncType>,
    funcsec: Vec<FuncDecl>,
}

#[derive(Debug, Clone)]
struct SideTable {
    entries: Vec<SideTableEntry>,
}

#[derive(Debug, Clone)]
struct SideTableEntry {
    ip: i32,
    vals: usize,
    pop: usize,
}

#[derive(Debug)]
struct Function<'a> {
    locals: Vec<ValType>,
    signature: &'a FuncType,
    start: usize,
    end: usize,
    side_table: SideTable,
}

fn retrieve_section_content<'a>(reader: &'a mut WasmReader) -> Option<&'a [u8]> {
    let section_size = reader.next_leb128_u32().expect("No Section size found");
    reader.next_chunk(section_size as usize)
}

fn run(input: &[u8]) {
    let mut reader = WasmReader::new(input);
    let mut module = Module {
        ..Default::default()
    };

    reader
        .compare_chunk(4, &[0x00, 0x61, 0x73, 0x6D])
        .expect("No preamble found");
    reader
        .compare_chunk(4, &[0x01, 0x00, 0x00, 0x00])
        .expect("No version number found");

    loop {
        if let Some(section_code) = reader.next() {
            println!("Section Code: {section_code:?}");
            match section_code {
                0x00 => break,
                0x01 => {
                    let ssize = reader.next_leb128_u32().expect("No Section size found");
                    // Size is not required for decoding, so just check if size is present
                    println!("Section size: {ssize}");
                    let mut function_types = Vec::new();
                    let number_of_types = reader
                        .next_leb128_u32()
                        .expect("No type defined in type section");
                    println!("Number of types: {number_of_types:?}");

                    for _ in 0..number_of_types {
                        reader
                            .next_leb128_u32()
                            .expect("Could not parse funct type id"); // Can be ignored
                        let number_of_params = reader
                            .next_leb128_u32()
                            .expect("No number of params defined");
                        println!("Number of params: {number_of_params:?}");
                        let params = reader
                            .next_chunk(number_of_params as usize)
                            .expect("Can't get defined amount of params")
                            .iter()
                            .map(ValType::lookup)
                            .collect();
                        let number_of_results = reader
                            .next_leb128_u32()
                            .expect("No number of results defined");
                        println!("Number of results: {number_of_results:?}");
                        let results = reader
                            .next_chunk(number_of_results as usize)
                            .expect("Can't get defined amount of results")
                            .iter()
                            .map(ValType::lookup)
                            .collect();
                        function_types.push(FuncType {
                            parameter: params,
                            result: results,
                        });
                    }
                    module.typesec = function_types;
                }
                0x03 => {
                    reader.next_leb128_u32().expect("No Section size found");
                    // Size is not required for decoding, so just check if size is present
                    let number_of_functions = reader
                        .next_leb128_u32()
                        .expect("No function defined in Function section");

                    let mut function_decls = Vec::new();
                    println!("{number_of_functions:?}");
                    for i in 0..number_of_functions {
                        let decl = FuncDecl {
                            index: i,
                            sig_index: reader.next_leb128_u32().expect("No sig index given"),
                        };
                        function_decls.push(decl);
                    }
                    module.funcsec = function_decls;
                }
                0x0a => {
                    reader.next_leb128_u32().expect("No Section size found");
                    // Size is not required for decoding, so just check if size is present
                    let number_of_codes = reader
                        .next_leb128_u32()
                        .expect("No function defined in Code section");

                    for index in 0..number_of_codes {
                        let func_body_size = reader
                            .next_leb128_u32()
                            .expect("No function body size in Code section");
                        let end_pos = reader.calculate_offset(func_body_size as usize);
                        println!("End position {}", end_pos);
                        let number_of_locals = reader
                            .next_leb128_u32()
                            .expect("No number of locals in Code section");
                        let mut locals = Vec::new();

                        println!("Number of locals {}", number_of_locals);

                        for _ in 0..number_of_locals {
                            let count = reader
                                .next_leb128_u32()
                                .expect("No local decl count in Code section");
                            let local_type =
                                ValType::lookup(reader.next().expect("No local type decl"));
                            for _ in 0..count {
                                locals.push(local_type);
                            }
                        }
                        let sig_index = module
                            .funcsec
                            .get(index as usize)
                            .expect("Function Mapping failed")
                            .sig_index;

                        let signature = module
                            .typesec
                            .get(sig_index as usize)
                            .expect("Function Mapping failed.");
                        let start_pos = reader.curr() + 1;
                        let span = reader
                            .get_slice(start_pos, end_pos)
                            .expect("given start and end pos invalid");
                        let mut validator =
                            Validator::new(&locals, &signature, span, &module.typesec);

                        let side_table = validator.validate().expect("Validation failed");

                        let function = Function {
                            locals,
                            signature: &signature,
                            start: start_pos,
                            end: end_pos,
                            side_table,
                        };
                        reader.seek(end_pos);
                        println!("Function: {:?}", function);
                    }
                    println!("Rest: {:?}", reader.rest())
                }
                _ => {}
            }
        } else {
            break;
        };
    }
    println!("{module:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        let wasm = wat::parse_str(
            "(module
              (func (param i32)
                 block $label
                   block 
                     br $label
                   end
                 end
              )
             )",
        )
        .unwrap();
        println!("{wasm:?}");
        let mut reader = run(&wasm);
        // let mut reader = WasmReader::new(&[10, 2]);
        // println!("{}",reader.next_leb128_u32().unwrap());
    }
}
