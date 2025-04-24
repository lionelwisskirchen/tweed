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
use execution::instance::{Instance, Linker};
use validation::{module::Module, validator::Validator};

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>> {
    let module = initialize(input)?;
    let mut instance = instantiate(module)?;
    invoke(&mut instance, name, params)
}
pub fn initialize(input: &[u8]) -> Result<Module> {
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader)?;
    validate(reader, module_environment)
}

pub fn instantiate(module: Module) -> Result<Instance> {
    let mut linker = Linker::new(module);
    linker.instantiate()
}

pub fn invoke(instance: &mut Instance, name: String, params: Vec<Value>) -> Result<Vec<Value>> {
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
        memorysec: module_environment.memorysec,
        datasec: module_environment.datasec,
        globalsec: module_environment.globalsec,
        importsec: module_environment.importsec,
        sidetables,
    })
}
