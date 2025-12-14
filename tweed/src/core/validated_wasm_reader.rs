use super::error::ExecutionError;
use alloc::vec::Vec;

pub type Result<T> = core::result::Result<T, ExecutionError>;
const CONTINUATION_BIT_MASK: u8 = 0x80;
const VALUE_BIT_MASK: u8 = 0x7F;

/// An **unsafe** bytecode reader for WebAssembly modules.
///
/// `ValidatedWasmReader` provides fast, sequential read access to a byte
/// buffer containing WebAssembly (Wasm) binary data.
///
/// This reader is intended exclusively for the *execution* phase, where the
/// input has already been fully validated. To achieve maximum performance,
/// it performs unchecked and potentially unsafe memory access.
///
/// # Safety
///
/// **Using this reader on unvalidated input can cause undefined behavior.  
/// It must *never* be used during the validation phase.**
#[derive(Debug, Default, Clone)]
pub struct ValidatedWasmReader {
    bytecode: Vec<u8>,
    byte_pointer: usize,
}

impl ValidatedWasmReader {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            bytecode: input,
            byte_pointer: 0,
        }
    }
    pub fn read_mem_args(&mut self) -> (u32, u32) {
        let _align = self.next_leb128_u32();
        let offset = self.next_leb128_u32();
        (_align, offset)
    }
    pub fn skip_block_type(&mut self) {
        self.next_leb128_u32();
    }
    pub fn next_byte(&mut self) -> u8 {
        let curr = self.bytecode[self.byte_pointer];
        self.byte_pointer += 1;
        curr
    }
    pub fn seek(&mut self, position: usize) {
        self.byte_pointer = position
    }
    pub fn current_index(&self) -> usize {
        self.byte_pointer.saturating_sub(1)
    }
    pub fn next_leb128_u32(&mut self) -> u32 {
        let mut result: u32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte();
            let low = (byte & VALUE_BIT_MASK) as u32;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                return result;
            }
        }
        unreachable!("Unreachable due to validation")
    }
    pub fn next_leb128_i64(&mut self) -> i64 {
        let mut result: i64 = 0;
        for i in 0..=9 {
            let byte = self.next_byte();
            let low = (byte & VALUE_BIT_MASK) as i64;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                let sign = byte & 0b01000000;
                if i == 9 {
                    return result; // No sign extend needed, as it's shifted by 63!
                }
                // sign-extend if needed
                if (sign) != 0 {
                    result |= !0 << ((i + 1) * 7);
                }

                return result;
            }
        }
        unreachable!("Unreachable due to validation")
    }
    pub fn next_leb128_i32(&mut self) -> i32 {
        let mut result: i32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte();
            let low = (byte & VALUE_BIT_MASK) as i32;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                let sign = byte & 0b01000000;
                if i == 4 {
                    return result; // No sign extend needed, as it's shifted by 28!
                }
                // sign-extend if needed
                if (sign) != 0 {
                    result |= !0 << ((i + 1) * 7);
                }

                return result;
            }
        }
        unreachable!("Unreachable due to validation")
    }
    pub fn next_f32(&mut self) -> f32 {
        let bytes = self.next_chunk(4);
        let word = u32::from_le_bytes(bytes.try_into().unwrap());
        f32::from_bits(word)
    }
    pub fn next_f64(&mut self) -> f64 {
        let bytes = self.next_chunk(8);
        let word = u64::from_le_bytes(bytes.try_into().unwrap());
        f64::from_bits(word)
    }
    fn next_chunk(&mut self, number_of_bytes: usize) -> &[u8] {
        let chunk = &self.bytecode[self.byte_pointer..self.byte_pointer + number_of_bytes];
        self.byte_pointer += number_of_bytes;
        chunk
    }
}
