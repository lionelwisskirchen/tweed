use super::value_types::{RefType, ValType};

/// Value type for uniform handle of vector shapes
///
/// The shape of a v128 describes how the internal 128-bit are divided.
/// The lane is a single data slot within that shape.
///
/// Example for a shape: i32x4: 4 lanes of 32-bit integers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum V128Shape {
    I8x16([i8; 16]),
    I16x8([i16; 8]),
    I32x4([i32; 4]),
    I64x2([i64; 2]),
    F32x4([f32; 4]),
    F64x2([f64; 2]),
    Bytes([u8; 16]),
}

impl FromLeBytes for V128Shape {
    fn from_le_bytes(bytes: &[u8]) -> V128Shape {
        if bytes.len() != 16 {
            unreachable!("V128 requires exactly 16 bytes, (validated)")
        }

        V128Shape::I8x16(core::array::from_fn(|i| bytes[i] as i8))
    }
}

impl V128Shape {
    /// Converts the SIMD value into its underlying 16-byte representation.
    ///
    /// This method uses the native byte order of the target platform.
    pub fn to_bytes(&self) -> [u8; 16] {
        match self {
            V128Shape::I8x16(int_array) => {
                let mut bytes = [0u8; 16];
                for (i, &val) in int_array.iter().enumerate() {
                    bytes[i] = val as u8;
                }
                bytes
            }
            V128Shape::I16x8(int_array) => {
                let result: [u8; 16] = int_array
                    .iter()
                    .flat_map(|&x| x.to_ne_bytes())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                result
            }
            V128Shape::I32x4(int_array) => {
                let result: [u8; 16] = int_array
                    .iter()
                    .flat_map(|&x| x.to_ne_bytes())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                result
            }
            V128Shape::I64x2(int_array) => {
                let result: [u8; 16] = int_array
                    .iter()
                    .flat_map(|&x| x.to_ne_bytes())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                result
            }
            V128Shape::F32x4(float_array) => {
                let result: [u8; 16] = float_array
                    .iter()
                    .flat_map(|&x| x.to_ne_bytes())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                result
            }
            V128Shape::F64x2(float_array) => {
                let result: [u8; 16] = float_array
                    .iter()
                    .flat_map(|&x| x.to_ne_bytes())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                result
            }
            V128Shape::Bytes(b) => *b,
        }
    }

    pub fn to_i8x16(&self) -> [i8; 16] {
        match self {
            V128Shape::I8x16(int_array) => *int_array,
            _ => {
                let bytes = self.to_bytes();
                bytes.map(|bytes| bytes as i8)
            }
        }
    }

    pub fn to_i16x8(&self) -> [i16; 8] {
        match self {
            V128Shape::I16x8(int_array) => *int_array,
            _ => {
                let bytes = self.to_bytes();
                let result = bytes
                    .chunks_exact(2)
                    .map(|bytes| i16::from_ne_bytes([bytes[0], bytes[1]]))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                result
            }
        }
    }

    pub fn to_i32x4(&self) -> [i32; 4] {
        match self {
            V128Shape::I32x4(int_array) => *int_array,
            _ => {
                let bytes = self.to_bytes();
                let result = bytes
                    .chunks_exact(4)
                    .map(|bytes| i32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                result
            }
        }
    }

    pub fn to_i64x2(&self) -> [i64; 2] {
        match self {
            V128Shape::I64x2(int_array) => *int_array,
            _ => {
                let bytes = self.to_bytes();
                let result = bytes
                    .chunks_exact(8)
                    .map(|bytes| {
                        i64::from_ne_bytes([
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[4], bytes[6],
                            bytes[7],
                        ])
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                result
            }
        }
    }

    pub fn to_f32x4(&self) -> [f32; 4] {
        match self {
            V128Shape::F32x4(float_array) => *float_array,
            _ => {
                let bytes = self.to_bytes();
                let result = bytes
                    .chunks_exact(8)
                    .map(|bytes| f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                result
            }
        }
    }

    pub fn to_f64x2(&self) -> [f64; 2] {
        match self {
            V128Shape::F64x2(float_array) => *float_array,
            _ => {
                let bytes = self.to_bytes();
                let result = bytes
                    .chunks_exact(8)
                    .map(|bytes| {
                        f64::from_ne_bytes([
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[4], bytes[6],
                            bytes[7],
                        ])
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                result
            }
        }
    }
}

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
    V128(V128Shape),
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
            ValType::V128Type => Value::V128(V128Shape::Bytes([0; 16])),
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

impl From<V128Shape> for Value {
    fn from(value: V128Shape) -> Self {
        Value::V128(value as V128Shape)
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

impl FromLeBytes for V128Shape {
    fn from_le_bytes(bytes: &[u8]) -> V128Shape {
        if bytes.len() != 16 {
            unreachable!("V128 requires exactly 16 bytes, (validated)")
        }

        V128Shape::I8x16(core::array::from_fn(|i| bytes[i] as i8))
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
