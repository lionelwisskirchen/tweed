#![cfg_attr(not(test), no_std)]
#[macro_use]
extern crate alloc;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;
use crate::core::wasm_reader::WasmReader;
use crate::instance::Value;
use alloc::vec::Vec;
use binary::module::{create_module, Module};
use execution::instance;
use validation::validator::Validator;

pub fn run(input: &[u8]) -> Vec<Value> {
    let mut reader = WasmReader::new(input.to_vec());
    let mut module = create_module(&mut reader);
    validate(&mut reader, &mut module);
    #[cfg(test)]
    println!("----- Created and validated Module -----\n {:#?}", module);
    let mut instance = instance::instantiate(module, &mut reader);

    instance.call(0, vec![]).unwrap().to_vec()
}

fn validate(reader: &mut WasmReader, module: &mut Module) {
    let mut validator = Validator::new(reader, &module.typesec);
    let mut side_tables = Vec::new();

    for fun in &module.codesec {
        let side_table = validator.validate(fun);
        side_tables.push(side_table);
    }
    module.sidetables = side_tables;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_1() {
        let wasm = wat::parse_str(
            "
    (module
                 (func
		      (block $l
                       i32.const 1
		       i32.const 2
		       br $l
		       i32.const 3
		       i32.const 2

		       )
                     
                )
             )
",
        )
        .unwrap();
        run(&wasm);
    }
    #[test]
    fn is_valid_2() {
        let wasm = wat::parse_str(
            "
    (module
(func (result i32 i32)

    i32.const 0 
    (if $i (result i32 i32)
      (then
        i32.const 1
        i32.const 1
        br $i
	i32.add
      )
      (else
        i32.const 0
        i32.const 1
      )
    )
  )
)
",
        )
        .unwrap();
        run(&wasm);
    }
}
