use crate::{
    binary::memory_section::Limits,
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_f32_const(&mut self) -> Result<()> {
        self.reader.next_f32()?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_unop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_cmp(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_f32_binop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_convert_i64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_reinterpret_i32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_copysign(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_convert_i32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_demote_f64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }

    pub fn validate_f32_store(&mut self, memories: &[Limits]) -> Result<()> {
        if memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(2)?;
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_f32_load(&mut self, memories: &[Limits]) -> Result<()> {
        if memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(2)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::F32);
        Ok(())
    }
}
