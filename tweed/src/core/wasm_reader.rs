use crate::core::error::{Error, Result};
use alloc::string::String;
use alloc::vec::Vec;

const CONT: u8 = 0x80;
const MASK: u8 = 0x7F;

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
    pub fn next_byte(&mut self) -> Option<&u8> {
        let curr = self.bytecode.get(self.byte_pointer);
        self.byte_pointer += 1;
        curr
    }
    pub fn skip_block_type(&mut self) -> Result<u32> {
        self.next_leb128_u32()
    }
    pub fn skip_depth(&mut self) -> Result<u32> {
        self.next_leb128_u32()
    }
    pub fn read_mem_args(&mut self) -> Result<(u32, u32)> {
        let _align = self.next_leb128_u32().map_err(|_| Error::UndefinedMemory)?;
        let offset = self.next_leb128_u32().map_err(|_| Error::UndefinedMemory)?;
        Ok((_align, offset))
    }
    pub fn seek(&mut self, position: usize) {
        self.byte_pointer = position
    }
    pub fn read_until(&mut self, target: u8) -> Option<usize> {
        while let Some(byte) = self.next_byte() {
            if byte == &target {
                return Some(self.current_index());
            }
        }
        None
    }
    pub fn current_index(&self) -> usize {
        self.byte_pointer.saturating_sub(1)
    }
    pub fn next_leb128_u32(&mut self) -> Result<u32> {
        let mut result: u32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte().ok_or(Error::ReadingOutOfBounds)?;
            let low = (byte & MASK) as u32;
            result |= low << (7 * i);
            if (byte & CONT) == 0 {
                if i == 4 {
                    let padding = byte & 0b01110000;
                    if padding != 0 {
                        return Err(Error::IntegerTooLarge);
                    }
                }
                return Ok(result);
            }
        }
        Err(Error::InvalidIntegerRepresentation)
    }
    pub fn next_leb128_i64(&mut self) -> Result<i64> {
        let mut result: i64 = 0;
        for i in 0..=9 {
            let byte = self.next_byte().ok_or(Error::ReadingOutOfBounds)?;
            let low = (byte & MASK) as i64;
            result |= low << (7 * i);
            if (byte & CONT) == 0 {
                let mut sign = byte & 0b01000000;
                if i == 9 {
                    let padding = byte & 0b01111110;
                    sign = byte & 0b00000001; // Last Byte has different sign position
                    let padding_valid = if sign == 0 {
                        padding == 0
                    } else {
                        padding == 0b01111110
                    };
                    if !padding_valid {
                        return Err(Error::IntegerTooLarge);
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
        Err(Error::IntegerTooLarge)
    }
    pub fn next_leb128_i32(&mut self) -> Result<i32> {
        let mut result: i32 = 0;
        for i in 0..=4 {
            let byte = self.next_byte().ok_or(Error::ReadingOutOfBounds)?;
            let low = (byte & MASK) as i32;
            result |= low << (7 * i);
            if (byte & CONT) == 0 {
                let mut sign = byte & 0b01000000;
                if i == 4 {
                    let padding = byte & 0b01110000;
                    sign = byte & 0b00001000; // Last Byte has different sign position
                    let padding_valid = if sign == 0 {
                        padding == 0
                    } else {
                        padding == 0b01110000
                    };

                    if !padding_valid {
                        return Err(Error::IntegerTooLarge);
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
        Err(Error::IntegerTooLarge)
    }
    pub fn next_f32(&mut self) -> Result<f32> {
        let bytes = self.next_chunk(4).ok_or(Error::ReadingOutOfBounds)?;
        let word = u32::from_le_bytes(bytes.try_into().unwrap());
        Ok(f32::from_bits(word))
    }
    pub fn next_f64(&mut self) -> Result<f64> {
        let bytes = self.next_chunk(8).ok_or(Error::ReadingOutOfBounds)?;
        let word = u64::from_le_bytes(bytes.try_into().unwrap());
        Ok(f64::from_bits(word))
    }
    pub fn calculate_offset(&self, offset: usize) -> usize {
        offset + self.current_index()
    }
    pub fn next_chunk(&mut self, number_of_bytes: usize) -> Option<&[u8]> {
        let chunk = self
            .bytecode
            .get(self.byte_pointer..self.byte_pointer + number_of_bytes);
        self.byte_pointer += number_of_bytes;
        chunk
    }
    pub fn next_chunk_as_string(&mut self, number_of_bytes: usize) -> Option<String> {
        let bytes = self.next_chunk(number_of_bytes)?;
        String::from_utf8(bytes.to_vec()).ok()
    }
    pub fn compare_chunk(&mut self, expected: &[u8]) -> Option<&[u8]> {
        let slice = self.next_chunk(expected.len())?;

        if slice == expected {
            Some(slice)
        } else {
            None
        }
    }
    pub fn rest(&self) -> &[u8] {
        &self.bytecode[self.byte_pointer..]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_next_chunk() {
//         let bytes = [0x04, 0x03, 0x05];
//         let mut reader = WasmReader::new(bytes.to_vec());
//         reader.next_byte();
//         assert_eq!(*reader.next_chunk(0).unwrap(), vec![]);
//     }

//     #[test]
//     fn test_calculate_offset() {
//         let bytes = [0x04, 0x03, 0x05];
//         let mut reader = WasmReader::new(bytes.to_vec());
//         reader.next_byte();
//         reader.next_byte();
//         assert_eq!(reader.calculate_offset(1), 2);
//     }
//     #[test]
//     fn test_compare_chunk_match() {
//         let mut reader = WasmReader::new(vec![1, 2, 3, 4, 5]);
//         assert_eq!(reader.compare_chunk(&[1, 2, 3]), Some(&[1, 2, 3][..]));
//     }

//     #[test]
//     fn test_compare_chunk_no_match() {
//         let mut reader = WasmReader::new(vec![1, 2, 3, 4, 5]);
//         assert_eq!(reader.compare_chunk(&[1, 2, 4]), None);
//     }

//     #[test]
//     fn test_compare_chunk_length_exceeds_data() {
//         let mut reader = WasmReader::new(vec![1, 2, 3]);
//         assert_eq!(reader.compare_chunk(&[1, 2, 3, 4]), None);
//     }

//     #[test]
//     fn test_compare_chunk_partial_match() {
//         let mut reader = WasmReader::new(vec![1, 2, 3, 4, 5]);
//         assert_eq!(reader.compare_chunk(&[1, 2, 3, 6]), None);
//     }

//     #[test]
//     fn test_compare_chunk_empty_expected() {
//         let mut reader = WasmReader::new(vec![1, 2, 3, 4, 5]);
//         assert_eq!(reader.compare_chunk(&[]), Some(&[][..]));
//     }

//     #[test]
//     fn test_compare_chunk_not_empty_but_mismatch() {
//         let mut reader = WasmReader::new(vec![1, 2, 3]);
//         assert_eq!(reader.compare_chunk(&[3, 2, 1]), None);
//     }

//     #[test]
//     fn test_pointer_movement() {
//         let bytes = [0x04, 0x03, 0x05];
//         let mut reader = WasmReader::new(bytes.to_vec());
//         assert_eq!(reader.current_index(), 0);
//         assert_eq!(*reader.next_byte().unwrap(), 0x04);
//         assert_eq!(reader.current_index(), 0);

//         assert_eq!(*reader.next_byte().unwrap(), 0x03);
//         assert_eq!(reader.current_index(), 1);

//         assert_eq!(*reader.next_byte().unwrap(), 0x05);
//         assert_eq!(reader.current_index(), 2);

//         reader.seek(0);
//         assert_eq!(reader.current_index(), 0);
//     }
//     #[test]
//     fn test_next_leb128_i32_no_byte() {
//         let mut decoder = WasmReader::new(vec![]);
//         assert_eq!(decoder.next_leb128_i32(), None);
//     }
//     #[test]
//     fn test_next_leb128_u32_no_byte() {
//         let mut decoder = WasmReader::new(vec![]);
//         assert_eq!(decoder.next_leb128_u32(), None);
//     }
//     #[test]
//     fn test_next_leb128_u32_single_byte() {
//         let mut decoder = WasmReader::new(vec![0x01]);
//         assert_eq!(decoder.next_leb128_u32(), Some(1));
//     }
//     #[test]
//     fn test_next_leb128_u32_multi_byte() {
//         let mut decoder = WasmReader::new(vec![0xE5, 0x8E, 0x26]);
//         assert_eq!(decoder.next_leb128_u32(), Some(624485));
//     }

//     #[test]
//     fn test_next_leb128_u32_overflow() {
//         let mut decoder = WasmReader::new(vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
//         assert_eq!(decoder.next_leb128_u32(), None);
//     }

//     #[test]
//     fn test_next_leb128_u32_large_value() {
//         let mut decoder = WasmReader::new(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07]);
//         assert_eq!(decoder.next_leb128_u32(), Some(2147483647));
//     }

//     #[test]
//     fn test_next_leb128_u32_zero() {
//         let mut decoder = WasmReader::new(vec![0x00]);
//         assert_eq!(decoder.next_leb128_u32(), Some(0));
//     }
//     #[test]
//     fn test_next_leb128_i32_negative() {
//         let mut decoder = WasmReader::new(vec![0x7f]);
//         assert_eq!(decoder.next_leb128_i32(), Some(-1));
//     }
//     #[test]
//     fn test_next_leb128_i64_negative() {
//         let mut decoder = WasmReader::new(vec![0x7f]);
//         assert_eq!(decoder.next_leb128_i64(), Some(-1));
//     }
// }
