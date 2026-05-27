use crate::{binary::global_section::GlobalType, Value};

#[derive(Debug, Clone)]
pub struct GlobalInstance {
    pub global_type: GlobalType,
    pub value: Value,
}
