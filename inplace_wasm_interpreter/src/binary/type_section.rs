use crate::core::{values::ValType, wasm_reader::WasmReader};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncType {
    pub parameter: Vec<ValType>,
    pub result: Vec<ValType>,
}

pub fn process_type_section(reader: &mut WasmReader) -> Vec<FuncType> {
    reader.next_leb128_u32().expect("No Section size found");
    // Size is not required for decoding, so just check if size is present
    let mut function_types = Vec::new();
    let number_of_types = reader
        .next_leb128_u32()
        .expect("No type defined in type section");

    for _ in 0..number_of_types {
        reader
            .next_leb128_u32()
            .expect("Could not parse funct type id"); // Can be ignored
        let number_of_params = reader
            .next_leb128_u32()
            .expect("No number of params defined");
        let params = reader
            .next_chunk(number_of_params as usize)
            .expect("Can't get defined amount of params")
            .iter()
            .map(ValType::lookup)
            .collect();
        let number_of_results = reader
            .next_leb128_u32()
            .expect("No number of results defined");
        let results = reader
            .next_chunk(number_of_results as usize)
            .expect("Can't get defined amount of results")
            .iter()
            .map(ValType::lookup)
            .collect();
        function_types.push(FuncType {
            parameter: params,
            result: results,
        });
    }
    function_types
}
