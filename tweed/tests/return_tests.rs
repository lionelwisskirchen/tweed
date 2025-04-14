use tweed::{run, Value};

#[test]
fn test_invalid_return() {
    let wasm = wat::parse_str(
        r#"
               (func (export "fun") (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
                   i32.const 12
               )
"#,
    )
    .unwrap();
    assert!(run(&wasm, "fun".to_string(), vec![]).is_err());
}

#[test]
fn test_valid_simple() {
    let wasm = wat::parse_str(
        r#"
               (func (export "fun")(result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
               )
"#,
    )
    .unwrap();
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![]).unwrap(),
        vec![Value::from(12)]
    );
}

#[test]
fn test_loop() {
    let wasm = wat::parse_str(
        r#"
               (func (export "fun")(result i32)
    (loop (result i32) (return (i32.const 3)) (i32.const 2))
  )
"#,
    )
    .unwrap();
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![]).unwrap(),
        vec![Value::from(3)]
    );
}
#[test]
fn test_valid_if_else() {
    let wasm = wat::parse_str(
        r#"
              (func (export "fun")(param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (local.get 1)) (else (return (i32.const 4)))
    )
  )
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0), Value::from(6)]
        )
        .unwrap(),
        vec![Value::from(4)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(6)]
        )
        .unwrap(),
        vec![Value::from(6)]
    );
}
