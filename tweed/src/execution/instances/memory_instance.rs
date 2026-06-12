use crate::binary::import_section::MemoryImport;
use crate::binary::memory_section::{Limits, Memory, DEFAULT_WASM_PAGE_SIZE};

use crate::core::error::ExecutionError;
use crate::core::values::FromLeBytes;

use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, Clone)]
pub struct MemoryInstance {
    pub memory_type: Memory,
    pub bytes: Vec<u8>,
}

impl MemoryInstance {
    pub fn new(bytes: Vec<u8>, limits: Limits, mempagesize: u32) -> Self {
        MemoryInstance {
            memory_type: Memory {
                limits,
                mempagesize,
            },
            bytes,
        }
    }

    /// Check if the given memory has the same limits
    ///
    /// # Arguments
    /// * `memory_import`: reference to the import memory.
    ///
    /// # Return
    /// Error if the limits are not compatible
    pub fn ensure_limits_are_correct(&self, memory_import: &MemoryImport) -> Result<()> {
        let export = &self.memory_type.limits;
        let import = &memory_import.context.limits;

        if export.min < import.min {
            return Err(ExecutionError::IncompatibleImportType);
        }

        match (export.max, import.max) {
            (Some(exp_max), Some(imp_max)) => {
                if exp_max > imp_max {
                    return Err(ExecutionError::IncompatibleImportType);
                }
            }

            (None, Some(_)) => {
                return Err(ExecutionError::IncompatibleImportType);
            }

            _ => {}
        }
        Ok(())
    }

    pub fn load_slice(&mut self, address: usize, bytes: usize) -> Result<&[u8]> {
        let dst_end = address
            .checked_add(bytes)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;

        self.bytes
            .get(address..dst_end)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)
    }

    /// The size of the memory in bytes
    pub fn size(&self) -> u32 {
        self.bytes.len() as u32
    }

    /// The number of pages the memory holds
    pub fn pages(&self) -> u32 {
        (self
            .bytes
            .len()
            .div_ceil(self.memory_type.mempagesize as usize)) as u32
    }

    ///  Grow the memory by n pages
    ///
    /// # Arguments
    /// * `amount`: the number of pages appended to the memory.
    ///
    /// # Return
    /// Ok() if everything is fine.
    ///
    /// # Error
    /// * `ExecutionError::MemoryAccessOutOfBounds`: The memory is grown by too much.
    pub fn grow(&mut self, amount: u32) -> Result<()> {
        let mempagesize = self.memory_type.mempagesize;
        // Assert due to validation that pagesize is either the value 1 or the value 65536
        debug_assert!(mempagesize == 1 || mempagesize == DEFAULT_WASM_PAGE_SIZE);

        let len = amount + self.pages();
        if len > u32::MAX.div_ceil(mempagesize) {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        }

        if let Some(max) = self.memory_type.limits.max {
            if len > max {
                return Err(ExecutionError::MemoryAccessOutOfBounds);
            }
        }
        self.bytes
            .resize((len * self.memory_type.mempagesize) as usize, 0);
        self.memory_type.limits.min += amount;

        Ok(())
    }

    pub fn load<T>(&mut self, address: usize, bytes: usize) -> Result<T>
    where
        T: FromLeBytes,
    {
        let data_bytes = self.load_slice(address, bytes)?;
        let data = T::from_le_bytes(data_bytes);
        Ok(data)
    }
    pub fn store(&mut self, address: usize, offset: usize, data: &[u8]) -> Result<()> {
        let abs_address = offset
            .checked_add(address)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
        let end = abs_address
            .checked_add(data.len())
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
        if end > self.bytes.len() {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        }

        self.bytes[abs_address..end].copy_from_slice(data);
        Ok(())
    }
}
