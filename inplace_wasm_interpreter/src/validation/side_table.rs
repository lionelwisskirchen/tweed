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
    pub fn len(&self) -> usize {
	self.entries.len()
    }
    pub fn create_entry(&mut self, ip: i32, stp: i32, vals: usize, pop: usize) -> usize {
        self.entries.push(SideTableEntry { ip, stp, vals, pop });
	self.entries.len().saturating_sub(1)
    }
    pub fn get_entry(&mut self, index: usize) -> &mut SideTableEntry{
	self.entries.get_mut(index).expect("Sidetable entry does not exist")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideTableEntry {
    pub ip: i32,
    pub stp: i32,
    pub vals: usize,
    pub pop: usize,
}
