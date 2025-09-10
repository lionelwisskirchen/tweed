use crate::core::error::Result;
use crate::core::wasm_reader::WasmReader;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub sig_index: u32,
}

pub fn process_function_section(reader: &mut WasmReader) -> Result<Vec<FuncDecl>> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_functions = reader.next_leb128_u32()?;

    let mut function_decls = Vec::new();
    for _ in 0..number_of_functions {
        let decl = FuncDecl {
            sig_index: reader.next_leb128_u32()?,
        };
        function_decls.push(decl);
    }
    Ok(function_decls)
}
