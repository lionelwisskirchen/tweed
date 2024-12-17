use inplace_wasm_interpreter::{run, Value};

#[test]
fn test_multiple_value_1() {
    let wasm = wat::parse_str(
        r#"
(func (export "fun") (param $p i32) (result i32)
  (block
    (block
      (block
        (block (local.get $p)
               (br_table
                         2   ;; p == 0 => (br 2)
                         1   ;; p == 1 => (br 1)
                         0   ;; p == 2 => (br 0)
                         3)) ;; else => (br 3)
        ;; Target for (br 0)
        (i32.const 100)
        (return))
      ;; Target for (br 1)
      (i32.const 101)
      (return))
    ;; Target for (br 2)
    (i32.const 102)
    (return))
  ;; Target for (br 3)
  (i32.const 103)
  (return))"#,
    )
    .unwrap();
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(0)]),
        vec![Value::from(102)]
    );
}
#[test]
fn test_multiple_value_2() {
    let wasm = wat::parse_str(
        r#"
(func (export "fun") (param i32) (result i32)
    (local i32)
    (local.set 1 (block (result i32)
      (local.set 1 (block (result i32)
        (local.set 1 (block (result i32)
          (local.set 1 (block (result i32)
            (local.set 1 (block (result i32)
              (br_table 3 2 1 0 4 (i32.const 200) (local.get 0))
              (return (i32.add (local.get 1) (i32.const 99)))
            ))
            (return (i32.add (local.get 1) (i32.const 10)))
          ))
          (return (i32.add (local.get 1) (i32.const 11)))
        ))
        (return (i32.add (local.get 1) (i32.const 12)))
      ))
      (return (i32.add (local.get 1) (i32.const 13)))
    ))
    (i32.add (local.get 1) (i32.const 14))
  )"#,
    )
    .unwrap();
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(0)]),
        vec![Value::from(213)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(1)]),
        vec![Value::from(212)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(2)]),
        vec![Value::from(211)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(3)]),
        vec![Value::from(210)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(4)]),
        vec![Value::from(214)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(5)]),
        vec![Value::from(214)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(6)]),
        vec![Value::from(214)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(10)]),
        vec![Value::from(214)]
    );
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::from(-1)]),
        vec![Value::from(214)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0xffffffff as u32)]
        ),
        vec![Value::from(214)]
    );
}

#[test]
fn test_loop_first() {
    let wasm = wat::parse_str(
        r#"
(module
(func (export "fun") (result i32)
(loop (result i32)
   (br_table
     1
     1
       (i32.const 3)
       (i32.const 0)
   )
(i32.const 1))))"#,
    )
    .unwrap();
    assert_eq!(run(&wasm, "fun".to_string(), vec![]), vec![Value::from(3)]);
}
