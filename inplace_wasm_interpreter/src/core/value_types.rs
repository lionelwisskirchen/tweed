#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    Unknown
}

impl ValType {
    pub fn lookup(val: &u8) -> ValType {
        match val {
            0x7F => ValType::I32,
            0x7E => ValType::I64,
            _ => panic!("Unknown value type {}", val),
        }
    }
}



