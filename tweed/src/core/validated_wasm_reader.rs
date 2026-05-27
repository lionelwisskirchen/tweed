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

    /// Decodes the next unsigned LEB128-encoded `u32` value without unsafe.
    /// Compared to the `unsafe` variant, this implementation prioritizes safety
    /// over performance and avoids unchecked memory access.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is not allowed.
    #[cfg(not(feature = "unsafe"))]
    pub fn next_leb128_u32(&mut self) -> u32 {
        let mut byte = self.next_byte();
        let mut result: u32 = (byte & VALUE_BIT_MASK) as u32;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as u32) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as u32) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as u32) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as u32) << 28;
        result
    }

    /// Reads the next unsigned LEB128-encoded `u32` value using unchecked
    /// bytecode access. This implementation skips bounds checks for maximum decoding
    /// performance and is theoretically safe,if the bytecode stream was validated
    /// during an earlier verification phase.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is allowed.
    #[cfg(feature = "unsafe")]
    pub fn next_leb128_u32(&mut self) -> u32 {
        let b0 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer) };
        let mut res = (b0 & VALUE_BIT_MASK) as u32;
        if b0 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 1;
            return res;
        }

        let b1 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 1) };
        res |= ((b1 & VALUE_BIT_MASK) as u32) << 7;
        if b1 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 2;
            return res;
        }

        let b2 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 2) };
        res |= ((b2 & VALUE_BIT_MASK) as u32) << 14;
        if b2 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 3;
            return res;
        }

        let b3 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 3) };
        res |= ((b3 & VALUE_BIT_MASK) as u32) << 21;
        if b3 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 4;
            return res;
        }

        let b4 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 4) };
        res |= ((b4 & VALUE_BIT_MASK) as u32) << 28;
        self.byte_pointer += 5;

        res
    }

    /// Reads the next signed LEB128-encoded `i64` value using unchecked
    /// bytecode access. This implementation skips bounds checks for maximum decoding
    /// performance and is theoretically safe,if the bytecode stream was validated
    /// during an earlier verification phase.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is allowed.    
    #[cfg(feature = "unsafe")]
    pub fn next_leb128_i64(&mut self) -> i64 {
        let b0 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer) };
        let mut result = (b0 & VALUE_BIT_MASK) as i64;
        if b0 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 1;
            let sign = b0 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return result;
        }

        let b1 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 1) };
        result |= ((b1 & VALUE_BIT_MASK) as i64) << 7;
        if b1 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 2;
            let sign = b1 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return result;
        }

        let b2 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 2) };
        result |= ((b2 & VALUE_BIT_MASK) as i64) << 14;
        if b2 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 3;
            let sign = b2 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return result;
        }

        let b3 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 3) };
        result |= ((b3 & VALUE_BIT_MASK) as i64) << 21;
        if b3 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 4;
            let sign = b3 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return result;
        }

        let b4 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 4) };
        result |= ((b4 & VALUE_BIT_MASK) as i64) << 28;
        if b4 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 5;
            let sign = b4 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 35;
            };
            return result;
        }

        let b5 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 5) };
        result |= ((b5 & VALUE_BIT_MASK) as i64) << 35;
        if b5 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 6;
            let sign = b5 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 42;
            };
            return result;
        }

        let b6 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 6) };
        result |= ((b6 & VALUE_BIT_MASK) as i64) << 42;
        if b6 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 7;
            let sign = b6 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 49;
            };
            return result;
        }

        let b7 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 7) };
        result |= ((b7 & VALUE_BIT_MASK) as i64) << 49;
        if b7 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 8;
            let sign = b7 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 56;
            };
            return result;
        }

        let b8 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 8) };
        result |= ((b8 & VALUE_BIT_MASK) as i64) << 56;
        if b8 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 9;
            let sign = b8 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 63;
            };
            return result;
        }
        let b9 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 9) };
        result |= ((b9 & VALUE_BIT_MASK) as i64) << 63;
        self.byte_pointer += 10;
        result
    }
    /// Decodes the next signed LEB128-encoded `i64` value without unsafe.
    /// Compared to the `unsafe` variant, this implementation prioritizes safety
    /// over performance and avoids unchecked memory access.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is not allowed.
    #[cfg(not(feature = "unsafe"))]
    pub fn next_leb128_i64(&mut self) -> i64 {
        let mut byte = self.next_byte();
        let mut result: i64 = (byte & VALUE_BIT_MASK) as i64;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return result;
        }

        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 28;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 35;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 35;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 42;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 42;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 49;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 49;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 56;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 56;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 63;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i64) << 63;
        result
    }

    /// Decodes the next signed LEB128-encoded `i32` value without unsafe.
    /// Compared to the `unsafe` variant, this implementation prioritizes safety
    /// over performance and avoids unchecked memory access.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is not allowed.
    #[cfg(not(feature = "unsafe"))]
    pub fn next_leb128_i32(&mut self) -> i32 {
        let mut byte = self.next_byte();
        let mut result: i32 = (byte & VALUE_BIT_MASK) as i32;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i32) << 7;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i32) << 14;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i32) << 21;
        if (byte & CONTINUATION_BIT_MASK) == 0 {
            let sign = byte & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return result;
        }
        byte = self.next_byte();
        result |= ((byte & VALUE_BIT_MASK) as i32) << 28;
        result
    }

    /// Reads the next signed LEB128-encoded `i32` value using unchecked
    /// bytecode access. This implementation skips bounds checks for maximum decoding
    /// performance and is theoretically safe,if the bytecode stream was validated
    /// during an earlier verification phase.
    ///
    /// Benchmarks indicate that this implementation is currently the
    /// fastest available decoding variant, when unsafe is allowed.
    #[cfg(feature = "unsafe")]
    pub fn next_leb128_i32(&mut self) -> i32 {
        let b0 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer) };
        let mut result = (b0 & VALUE_BIT_MASK) as i32;
        if b0 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 1;
            let sign = b0 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 7;
            };
            return result;
        }

        let b1 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 1) };
        result |= ((b1 & VALUE_BIT_MASK) as i32) << 7;
        if b1 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 2;
            let sign = b1 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 14;
            };
            return result;
        }

        let b2 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 2) };
        result |= ((b2 & VALUE_BIT_MASK) as i32) << 14;
        if b2 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 3;
            let sign = b2 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 21;
            };
            return result;
        }

        let b3 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 3) };
        result |= ((b3 & VALUE_BIT_MASK) as i32) << 21;
        if b3 & CONTINUATION_BIT_MASK == 0 {
            self.byte_pointer += 4;
            let sign = b3 & 0b01000000;
            // sign-extend if needed
            if (sign) != 0 {
                result |= !0 << 28;
            };
            return result;
        }

        let b4 = unsafe { *self.bytecode.get_unchecked(self.byte_pointer + 4) };
        result |= ((b4 & VALUE_BIT_MASK) as i32) << 28;
        self.byte_pointer += 5;

        result
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
