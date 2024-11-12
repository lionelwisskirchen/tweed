use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::binary::type_section::FuncType;

use super::values::ValType;

fn shorthand_lookup(val: u32) -> Option<FuncType> {
    let lookup_map = BTreeMap::from([(
        0x7f,
        FuncType {
            parameter: Vec::new(),
            result: vec![ValType::I32],
        },
    )]);
    lookup_map.get(&val).cloned()
}


pub fn lookup_signature(func_idx: u32, type_section: &[FuncType]) -> FuncType {
    if func_idx == 0x40 {
        return FuncType {
            ..Default::default()
        };
    };
    if let Some(signature) = shorthand_lookup(func_idx) {
        return signature;
    };
    type_section
        .get(func_idx as usize)
        .expect("No func idx for sig")
        .clone()
}
