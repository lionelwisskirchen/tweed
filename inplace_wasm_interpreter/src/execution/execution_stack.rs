use alloc::vec::Vec;

use crate::core::error::{Error, Result};
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
    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::ExpectedAValueOnTheStack)
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn pop_i32(&mut self) -> Result<i32> {
        let Value::I32(value) = self.pop()? else {
            return Err(Error::UnexpectedValueType);
        };
        Ok(value)
    }
    pub fn pop_i64(&mut self) -> Result<i64> {
        let Value::I64(value) = self.pop()? else {
            return Err(Error::UnexpectedValueType);
        };
        Ok(value)
    }
    pub fn pop_n(&mut self, count: usize) -> Result<Vec<Value>> {
        if count > self.stack.len() {
            return Err(Error::ExpectedAValueOnTheStack);
        }
        Ok(self.stack.split_off(self.stack.len() - count).to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_empty() {
        let mut stack = ExecutionStack::default();
        assert!(stack.is_empty());
        stack.push(Value::I32(2));
        assert!(!stack.is_empty());
    }
    #[should_panic]
    #[test]
    fn test_underflow() {
        let mut stack = ExecutionStack::default();
        stack.unwind_to(1);
    }
    #[test]
    fn test_pop_unexpected_i64() {
        let mut stack = ExecutionStack::default();
        stack.push(Value::I32(2));
        assert!(stack.pop_i64().is_err());
    }

    #[test]
    fn test_pop_unexpected_i32() {
        let mut stack = ExecutionStack::default();
        stack.push(Value::I64(2));
        assert!(stack.pop_i32().is_err());
    }

    #[test]
    fn test_pop_unexpected_vec() {
        let mut stack = ExecutionStack::default();
        stack.push(Value::I64(2));
        assert!(stack.pop_n(2).is_err());
    }

    #[test]
    fn test_pop_i64() {
        let mut stack = ExecutionStack::default();
        stack.push(Value::I64(2));
        assert_eq!(stack.pop_i64().unwrap(), 2);
    }
}
