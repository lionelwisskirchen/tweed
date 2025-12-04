use crate::binary::export_section::ExportContext;

use crate::binary::type_section::FuncType;
use crate::core::error::{Result, ValidationError};

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use super::store::{GlobalInstance, MemoryInstance, Store};

// An instantiated Module
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Instance {
    pub name: String,
    pub types: Vec<FuncType>,
    pub funcaddr: Vec<usize>,
    pub tableaddr: Vec<usize>,
    pub memaddr: Vec<usize>,
    pub globaladdr: Vec<usize>,
    pub elemaddr: Vec<usize>,
    pub dataddr: Vec<usize>,
    pub exports: BTreeMap<String, ExportContext>,
}

impl Instance {
    pub fn get_func_by_name(&self, name: &str) -> Result<usize> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ValidationError::UnknownExportCall);
        };
        let res = match export_instance {
            ExportContext::FunctionExport(index) => self
                .funcaddr
                .get(*index as usize)
                .ok_or(ValidationError::UnknownExportCall),
            _ => Err(ValidationError::UnknownExportCall),
        };
        Ok(*res?)
    }
    pub fn get_memory_by_name<'a>(
        &mut self,
        name: &str,
        store: &'a mut Store,
    ) -> Result<&'a MemoryInstance> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ValidationError::UnknownExportCall);
        };

        match export_instance {
            ExportContext::MemoryExport(index) => {
                let store_index = self
                    .memaddr
                    .get(*index as usize)
                    .ok_or(ValidationError::UnknownExportCall)?;
                let st = store
                    .data
                    .memories
                    .get(*store_index)
                    .ok_or(ValidationError::UnknownExportCall)?;
                Ok(st)
            }
            _ => Err(ValidationError::UnknownExportCall),
        }
    }
    pub fn get_global_by_name<'a>(
        &self,
        name: &str,
        store: &'a Store,
    ) -> Result<&'a GlobalInstance> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ValidationError::UnknownExportCall);
        };

        match export_instance {
            ExportContext::GlobalExport(index) => {
                let store_index = self
                    .globaladdr
                    .get(*index as usize)
                    .ok_or(ValidationError::UnknownExportCall)?;
                let gl = store
                    .data
                    .globals
                    .get(*store_index)
                    .ok_or(ValidationError::UnknownExportCall)?;
                Ok(gl)
            }
            _ => Err(ValidationError::UnknownExportCall),
        }
    }
}
