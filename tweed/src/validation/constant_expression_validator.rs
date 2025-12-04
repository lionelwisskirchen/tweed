use crate::binary::function_section::FuncDecl;
use crate::binary::import_section::GlobalImport;
use crate::core::opcode::*;

use crate::core::error::{Result, ValidationError};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;

use super::validation_stack::ValidationStack;

pub fn validate_const_expr(
    reader: &mut WasmReader,
    globals: &[GlobalImport],
    result: &[ValType],
    functions: &[FuncDecl],
) -> Result<()> {
    let mut abstract_value_stack: ValidationStack = ValidationStack::default();
    loop {
        let byte = reader.next_byte()?;
        trace!("Validate Const Expression Byte: {byte:#04X}");
        match byte {
            I32_CONST => {
                reader.next_leb128_i32()?;
                abstract_value_stack.push(ValType::I32);
            }
            I64_CONST => {
                reader.next_leb128_i64()?;
                abstract_value_stack.push(ValType::I64);
            }
            F32_CONST => {
                reader.next_f32()?;
                abstract_value_stack.push(ValType::F32);
            }
            F64_CONST => {
                reader.next_f64()?;
                abstract_value_stack.push(ValType::F64);
            }
            I32_ADD | I32_SUB => {
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.push(ValType::I32);
            }
            I64_ADD | I64_SUB => {
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.push(ValType::I64);
            }
            GLOBAL_GET => {
                let idx = reader.next_leb128_u32()?;
                let global = globals
                    .get(idx as usize)
                    .ok_or(ValidationError::UnknownGlobal)?;

                if global.context.is_mutable {
                    return Err(ValidationError::UndefinedContantExpression);
                }
                abstract_value_stack.push(global.context.value_type);
            }
            REF_NULL => {
                let ref_type = reader.next_byte()?;
                abstract_value_stack.push(ValType::lookup(&ref_type)?);
            }
            REF_FUNC => {
                let func_idx = reader.next_leb128_u32()? as usize;

                functions
                    .get(func_idx)
                    .ok_or(ValidationError::InvalidFunctionIndex)?;
                abstract_value_stack.push(ValType::FuncRef);
            }
            END => {
                abstract_value_stack.pop_expected_vec(result)?;
                if !abstract_value_stack.is_empty() {
                    return Err(ValidationError::TypeMismatch);
                }
                return Ok(());
            }
            _ => return Err(ValidationError::UndefinedContantExpression),
        }
    }
}
