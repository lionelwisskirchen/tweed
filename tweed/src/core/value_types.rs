use crate::{
    core::error::{Error, Result},
    Value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    Unknown,
}

impl ValType {
    pub fn lookup(val: &u8) -> Result<ValType> {
        match val {
            0x7F => Ok(ValType::I32),
            0x7E => Ok(ValType::I64),
            _ => Err(Error::UnsupportedValueType),
        }
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            ValType::I32 | ValType::I64 | ValType::Unknown => true,
        }
    }
    pub fn is_type(&self, val: Value) -> bool {
        matches!(
            (self, val),
            (ValType::I32, Value::I32(_)) | (ValType::I64, Value::I64(_))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    use super::*;

    #[test]
    fn test_lookup_unknown() {
        assert!(ValType::lookup(&0x7D).is_err());
    }
    #[should_panic]
    #[test]
    fn test_conversion_unknown() {
        let _: Value = ValType::Unknown.into();
    }
    #[test]
    fn test_i64_into() {
        let res: Value = ValType::I64.into();
        assert_eq!(res, Value::I64(0));
    }
}
