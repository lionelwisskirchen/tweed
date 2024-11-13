use crate::{core::wasm_reader::WasmReader, validation::side_table::SideTable};
use alloc::vec::Vec;

use super::{
    code_section::{process_code_section, FuncCode},
    function_section::{process_function_section, FuncDecl},
    type_section::{process_type_section, FuncType},
};

#[derive(Debug, Default)]
pub struct Module {
    pub typesec: Vec<FuncType>,
    pub funcsec: Vec<FuncDecl>,
    pub codesec: Vec<FuncCode>,
    pub sidetables: Vec<SideTable>,
}

pub fn create_module(reader: &mut WasmReader) -> Module {
    let mut module = Module {
        ..Default::default()
    };
    reader
        .compare_chunk(&[0x00, 0x61, 0x73, 0x6D])
        .expect("No preamble found");
    reader
        .compare_chunk(&[0x01, 0x00, 0x00, 0x00])
        .expect("No version number found");

    while let Some(section_code) = reader.next_byte() {
        match section_code {
            0x00 => break,
            0x01 => {
                module.typesec = process_type_section(reader);
            }
            0x03 => {
                module.funcsec = process_function_section(reader);
            }
            0x0a => {
                module.codesec = process_code_section(reader, &module);
            }
            _ => {
                panic!("Unknown section code");
            }
        }
    }
    module
}

#[cfg(test)]
mod tests {
    use super::*;
    #[should_panic]
    #[test]
    fn test_lookup_unknown() {
        let mut reader =
            WasmReader::new(vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, 0x77]);
        create_module(&mut reader);
    }
    #[test]
    fn test_empty_sequence() {
        let mut reader = WasmReader::new(vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]);
        create_module(&mut reader);
    }
}
