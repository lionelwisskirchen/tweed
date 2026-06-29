use super::adresses::{FuncAddr, InstanceAddr};
use crate::{
    binary::custom_section::{FunctionName, InterfaceId, InterfaceKey, LinkingInformation},
    core::error::ExecutionError,
};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use core::fmt::Write;
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, Default, Clone)]
pub struct InterfaceBinding {
    pub interface: InterfaceId,
    pub functions: BTreeMap<FunctionName, FuncAddr>,
    pub instance_addr: InstanceAddr,
}

#[derive(Debug, Default)]
pub struct Edge {
    pub from: InterfaceId,
    pub to: InterfaceId,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    bindings: BTreeMap<InterfaceId, InterfaceBinding>,
    dependencies: Vec<Edge>,
    versions: Vec<Edge>,
    interfaces: BTreeMap<InterfaceKey, BTreeSet<InterfaceId>>,
}

#[derive(Debug, Default)]
pub struct UpgradeInfos {
    pub current_binding: InterfaceBinding,
    pub target_binding: InterfaceBinding,
    pub dependent_bindings: Vec<InterfaceBinding>,
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

    /// Adds an implementation to an interface.
    pub fn add_binding(&mut self, binding: InterfaceBinding) -> Result<()> {
        self.bindings.insert(binding.interface.clone(), binding);
        Ok(())
    }

    /// Adds an interface to the dependency graph.
    pub fn add_interface(&mut self, linking_information: LinkingInformation) -> Result<()> {
        for iface in &linking_information.implements {
            if let Some(interfaces) = self.interfaces.get_mut(&iface.interface.key) {
                for interface in interfaces.iter() {
                    // TODO check if version is higher or lower
                    self.versions.push(Edge {
                        from: interface.clone(),
                        to: iface.interface.clone(),
                    });
                }
                interfaces.insert(iface.interface.clone());
            } else {
                self.interfaces.insert(
                    iface.interface.key.clone(),
                    BTreeSet::from([iface.interface.clone()]),
                );
            }
            for dependency in &linking_information.depends {
                if !self.bindings.contains_key(dependency) {
                    return Err(ExecutionError::MissingFunctionImport);
                }
                self.dependencies.push(Edge {
                    from: iface.interface.clone(),
                    to: dependency.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn get_binding(&self, iface: &InterfaceId) -> Result<InterfaceBinding> {
        self.bindings
            .get(iface)
            .ok_or(ExecutionError::InterfaceNotFound)
            .cloned()
    }
    pub fn get_upgrade_binding(&self, iface: &InterfaceId) -> Result<InterfaceBinding> {
        let target_version_edge = self
            .versions
            .iter()
            .find(|edge| edge.from == *iface)
            .ok_or(ExecutionError::InterfaceNotFound)?;

        self.bindings
            .get(&target_version_edge.to)
            .ok_or(ExecutionError::InterfaceNotFound)
            .cloned()
    }
    pub fn get_dependent_bindings(&self, iface: &InterfaceId) -> Result<Vec<InterfaceBinding>> {
        Ok(self
            .dependencies
            .iter()
            .filter(|edge| edge.to == *iface)
            .map(|e| self.bindings.get(&e.from).unwrap().clone())
            .collect())
    }

    /// Upgrades the graph according to the Upgrade Infos
    pub fn apply_upgrade(&mut self, upgrade_info: &UpgradeInfos) -> Result<()> {
        // Delete all outgoing from upgraded version
        self.dependencies
            .retain(|edge| edge.from != upgrade_info.current_binding.interface);

        // Create new edges to new target iface
        let target_iface = upgrade_info.target_binding.interface.clone();

        for dependent_interface in &upgrade_info.dependent_bindings {
            self.dependencies.push(Edge {
                from: dependent_interface.interface.clone(),
                to: target_iface.clone(),
            });
        }
        Ok(())
    }
    pub fn to_dot(&self) -> String {
        let mut out = String::new();

        let _ = writeln!(&mut out, "digraph deps {{");
        let _ = writeln!(&mut out, "  rankdir=LR;");

        for edge in &self.dependencies {
            let _ = writeln!(
                &mut out,
                "  \"{}\" -> \"{}\" [color=black];",
                &edge.from.interface_name(),
                &edge.to.interface_name(),
            );
        }

        for edge in &self.versions {
            let _ = writeln!(
                &mut out,
                "  \"{}\" -> \"{}\" [color=blue, style=dashed, label=\"version\"];",
                &edge.from.interface_name(),
                &edge.to.interface_name(),
            );
        }

        let _ = writeln!(&mut out, "}}");
        out
    }
}
