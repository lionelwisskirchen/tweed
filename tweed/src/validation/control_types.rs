use crate::{binary::type_section::FuncType, core::value_types::ValType};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlType {
    Func {
        func_type: FuncType,
        sidetable_idxs: Vec<usize>,
        height_of_value_stack: u32,
        reachable: bool,
    },
    Block {
        func_type: FuncType,
        sidetable_idxs: Vec<usize>,
        height_of_value_stack: u32,
        reachable: bool,
    },
    If {
        func_type: FuncType,
        sidetable_idxs: Vec<usize>,
        side_table_entry: usize,
        reachable: bool,
        height_of_value_stack: u32,
        has_else: bool,
    },
    For {
        instruction_pointer: usize,
        func_type: FuncType,
        side_table_entry: usize,
        reachable: bool,
        height_of_value_stack: u32,
    },
}

impl ControlType {
    pub fn types(&self) -> &[ValType] {
        match self {
            ControlType::Block { func_type, .. }
            | ControlType::If { func_type, .. }
            | ControlType::Func { func_type, .. } => &func_type.result,
            ControlType::For { func_type, .. } => &func_type.parameter,
        }
    }
}
