use crate::{
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_memory_init(&mut self) -> Result<()> {
        let data_index = self.reader.next_leb128_u32()?;
        let idx = self.reader.next_byte()?;
        if idx != 0 {
            Err(ValidationError::MultipleMemoriesNotImplemented)?;
        }

        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemoryIdx(idx as usize))?;
        }
        if self.module_environment.datacountsec.is_none() {
            Err(ValidationError::DataCountSectionRequired)?;
        }
        if self
            .module_environment
            .datacountsec
            .is_some_and(|x| x <= data_index)
        {
            Err(ValidationError::NoDataSegmentFound(Some(
                data_index as usize,
            )))?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_memory_copy(&mut self) -> Result<()> {
        let (dest, src) = (self.reader.next_byte()?, self.reader.next_byte()?);
        if dest != 0 || src != 0 {
            Err(ValidationError::MultipleMemoriesNotImplemented)?;
        }

        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemoryIdx(dest.into()))?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_memory_fill(&mut self) -> Result<()> {
        let idx = self.reader.next_byte()?;
        if idx != 0 {
            Err(ValidationError::MultipleMemoriesNotImplemented)?;
        }

        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemoryIdx(idx.into()))?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_memory_grow(&mut self) -> Result<()> {
        let idx = self.reader.next_byte()?;
        if idx != 0 {
            Err(ValidationError::ZeroByteExpected)?;
        }

        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_memory_size(&mut self) -> Result<()> {
        let idx = self.reader.next_byte()?;
        if idx != 0 {
            Err(ValidationError::ZeroByteExpected)?;
        }

        if self.validation_context.memories.is_empty() {
            Err(ValidationError::UnknownMemory)?;
        }
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_data_drop(&mut self) -> Result<()> {
        let data_index = self.reader.next_leb128_u32()?;
        if self.module_environment.datacountsec.is_none() {
            Err(ValidationError::DataCountSectionRequired)?;
        }

        if self.module_environment.datacountsec.is_some_and(|x| x == 0) {
            Err(ValidationError::NoDataSegmentFound(None))?;
        }
        if self
            .module_environment
            .datacountsec
            .is_some_and(|x| x <= data_index)
        {
            Err(ValidationError::NoDataSegmentFound(None))?;
        }

        Ok(())
    }
}
