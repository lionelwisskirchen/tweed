use crate::binary::type_section::FuncType;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlType {
    Func {
        instruction_pointer: usize,
        func_type: FuncType,
    },
    Block {
        instruction_pointer: usize,
        func_type: Option<FuncType>,
        sidetable_idxs: Vec<usize>,
    },
    If {
        instruction_pointer: usize,
        func_type: Option<FuncType>,
        sidetable_idxs: Vec<usize>,
        side_table_entry: usize,
    },
    For {
        instruction_pointer: usize,
        func_type: Option<FuncType>,
        side_table_entry: usize,
    },
}
