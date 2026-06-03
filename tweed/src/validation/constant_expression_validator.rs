use crate::binary::module_environment::ModuleEnvironment;
use crate::core::opcode::*;

use crate::core::error::ValidationError;
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;

use super::validation_stack::ValidationStack;
pub type Result<T> = core::result::Result<T, ValidationError>;

pub fn validate_const_expr(
    reader: &mut WasmReader,
    result: &[ValType],
    section_end: usize,
    module: &mut ModuleEnvironment,
    number_of_globals: usize,
) -> Result<()> {
    let mut abstract_value_stack: ValidationStack = ValidationStack::default();
    loop {
        // Reading out of the section size
        if reader.current_index() == section_end {
            return Err(ValidationError::UnexpectedEndOfFunctionOrSection);
        }
        let byte = reader.next_byte()?;
        info!("Validate Const Expression Byte: {byte:#04X}");

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
            I32_ADD | I32_SUB | I32_MUL => {
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.pop_expected(ValType::I32)?;
                abstract_value_stack.push(ValType::I32);
            }
            I64_ADD | I64_SUB | I64_MUL => {
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.pop_expected(ValType::I64)?;
                abstract_value_stack.push(ValType::I64);
            }
            GLOBAL_GET => {
                let idx = reader.next_leb128_u32()? as usize;
                let global = module.importsec.globals.get(idx);
                match global {
                    Some(gl) => {
                        if gl.context.is_mutable {
                            return Err(ValidationError::ConstantExpressionRequired);
                        }
                        abstract_value_stack.push(gl.context.value_type);
                    }
                    None => {
                        let all_globals_len = module.importsec.globals.len() + number_of_globals;
                        if idx < all_globals_len {
                            return Err(ValidationError::UnknownGlobal(None));
                        }
                        return Err(ValidationError::UnknownGlobal(Some(idx)));
                    }
                }
            }
            REF_NULL => {
                let ref_type = reader.next_byte()?;
                abstract_value_stack.push(ValType::lookup(&ref_type)?);
            }
            REF_FUNC => {
                let func_idx = reader.next_leb128_u32()? as usize;

                module
                    .funcsec
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
            _ => {
                if is_known_opcode(byte) {
                    return Err(ValidationError::ConstantExpressionRequired);
                } else {
                    return Err(ValidationError::UndefinedConstantExpression);
                }
            }
        }
    }
}
