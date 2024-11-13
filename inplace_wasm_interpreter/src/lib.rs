#![cfg_attr(not(test), no_std)]
#[macro_use]
extern crate alloc;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;

pub use core::values::Value;

use crate::core::wasm_reader::WasmReader;

use alloc::vec::Vec;
use binary::module::{create_module, Module};
use execution::instance;
use validation::validator::Validator;

pub fn run(input: &[u8], params: Vec<Value>) -> Vec<Value> {
    let mut reader = WasmReader::new(input.to_vec());
    let mut module = create_module(&mut reader);
    validate(&mut reader, &mut module);
    #[cfg(test)]
    println!("----- Created and validated Module -----\n {:#?}", module);
    let mut instance = instance::instantiate(module, &mut reader);

    instance.call(0, params)
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
