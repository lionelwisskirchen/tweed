use crate::core::opcode::*;

use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;

use super::validation_stack::ValidationStack;

pub fn validate_const_expr(reader: &mut WasmReader, result: &[ValType]) -> Result<()> {
    let mut abstract_value_stack: ValidationStack = ValidationStack::default();
    loop {
        let byte = reader.next_byte().ok_or(Error::InvalidControlFlow)?;
        match *byte {
            I32_CONST => {
                reader.next_leb128_i32().ok_or(Error::MissingValue)?;
                abstract_value_stack.push(ValType::I32);
            }
            I64_CONST => {
                reader.next_leb128_i64().ok_or(Error::MissingValue)?;
                abstract_value_stack.push(ValType::I64);
            }
            I32_ADD | I32_SUB => {
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.push(ValType::I32);
            }
            I64_ADD => {
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.push(ValType::I64);
            }
            END => {
                abstract_value_stack.pop_expected_vec(result)?;
                if !abstract_value_stack.is_empty() {
                    return Err(Error::TypeMismatch);
                }
                return Ok(());
            }
            _ => return Err(Error::UndefinedOpCode(*byte)),
        }
    }
}
