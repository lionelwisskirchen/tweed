extern crate alloc;

use crate::core::error::ExecutionError;
use core::convert::TryFrom;
use core::fmt::{self, Display};
pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Address types used to reference entities stored in the runtime store.
///
/// Each address is a typed wrapper around a `usize` index into a corresponding
/// store collection (e.g., functions, globals, memories).
/// This provides type safety and prevents mixing different entity kinds at compile time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalAddr(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncAddr(pub usize);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TableAddr(pub usize);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryAddr(pub usize);
#[derive(Debug, Default, Clone, Copy)]
pub struct DataAddr(pub usize);
#[derive(Debug, Default, Clone, Copy)]
pub struct ElementAddr(pub usize);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InstanceAddr(pub usize);
#[derive(Debug, Default, Clone, Copy)]
pub struct ReaderAddr(pub usize);

pub trait Address: Copy {
    fn from_usize(v: usize) -> Self;
    fn as_usize(self) -> usize;
}
macro_rules! impl_address {
    ($name:ident) => {
        impl Address for $name {
            fn from_usize(v: usize) -> Self {
                Self(v)
            }

            fn as_usize(self) -> usize {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_usize())
            }
        }
    };
}

impl_address!(GlobalAddr);
impl_address!(FuncAddr);
impl_address!(TableAddr);
impl_address!(MemoryAddr);
impl_address!(DataAddr);
impl_address!(ElementAddr);
impl_address!(InstanceAddr);
impl_address!(ReaderAddr);

/// The runtime address (Store) of an entity that can be imported or exported.
/// https://webassembly.github.io/spec/core/exec/runtime.html#external-addresses
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExternAddr {
    Func(FuncAddr),
    Table(TableAddr),
    Memory(MemoryAddr),
    Global(GlobalAddr),
}
/// Macro for converting ExternalAddresses into StoreAddresses
macro_rules! impl_extern_tryfrom {
    ($ty:ty, $variant:ident) => {
        impl TryFrom<ExternAddr> for $ty {
            type Error = ExecutionError;

            fn try_from(addr: ExternAddr) -> Result<Self> {
                match addr {
                    ExternAddr::$variant(a) => Ok(a),
                    _ => Err(ExecutionError::TypeMismatch),
                }
            }
        }
    };
}
impl_extern_tryfrom!(MemoryAddr, Memory);
impl_extern_tryfrom!(FuncAddr, Func);
impl_extern_tryfrom!(TableAddr, Table);
impl_extern_tryfrom!(GlobalAddr, Global);
