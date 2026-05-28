use alloc::vec::Vec;

use crate::core::{
    error::ExecutionError,
    values::{V128Shape, Value},
};

use super::executor::Frame;

pub const MAX_VALUE_STACK_SIZE: usize = 0xf1000;
pub const MAX_CALL_STACK_SIZE: usize = 0xf1000;
pub type Result<T> = core::result::Result<T, ExecutionError>;

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
            return Err(ExecutionError::CallStackExhaustion);
        }
        self.stack.push(value);
        Ok(())
    }
    pub fn pop(&mut self) -> Option<Frame> {
        self.stack.pop()
    }

    pub fn reset(&mut self) {
        self.stack.clear();
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
            return Err(ExecutionError::StackExhaustion);
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
            panic!("ValidationError: Stack underflow happened in function!");
        }
        self.stack.drain(size..);
    }
    #[cfg(not(feature = "unsafe"))]
    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
    #[cfg(feature = "unsafe")]
    pub fn pop(&mut self) -> Value {
        unsafe {
            let len = self.len();
            let ptr = self.stack.as_mut_ptr().add(len - 1);
            self.stack.set_len(len - 1);
            ptr.read()
        }
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
    pub fn pop_i32(&mut self) -> i32 {
        let value = self.pop();
        let Value::I32(v) = value else {
            unreachable!("Expected I32 on stack (validated)");
        };
        v
    }
    pub fn pop_i64(&mut self) -> i64 {
        let value = self.pop();
        let Value::I64(v) = value else {
            unreachable!("Expected I64 on stack (validated)");
        };
        v
    }
    pub fn pop_f64(&mut self) -> f64 {
        let value = self.pop();
        let Value::F64(v) = value else {
            unreachable!("Expected F64 on stack (validated)");
        };
        v
    }
    pub fn pop_f32(&mut self) -> f32 {
        let value = self.pop();
        let Value::F32(v) = value else {
            unreachable!("Expected F32 on stack (validated)");
        };
        v
    }
    pub fn pop_func_ref(&mut self) -> usize {
        let value = self.pop();
        let Value::FuncRef(v) = value else {
            unreachable!("Expected FuncRef on stack (validated)");
        };
        v
    }
    pub fn pop_v128(&mut self) -> Result<V128Shape> {
        let Value::V128(value) = self.pop() else {
            return Err(ExecutionError::UnexpectedValueType);
        };
        Ok(value)
    }

    pub fn pop_n(&mut self, count: usize) -> Result<Vec<Value>> {
        if count > self.stack.len() {
            return Err(ExecutionError::ExpectedAValueOnTheStack);
        }
        Ok(self.stack.split_off(self.stack.len() - count))
    }
}
