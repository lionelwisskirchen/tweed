use super::error::ValidationError;
use super::validated_wasm_reader::ValidatedWasmReader;

use alloc::string::String;
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ValidationError>;
const CONTINUATION_BIT_MASK: u8 = 0x80;
const VALUE_BIT_MASK: u8 = 0x7F;
const PADDING_BIT_MASK_32: u8 = 0b01110000;
const PADDING_BIT_MASK_64: u8 = 0b01111110;

#[cfg(feature = "custom_page_size")]
const MAX_MEMORY_TYPE_FLAG_INT: u32 = 0x0F; // bits 0–3 valid
#[cfg(not(feature = "custom_page_size"))]
const MAX_MEMORY_TYPE_FLAG_INT: u32 = 0x01;

#[derive(Debug)]
pub struct MemoryTypeFlags(pub u32);

impl MemoryTypeFlags {
    /// Bit 0: a `limits.max` field (u32 or u64) follows `limits.min`.
    pub const HAS_MAX: u32 = 0x01;
    /// Bit 1 [threads]: memory is shared (requires HAS_MAX).
    pub const IS_SHARED: u32 = 0x02;
    /// Bit 2 [memory64]: memory uses 64-bit (i64) index type instead of i32.
    pub const IS_64: u32 = 0x04;
    /// Bit 3 [custom-page-sizes]: a LEB128-encoded page_sizefollows the limits.
    /// Valid values: 0 (1-byte pages), 16 (64 KiB pages).
    pub const HAS_PAGE_SIZE: u32 = 0x08;

    pub fn has_max(&self) -> bool {
        self.0 & Self::HAS_MAX != 0
    }
    pub fn is_shared(&self) -> bool {
        self.0 & Self::IS_SHARED != 0
    }
    pub fn is_64(&self) -> bool {
        self.0 & Self::IS_64 != 0
    }
    pub fn has_page_size(&self) -> bool {
        self.0 & Self::HAS_PAGE_SIZE != 0
    }
}
#[derive(Debug)]
pub struct TableTypeFlags(pub u32);

impl TableTypeFlags {
    /// Bit 0: a `limits.max` field (u32 or u64) follows `limits.min`.
    pub const HAS_MAX: u32 = 0x01;

    pub fn has_max(&self) -> bool {
        self.0 & Self::HAS_MAX != 0
    }
}

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
        String::from_utf8(bytes.to_vec()).map_err(|_| ValidationError::MalformedUTF8Encoding)
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
    /// Reads the next LEB128-encoded `u32` flag.
    ///
    /// The flag must use the canonical single-byte encoding.
    /// Multi-byte representations such as `0x81 0x00` are rejected
    /// with [`ValidationError::InvalidIntegerRepresentation`].
    /// Value not 0 or 1 are rejected with [`ValidationError::IntegerTooLarge`].
    pub fn next_flag(&mut self) -> Result<u32> {
        let before_reading = self.byte_pointer;
        let flag = self.next_leb128_u32()?;
        if self.byte_pointer - before_reading != 1 {
            return Err(ValidationError::InvalidIntegerRepresentation);
        };
        if flag > MAX_MEMORY_TYPE_FLAG_INT {
            return Err(ValidationError::IntegerTooLarge);
        }

        Ok(flag)
    }
    /// Decodes the next `count` unsigned LEB128-encoded `u32`.
    pub fn next_leb128_u32_vec(&mut self, count: u32) -> Result<Vec<u32>> {
        let mut result = vec![];
        for _ in 0..count {
            result.push(self.next_leb128_u32()?);
        }
        Ok(result)
    }

    /// Decodes the next unsigned LEB128-encoded `u32`.
    ///
    /// To be exactly conform with the specification we
    /// have the following error classes:
    /// `ValidationError::IntegerTooLarge`: PaddingBitMask is not 0
    /// or the decoded integer is not in the value range of u32
    pub fn next_leb128_u32(&mut self) -> Result<u32> {
        let mut byte = self.next_byte()?;
        let mut result: u32 = (byte & VALUE_BIT_MASK) as u32;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as u32) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as u32) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as u32) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as u32) << 28;

        let padding = byte & PADDING_BIT_MASK_32;
        if padding != 0 {
            return Err(ValidationError::IntegerTooLarge);
        }
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return Ok(result);
        }

        Err(ValidationError::InvalidIntegerRepresentation)
    }
    /// Decodes the next signed LEB128-encoded `i64`.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant.
    pub fn next_leb128_i64(&mut self) -> Result<i64> {
        let mut byte = self.next_byte()?;
        let mut result: i64 = (byte & VALUE_BIT_MASK) as i64;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return Ok(result);
        }

        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 28;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 35;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 35;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 42;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 42;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 49;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 49;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 56;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 56;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 63;
            };
            return Ok(result);
        }

        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i64) << 63;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b00000001; // Last Byte has different sign position
            let padding = byte & PADDING_BIT_MASK_64;
            let padding_valid = if sign == 0 {
                padding == 0
            } else {
                padding == PADDING_BIT_MASK_64
            };
            if !padding_valid {
                return Err(ValidationError::IntegerTooLarge);
            }
            return Ok(result); // No sign extend needed, as it's shifted by 28!
        }

        Err(ValidationError::InvalidIntegerRepresentation)
    }

    /// Decodes the next signed LEB128-encoded `i32`.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant.
    pub fn next_leb128_i32(&mut self) -> Result<i32> {
        let mut byte = self.next_byte()?;
        let mut result: i32 = (byte & VALUE_BIT_MASK) as i32;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i32) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i32) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i32) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return Ok(result);
        }
        byte = self.next_byte()?;
        result |= ((byte & VALUE_BIT_MASK) as i32) << 28;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b00001000; // Last Byte has different sign position
            let padding = byte & PADDING_BIT_MASK_32;
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
