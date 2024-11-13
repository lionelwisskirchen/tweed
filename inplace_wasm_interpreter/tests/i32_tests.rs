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
