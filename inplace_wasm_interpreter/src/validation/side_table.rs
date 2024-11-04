use alloc::vec::Vec;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideTable {
    pub entries: Vec<SideTableEntry>,
}
impl Default for SideTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SideTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideTableEntry {
    pub ip: i32,
    pub stp: i32,
    pub vals: usize,
    pub pop: usize,
}
