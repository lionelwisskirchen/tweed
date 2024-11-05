use crate::core::values::ValType;
use alloc::vec::Vec;

use super::control_types::ControlType;

#[derive(Debug)]
pub struct ValidationStack {
    stack: Vec<ValType>,
}

impl ValidationStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn set_top(&mut self, length: u32) {
	self.stack.drain(length as usize..self.height() as usize);
    }
    pub fn height(&self) -> u32 {
	self.stack.len() as u32
    } 
    pub fn push(&mut self, val_type: ValType) {
        self.stack.push(val_type);
    }
    pub fn push_vec(&mut self, val_types: &[ValType]) {
        for val_type in val_types {
            self.stack.push(*val_type);
        }
    }
    pub fn pop_val(&mut self,reachable: bool) -> ValType {
	let val = self.stack.pop();
	if reachable {
            val.expect("Value stack is empty.")
	} else {
	    if val.is_none(){
		ValType::Unknown
	    }else {
		val.unwrap()
	    }
	}
    }

    pub fn pop_expected_vec(&mut self, val_types: &[ValType],  unreachable: bool) {
	for val_type in val_types.iter().rev(){
	    self.pop_expected(*val_type, unreachable);
	}
    }
    pub fn pop_expected(&mut self, expected: ValType, unreachable: bool) {
        let actual = self.pop_val(unreachable);
	
        if expected != actual && actual != ValType::Unknown && expected != ValType::Unknown{
            panic!("Not expected Value Type on validation stack.");
        }
    }
}
