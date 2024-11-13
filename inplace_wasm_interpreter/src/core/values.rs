
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl core::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left + right),
            (Value::I64(left), Value::I64(right)) => Value::I64(left + right),
            _ => panic!("type mismatch"),
        }
    }
}
impl core::ops::Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left - right),
            (Value::I64(left), Value::I64(right)) => Value::I64(left - right),
            _ => panic!("type mismatch"),
        }
    }
}
