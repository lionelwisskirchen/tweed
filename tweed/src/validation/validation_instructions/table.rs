use crate::{
    core::{error::ValidationError, value_types::ValType},
    validation::validator::CodeValidator,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn validate_table_get(&mut self) -> Result<()> {
        let table_idx = self.reader.next_leb128_u32()? as usize;

        if self.validation_context.tables.len() <= table_idx {
            Err(ValidationError::UndefinedTable)?;
        }
        let ref_type = self.validation_context.tables.get(table_idx).unwrap();
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.push(ref_type.to_val_type());
        Ok(())
    }

    pub fn validate_table_set(&mut self) -> Result<()> {
        let table_idx = self.reader.next_leb128_u32()? as usize;
        if self.validation_context.tables.len() <= table_idx {
            Err(ValidationError::UndefinedTable)?;
        }
        let ref_type = self.validation_context.tables.get(table_idx).unwrap();
        self.abstract_value_stack
            .pop_expected(ref_type.to_val_type())?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_ref_is_null(&mut self) -> Result<()> {
        let value = self.abstract_value_stack.pop_val()?;
        if !value.is_ref() && self.abstract_control_stack.top_is_reachable() {
            Err(ValidationError::TypeMismatch)?;
        }
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_ref_null(&mut self) -> Result<()> {
        let ref_type = self.reader.next_byte()?;
        self.abstract_value_stack.push(ValType::lookup(&ref_type)?);
        Ok(())
    }

    pub fn validate_ref_func(&mut self) -> Result<()> {
        let func_idx = self.reader.next_leb128_u32()? as usize;
        info!("{:?}", self.refs);
        if !self.refs.contains(&func_idx) {
            Err(ValidationError::UndeclaredFunctionRef)?;
        };
        self.module_environment
            .funcsec
            .get(func_idx)
            .ok_or(ValidationError::InvalidFunctionIndex)?;
        self.abstract_value_stack.push(ValType::FuncRef);
        Ok(())
    }

    pub fn validate_table_init(&mut self) -> Result<()> {
        let element_idx = self.reader.next_leb128_u32()? as usize;
        let table_idx = self.reader.next_leb128_u32()? as usize;
        let table = self
            .validation_context
            .tables
            .get(table_idx)
            .ok_or(ValidationError::TypeMismatch)?;
        let element = self
            .module_environment
            .elementsec
            .get(element_idx)
            .ok_or(ValidationError::UnknownElem)?;

        if table.to_val_type() != element.ref_type {
            Err(ValidationError::IllegalSelectType)?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_table_size(&mut self) -> Result<()> {
        let table_idx = self.reader.next_leb128_u32()? as usize;
        self.validation_context
            .tables
            .get(table_idx)
            .ok_or(ValidationError::UnknownTable)?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_table_copy(&mut self) -> Result<()> {
        let table_idx_x = self.reader.next_leb128_u32()? as usize;
        let table_idx_y = self.reader.next_leb128_u32()? as usize;
        let table_x = self
            .validation_context
            .tables
            .get(table_idx_x)
            .ok_or(ValidationError::UnknownTable)?;
        let table_y = self
            .validation_context
            .tables
            .get(table_idx_y)
            .ok_or(ValidationError::UnknownTable)?;
        if table_x != table_y {
            Err(ValidationError::TypeMismatch)?;
        }
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_table_fill(&mut self) -> Result<()> {
        let table_idx = self.reader.next_leb128_u32()? as usize;
        let table = self
            .validation_context
            .tables
            .get(table_idx)
            .ok_or(ValidationError::UnknownTable)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack
            .pop_expected(table.to_val_type())?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        Ok(())
    }

    pub fn validate_table_grow(&mut self) -> Result<()> {
        let table_idx = self.reader.next_leb128_u32()? as usize;
        let table = self
            .validation_context
            .tables
            .get(table_idx)
            .ok_or(ValidationError::UnknownTable)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack
            .pop_expected(table.to_val_type())?;
        self.abstract_value_stack.push(ValType::I32);
        Ok(())
    }

    pub fn validate_elem_drop(&mut self) -> Result<()> {
        let element_idx = self.reader.next_leb128_u32()? as usize;
        self.module_environment
            .elementsec
            .get(element_idx)
            .ok_or(ValidationError::UnknownElem)?;

        Ok(())
    }
}
