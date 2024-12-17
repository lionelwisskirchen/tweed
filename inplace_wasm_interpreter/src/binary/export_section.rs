extern crate alloc;
use crate::core::wasm_reader::WasmReader;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExportContext {
    FunctionExport(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Export {
    pub name: String,
    pub context: ExportContext,
}

pub fn process_export_section(reader: &mut WasmReader) -> Vec<Export> {
    reader.next_leb128_u32().expect("No Section size found");
    // Size is not required for decoding, so just check if size is present
    let number_of_exports = reader
        .next_leb128_u32()
        .expect("No export defined in Export section");
    let mut aggregated_exports = Vec::new();
    for _ in 0..number_of_exports {
        let length_of_export_name = reader
            .next_leb128_u32()
            .expect("Could not parse length of export name");
        let export_name = reader
            .next_chunk_as_string(length_of_export_name as usize)
            .expect("Could not parse export name");

        let export_kind = reader.next_byte().expect("Could not parse export kind");
        match export_kind {
            0x00 => {
                // Function index
                let index = reader
                    .next_leb128_u32()
                    .expect("Could not parse function index");
                let context = ExportContext::FunctionExport(index);
                aggregated_exports.push(Export {
                    name: export_name,
                    context,
                });
            }
            _ => panic!("Unsupported export kind identifier"),
        }
    }
    aggregated_exports
}

#[cfg(test)]
mod tests {
    use super::process_export_section;
    use super::*;
    use crate::{binary::export_section::ExportContext, core::wasm_reader::WasmReader};
    #[test]
    fn test_export_aggregation() {
        let bytes = [0x09, 0x01, 0x05, 0x64, 0x75, 0x6d, 0x6d, 0x79, 0x00, 0x00];
        let mut reader = WasmReader::new(bytes.to_vec());
        let actual_exports = process_export_section(&mut reader);
        let expected_exports = vec![Export {
            name: String::from("dummy"),
            context: ExportContext::FunctionExport(0),
        }];
        assert_eq!(actual_exports, expected_exports)
    }
}
