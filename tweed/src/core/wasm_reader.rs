use super::error::ValidationError;
use super::validated_wasm_reader::ValidatedWasmReader;

use alloc::string::String;
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ValidationError>;
const CONTINUATION_BIT_MASK: u8 = 0x80;
const VALUE_BIT_MASK: u8 = 0x7F;
const PADDING_BIT_MASK_32: u8 = 0b01110000;
const PADDING_BIT_MASK_64: u8 = 0b01111110;

/// A **safe** bytecode reader for WebAssembly modules.
///
/// `WasmReader` provides sequential read access to a byte vector
/// containing WebAssembly binary data.
///
/// This reader is intended for the *validation* phase and performs
/// strict error handling to ensure that malformed or truncated input
/// is detected early.
#[derive(Debug, Default, Clone)]
pub struct WasmReader {
    bytecode: Vec<u8>,
    byte_pointer: usize,
}

impl WasmReader {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            bytecode: input,
            byte_pointer: 0,
        }
    }
    /// Converts this reader into a validated reader with unchecked access.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the underlying bytecode has been fully
    /// validated according to the WebAssembly specification. Using the returned
    /// reader on unvalidated input may cause undefined behavior.
    pub fn assume_valid(self) -> ValidatedWasmReader {
        let mut reader = ValidatedWasmReader::new(self.bytecode);
        reader.seek(self.byte_pointer);
        reader
    }
    pub fn next_chunk_as_string(&mut self, number_of_bytes: usize) -> Result<String> {
        let bytes = self.next_chunk(number_of_bytes)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| ValidationError::InvalidStringRepresentation)
    }

    pub fn next_byte(&mut self) -> Result<u8> {
        let curr = self.bytecode.get(self.byte_pointer);
        self.byte_pointer += 1;
        curr.ok_or(ValidationError::ReadingOutOfBounds).copied()
    }
    pub fn seek(&mut self, position: usize) {
        self.byte_pointer = position
    }
    pub fn current_index(&self) -> usize {
        self.byte_pointer.saturating_sub(1)
    }

    pub fn get_bytecode_size(&self) -> usize {
        self.bytecode.len()
    }

    pub fn next_leb128_u32(&mut self) -> Result<u32> {
        let mut result: u32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte()?;
            let low = (byte & VALUE_BIT_MASK) as u32;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                if i == 4 {
                    let padding = byte & PADDING_BIT_MASK_32;
                    if padding != 0 {
                        return Err(ValidationError::IntegerTooLarge);
                    }
                }
                return Ok(result);
            }
        }
        Err(ValidationError::InvalidIntegerRepresentation)
    }
    pub fn next_leb128_i64(&mut self) -> Result<i64> {
        let mut result: i64 = 0;
        for i in 0..=9 {
            let byte = self.next_byte()?;
            let low = (byte & VALUE_BIT_MASK) as i64;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                let mut sign = byte & 0b01000000;
                if i == 9 {
                    let padding = byte & PADDING_BIT_MASK_64;
                    sign = byte & 0b00000001; // Last Byte has different sign position
                    let padding_valid = if sign == 0 {
                        padding == 0
                    } else {
                        padding == PADDING_BIT_MASK_64
                    };
                    if !padding_valid {
                        return Err(ValidationError::IntegerTooLarge);
                    }
                    return Ok(result); // No sign extend needed, as it's shifted by 63!
                }
                // sign-extend if needed
                if (sign) != 0 {
                    result |= !0 << ((i + 1) * 7);
                }

                return Ok(result);
            }
        }
        Err(ValidationError::InvalidIntegerRepresentation)
    }
    pub fn next_leb128_i32(&mut self) -> Result<i32> {
        let mut result: i32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte()?;
            let low = (byte & VALUE_BIT_MASK) as i32;
            result |= low << (7 * i);
            if (byte & CONTINUATION_BIT_MASK) == 0 {
                let mut sign = byte & 0b01000000;
                if i == 4 {
                    let padding = byte & PADDING_BIT_MASK_32;
                    sign = byte & 0b00001000; // Last Byte has different sign position
                    let padding_valid = if sign == 0 {
                        padding == 0
                    } else {
                        padding == PADDING_BIT_MASK_32
                    };

                    if !padding_valid {
                        return Err(ValidationError::IntegerTooLarge);
                    }
                    return Ok(result); // No sign extend needed, as it's shifted by 28!
                }
                // sign-extend if needed
                if (sign) != 0 {
                    result |= !0 << ((i + 1) * 7);
                }

                return Ok(result);
            }
        }
        Err(ValidationError::InvalidIntegerRepresentation)
    }
    pub fn next_f32(&mut self) -> Result<f32> {
        let bytes = self.next_chunk(4)?;
        let word = u32::from_le_bytes(bytes.try_into().unwrap());
        Ok(f32::from_bits(word))
    }
    pub fn next_f64(&mut self) -> Result<f64> {
        let bytes = self.next_chunk(8)?;
        let word = u64::from_le_bytes(bytes.try_into().unwrap());
        Ok(f64::from_bits(word))
    }
    pub fn next_chunk(&mut self, number_of_bytes: usize) -> Result<&[u8]> {
        let chunk = self
            .bytecode
            .get(self.byte_pointer..self.byte_pointer + number_of_bytes);
        self.byte_pointer += number_of_bytes;
        chunk.ok_or(ValidationError::ReadingOutOfBounds)
    }

    pub fn compare_chunk(&mut self, expected: &[u8]) -> Result<&[u8]> {
        let slice = self.next_chunk(expected.len())?;

        if slice == expected {
            Ok(slice)
        } else {
            Err(ValidationError::UnequalChunkComparison)
        }
    }
    pub fn rest(&self) -> &[u8] {
        &self.bytecode[self.byte_pointer..]
    }
}
