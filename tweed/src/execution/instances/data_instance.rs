use crate::core::error::ExecutionError;

use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataInstance {
    pub data: Vec<u8>,
}
impl DataInstance {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn load_slice(&self, address: usize, bytes: usize) -> Result<Vec<u8>> {
        self.data
            .get(address..address + bytes)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)
            .map(|s| s.to_vec())
    }
}
