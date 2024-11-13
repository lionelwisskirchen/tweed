use alloc::vec::Vec;

use crate::core::values::Value;

#[derive(Debug)]
pub struct ExecutionStack {
    stack: Vec<Value>,
}
impl Default for ExecutionStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
    }
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    pub fn push_vec(&mut self, values: &[Value]) {
        for value in values {
            self.push(*value);
        }
    }
    pub fn unwind_to(&mut self, size: usize) {
        if size > self.stack.len() {
            panic!("Error: Stack underflow happened in function!");
        }
        self.stack.drain(size..);
    }
    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("No value on Stack!")
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn pop_i32(&mut self) -> i32 {
        let Value::I32(value) = self.pop() else {
            panic!("Unexpected value type.")
        };
        value
    }
    pub fn pop_i64(&mut self) -> i64 {
        let Value::I64(value) = self.pop() else {
            panic!("Unexpected value type.")
        };
        value
    }
    pub fn pop_n(&mut self, count: usize) -> Vec<Value> {
        if count > self.stack.len() {
            panic!("Not enough value on stack for count.");
        }
        self.stack.split_off(self.stack.len() - count).to_vec()
    }
}
