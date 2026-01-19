use crate::execution::store::TableInstance;

#[derive(Debug, Clone)]
pub struct TableRegistryEntry {
    /// Index of the table inside the current wasm module
    pub index_in_module: u32,

    /// Index of the table on the store
    pub index_in_store: usize,

    /// the concrete table instance
    pub instance: TableInstance,
}
