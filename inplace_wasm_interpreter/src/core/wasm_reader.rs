use alloc::vec::Vec;

#[derive(Debug)]
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
    pub fn seek(&mut self, position: usize) {
        self.byte_pointer = position
    }
    pub fn current_index(&self) -> usize {
        self.byte_pointer.saturating_sub(1)
    }
    pub fn next_leb128_u32(&mut self) -> Option<u32> {
        let mut result: u32 = 0;
        let mut shift = 0;
        while let Some(byte) = self.next_byte() {
            let low = (byte & 0x7f) as u32; // 0111 1111 Mask -> MSB signals next byte is also relevant

            result |= low << shift;
            if (byte & 0x80) == 0 {
                // No next byte, Algorithm is finished
                break;
            }
            shift += 7;
            if shift > 32 {
                return None;
            }
        }
        Some(result)
    }
    pub fn next_leb128_i32(&mut self) -> Option<i32> {
        let mut result: i32 = 0;
        let mut shift = 0;
        while let Some(byte) = self.next_byte() {
            let low = (byte & 0x7f) as i32; // 0111 1111 Mask -> MSB signals next byte is also relevant

            result |= low << shift;
            if (byte & 0x80) == 0 {
                // No next byte, Algorithm is finished
                break;
            }
            shift += 7;
            if shift > 32 {
                return None;
            }
        }
        Some(result)
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
    pub fn get_slice(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.bytecode.get(start..=end)
    }
    pub fn compare_chunk(&mut self, number_of_bytes: usize, expected: &[u8]) -> Option<&[u8]> {
        let slice = self.next_chunk(number_of_bytes)?;
        if slice == expected {
            return Some(slice);
        }
        None
    }
    pub fn rest(&self) -> &[u8] {
        &self.bytecode[self.byte_pointer..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_chunk() {
        let bytes = [0x04, 0x03, 0x05];
        let mut reader = WasmReader::new(bytes.to_vec());
        reader.next_byte();
        assert_eq!(*reader.next_chunk(0).unwrap(), vec![]);
    }
    #[test]
    fn test_calculate_offset() {
        let bytes = [0x04, 0x03, 0x05];
        let mut reader = WasmReader::new(bytes.to_vec());
        reader.next_byte();
        reader.next_byte();
        assert_eq!(reader.calculate_offset(1), 2);
    }

    #[test]
    fn test_pointer_movement() {
        let bytes = [0x04, 0x03, 0x05];
        let mut reader = WasmReader::new(bytes.to_vec());
        assert_eq!(reader.current_index(), 0);
        assert_eq!(*reader.next_byte().unwrap(), 0x04);
        assert_eq!(reader.current_index(), 0);

        assert_eq!(*reader.next_byte().unwrap(), 0x03);
        assert_eq!(reader.current_index(), 1);

        assert_eq!(*reader.next_byte().unwrap(), 0x05);
        assert_eq!(reader.current_index(), 2);

        reader.seek(0);
        assert_eq!(reader.current_index(), 0);
    }
}
