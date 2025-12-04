#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;

pub use core::values::Value;

use execution::executor::Executor;
pub use log::LevelFilter;

use crate::core::error::Result;
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;
use binary::module_environment::create_module_environment;
use execution::linker::Linker;
use execution::store::Store;
use validation::module::Module;

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>> {
    let mut module = initialize(input)?;
    let mut store = Store::default();
    let instance_name = instantiate(&mut module, &mut store, None)?;
    invoke(instance_name, name, params, &mut store)
}
pub fn initialize(input: &[u8]) -> Result<Module> {
    trace!("Initialize module");
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader)?;

    Ok(Module {
        reader: reader.to_validated(),
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

pub fn instantiate(
    module: &mut Module,
    store: &mut Store,
    instance_name: Option<&str>,
) -> Result<String> {
    trace!("Instantiate module");
    let mut linker = Linker::default();
    linker.instantiate(module, store, instance_name)
}

pub fn invoke(
    instance_name: String,
    name: String,
    params: Vec<Value>,
    store: &mut Store,
) -> Result<Vec<Value>> {
    trace!("Invoke `{name}` with {params:?}");
    let mut exec = Executor::new(store);
    exec.invoke(&instance_name, &name, &params)
}
