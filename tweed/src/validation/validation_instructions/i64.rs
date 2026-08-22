use crate::{
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_i64_const(&mut self) -> Result<()> {
        self.reader.next_leb128_i64()?;
        self.abstract_value_stack.push(ValType::I64);

        Ok(())
    }

    pub fn validate_i64_cmp(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i64_unop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_binop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_extend_i32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_eqz(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i64_trunc_f32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_trunc_f64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_reinterpret_f64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }

    pub fn validate_i64_store(&mut self, alignment: u32) -> Result<()> {
        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(alignment)?;
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_i64_load(&mut self, alignment: u32) -> Result<()> {
        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(alignment)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I64);
        Ok(())
    }
}
