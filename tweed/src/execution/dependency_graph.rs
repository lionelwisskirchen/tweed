use super::adresses::{FuncAddr, InstanceAddr};
use crate::{
    binary::custom_section::{FunctionName, InterfaceId, InterfaceKey, LinkingInformation},
    core::error::ExecutionError,
};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;
use core::{fmt::Write, ops::Bound};
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
    interfaces: BTreeMap<InterfaceKey, BTreeSet<InterfaceId>>,
    garbage: BTreeSet<InterfaceId>,
}

#[derive(Debug, Default)]
pub struct UpgradeInfos {
    pub current_binding: InterfaceBinding,
    pub target_binding: InterfaceBinding,
    pub selection: Vec<InterfaceId>,
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
    pub fn add_binding(&mut self, binding: InterfaceBinding) -> Result<InterfaceId> {
        let iface = binding.interface.clone();
        self.bindings.insert(binding.interface.clone(), binding);
        Ok(iface)
    }

    /// Adds an interface to the dependency graph.
    pub fn add_interface(&mut self, linking_information: LinkingInformation) -> Result<()> {
        for iface in &linking_information.implements {
            if let Some(interfaces) = self.interfaces.get_mut(&iface.interface.key) {
                interfaces.insert(iface.interface.clone());
            } else {
                self.interfaces.insert(
                    iface.interface.key.clone(),
                    BTreeSet::from([iface.interface.clone()]),
                );
            }
            for dependency in &linking_information.depends {
                if !self.bindings.contains_key(dependency) {
                    return Err(ExecutionError::InterfaceImplementationNotFound);
                }
                // Remove dependency from garbage list
                self.garbage.remove(dependency);

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
        let target_interface_versions = self
            .interfaces
            .get(&iface.key)
            .ok_or(ExecutionError::InterfaceNotFound)?;

        let current_version = target_interface_versions
            .get(iface)
            .ok_or(ExecutionError::InterfaceNotFound)?;

        let next_version = target_interface_versions
            .range((Bound::Excluded(current_version), Bound::Unbounded))
            .next()
            .ok_or(ExecutionError::InterfaceNotFound)?;

        self.bindings
            .get(next_version)
            .ok_or(ExecutionError::InterfaceImplementationNotFound)
            .cloned()
    }
    pub fn get_dependent_interfaces(&self, iface: &InterfaceId) -> Result<Vec<InterfaceId>> {
        Ok(self
            .dependencies
            .iter()
            .filter(|edge| edge.to == *iface)
            .map(|e| e.from.clone())
            .collect())
    }

    /// Upgrades the graph according to the Upgrade Infos
    pub fn apply_upgrade(&mut self, upgrade_info: &UpgradeInfos) -> Result<()> {
        // Create new edges to new target iface
        let target_iface = upgrade_info.target_binding.interface.clone();

        for dependent_interface in &upgrade_info.selection {
            // Delete all outgoing from upgraded version
            self.dependencies.retain(|edge| {
                !(edge.to == upgrade_info.current_binding.interface
                    && edge.from == *dependent_interface)
            });

            self.dependencies.push(Edge {
                from: dependent_interface.clone(),
                to: target_iface.clone(),
            });
        }
        // Mark as garbage, when no interface is dependent.
        // The interface has no outgoing and no incoming dependency edges!
        if self
            .get_dependent_interfaces(&upgrade_info.current_binding.interface)?
            .is_empty()
        {
            self.garbage
                .insert(upgrade_info.current_binding.interface.clone());
        }
        Ok(())
    }
    pub fn collect_garbage(&mut self) -> Result<()> {
        for interface in self.garbage.iter() {
            // Remove Bindings
            self.bindings.remove(interface);
            // Remove Version from Interfaces
            self.interfaces
                .get_mut(&interface.key)
                .expect("Unexpected Error")
                .remove(interface);
        }
        // Clear Garbage List
        self.garbage.clear();
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
        let _ = writeln!(&mut out, "}}");
        out
    }
}
