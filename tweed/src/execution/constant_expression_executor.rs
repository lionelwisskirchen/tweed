use alloc::vec::Vec;

use crate::core::value_types::RefType;
use crate::core::{opcode::*, value_types::ValType};

use crate::core::values::Value;
use crate::core::wasm_reader::WasmReader;

use super::execution_stack::ExecutionStack;
use super::store::{FuncInstance, GlobalInstance};
use crate::core::error::{Error, Result};

pub fn execute_const_expr(
    reader: &mut WasmReader,
    globals: &[GlobalInstance],
    result: &[ValType],
    functions: &[FuncInstance],
) -> Result<Vec<Value>> {
    let mut value_stack: ExecutionStack = ExecutionStack::default();
    loop {
        let byte = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
        trace!("Execute Const Expression Byte: {byte:#04X}");
        match *byte {
            I32_CONST => {
                let value = Value::I32(reader.next_leb128_i32().unwrap());
                value_stack.push(value)?;
            }
            I64_CONST => {
                let value = Value::I64(reader.next_leb128_i64().unwrap());
                value_stack.push(value)?;
            }
            F32_CONST => {
                let value = reader.next_f32().unwrap();
                value_stack.push(Value::F32(value))?;
            }
            F64_CONST => {
                let value = reader.next_f64().unwrap();
                value_stack.push(Value::F64(value))?;
            }
            I64_ADD => {
                let (right, left) = (value_stack.pop_i64()?, value_stack.pop_i64()?);
                let result = right + left;
                value_stack.push(result.into())?;
            }
            I64_SUB => {
                let (right, left) = (value_stack.pop_i64()?, value_stack.pop_i64()?);
                let result = left.wrapping_sub(right);
                value_stack.push(result.into())?;
            }
            GLOBAL_GET => {
                let idx = reader.next_leb128_u32()?;
                let global = globals.get(idx as usize).ok_or(Error::UnknownGlobal)?;
                if global.global_type.is_mutable {
                    return Err(Error::ConstIsMutable);
                }
                value_stack.push(global.value)?;
            }
            I32_ADD => {
                let (right, left) = (value_stack.pop_i32()?, value_stack.pop_i32()?);
                let result = right + left;
                value_stack.push(result.into())?;
            }
            I32_SUB => {
                let (right, left) = (value_stack.pop()?, value_stack.pop()?);

                value_stack.push(left - right)?;
            }
            REF_NULL => {
                let ref_type = RefType::from_byte(*reader.next_byte().ok_or(Error::MissingValue)?)
                    .ok_or(Error::UnexpectedValueType)?;
                value_stack.push(Value::RefNull(ref_type))?;
            }
            REF_FUNC => {
                let func_idx = reader.next_leb128_u32()? as usize;

                functions.get(func_idx).ok_or(Error::InvalidFunctionIndex)?;
                value_stack.push(Value::FuncRef(func_idx))?;
            }
            END => {
                let mut unpacked_result = vec![];
                for ty in result {
                    let val = value_stack.pop()?;
                    unpacked_result.push(val);
                    if !ty.is_type(val) {
                        return Err(Error::TypeMismatch);
                    };
                }
                return Ok(unpacked_result);
            }
            _ => return Err(Error::UndefinedContantExpression),
        }
    }
}
