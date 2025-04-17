use tweed::{run, Value};

#[test]
fn test_add() {
    let wasm = wat::parse_str(
        r#"
(func (export "fun") (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y)))
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(2)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(-1)]
        )
        .unwrap(),
        vec![Value::from(-2)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0x3fffffff), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0x40000000)]
    );
}
#[test]
fn test_sub() {
    let wasm = wat::parse_str(
        r#"
(func (export "fun") (param $x i32) (param $y i32) (result i32) (i32.sub (local.get $x) (local.get $y)))
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(-1)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0x7fffffff), Value::from(-1)]
        )
        .unwrap(),
        vec![Value::from(0x80000000 as u32)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0x80000000 as u32), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0x7fffffff as u32)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![
                Value::from(0x80000000 as u32),
                Value::from(0x80000000 as u32)
            ]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0x3fffffff), Value::from(-1)]
        )
        .unwrap(),
        vec![Value::from(0x40000000)]
    );
}

#[test]
fn test_ges() {
    let wasm = wat::parse_str(
        r#"
  (func (export "fun")(param $x i32) (param $y i32) (result i32) (i32.ge_s (local.get $x) (local.get $y)))
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
}
#[test]
fn test_lts() {
    let wasm = wat::parse_str(
        r#"
  (func (export "fun") (param $x i32) (param $y i32) (result i32) (i32.lt_s (local.get $x) (local.get $y)))
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(1)]
    );
}

#[test]
fn test_gts() {
    let wasm = wat::parse_str(
        r#"
  (func (export "fun") (param $x i32) (param $y i32) (result i32) (i32.gt_s (local.get $x) (local.get $y)))
"#,
    )
    .unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(0), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(1), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(1)]
    );
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(-1), Value::from(1)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
}

#[test]
fn test_mul_i32() {
    let wasm = wat::parse_str(
        r#"
  (func (export "fun") (param $x i32) (param $y i32) (result i32) (i32.mul (local.get $x) (local.get $y)))
"#,
    )
	.unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(3), Value::from(0)]
        )
        .unwrap(),
        vec![Value::from(0)]
    );
}

#[test]
fn test_mul_i64() {
    let wasm = wat::parse_str(
        r#"
  (func (export "fun") (param $x i64) (param $y i64) (result i64) (i64.mul (local.get $x) (local.get $y)))
"#,
    )
	.unwrap();
    assert_eq!(
        run(
            &wasm,
            "fun".to_string(),
            vec![Value::from(3 as i64), Value::from(0 as i64)]
        )
        .unwrap(),
        vec![Value::from(0 as i64)]
    );
}
