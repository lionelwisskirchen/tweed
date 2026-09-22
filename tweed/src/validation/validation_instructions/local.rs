use crate::{core::error::ValidationError, validation::validator::CodeValidator};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_local_get(&mut self) -> Result<()> {
        let idx = self.reader.next_leb128_u32()?;
        let value = self
            .validation_context
            .locals
            .get(idx as usize)
            .ok_or(ValidationError::InvalidLocalIndex)?;
        self.abstract_value_stack.push(*value);
        Ok(())
    }

    pub fn validate_local_set(&mut self) -> Result<()> {
        let idx = self.reader.next_leb128_u32()?;
        let value = self
            .validation_context
            .locals
            .get(idx as usize)
            .ok_or(ValidationError::InvalidLocalIndex)?;
        self.abstract_value_stack.pop_expected(*value)?;
        Ok(())
    }

    pub fn validate_local_tee(&mut self) -> Result<()> {
        let idx = self.reader.next_leb128_u32()?;
        let value = self
            .validation_context
            .locals
            .get(idx as usize)
            .ok_or(ValidationError::InvalidLocalIndex)?;
        self.abstract_value_stack.pop_expected(*value)?;
        self.abstract_value_stack.push(*value);
        Ok(())
    }
}
