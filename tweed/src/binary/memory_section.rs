use crate::core::{
    error::ValidationError,
    wasm_reader::{MemoryTypeFlags, WasmReader},
};
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ValidationError>;

pub const DEFAULT_WASM_PAGE_SIZE: u32 = 65536; // 64 KiB
pub const MAX_WASM_PAGES: u64 = 65536;
// For 32-bit systems usize would overflow
pub const MAX_MEMORY: u64 = (DEFAULT_WASM_PAGE_SIZE as u64) * MAX_WASM_PAGES;

#[derive(Debug, Clone)]
pub struct Memory {
    pub limits: Limits,
    pub mempagesize: u32,
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
    let flags = MemoryTypeFlags(reader.next_flag()?);
    let min = reader.next_leb128_u32()?;
    let mut max = None;
    if flags.has_max() {
        max = Some(reader.next_leb128_u32()?);
    };

    let mempagesize = if flags.has_page_size() {
        let exp = reader.next_leb128_u32()?;
        1u32.checked_shl(exp)
            .ok_or(ValidationError::InvalidCustomPageSize)
    } else {
        Ok(DEFAULT_WASM_PAGE_SIZE)
    }?;

    // The pagesize must be the value 1 or the value 65536.
    if mempagesize != 1 && mempagesize != DEFAULT_WASM_PAGE_SIZE {
        return Err(ValidationError::InvalidCustomPageSize);
    }

    if min as u64 * mempagesize as u64 > MAX_MEMORY {
        return Err(ValidationError::MemorySizeExceeded);
    }

    if let Some(max_val) = max {
        if max_val < min {
            return Err(ValidationError::SizeMinimumMustNotBeGreaterThanMaximum);
        }
        if max_val > u32::MAX - 1 || max_val > u32::MAX.div_ceil(mempagesize) {
            return Err(ValidationError::MemorySizeExceeded);
        }
    }

    let limits = Limits { min, max };
    Ok(Memory {
        limits,
        mempagesize,
    })
}
