use crate::core::{error::ValidationError, wasm_reader::WasmReader};
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ValidationError>;

pub const WASM_PAGE_SIZE: u32 = 65536; // 64 KiB
pub const MAX_MEM_PAGES: u32 = 65536;

#[derive(Debug, Clone)]
pub struct Memory {
    pub limits: Limits,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

/// Processes the **memory section** according to
/// [WebAssembly specification: Memory Section](https://webassembly.github.io/spec/core/binary/modules.html#memory-section).
///
/// This function creates at most one memory, because in WebAssembly Version(2.0), only one memory
/// is allowed. This might change in the future.
///
/// # Arguments
/// * `reader`: a mutable reference to the WasmReader to read more bytes.
///
/// # Return
/// Ok(Vec<Memory>) if everything is fine.
///
/// # Error
/// * `ValidationError::MultipleMemories`: Multiple Memories are used.
/// * `ValidationError::SizeMinimumMustNotBeGreaterThanMaximum`: The min value of the memory size
///   is larger than the max value.
/// * `ValidationError::MemorySizeExceeded`: The memory size is larger than 65536 (64 KiB)
pub fn process_memory_section(reader: &mut WasmReader) -> Result<Vec<Memory>> {
    let section_size = reader.next_leb128_u32()?;
    let start_of_section = reader.current_index();
    let mut memories = Vec::new();
    let number_of_memories = reader.next_leb128_u32()?;

    // currently (WebAssembly V2) there is only one memory allowed at once
    if number_of_memories > 1 {
        return Err(ValidationError::MultipleMemories);
    }

    for _ in 0..number_of_memories {
        memories.push(process_memory(reader)?);
    }
    if reader.current_index() != start_of_section + section_size as usize {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(memories)
}
fn process_memory(reader: &mut WasmReader) -> Result<Memory> {
    let flags = reader.next_flag()?;
    let min = reader.next_leb128_u32()?;
    let max = if flags != 0 {
        let max_val = reader.next_leb128_u32()?;
        if max_val < min {
            return Err(ValidationError::SizeMinimumMustNotBeGreaterThanMaximum);
        }
        if max_val > MAX_MEM_PAGES {
            return Err(ValidationError::MemorySizeExceeded);
        }
        Some(max_val)
    } else {
        None
    };

    if min > MAX_MEM_PAGES {
        return Err(ValidationError::MemorySizeExceeded);
    }

    let limits = Limits { min, max };
    Ok(Memory { limits })
}
