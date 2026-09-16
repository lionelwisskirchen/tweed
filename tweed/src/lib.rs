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

use binary::custom_section::InterfaceId;
use binary::type_section::FuncType;
use execution::executor::ExecutionState;
pub use log::LevelFilter;

use crate::alloc::string::ToString;
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
    if let ExecutionState::Finished(res) = result {
        Ok(res)
    } else {
        Err(TweedError::Execution(ExecutionError::FunctionIsRunning))
    }
}
pub fn create_and_bind(
    input: &[u8],
    engine: &mut Engine,
    artifact_hash: Option<String>,
) -> Result<Vec<InterfaceId>, TweedError> {
    let mut module = initialize(input)?;
    module.artifact_hash = artifact_hash;
    let instance = instantiate(module, engine)?;
    Ok(engine.bind_interfaces(&instance)?)
}

pub fn register_safe_point(engine: &mut Engine) -> Result<(), TweedError> {
    engine.add_function_import(
        "env".to_string(),
        "safe_point".to_string(),
        FuncType {
            parameter: vec![],
            result: vec![],
        },
        |_, _| Err(ExecutionError::SafePointReached),
    )?;
    Ok(())
}

pub fn bind_interfaces(
    instance: InstanceRef,
    engine: &mut Engine,
) -> Result<Vec<InterfaceId>, TweedError> {
    Ok(engine.bind_interfaces(&instance)?)
}
pub fn upgrade_interface(
    id: String,
    selection: Vec<String>,
    engine: &mut Engine,
) -> Result<(), TweedError> {
    let interface = InterfaceId::try_from(id)?;
    let select: Vec<InterfaceId> = selection
        .iter()
        .map(|id| InterfaceId::try_from(id.clone()))
        .collect::<Result<Vec<InterfaceId>, _>>()?;
    Ok(engine.upgrade_interface(&interface, select)?)
}
pub fn force_upgrade_interface(
    id: String,
    selection: Vec<String>,
    engine: &mut Engine,
) -> Result<(), TweedError> {
    let interface = InterfaceId::try_from(id)?;
    let select: Vec<InterfaceId> = selection
        .iter()
        .map(|id| InterfaceId::try_from(id.clone()))
        .collect::<Result<Vec<InterfaceId>, _>>()?;
    Ok(engine.force_upgrade_interface(&interface, select)?)
}
pub fn collect_garbage(engine: &mut Engine) -> Result<(), TweedError> {
    Ok(engine.collect_garbage()?)
}

pub fn invoke_interface(
    id: String,
    function: String,
    params: Vec<Value>,
    engine: &mut Engine,
) -> Result<ExecutionState, TweedError> {
    let interface_id = InterfaceId::try_from(id)?;
    Ok(engine.invoke_interface(interface_id, function, params)?)
}
pub fn step(engine: &mut Engine) -> Result<ExecutionState, TweedError> {
    Ok(engine.step()?)
}
pub fn instantiate(module: Module, engine: &mut Engine) -> Result<InstanceRef, TweedError> {
    Ok(engine.instantiate(module)?)
}
pub fn get_artifacts(engine: &mut Engine) -> Vec<String> {
    engine.get_artifacts()
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
        linkingsec: module_environment.linking_sec,
        artifact_hash: None,
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
