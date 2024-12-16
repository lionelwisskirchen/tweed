extern crate alloc;
use crate::core::{value_types::ValType, wasm_reader::WasmReader};
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncCode {
    pub locals: Vec<ValType>,
    pub start: usize,
    pub end: usize,
}

pub fn process_code_section(reader: &mut WasmReader) -> Vec<FuncCode> {
    reader.next_leb128_u32().expect("No Section size found");
    // Size is not required for decoding, so just check if size is present
    let number_of_codes = reader
        .next_leb128_u32()
        .expect("No function defined in Code section");

    let mut aggregated_functions = Vec::new();
    for _ in 0..number_of_codes {
        let func_body_size = reader
            .next_leb128_u32()
            .expect("No function body size in Code section");

        let position_of_last_instruction = reader.calculate_offset(func_body_size as usize);

        let number_of_locals = reader
            .next_leb128_u32()
            .expect("No number of locals in Code section");
        let mut locals = Vec::new();
        for _ in 0..number_of_locals {
            let count = reader
                .next_leb128_u32()
                .expect("No local decl count in Code section");
            let local_type = ValType::lookup(reader.next_byte().expect("No local type decl"));
            for _ in 0..count {
                locals.push(local_type);
            }
        }
        let function = FuncCode {
            locals,
            start: reader.current_index() + 1,
            end: position_of_last_instruction,
        };

        aggregated_functions.push(function);
        reader.seek(position_of_last_instruction + 1);
    }
    aggregated_functions
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_aggregation() {
        let bytes = [
            0x12, 0x01, 0x10, // Number of Function Bodies (1), Body Size (16),
            0x01, 0x01, 0x7f, // Number of local declarations (1), 1 x I32
            0x20, 0x00, 0x20, 0x01, // Local.get(Index 0), Local.get(Index 1)
            0x6a, // I32Add
            0x21, 0x00, // Local.set(Index 0)
            0x20, 0x00, 0x20, 0x01, // Local.get(Index 0), Local.get(Index 1)
            0x6a, // I32Add
            0x0b, // Terminal
        ];
        let mut reader = WasmReader::new(bytes.to_vec());
        let actual_functions = process_code_section(&mut reader);
        let expected_functions = vec![FuncCode {
            locals: vec![ValType::I32],
            start: 6,
            end: 18,
        }];
        assert_eq!(actual_functions, expected_functions);
    }
}
