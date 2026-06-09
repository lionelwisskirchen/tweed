use std::collections::BTreeSet;

use alloc::vec::Vec;

use crate::core::error::ValidationError;
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;

use super::module_environment::ModuleEnvironment;
pub type Result<T> = core::result::Result<T, ValidationError>;

/// Structure to differentiate between active and passive data segments
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SegmentMode {
    /// Contents of active data segments are copied during instantiation
    Active {
        memory_idx: u32,
        offset_start: usize,
        offset_end: usize,
    },
    /// Contents of passive data segments can be copied using explicit instructions during runtime.
    Passive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataSegment {
    pub mode: SegmentMode,
    pub init: Vec<u8>,
}

/// Processes the **data section** according to
/// [WebAssembly specification: Data Section](https://webassembly.github.io/spec/core/binary/modules.html#data-section).
///
/// This function differs between active and passive data segments. Active data segments have
/// mode 0 (implicit memory index) or 2 (explicit memory index). Passive data segments have mode 1.
/// This function creates a [`DataSegment`] for every entry.
///
/// # Arguments
/// * `reader`: a mutable reference to the WasmReader to read more bytes.
/// * `global_imports`: a reference to an array of global imported ValTypes.
/// * `functions`: a reference to all registered functions
/// * `number_of_memories`: number of all available memories (imported + local)
///
/// # Return
/// Ok(Vec<DataSegment>) if everything is fine.
///
/// # Error
/// * `ValidationError::UnknownMemory`: The memory index does not point to a valid memory
pub fn process_data_section(
    reader: &mut WasmReader,
    module: &mut ModuleEnvironment,
    refs: &mut BTreeSet<usize>,
) -> Result<Vec<DataSegment>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();

    let section_end = reader.current_index() + section_size as usize;
    let number_of_memories = module.memorysec.len() + module.importsec.memories.len();
    let number_of_segments = reader.next_leb128_u32()?;
    if module.datacountsec.is_some_and(|x| x != number_of_segments) {
        return Err(ValidationError::InconsistentDataDataCountSectionLengths);
    }
    let mut data_segments = Vec::new();
    for _ in 0..number_of_segments {
        let mode = reader.next_leb128_u32()?;
        match mode {
            0 => {
                // mode active
                // memory index is implicit 0
                let memory_index = 0;
                // currently (WebAssembly V2) there is only one memory allowed at once

                if memory_index >= number_of_memories {
                    return Err(ValidationError::UnknownMemoryIdx(memory_index));
                }

                data_segments.push(create_data_segment(reader, module, 0, section_end, refs)?);
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
                // mode active
                // memory index is explicit given
                let memory_index = reader.next_leb128_u32()?;
                // currently (WebAssembly V2) there is only one memory allowed at once
                if memory_index != 0 {
                    return Err(ValidationError::UnknownMemoryIdx(memory_index as usize));
                }

                data_segments.push(create_data_segment(
                    reader,
                    module,
                    memory_index,
                    section_end,
                    refs,
                )?);
            }
            _ => unreachable!(),
        }
    }
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(data_segments)
}

fn create_data_segment(
    reader: &mut WasmReader,
    module: &mut ModuleEnvironment,
    memory_index: u32,
    section_end: usize,
    refs: &mut BTreeSet<usize>,
) -> Result<DataSegment> {
    // Mode active with explicit memory index
    let start = reader.current_index();

    /* We need to validate the constant expression here, as the size of the expression body is implicit.
    We can't seek to the end (0x0b) byte because there may be a 0x0b byte before it.
    For example, i32.const(900b). */
    validate_const_expr(
        reader,
        &[ValType::I32],
        section_end,
        module,
        module.globalsec.len(),
        refs,
    )?;
    let end = reader.current_index();
    let data_size = reader.next_leb128_u32()?;
    let data = reader.next_chunk(data_size as usize).map_err(|e| match e {
        ValidationError::ReadingOutOfBounds => ValidationError::UnexpectedEndOfFunctionOrSection,
        _ => e,
    })?;
    Ok(DataSegment {
        mode: SegmentMode::Active {
            memory_idx: memory_index,
            offset_start: start,
            offset_end: end,
        },
        init: data.to_vec(),
    })
}
