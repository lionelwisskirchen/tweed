use crate::core::error::{Result, ValidationError};
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
    pub fn is_empty(&mut self) -> bool {
        self.stack.is_empty()
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
            self.push(*val_type);
        }
    }
    pub fn pop_val(&mut self) -> Result<ValType> {
        if self.stack.len() == self.top_height as usize && !self.top_reachable {
            return Ok(ValType::Unknown);
        };
        if self.stack.len() == self.top_height as usize {
            return Err(ValidationError::TypeMismatch);
        }
        self.stack.pop().ok_or(ValidationError::TypeMismatch)
    }

    pub fn pop_expected_vec(&mut self, val_types: &[ValType]) -> Result<()> {
        for val_type in val_types.iter().rev() {
            self.pop_expected(*val_type)?;
        }
        Ok(())
    }
    pub fn pop_expected(&mut self, expected: ValType) -> Result<()> {
        let actual = self.pop_val()?;

        if expected != actual && actual != ValType::Unknown && expected != ValType::Unknown {
            return Err(ValidationError::TypeMismatch);
        }
        Ok(())
    }
    pub fn is_on_top(&mut self, expected: &ValType) -> Result<()> {
        if self.stack.len() == self.top_height as usize && !self.top_reachable {
            return Ok(());
        };
        let val = self.stack.last().ok_or(ValidationError::TypeMismatch)?;
        if val == expected {
            Ok(())
        } else {
            Err(ValidationError::TypeMismatch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_top() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::I32);
        validator.push(ValType::I64);

        assert!(validator.is_on_top(&ValType::I64).is_ok());
    }

    #[test]
    fn test_pop_expected_with_matching_type() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::I32);

        assert!(validator.pop_expected(ValType::I32).is_ok());
    }

    #[test]
    fn test_pop_expected_with_different_type() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::I64);

        assert!(validator.pop_expected(ValType::I32).is_err());
    }

    #[test]
    fn test_pop_expected_with_unknown_actual() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::Unknown);

        assert!(validator.pop_expected(ValType::I32).is_ok());
    }

    #[test]
    fn test_pop_expected_with_unknown_expected() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::I32);

        assert!(validator.pop_expected(ValType::Unknown).is_ok());
    }

    #[test]
    fn test_pop_expected_with_both_unknown() {
        let mut validator = ValidationStack::new();
        validator.push(ValType::Unknown);

        assert!(validator.pop_expected(ValType::Unknown).is_ok());
    }
}
