use crate::binary::type_section::FuncType;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlType {
    Func {
        instruction_pointer: usize,
        func_type: FuncType,
	reachable: bool,

    },
    Block {
        instruction_pointer: usize,
        func_type: FuncType,
        sidetable_idxs: Vec<usize>,
	reachable: bool,
	height_of_value_stack: u32

    },
    If {
        instruction_pointer: usize,
        func_type: FuncType,
        sidetable_idxs: Vec<usize>,
        side_table_entry: usize,
	reachable: bool,
	height_of_value_stack: u32
    },
    For {
        instruction_pointer: usize,
        func_type: FuncType,
        side_table_entry: usize,
	reachable: bool,
	height_of_value_stack: u32

    },
}
