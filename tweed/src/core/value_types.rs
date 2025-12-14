use crate::{core::error::ValidationError, Value};

pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    ExternRef,
    Unknown,
}
impl From<Value> for ValType {
    fn from(value: Value) -> Self {
        match value {
            Value::I32(_) => ValType::I32,
            Value::I64(_) => ValType::I64,
            Value::F32(_) => ValType::F32,
            Value::F64(_) => ValType::F64,
            Value::ExternRef(_) => ValType::ExternRef,
            Value::FuncRef(_) => ValType::FuncRef,
            _ => panic!("Value not implemented yet"),
        }
    }
}
impl RefType {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x70 => Some(RefType::FuncRef),
            0x6F => Some(RefType::ExternRef),
            _ => None,
        }
    }
    pub fn to_val_type(self) -> ValType {
        match self {
            Self::FuncRef => ValType::FuncRef,
            Self::ExternRef => ValType::ExternRef,
        }
    }
}

impl ValType {
    pub fn lookup(val: &u8) -> Result<ValType> {
        match val {
            0x7F => Ok(ValType::I32),
            0x7E => Ok(ValType::I64),
            0x7D => Ok(ValType::F32),
            0x7C => Ok(ValType::F64),
            0x70 => Ok(ValType::FuncRef),
            0x6F => Ok(ValType::ExternRef),
            _ => {
                warn!("Unsupported Value type {val:#04X}");
                Err(ValidationError::UnsupportedValueType)
            }
        }
    }
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            ValType::I32 | ValType::I64 | ValType::F64 | ValType::F32 | ValType::Unknown
        )
    }
    pub fn is_ref(&self) -> bool {
        matches!(self, ValType::FuncRef | ValType::ExternRef)
    }
    pub fn is_type(&self, val: Value) -> bool {
        matches!(
            (self, val),
            (ValType::I32, Value::I32(_))
                | (ValType::I64, Value::I64(_))
                | (ValType::F64, Value::F64(_))
                | (ValType::F32, Value::F32(_))
                | (ValType::FuncRef, Value::FuncRef(_))
                | (ValType::ExternRef, Value::ExternRef(_))
                | (ValType::FuncRef, Value::RefNull(RefType::FuncRef))
                | (ValType::ExternRef, Value::RefNull(RefType::ExternRef))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    use super::*;

    #[test]
    fn test_lookup_unknown() {
        assert!(ValType::lookup(&0x3F).is_err());
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
