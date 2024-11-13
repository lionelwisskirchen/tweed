use inplace_wasm_interpreter::{run, Value};

#[should_panic]
#[test]
fn test_invalid_return() {
    let wasm = wat::parse_str(
        "
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
                   i32.const 12
               )
",
    )
    .unwrap();
    run(&wasm, vec![]);
}

#[test]
fn test_valid_simple() {
    let wasm = wat::parse_str(
        "
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
               )
",
    )
    .unwrap();
    assert_eq!(run(&wasm, vec![]), vec![Value::from(12)]);
}

#[test]
fn test_loop() {
    let wasm = wat::parse_str(
        "
               (func (result i32)
    (loop (result i32) (return (i32.const 3)) (i32.const 2))
  )
",
    )
    .unwrap();
    assert_eq!(run(&wasm, vec![]), vec![Value::from(3)]);
}
#[test]
fn test_valid_if_else() {
    let wasm = wat::parse_str(
        "
              (func (param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (local.get 1)) (else (return (i32.const 4)))
    )
  )
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(0), Value::from(6)]),
        vec![Value::from(4)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(6)]),
        vec![Value::from(6)]
    );
}
