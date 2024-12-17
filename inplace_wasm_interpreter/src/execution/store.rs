extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::binary::export_section::ExportContext;

use super::instance::FunctionInstance;

#[derive(Debug, Default)]
pub struct Store {
    pub functions: Vec<FunctionInstance>,
    pub exports: BTreeMap<String, ExportContext>,
}
