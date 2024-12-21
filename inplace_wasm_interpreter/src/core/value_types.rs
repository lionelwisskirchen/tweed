use crate::core::error::{Error, Result};
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
            _ => Err(Error::UnsupportedValueType),
        }
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
