use alloc::vec::Vec;

use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;

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

pub fn process_data_section(reader: &mut WasmReader) -> Result<Vec<DataSegment>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let number_of_segments = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    let mut data_segments = Vec::new();
    for _ in 0..number_of_segments {
        let mode = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;

        match mode {
            0 => {
                // Mode active
                let start = reader.current_index();
                // TODO Pass imported globals! -> extract this to validator
                validate_const_expr(reader, &[], &[ValType::I32])?;
                let end = reader.current_index();
                let data_size = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let data = reader
                    .next_chunk(data_size as usize)
                    .ok_or(Error::InvalidSectionBytecode)?;
                let seg = DataSegment {
                    mode: SegmentMode::Active {
                        memory: 0,
                        offset_start: start,
                        offset_end: end,
                    },
                    init: data.to_vec(),
                };
                data_segments.push(seg);
            }
            1 => {
                // Mode passive
                let data_size = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let data = reader
                    .next_chunk(data_size as usize)
                    .ok_or(Error::InvalidSectionBytecode)?;
                let seg = DataSegment {
                    mode: SegmentMode::Passive,
                    init: data.to_vec(),
                };
                data_segments.push(seg);
            }
            _ => unreachable!(),
        }
    }
    Ok(data_segments)
}
