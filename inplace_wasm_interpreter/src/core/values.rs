use super::value_types::ValType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}
impl From<ValType> for Value {
    fn from(value: ValType) -> Self {
        match value {
            ValType::I32 => Value::I32(0),
            ValType::I64 => Value::I64(0),
            ValType::Unknown => panic!("Unknown value type cant be converted to value."),
        }
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::I32(value as i32)
    }
}
impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::I64(value as i64)
    }
}

pub trait FromLeBytes: Sized {
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

impl FromLeBytes for i32 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        i32::from_le_bytes(bytes.try_into().expect("Invalid length for i32"))
    }
}

impl FromLeBytes for i64 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        i64::from_le_bytes(bytes.try_into().expect("Invalid length for i64"))
    }
}

impl core::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left.overflowing_add(right).0),
            (Value::I64(left), Value::I64(right)) => Value::I64(left.overflowing_add(right).0),
            _ => panic!("type mismatch"),
        }
    }
}
impl core::ops::Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left.overflowing_sub(right).0),
            (Value::I64(left), Value::I64(right)) => Value::I64(left.overflowing_sub(right).0),
            _ => panic!("type mismatch"),
        }
    }
}
