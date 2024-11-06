#![cfg_attr(not(test), no_std)]
#[macro_use]
extern crate alloc;

pub mod binary;
pub mod core;
pub mod execution;
pub mod validation;
use crate::core::wasm_reader::WasmReader;
use crate::instance::Value;
use alloc::vec::Vec;
use binary::module::{create_module, Module};
use execution::instance;
use validation::validator::Validator;

pub fn run(input: &[u8], params: Vec<Value>) -> Vec<Value> {
    let mut reader = WasmReader::new(input.to_vec());
    let mut module = create_module(&mut reader);
    validate(&mut reader, &mut module);
    #[cfg(test)]
    println!("----- Created and validated Module -----\n {:#?}", module);
    let mut instance = instance::instantiate(module, &mut reader);

    instance.call(0, params).unwrap().to_vec()
}

fn validate(reader: &mut WasmReader, module: &mut Module) {
    let mut validator = Validator::new(reader, &module.typesec);
    let mut side_tables = Vec::new();

    for fun in &module.codesec {
        let side_table = validator.validate(fun);
        side_tables.push(side_table);
    }
    module.sidetables = side_tables;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn is_invalid_1() {
        let wasm = wat::parse_str(
            "(module
           (func (result i32)
	       i32.const 12	       
               (block $label1 (result i32)
	    	   i32.const 10
		   i32.add
	         )
               )
)",
        )
        .unwrap();
        run(&wasm, vec![]);
    }
    #[test]
    #[should_panic]
    fn is_invalid_2() {
        let wasm = wat::parse_str(
            "
    (module
                 (func
		      (block $l
                       i32.const 1
		       i32.const 2
		       br $l
		       i32.const 3
		       i32.const 2

		       )
                     
                )
             )
",
        )
        .unwrap();
        run(&wasm, vec![]);
    }

    
    #[test]
    fn is_valid_1() {
        let wasm = wat::parse_str(
            "
    (module
	   
           (func (result i32)
                (block $bl (result i32)
                  i32.const 0
                  i32.const 1
                  br_if $bl
                  i32.const 3
                  return
               )
               )
)
",
        )
        .unwrap();
        assert_eq!(run(&wasm,vec![]), vec![Value::I32(0)]);
    }
    #[test]
    fn is_valid_2() {
        let wasm = wat::parse_str(
            "
     (module
	   
           (func (result i32)
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
",
        )
        .unwrap();
        assert_eq!(run(&wasm, vec![]), vec![Value::I32(20)]);
    }

    
    #[test]
    fn is_valid_3() {
        let wasm = wat::parse_str(
            "
(module
  (func (param $x i32) (result i32)
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

",
        )
        .unwrap();
        assert_eq!(run(&wasm, vec![Value::I32(3)]), vec![Value::I32(6)]);
    }

     
}
