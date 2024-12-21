use inplace_wasm_interpreter::{run, Value};
#[test]
fn test_sum() {
    let wasm = wat::parse_str(
        r#"
(module
  (func (export "fun")(param $x i32) (result i32)
        ;; set variables
       (local $i i32)
       (local $res i32)

       i32.const 0
       local.set $i

       i32.const 0
       local.set $res

       (block $logic
        local.get $i
        local.get $x
        i32.gt_s
        br_if $logic ;; branch out of $logic block

        (loop $lp
            ;; result = result + i
            local.get $res
            local.get $i
            i32.add
            local.set $res
            
            ;; i = i + 1
            local.get $i
            i32.const 1
            i32.add
            local.set $i
            
            ;; check if this the last loop
            local.get $i
            local.get $x
            i32.gt_s
            br_if $logic ;; branch out of $logic block

            br $lp ;; branch to $lp
        )
       )


       local.get $res
  )
)

"#,
    )
    .unwrap();
    assert_eq!(
        run(&wasm, "fun".to_string(), vec![Value::I32(6)]).unwrap(),
        vec![Value::I32(21)]
    );
}
