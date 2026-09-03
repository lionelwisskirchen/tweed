use crate::{
    binary::memory_section::Limits,
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_i32_const(&mut self) -> Result<()> {
        self.reader.next_leb128_i32()?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_cmp(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_unop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_binop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_reinterpret_f32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_trunc_f32(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_trunc_f64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::F64)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_wrap_i64(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I64)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_extend(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_i32_store(&mut self, memories: &[Limits], alignment: u32) -> Result<()> {
        if memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(alignment)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_i32_load(&mut self, memories: &[Limits], alignment: u32) -> Result<()> {
        if memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.check_align(alignment)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }
}
