pub mod code_section;
pub mod type_section;
pub mod validator;
pub mod wasm_reader;
use code_section::process_code_section;
use std::{collections::HashMap, iter::zip};
use validator::{SideTable, Validator};
use wasm_reader::WasmReader;

use crate::type_section::process_type_section;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncCode {
    signature: FuncType,
    locals: Vec<ValType>,
    start: usize,
    end: usize,
}
#[derive(Debug)]
struct FuncDecl {
    sig_index: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
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
pub struct FuncType {
    pub parameter: Vec<ValType>,
    pub result: Vec<ValType>,
}

#[derive(Debug, Default)]
pub struct Module {
    typesec: Vec<FuncType>,
    funcsec: Vec<FuncDecl>,
    codesec: Vec<FuncCode>,
    sidetables: Vec<SideTable>,
}


fn run(input: &[u8]) {
    let mut reader = WasmReader::new(input.to_vec());
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
            match section_code {
                0x00 => break,
                0x01 => {
                    module.typesec = process_type_section(&mut reader);
                }
                0x03 => {
                    reader.next_leb128_u32().expect("No Section size found");
                    // Size is not required for decoding, so just check if size is present
                    let number_of_functions = reader
                        .next_leb128_u32()
                        .expect("No function defined in Function section");

                    let mut function_decls = Vec::new();
                    for _ in 0..number_of_functions {
                        let decl = FuncDecl {
                            sig_index: reader.next_leb128_u32().expect("No sig index given"),
                        };
                        function_decls.push(decl);
                    }
                    module.funcsec = function_decls;
                }
                0x0a => {
                    let functions = process_code_section(&mut reader, &mut module);
                    let mut validator =
                        Validator::new(&mut reader, &module.typesec);
                    let mut side_tables = Vec::new();

                    println!("\n\nCreated Module: \n {functions:#?}");
                    println!("\n\nCreated Module: \n {module:#?}");

                    for fun in &functions {
                        let side_table = validator.validate(&fun);
                        side_tables.push(side_table);
                    }
                    module.sidetables = side_tables;
                    module.codesec = functions;
                }
                _ => {}
            }
        } else {
            break;
        };
    }
    println!("\n\nCreated Module: \n {module:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_if() {
        let wasm = wat::parse_str(
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
    )
  )
)",
        )
        .unwrap();
        run(&wasm);
    }
}
