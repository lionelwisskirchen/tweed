use crate::{
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_f64_const(&mut self) -> Result<()> {
        self.reader.next_f64()?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_unop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_binop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_copysign(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_promote_f32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_convert_i32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_reinterpret_i64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_cmp(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_f64_convert_i64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }

    pub fn validate_f64_store(&mut self) -> Result<()> {
        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(3)?;
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_f64_load(&mut self) -> Result<()> {
        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(3)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::F64);
        Ok(())
    }
}
