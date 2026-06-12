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

use crate::core::error::ExecutionError;
use crate::core::wasm_reader::WasmReader;
use crate::execution::engine::InstanceRef;
use alloc::string::String;
use alloc::vec::Vec;
use binary::module_environment::create_module_environment;
use execution::engine::Engine;
use validation::module::Module;

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>, TweedError> {
    let module = initialize(input)?;
    let mut engine = Engine::default();
    let instance_addr = engine.instantiate(module)?;
    let result = engine.invoke(&instance_addr, name, params)?;
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

/// Convert the offset of a linear memory instance to a global address
///
/// This function takes an offset into the first linear memory of a given instance. Then it calculates the
/// corresponding global address and returns it.
///
/// # Return
/// The address of the given offset.
///
/// # Errors
///  [`ExecutionError::MemoryAccessOutOfBounds`] if `offset` exceeds the length of the instance's memory.
pub fn app_offset_to_global(
    engine: &Engine,
    instance: &InstanceRef,
    offset: u64,
) -> Result<u64, TweedError> {
    let mem = engine.get_memory(instance, 0)?;
    let global_addr = mem.bytes.as_ptr() as u64;

    if offset as usize > mem.bytes.len() {
        return Err(TweedError::Execution(
            ExecutionError::MemoryAccessOutOfBounds,
        ));
    }

    Ok(global_addr + offset)
}

/// Convert a global address into an offset into linear memory instance.
///
/// This function takes a global address and the corresponding instance. If the global address is
/// part of the instance's linear memory it calculates the offset and returns it.
///
/// # Return
/// The offset for the given address.
///
/// # Errors
///  [`ExecutionError::MemoryAccessOutOfBounds`] if `global_addr` exceeds the length of the
/// instance's memory.
pub fn global_to_app_offset(
    engine: &Engine,
    instance: &InstanceRef,
    global_addr: u64,
) -> Result<u64, TweedError> {
    let mem = engine.get_memory(instance, 0)?;
    let range = mem.bytes.as_ptr_range();

    match range.contains(&(global_addr as *const u8)) {
        true => Ok(global_addr - range.start as u64),
        false => Err(TweedError::Execution(
            ExecutionError::MemoryAccessOutOfBounds,
        )),
    }
}
