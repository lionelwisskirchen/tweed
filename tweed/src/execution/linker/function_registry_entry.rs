use crate::execution::store::FuncInstance;

pub struct FunctionRegistryEntry {
    /// Index of the function on the store
    pub index_in_store: usize,

    /// the concrete function instance
    pub instance: FuncInstance,
}
