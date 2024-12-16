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
use binary::module_environment::{create_module_environment, ModuleEnvironment};
use execution::instance;
use validation::{module::Module, validator::Validator};

pub fn run(input: &[u8], params: Vec<Value>) -> Vec<Value> {
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader);
    let module = validate(&mut reader, module_environment);
    #[cfg(test)]
    println!("----- Created and validated Module -----\n {:#?}", module);
    let mut instance = instance::instantiate(module, &mut reader);

    instance.call(0, params)
}

fn validate(reader: &mut WasmReader, module_environment: ModuleEnvironment) -> Module {
    let mut validator = Validator::new(reader, &module_environment);
    let sidetables = validator.validate();
    Module {
        typesec: module_environment.typesec,
        funcsec: module_environment.funcsec,
        codesec: module_environment.codesec,
        sidetables,
    }
}
