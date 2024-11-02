pub mod binary;
pub mod core;
pub mod validation;

use binary::module::{create_module, Module};
use std::{collections::HashMap, iter::zip};
use validation::validator::Validator;

use crate::core::wasm_reader::WasmReader;

pub fn run(input: &[u8]) {
    let mut reader = WasmReader::new(input.to_vec());
    let mut module = create_module(&mut reader);
    validate(&mut reader, &mut module);
    println!("----- Created and validated Module -----\n {:#?}", module);
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
    fn example() {
        let wasm = wat::parse_str(
            "
(module

(func
      (local $i i32)
      (loop $my_loop
      local.get $i
      i32.const 1
      i32.add
      local.set $i

      local.get $i
      i32.const 10
      i32.lt_s
      br_if $my_loop
    )
 )
)
",
        )
        .unwrap();
        run(&wasm);
    }
}
