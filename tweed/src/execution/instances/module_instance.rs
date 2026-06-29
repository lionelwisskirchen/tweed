use crate::binary::custom_section::InterfaceId;
use crate::binary::export_section::ExportContext;

use crate::binary::type_section::FuncType;
use crate::core::error::ExecutionError;

use crate::execution::dependency_graph::InterfaceBinding;
use crate::execution::instances::MemoryInstance;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::execution::adresses::{
    DataAddr, ElementAddr, FuncAddr, GlobalAddr, MemoryAddr, ReaderAddr, TableAddr,
};

use crate::execution::store::Store;

pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, Default, Clone)]
pub struct DependencyInstance {
    pub depends: Vec<InterfaceId>,
    pub implements: Vec<InterfaceBinding>,
}

impl DependencyInstance {
    pub fn module_name(&self) -> Option<String> {
        if let Some(imp) = self.implements.first() {
            return Some(format!(
                "{:?}:{:?}@{}",
                imp.interface.namespace, imp.interface.package, imp.interface.version
            ));
        }
        None
    }
    pub fn has_information(&self) -> bool {
        !self.implements.is_empty()
    }
}

// An instantiated Module
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ModuleInstance {
    pub types: Vec<FuncType>,
    pub funcaddr: Vec<FuncAddr>,
    pub tableaddr: Vec<TableAddr>,
    pub memaddr: Vec<MemoryAddr>,
    pub globaladdr: Vec<GlobalAddr>,
    pub elemaddr: Vec<ElementAddr>,
    pub dataddr: Vec<DataAddr>,
    pub exports: BTreeMap<String, ExportContext>,
    pub reader: ReaderAddr,
    pub start: Option<FuncAddr>,
    pub implementations: Vec<InterfaceBinding>,
}

impl ModuleInstance {
    pub fn get_func_by_name(&self, name: &str) -> Result<FuncAddr> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ExecutionError::UnknownExportCall);
        };
        let res = match export_instance {
            ExportContext::FunctionExport(index) => self
                .funcaddr
                .get(*index as usize)
                .ok_or(ExecutionError::UnknownExportCall),
            _ => Err(ExecutionError::UnknownExportCall),
        };
        Ok(*res?)
    }
    pub fn get_memory_by_name<'a>(
        &mut self,
        name: &str,
        store: &'a mut Store,
    ) -> Result<&'a MemoryInstance> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ExecutionError::UnknownExportCall);
        };

        match export_instance {
            ExportContext::MemoryExport(index) => {
                let store_index = self
                    .memaddr
                    .get(*index as usize)
                    .ok_or(ExecutionError::UnknownExportCall)?;
                let st = store.memories.get(*store_index)?;
                Ok(st)
            }
            _ => Err(ExecutionError::UnknownExportCall),
        }
    }
    pub fn get_global_by_name(&self, name: &str) -> Result<GlobalAddr> {
        let Some(export_instance) = self.exports.get(name) else {
            return Err(ExecutionError::UnknownExportCall);
        };

        match export_instance {
            ExportContext::GlobalExport(index) => self
                .globaladdr
                .get(*index as usize)
                .cloned()
                .ok_or(ExecutionError::UnknownExportCall),
            _ => Err(ExecutionError::UnknownExportCall),
        }
    }
}
