use crate::binary::export_section::ExportContext;

use crate::core::values::Value;
use crate::core::{
    error::{Error, Result},
    wasm_reader::WasmReader,
};

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use super::executor::Executor;
use super::store::{MemoryInstance, Store};

// An instantiated Module
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Instance {
    pub funcaddr: Vec<usize>,
    pub tableaddr: Vec<usize>,
    pub memaddr: Vec<usize>,
    pub globaladdr: Vec<usize>,
    pub elemaddr: Vec<usize>,
    pub dataddr: Vec<usize>,
    pub exports: BTreeMap<String, ExportContext>,
}

impl Instance {
    pub fn new(
        funcaddr: Vec<usize>,
        tableaddr: Vec<usize>,
        memaddr: Vec<usize>,
        globaladdr: Vec<usize>,
        elemaddr: Vec<usize>,
        dataddr: Vec<usize>,
        exports: BTreeMap<String, ExportContext>,
    ) -> Self {
        Instance {
            funcaddr,
            tableaddr,
            memaddr,
            globaladdr,
            elemaddr,
            dataddr,
            exports,
        }
    }
    pub fn call_by_name(
        &mut self,
        name: &str,
        args: Vec<Value>,
        store: &mut Store,
        reader: &mut WasmReader,
    ) -> Result<Vec<Value>> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };
        match export_instance {
            ExportContext::FunctionExport(index) => self.call(*index as usize, args, store, reader),
            _ => Err(Error::UnknownExportCall),
        }
    }
    pub fn call(
        &self,
        index: usize,
        args: Vec<Value>,
        store: &mut Store,
        reader: &mut WasmReader,
    ) -> Result<Vec<Value>> {
        store.stack.reset(); // Reset stack. Maybe not the perfect location
        store.stack.push_vec(&args);
        let mut executor = Executor::new(self, store, reader);
        executor.invoke(index)
    }
    pub fn get_mut_memory_by_name<'a>(
        &mut self,
        name: &str,
        store: &'a mut Store,
    ) -> Result<&'a mut MemoryInstance> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };

        match export_instance {
            ExportContext::MemoryExport(index) => store
                .memories
                .get_mut(*index as usize)
                .ok_or(Error::UnknownExportCall),
            _ => Err(Error::UnknownExportCall),
        }
    }
    pub fn get_memory_by_name<'a>(
        &mut self,
        name: &str,
        store: &'a mut Store,
    ) -> Result<&'a MemoryInstance> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };

        match export_instance {
            ExportContext::MemoryExport(index) => store
                .memories
                .get(*index as usize)
                .ok_or(Error::UnknownExportCall),
            _ => Err(Error::UnknownExportCall),
        }
    }
    pub fn get_global_by_name<'a>(
        &mut self,
        name: &str,
        store: &'a mut Store,
    ) -> Result<&'a Value> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };

        match export_instance {
            ExportContext::GlobalExport(index) => {
                let gl = store
                    .globals
                    .get(*index as usize)
                    .ok_or(Error::UnknownExportCall)?;
                Ok(&gl.value)
            }
            _ => Err(Error::UnknownExportCall),
        }
    }
    pub fn set_global_by_name(
        &mut self,
        name: &str,
        value: Value,
        store: &mut Store,
    ) -> Result<()> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };

        match export_instance {
            ExportContext::GlobalExport(index) => {
                let gl = store
                    .globals
                    .get_mut(*index as usize)
                    .ok_or(Error::UnknownExportCall)?;
                if !gl.global_type.is_mutable {
                    return Err(Error::GlobalIsImmutable);
                }
                if !gl.global_type.value_type.is_type(value) {
                    return Err(Error::TypeMismatch);
                }
                gl.value = value;
                Ok(())
            }
            _ => Err(Error::UnknownExportCall),
        }
    }
}
