use crate::core::value_types::ValType;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ValidationStack {
    stack: Vec<ValType>,
    top_height: u32,
    top_reachable: bool,
}

impl Default for ValidationStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            top_height: 0,
            top_reachable: true,
        }
    }
    pub fn update(&mut self, height: u32, reachable: bool) {
        self.top_height = height;
        self.top_reachable = reachable;
    }
    pub fn reset(&mut self) {
        self.stack.clear();
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
    pub fn pop_val(&mut self) -> ValType {
        if self.stack.len() == self.top_height as usize && !self.top_reachable {
            return ValType::Unknown;
        };
        if self.stack.len() == self.top_height as usize {
            panic!("Stack underflow.");
        }
        self.stack.pop().expect("Value stack is empty.")
    }

    pub fn pop_expected_vec(&mut self, val_types: &[ValType]) {
        for val_type in val_types.iter().rev() {
            self.pop_expected(*val_type);
        }
    }
    pub fn pop_expected(&mut self, expected: ValType) {
        let actual = self.pop_val();

        if expected != actual && actual != ValType::Unknown && expected != ValType::Unknown {
            panic!("Not expected Value Type on validation stack.");
        }
    }
}
