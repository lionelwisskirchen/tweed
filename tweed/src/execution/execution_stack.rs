use alloc::vec::Vec;

use crate::core::error::{Error, Result};
use crate::core::values::Value;

use super::executor::Frame;

pub const MAX_VALUE_STACK_SIZE: usize = 0xf1000;
pub const MAX_CALL_STACK_SIZE: usize = 0xf1000;

#[derive(Debug)]
pub struct CallStack {
    stack: Vec<Frame>,
}

impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}

impl CallStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn push(&mut self, value: Frame) -> Result<()> {
        if self.stack.len() > MAX_CALL_STACK_SIZE {
            return Err(Error::CallStackExhaustion);
        }
        self.stack.push(value);
        Ok(())
    }
    pub fn pop(&mut self) -> Option<Frame> {
        self.stack.pop()
    }
}

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
    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() > MAX_VALUE_STACK_SIZE {
            return Err(Error::StackExhaustion);
        }
        self.stack.push(value);
        Ok(())
    }
    pub fn push_vec(&mut self, values: &[Value]) -> Result<()> {
        for value in values {
            self.push(*value)?;
        }
        Ok(())
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
    pub fn rest(&mut self) -> Vec<Value> {
        self.stack.clone()
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
    pub fn pop_f64(&mut self) -> Result<f64> {
        let Value::F64(value) = self.pop()? else {
            return Err(Error::UnexpectedValueType);
        };
        Ok(value)
    }
    pub fn pop_f32(&mut self) -> Result<f32> {
        let Value::F32(value) = self.pop()? else {
            return Err(Error::UnexpectedValueType);
        };
        Ok(value)
    }
    pub fn pop_func_ref(&mut self) -> Result<usize> {
        let Value::FuncRef(value) = self.pop()? else {
            return Err(Error::UnexpectedValueType);
        };
        Ok(value)
    }
    pub fn pop_n(&mut self, count: usize) -> Result<Vec<Value>> {
        if count > self.stack.len() {
            return Err(Error::ExpectedAValueOnTheStack);
        }
        Ok(self.stack.split_off(self.stack.len() - count))
    }
}
