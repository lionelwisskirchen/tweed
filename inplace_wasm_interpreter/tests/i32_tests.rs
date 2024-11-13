use inplace_wasm_interpreter::{run, Value};

#[test]
fn test_add() {
    let wasm = wat::parse_str(
        "
(func (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y)))
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(1)]),
        vec![Value::from(2)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(-1)]),
        vec![Value::from(-2)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(1)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(0x3fffffff), Value::from(1)]),
        vec![Value::from(0x40000000)]
    );
}
#[test]
fn test_sub() {
    let wasm = wat::parse_str(
        "
(func (param $x i32) (param $y i32) (result i32) (i32.sub (local.get $x) (local.get $y)))
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(1)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(0)]),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(-1)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(0x7fffffff), Value::from(-1)]),
        vec![Value::from(0x80000000 as u32)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(0x80000000 as u32), Value::from(1)]),
        vec![Value::from(0x7fffffff as u32)]
    );
    assert_eq!(
        run(
            &wasm,
            vec![
                Value::from(0x80000000 as u32),
                Value::from(0x80000000 as u32)
            ]
        ),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(0x3fffffff), Value::from(-1)]),
        vec![Value::from(0x40000000)]
    );
}

#[test]
fn test_ges() {
    let wasm = wat::parse_str(
        "
  (func (param $x i32) (param $y i32) (result i32) (i32.ge_s (local.get $x) (local.get $y)))
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(0), Value::from(0)]),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(1)]),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(1)]),
        vec![Value::from(0)]
    );
}
#[test]
fn test_lts() {
    let wasm = wat::parse_str(
        "
  (func (param $x i32) (param $y i32) (result i32) (i32.lt_s (local.get $x) (local.get $y)))
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(0), Value::from(0)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(0)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(1)]),
        vec![Value::from(1)]
    );
}

#[test]
fn test_gts() {
    let wasm = wat::parse_str(
        "
  (func (param $x i32) (param $y i32) (result i32) (i32.gt_s (local.get $x) (local.get $y)))
",
    )
    .unwrap();
    assert_eq!(
        run(&wasm, vec![Value::from(0), Value::from(0)]),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(1), Value::from(0)]),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(&wasm, vec![Value::from(-1), Value::from(1)]),
        vec![Value::from(0)]
    );
}

// The following tests can be removed, as soon as I64 is supported
#[should_panic]
#[test]
fn test_type_mismatch_add() {
    let _ = Value::I32(3) + Value::I64(4);
}
#[should_panic]
#[test]
fn test_type_mismatch_sub() {
    let _ = Value::I32(3) - Value::I64(4);
}
