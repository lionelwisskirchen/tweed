use crate::binary::type_section::FuncType;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::error::ValidationError;
use super::value_types::ValType;
pub type Result<T> = core::result::Result<T, ValidationError>;

fn shorthand_lookup(val: u32) -> Option<FuncType> {
    let lookup_map = BTreeMap::from([
        (
            0x7f,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::I32],
            },
        ),
        (
            0x7e,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::I64],
            },
        ),
        (
            0x7d,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::F32],
            },
        ),
        (
            0x7c,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::F64],
            },
        ),
        (
            0x6F,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::ExternRef],
            },
        ),
        (
            0x70,
            FuncType {
                parameter: Vec::new(),
                result: vec![ValType::FuncRef],
            },
        ),
    ]);
    lookup_map.get(&val).cloned()
}

pub fn lookup_signature(func_idx: u32, type_section: &[FuncType]) -> Result<FuncType> {
    if func_idx == 0x40 {
        return Ok(FuncType {
            ..Default::default()
        });
    };
    if let Some(signature) = shorthand_lookup(func_idx) {
        return Ok(signature);
    };
    type_section
        .get(func_idx as usize)
        .cloned()
        .ok_or(ValidationError::UnknownType)
}
