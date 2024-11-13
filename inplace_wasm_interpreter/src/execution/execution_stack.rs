
use alloc::vec::Vec;

use crate::core::values::Value;

#[derive(Debug)]
pub struct ExecutionStack {
    stack: Vec<Value>,
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
    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("No value on Stack!")
    }
    pub fn len(&self) -> usize {
        self.stack.len()
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
