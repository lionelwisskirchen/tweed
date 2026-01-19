use crate::execution::store::MemoryInstance;

#[derive(Debug, Clone)]
pub struct MemoryRegistryEntry {
    /// Index of the memory inside the current wasm module
    pub index_in_module: u32,

    /// Index of the memory on the store
    pub index_in_store: usize,

    /// the concrete memory instance
    pub instance: MemoryInstance,
}
