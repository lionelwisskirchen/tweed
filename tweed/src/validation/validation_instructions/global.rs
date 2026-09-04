use crate::{core::error::ValidationError, validation::validator::CodeValidator};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_global_get(&mut self) -> Result<()> {
        let idx = self.reader.next_leb128_u32()? as usize;
        let value = self
            .validation_context
            .globals
            .get(idx)
            .ok_or(ValidationError::UnknownGlobal(None))?
            .value_type;
        self.abstract_value_stack.push(value);
        Ok(())
    }

    pub fn validate_global_set(&mut self) -> Result<()> {
        let idx = self.reader.next_leb128_u32()? as usize;
        let global = self
            .validation_context
            .globals
            .get(idx)
            .ok_or(ValidationError::UnknownGlobal(None))?;
        if !global.is_mutable {
            Err(ValidationError::GlobalIsImmutable)?;
        }
        self.abstract_value_stack.pop_expected(global.value_type)?;
        Ok(())
    }
}
