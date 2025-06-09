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
use binary::module_environment::{create_module_environment, ModuleEnvironment};
use execution::linker::Linker;
use execution::store::Store;
use validation::{module::Module, validator::Validator};

pub fn run(input: &[u8], name: String, params: Vec<Value>) -> Result<Vec<Value>> {
    let mut module = initialize(input)?;
    let mut store = Store::default();
    let instance_name = instantiate(&mut module, &mut store, None)?;
    invoke(instance_name, name, params, &mut store)
}
pub fn initialize(input: &[u8]) -> Result<Module> {
    info!("Initialize module");
    let mut reader = WasmReader::new(input.to_vec());
    let module_environment = create_module_environment(&mut reader)?;
    validate(reader, module_environment)
}

pub fn instantiate(module: &mut Module, store: &mut Store, name: Option<&str>) -> Result<String> {
    info!("Instantiate module");
    let mut linker = Linker::new(module);
    linker.instantiate(store, name)
}

pub fn invoke(
    instance_name: String,
    name: String,
    params: Vec<Value>,
    store: &mut Store,
) -> Result<Vec<Value>> {
    info!("Invoke `{name}` with {params:?}");
    let mut exec = Executor::new(store);
    exec.invoke(&instance_name, &name, &params)
}

fn validate(mut reader: WasmReader, module_environment: ModuleEnvironment) -> Result<Module> {
    info!("Validate module");
    debug!("Module Environment{:?}", module_environment);
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
        tablesec: module_environment.tablesec,
        elementsec: module_environment.elementsec,
        sidetables,
    })
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_element_section() {
//         let bytes = wat::parse_str(
//             r#"(module
//   (table $t2 2 externref)
//   (table $t3 3 funcref)
//   (elem (table $t3) (i32.const 1) func $dummy)
//   (func $dummy)

//   (func (export "init") (param $r externref)
//     (table.set $t2 (i32.const 1) (local.get $r))
//     (table.set $t3 (i32.const 2) (table.get $t3 (i32.const 1)))
//   )

//   (func (export "get-externref") (param $i i32) (result externref)
//     (table.get (local.get $i))
//   )
//   (func $f3 (export "get-funcref") (param $i i32) (result funcref)
//     (table.get $t3 (local.get $i))
//   )

//   (func (export "is_null-funcref") (param $i i32) (result i32)
//     (ref.is_null (call $f3 (local.get $i)))
//   )
// )"#,
//         )
//         .unwrap();

//         let mut module = initialize(&bytes).unwrap();
//         let mut store = Store::default();
//         let mut instance = instantiate(&mut module, &mut store).unwrap();
//         println!("{:?}",instance);
// 	println!("{:?}", store);
//         invoke(
//             &mut instance,
//             "init".to_string(),
//             vec![Value::ExternRef(1)],
//             &mut store,
//             &mut module.reader,
//         )
//         .unwrap();
//         let result = invoke(
//             &mut instance,
//             "get-externref".to_string(),
//             vec![Value::I32(0)],
//             &mut store,
//             &mut module.reader,
//         )
//         .unwrap();
//         println!("{:?}", result);
//         assert!(false);
//     }
// }
