#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    Unknown,
}

impl ValType {
    pub fn lookup(val: &u8) -> ValType {
        match val {
            0x7F => ValType::I32,
            _ => panic!("Unknown value type {}", val),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    use super::*;

    #[should_panic]
    #[test]
    fn test_lookup_unknown() {
        ValType::lookup(&0x7D);
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
