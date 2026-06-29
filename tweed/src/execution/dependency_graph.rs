use std::collections::BTreeMap;

use super::adresses::FuncAddr;
use crate::{
    binary::custom_section::{FunctionName, InterfaceId, LinkingInformation},
    core::error::ExecutionError,
};
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, Default, Clone)]
pub struct InterfaceBinding {
    pub interface: InterfaceId,
    pub functions: BTreeMap<FunctionName, FuncAddr>,
}

#[derive(Debug, Default)]
pub struct Edge {
    pub from: InterfaceId,
    pub to: InterfaceId,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    bindings: BTreeMap<InterfaceId, InterfaceBinding>,
    graph: Vec<Edge>,
}

impl DependencyGraph {
    /// Resolves an implementation of a function defined in an interface
    pub fn find(&self, interface: &InterfaceId, function_name: FunctionName) -> Result<&FuncAddr> {
        let interface = self
            .bindings
            .get(interface)
            .ok_or(ExecutionError::InterfaceNotFound)?;
        let function = interface
            .functions
            .get(&function_name)
            .ok_or(ExecutionError::FunctionNotFound)?;

        Ok(function)
    }
    pub fn add_binding(&mut self, binding: InterfaceBinding) -> Result<()> {
        self.bindings.insert(binding.interface.clone(), binding);
        Ok(())
    }
    pub fn add_interface(&mut self, linking_information: LinkingInformation) -> Result<()> {
        for iface in &linking_information.implements {
            for dependency in &linking_information.depends {
                if !self.bindings.contains_key(dependency) {
                    return Err(ExecutionError::MissingFunctionImport);
                }
                self.graph.push(Edge {
                    from: iface.interface.clone(),
                    to: dependency.clone(),
                });
            }
        }
        Ok(())
    }
}
