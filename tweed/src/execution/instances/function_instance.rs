extern crate alloc;

use crate::core::error::ExecutionError;

use crate::execution::adresses::InstanceAddr;
use crate::execution::store::HostEnv;
use crate::{binary::type_section::FuncType, validation::side_table::SideTable, Value};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Function instances in the runtime.
///
/// A function can either be:
/// - `Internal`: defined by a module instance and executed within the VM
/// - `External`: provided by the host environment (e.g., runtime/engine embedding)
#[derive(Debug, Clone)]
pub enum FuncInstance {
    Internal(Internal),
    External(HostFunc),
}
impl FuncInstance {
    pub fn get_signature(&self) -> &FuncType {
        match self {
            FuncInstance::Internal(internal) => &internal.signature,
            FuncInstance::External(host_func) => &host_func.signature,
        }
    }
}

pub type Func = Rc<RefCell<dyn FnMut(&mut HostEnv, Vec<Value>) -> Result<Vec<Value>>>>;

#[derive(Clone)]
pub struct HostFunc {
    pub signature: FuncType,
    pub func: Func,
}
impl core::fmt::Debug for HostFunc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HostFunc")
            .field("signature", &self.signature)
            .field("func", &"<host func>")
            .finish()
    }
}
#[derive(Debug, Clone)]
pub struct Internal {
    pub signature: FuncType,
    pub locals: Vec<Value>,
    pub start: usize,
    pub end: usize,
    pub side_table: SideTable,
    pub instance: InstanceAddr,
}
