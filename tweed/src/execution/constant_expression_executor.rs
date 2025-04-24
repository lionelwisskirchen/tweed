use alloc::vec::Vec;

use crate::core::{opcode::*, value_types::ValType};

use crate::core::values::Value;
use crate::core::wasm_reader::WasmReader;

use super::execution_stack::ExecutionStack;
use super::store::GlobalInstance;
use crate::core::error::{Error, Result};

pub fn execute_const_expr(
    reader: &mut WasmReader,
    globals: &[GlobalInstance],
    result: &[ValType],
) -> Result<Vec<Value>> {
    let mut value_stack: ExecutionStack = ExecutionStack::default();
    loop {
        let byte = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
        match *byte {
            I32_CONST => {
                let value = Value::I32(reader.next_leb128_i32().unwrap());
                value_stack.push(value);
            }
            I64_CONST => {
                let value = Value::I64(reader.next_leb128_i64().unwrap());
                value_stack.push(value);
            }
            I64_ADD => {
                let (right, left) = (value_stack.pop_i64()?, value_stack.pop_i64()?);
                let result = right + left;
                value_stack.push(result.into());
            }
            GLOBAL_GET => {
                let idx = reader.next_leb128_u32().ok_or(Error::InvalidLocalIndex)?;
                let global = globals.get(idx as usize).ok_or(Error::UnknownGlobal)?;

                if global.global_type.is_mutable {
                    return Err(Error::UndefinedContantExpression);
                }
                value_stack.push(global.value);
            }
            I32_ADD => {
                let (right, left) = (value_stack.pop_i32()?, value_stack.pop_i32()?);
                let result = right + left;
                value_stack.push(result.into());
            }
            I32_SUB => {
                let (right, left) = (value_stack.pop()?, value_stack.pop()?);

                value_stack.push(left - right);
            }
            END => {
                let mut unpacked_result = vec![];
                for ty in result {
                    match ty {
                        ValType::I32 => {
                            unpacked_result.push(Value::I32(value_stack.pop_i32()?));
                        }
                        ValType::I64 => {
                            unpacked_result.push(Value::I64(value_stack.pop_i64()?));
                        }
                        _ => {
                            return Err(Error::TypeMismatch);
                        }
                    }
                }
                return Ok(unpacked_result);
            }
            _ => return Err(Error::UndefinedContantExpression),
        }
    }
}
