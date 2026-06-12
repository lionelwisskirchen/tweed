use tweed::{
    app_offset_to_global,
    core::error::{ExecutionError, TweedError},
    execution::engine::Engine,
    global_to_app_offset, initialize,
    validation::module::Module,
};

static WAT_STRING: &str = r#"
    (module $add.wasm
      (type (;0;) (func (param i32 i32) (result i32)))
      (func $add (type 0) (param i32 i32) (result i32)
        local.get 1
        local.get 0
        i32.add)
      (memory (;0;) 1 1)
      (global $__stack_pointer (mut i32) (i32.const 4096))
      (global (;1;) i32 (i32.const 4096))
      (global (;2;) i32 (i32.const 4096))
      (export "memory" (memory 0))
      (export "add" (func $add))
      (export "__data_end" (global 1))
      (export "__heap_base" (global 2)))
  "#;

#[test]
fn offset_calculations_success() {
    // this is compiled and copied
    let wasm = wat::parse_str(WAT_STRING).unwrap();

    let module: Module = initialize(&wasm).unwrap();
    let mut engine = Engine::default();
    let instance = engine.instantiate(module).unwrap();

    let offset = 10;

    // Success
    let global_addr = app_offset_to_global(&engine, &instance, offset).unwrap();
    let app_offset = global_to_app_offset(&engine, &instance, global_addr).unwrap();

    assert_eq!(offset, app_offset);
}

#[test]
fn offset_calculations_global_addr_not_in_linear_mem() {
    // this is compiled and copied
    let wasm = wat::parse_str(WAT_STRING).unwrap();

    let module: Module = initialize(&wasm).unwrap();
    let mut engine = Engine::default();
    let instance = engine.instantiate(module).unwrap();

    // Try to get global mem that is not in linear mem
    let global_addr = app_offset_to_global(&engine, &instance, 0).unwrap();
    let res = global_to_app_offset(&engine, &instance, global_addr - 1);

    assert_eq!(
        res.unwrap_err(),
        TweedError::Execution(ExecutionError::MemoryAccessOutOfBounds,)
    );
}

#[test]
fn offset_calculations_offset_not_in_linear_mem() {
    // this is compiled and copied
    let wasm = wat::parse_str(WAT_STRING).unwrap();

    let module: Module = initialize(&wasm).unwrap();
    let mut engine = Engine::default();
    let instance = engine.instantiate(module).unwrap();

    // Try to get offset  that is not in linear mem
    let res = app_offset_to_global(&engine, &instance, 65537);

    assert_eq!(
        res.unwrap_err(),
        TweedError::Execution(ExecutionError::MemoryAccessOutOfBounds,)
    )
}
