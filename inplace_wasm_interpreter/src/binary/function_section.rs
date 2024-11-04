use crate::core::wasm_reader::WasmReader;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FuncDecl {
    pub sig_index: u32,
}

pub fn process_function_section(reader: &mut WasmReader) -> Vec<FuncDecl> {
    reader.next_leb128_u32().expect("No Section size found");
    // Size is not required for decoding, so just check if size is present
    let number_of_functions = reader
        .next_leb128_u32()
        .expect("No function defined in Function section");

    let mut function_decls = Vec::new();
    for _ in 0..number_of_functions {
        let decl = FuncDecl {
            sig_index: reader.next_leb128_u32().expect("No sig index given"),
        };
        function_decls.push(decl);
    }
    function_decls
}
