#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;

use core::error::{TweedError, ValidationError};
pub use core::values::Value;

pub use log::LevelFilter;

use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;
use binary::module_environment::create_module_environment;
use execution::engine::Engine;
use validation::module::Module;

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>, TweedError> {
    let module = initialize(input)?;
    let mut engine = Engine::default();
    let instance_addr = engine.instantiate(module)?;
    let result = engine.invoke(instance_addr, name, params)?;
    Ok(result)
}
pub fn initialize(input: &[u8]) -> Result<Module, ValidationError> {
    trace!("Initialize module");
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader)?;

    Ok(Module {
        reader: reader.assume_valid(),
        typesec: module_environment.typesec,
        funcsec: module_environment.funcsec,
        codesec: module_environment.codesec,
        exportsec: module_environment.exportsec,
        memorysec: module_environment.memorysec,
        datasec: module_environment.datasec,
        globalsec: module_environment.globalsec,
        importsec: module_environment.importsec,
        tablesec: module_environment.tablesec,
        elementsec: module_environment.elementsec,
        startsec: module_environment.startsec,
        sidetables: module_environment.sidetables,
    })
}
