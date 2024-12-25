#![cfg_attr(not(test), no_std)]
#[macro_use]
extern crate alloc;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;

pub use core::values::Value;

use crate::core::error::Result;
use crate::core::wasm_reader::WasmReader;

use alloc::string::String;
use alloc::vec::Vec;
use binary::module_environment::{create_module_environment, ModuleEnvironment};
use execution::instance::{self, Instance};
use validation::{module::Module, validator::Validator};

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>> {
    let module = initialize(input)?;
    let instance = instantiate(module)?;
    invoke(instance, name, params)
}
pub fn initialize(input: &[u8]) -> Result<Module> {
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader)?;
    validate(reader, module_environment)
}

pub fn instantiate(module: Module) -> Result<Instance> {
    instance::instantiate(module)
}

pub fn invoke(mut instance: Instance, name: String, params: Vec<Value>) -> Result<Vec<Value>> {
    instance.call_by_name(&name, params)
}

fn validate(mut reader: WasmReader, module_environment: ModuleEnvironment) -> Result<Module> {
    let mut validator = Validator::new(&mut reader, &module_environment);
    let sidetables = validator.validate()?;
    Ok(Module {
        reader,
        typesec: module_environment.typesec,
        funcsec: module_environment.funcsec,
        codesec: module_environment.codesec,
        exportsec: module_environment.exportsec,
        sidetables,
    })
}

// #[cfg(test)]
// mod tests {

//     use crate::initialize;

//     use super::*;
//     #[test]
//     fn figg(){
//  let bytes = wat::parse_str(
//             "(module
//      (func $type-arg-empty-in-select
//        (block (result i32)
//          (select (br 0) (i32.const 1) (i32.const 2))
//        )
//        (i32.eqz) (drop)
//      )
//    )",
//         )
//             .unwrap();
// 	println!("{:?}",initialize(&bytes));
// 	assert!(initialize(&bytes).is_ok());
//     }
//     }
