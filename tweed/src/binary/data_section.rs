use alloc::vec::Vec;

use crate::core::error::Result;
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;

use super::function_section::FuncDecl;
use super::import_section::GlobalImport;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SegmentMode {
    Active {
        memory: u32,
        offset_start: usize,
        offset_end: usize,
    },
    Passive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataSegment {
    pub mode: SegmentMode,
    pub init: Vec<u8>,
}

pub fn process_data_section(
    reader: &mut WasmReader,
    global_imports: &[GlobalImport],
    functions: &[FuncDecl],
) -> Result<Vec<DataSegment>> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let number_of_segments = reader.next_leb128_u32()?;
    let mut data_segments = Vec::new();
    for _ in 0..number_of_segments {
        let mode = reader.next_leb128_u32()?;
        match mode {
            0 => {
                data_segments.push(create_data_segment(reader, global_imports, functions, 0)?);
            }
            1 => {
                // Mode passive
                let data_size = reader.next_leb128_u32()?;
                let data = reader.next_chunk(data_size as usize)?;
                let seg = DataSegment {
                    mode: SegmentMode::Passive,
                    init: data.to_vec(),
                };
                data_segments.push(seg);
            }
            2 => {
                let memory_index = reader.next_leb128_u32()?;
                data_segments.push(create_data_segment(
                    reader,
                    global_imports,
                    functions,
                    memory_index,
                )?);
            }
            _ => unreachable!(),
        }
    }
    Ok(data_segments)
}

fn create_data_segment(
    reader: &mut WasmReader,
    global_imports: &[GlobalImport],
    functions: &[FuncDecl],
    memory_index: u32,
) -> Result<DataSegment> {
    // Mode active with explicit memory index
    let start = reader.current_index();

    /* We need to validate the constant expression here, as the size of the expression body is implicit.
    We can't seek to the end (0x0b) byte because there may be a 0x0b byte before it.
    For example, i32.const(900b). */
    validate_const_expr(reader, global_imports, &[ValType::I32], functions)?;
    let end = reader.current_index();
    let data_size = reader.next_leb128_u32()?;
    let data = reader.next_chunk(data_size as usize)?;
    Ok(DataSegment {
        mode: SegmentMode::Active {
            memory: memory_index,
            offset_start: start,
            offset_end: end,
        },
        init: data.to_vec(),
    })
}
