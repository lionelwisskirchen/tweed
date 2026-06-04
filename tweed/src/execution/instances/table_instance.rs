extern crate alloc;

use crate::binary::import_section::TableImport;
use crate::binary::table_section::Table;
use crate::core::error::ExecutionError;
use crate::core::value_types::RefType;

use crate::Value;
use alloc::vec::Vec;
use core::fmt::{self};
pub type Result<T> = core::result::Result<T, ExecutionError>;
#[derive(PartialEq, Clone)]
pub struct TableInstance {
    table_type: Table,
    reference_values: Vec<Value>,
}
impl TableInstance {
    pub fn new(table: Table) -> Self {
        let data = vec![Value::RefNull(table.ref_type); table.limit.min as usize];
        TableInstance {
            table_type: table.clone(),
            reference_values: data,
        }
    }
    pub fn get(&self, index: usize) -> Result<Value> {
        let res = self
            .reference_values
            .get(index)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        Ok(*res)
    }
    pub fn insert(&mut self, offset: usize, vals: &[Value]) -> Result<()> {
        let end = offset
            .checked_add(vals.len())
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        if end > self.reference_values.len() {
            return Err(ExecutionError::TableAccessOutOfBounds);
        }
        for (i, val) in vals.iter().enumerate() {
            self.reference_values[offset + i] = *val;
        }

        Ok(())
    }

    /// Checks if the table's reference type is `FuncRef`.
    ///
    /// Returns `false` if the type is `ExternRef`.
    fn is_func_ref(&self) -> bool {
        match self.table_type.ref_type {
            RefType::FuncRef => true,
            RefType::ExternRef => false,
        }
    }

    /// Checks if the table's reference type is `ExternRef`.
    ///
    /// Returns `false` if the type is `FuncRef`.
    fn is_extern_ref(&self) -> bool {
        match self.table_type.ref_type {
            RefType::FuncRef => false,
            RefType::ExternRef => true,
        }
    }

    /// Check if the given table has the same reftype
    ///
    /// # Arguments
    /// * `table_import`: reference to the import table.
    ///
    /// # Return
    /// Error if the import type is not compatible
    pub fn ensure_ref_type_is_correct(&self, table_import: &TableImport) -> Result<()> {
        if (table_import.context.ref_type == RefType::FuncRef && self.is_func_ref())
            || (table_import.context.ref_type == RefType::ExternRef && self.is_extern_ref())
        {
            Ok(())
        } else {
            Err(ExecutionError::IncompatibleImportType)
        }
    }
    /// Check if the given table has the same limits
    ///
    /// # Arguments
    /// * `table_import`: reference to the import table.
    ///
    /// # Return
    /// Error if the limits are not compatible
    pub fn ensure_limits_are_correct(&self, table_import: &TableImport) -> Result<()> {
        let export = &self.table_type.limit;
        let import = &table_import.context.limit;

        if export.min < import.min {
            return Err(ExecutionError::IncompatibleImportType);
        }

        match (export.max, import.max) {
            (Some(exp_max), Some(imp_max)) => {
                if exp_max > imp_max {
                    return Err(ExecutionError::IncompatibleImportType);
                }
            }

            (None, Some(_)) => {
                return Err(ExecutionError::IncompatibleImportType);
            }

            _ => {}
        }
        Ok(())
    }
}

impl fmt::Debug for TableInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TableInstance {{")?;
        writeln!(f, "  table_type: {:?},", self.table_type)?;
        write!(f, "  reference_values: ")?;

        // Format values with pipes
        if self.reference_values.is_empty() {
            write!(f, "|empty|")?;
        } else {
            for (i, value) in self.reference_values.iter().enumerate() {
                if i == 0 {
                    write!(f, "|")?;
                }
                write!(f, "({i}) {:?} |", value)?;
                if i % 10 == 0 && i != 0 {
                    writeln!(f)?;
                }
            }
        }

        writeln!(f, "\n}}")
    }
}
