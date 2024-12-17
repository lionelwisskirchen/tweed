use inplace_wasm_interpreter::{run, Value};

#[test]
#[should_panic]
fn block_underflow() {
    let wasm = wat::parse_str(
        r#"(module 
           (func (export "fun") (result i32)
	       i32.const 12	       
               (block $label1 (result i32)
	    	   i32.const 10
		   i32.add
	         )
               )
)"#,
    )
    .unwrap();
    run(&wasm, "fun".to_string(), vec![]);
}

#[test]
#[should_panic]
fn block_overflow() {
    let wasm = wat::parse_str(
        r#"(module 
                 (func (export "fun")
		      (block $l
                       i32.const 1
		       i32.const 2
		       br $l
		       i32.const 3
		       i32.const 2

		       )
                     
                )
             )
"#,
    )
    .unwrap();
    run(&wasm, "fun".to_string(), vec![]);
}

#[test]
fn with_br_if() {
    let wasm = wat::parse_str(
        r#"
    (module
	   
           (func (export "fun")(result i32)
                (block $bl (result i32)
                  i32.const 0
                  i32.const 1
                  br_if $bl
                  i32.const 3
                  return
               )
               )
)
"#,
    )
    .unwrap();
    assert_eq!(run(&wasm, "fun".to_string(), vec![]), vec![Value::I32(0)]);
}
#[test]
fn with_br() {
    let wasm = wat::parse_str(
        r#"(module
	   
           (func  (export "fun") (result i32)
               (block $label1 (result i32)
       	           i32.const 12	       
	    	   i32.const 10
		   i32.const 10
		   br $label1
	         )
		 i32.const 10
		 i32.add
               )
)
"#,
    )
    .unwrap();
    assert_eq!(run(&wasm, "fun".to_string(), vec![]), vec![Value::I32(20)]);
}
