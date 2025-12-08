use super::value_types::{RefType, ValType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    FuncRef(usize),
    ExternRef(usize),
    RefNull(RefType),
    CanonicalNan,
    ArithmeticNan,
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

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<ValType> for Value {
    fn from(value: ValType) -> Self {
        match value {
            ValType::I32 => Value::I32(0),
            ValType::I64 => Value::I64(0),
            ValType::F32 => Value::F32(0.0),
            ValType::F64 => Value::F64(0.0),
            ValType::ExternRef => Value::RefNull(RefType::ExternRef),
            ValType::FuncRef => Value::RefNull(RefType::FuncRef),
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
impl FromLeBytes for u32 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().expect("Invalid length for u32"))
    }
}
impl FromLeBytes for f32 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        f32::from_le_bytes(bytes.try_into().expect("Invalid length for f32"))
    }
}
impl FromLeBytes for f64 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        f64::from_le_bytes(bytes.try_into().expect("Invalid length for f64"))
    }
}
impl FromLeBytes for i64 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        i64::from_le_bytes(bytes.try_into().expect("Invalid length for i64"))
    }
}
impl FromLeBytes for u64 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u64::from_le_bytes(bytes.try_into().expect("Invalid length for u64"))
    }
}
impl FromLeBytes for u8 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u8::from_le_bytes(bytes.try_into().expect("Invalid length for u8"))
    }
}

impl FromLeBytes for u16 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().expect("Invalid length for u16"))
    }
}
impl FromLeBytes for i16 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        i16::from_le_bytes(bytes.try_into().expect("Invalid length for i16"))
    }
}
impl FromLeBytes for i8 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        i8::from_le_bytes(bytes.try_into().expect("Invalid length for u8"))
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

/// A trait for checking the **payload** of a IEEE 754 NaN-values
///
/// With this trait it is possible to differentiate between a normal NaN (Payload = 0) and an arithmetic NaN (Payload != 0).
///
pub trait NaN {
    /// Checks if the float value of the NaN has a payload
    fn is_arithmetic_nan(&self) -> bool;
    /// Returns the same NaN value but with the quiet bit set
    ///
    /// even though the quiet bit is ignored in wasm
    /// to meet requirement from float_exprs.wast spectest here the quiet bit is set
    fn with_quiet_nan_bit(&self) -> Self;
}

impl NaN for f32 {
    fn is_arithmetic_nan(&self) -> bool {
        self.is_nan() && self.to_bits() != f32::NAN.to_bits()
    }
    fn with_quiet_nan_bit(&self) -> Self {
        let mut bytes = self.to_bits();
        const QUIET_BIT_MASK: u32 = 0x00400000;
        bytes |= QUIET_BIT_MASK;
        f32::from_bits(bytes)
    }
}
impl NaN for f64 {
    fn is_arithmetic_nan(&self) -> bool {
        self.is_nan() && self.to_bits() != f64::NAN.to_bits()
    }
    fn with_quiet_nan_bit(&self) -> Self {
        let mut bytes = self.to_bits();
        const QUIET_BIT_MASK: u64 = 0x0008000000000000;
        bytes |= QUIET_BIT_MASK;
        f64::from_bits(bytes)
    }
}
