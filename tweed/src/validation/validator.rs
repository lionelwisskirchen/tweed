use crate::binary::global_section::GlobalType;
use crate::binary::module_environment::ModuleEnvironment;
use crate::core::error::{Error, Result};
use crate::core::value_types::{RefType, ValType};

use crate::core::signature_lookup::lookup_signature;

use crate::core::opcode::*;

use crate::binary::type_section::FuncType;

use crate::core::wasm_reader::WasmReader;
use crate::validation::control_stack::ControlStack;
use crate::validation::control_types::ControlType;
use crate::validation::validation_stack::ValidationStack;
use alloc::vec::Vec;
use extensions::*;

use super::side_table::SideTable;

pub struct CodeValidator<'a, 'b> {
    reader: &'a mut WasmReader,
    module_environment: &'b ModuleEnvironment,
    abstract_control_stack: ControlStack,
    abstract_value_stack: ValidationStack,
    side_table: SideTable,
}

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn new(reader: &'a mut WasmReader, module_environment: &'b ModuleEnvironment) -> Self {
        CodeValidator {
            reader,
            module_environment,
            abstract_control_stack: ControlStack::default(),
            abstract_value_stack: ValidationStack::default(),
            side_table: SideTable::default(),
        }
    }

    fn initialize_locals(&mut self, local_vars: &[ValType], signature: &FuncType) -> Vec<ValType> {
        let mut locals = Vec::new();
        locals.append(&mut signature.parameter.clone());
        locals.append(&mut local_vars.to_owned());
        locals
    }
    fn push_ctrl(&mut self, ctrl: ControlType) {
        match ctrl {
            ControlType::Block {
                reachable,
                height_of_value_stack,
                ..
            }
            | ControlType::Func {
                reachable,
                height_of_value_stack,
                ..
            }
            | ControlType::If {
                reachable,
                height_of_value_stack,
                ..
            }
            | ControlType::For {
                reachable,
                height_of_value_stack,
                ..
            } => {
                self.abstract_value_stack
                    .update(height_of_value_stack, reachable);
            }
        }

        self.abstract_control_stack.push(ctrl);
    }

    fn unreachable(&mut self) {
        self.abstract_value_stack
            .set_top(self.abstract_control_stack.height_of_top());
        self.abstract_control_stack.set_unreachable();
        self.abstract_value_stack
            .update(self.abstract_control_stack.height_of_top(), false);
    }
    fn initialize_globals(&mut self) -> Vec<GlobalType> {
        let mut imported_globals: Vec<GlobalType> = self
            .module_environment
            .importsec
            .globals
            .iter()
            .map(|g| g.context.clone())
            .collect();

        let globals: Vec<GlobalType> = self
            .module_environment
            .globalsec
            .iter()
            .map(|g| g.global_type.clone())
            .collect();
        imported_globals.extend(globals);
        imported_globals
    }
    pub fn validate_code(&mut self, locals: &[ValType], index: usize) -> Result<SideTable> {
        let signature_index = self
            .module_environment
            .funcsec
            .get(index)
            .ok_or(Error::InvalidSignatureIndex)?
            .sig_index;

        let signature = lookup_signature(signature_index, &self.module_environment.typesec);
        let locals = self.initialize_locals(locals, &signature);
        let globals = self.initialize_globals();
        self.push_ctrl(ControlType::Func {
            func_type: signature.clone(),
            sidetable_idxs: Vec::new(),
            height_of_value_stack: self.abstract_value_stack.height(),
            reachable: true,
        });

        loop {
            let byte = *self.reader.next_byte().ok_or(Error::InvalidControlFlow)?;
            info!("Validating Byte: {byte:#04X}");
            trace!("Stack: {:?}", self.abstract_value_stack);
            match byte {
                I32_CONST => {
                    self.reader.next_leb128_i32()?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_CONST => {
                    self.reader.next_leb128_i64()?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                F32_CONST => {
                    self.reader.next_f32()?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F64_CONST => {
                    self.reader.next_f64()?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                LOCAL_GET => {
                    let idx = self.reader.next_leb128_u32()?;
                    let value = locals.get(idx as usize).ok_or(Error::InvalidLocalIndex)?;
                    self.abstract_value_stack.push(*value);
                }
                LOCAL_SET => {
                    let idx = self.reader.next_leb128_u32()?;
                    let value = locals.get(idx as usize).ok_or(Error::InvalidLocalIndex)?;
                    self.abstract_value_stack.pop_expected(*value)?;
                }
                LOCAL_TEE => {
                    let idx = self.reader.next_leb128_u32()?;
                    let value = locals.get(idx as usize).ok_or(Error::InvalidLocalIndex)?;
                    self.abstract_value_stack.is_on_top(value)?;
                }
                I32_ADD | I32_SUB | I32_MUL | I32_DIV_S | I32_DIV_U | I32_REM_S | I32_REM_U => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                IF => {
                    let index_of_if_bytecode = self.reader.current_index() as i32;
                    let signature = self.read_block_type();
                    self.abstract_value_stack.pop_expected(ValType::I32)?; // bool value
                    self.abstract_value_stack
                        .pop_expected_vec(&signature.parameter)?;

                    let index_of_if_sidetable_entry =
                        self.side_table.create_entry(index_of_if_bytecode, 0, 0, 0);

                    self.push_ctrl(ControlType::If {
                        func_type: signature.clone(),
                        sidetable_idxs: Vec::new(),
                        side_table_entry: index_of_if_sidetable_entry,
                        reachable: true,
                        height_of_value_stack: self.abstract_value_stack.height(),
                        has_else: false,
                    });

                    self.abstract_value_stack.push_vec(&signature.parameter);
                }
                ELSE => {
                    let control_type = self.abstract_control_stack.top();
                    let index_of_else_sidetable_entry =
                        self.side_table
                            .create_entry(self.reader.current_index() as i32, 0, 0, 0);

                    match control_type {
                        ControlType::If {
                            func_type,
                            sidetable_idxs,
                            side_table_entry,
                            height_of_value_stack,
                            has_else,
                            ..
                        } => {
                            sidetable_idxs.push(index_of_else_sidetable_entry);
                            let expected_results = func_type.result.clone();

                            self.abstract_value_stack
                                .pop_expected_vec(&expected_results)?;

                            if *height_of_value_stack != self.abstract_value_stack.height() {
                                return Err(Error::TypeMismatch);
                            }
                            *has_else = true;

                            self.abstract_value_stack.push_vec(&func_type.parameter);

                            let side_table_len = self.side_table.len();
                            let entry = self.side_table.get_entry(*side_table_entry);
                            // branch from "if" will go to instruction after else
                            entry.ip = (self.reader.current_index() + 1) as i32 - entry.ip;
                            // branch if will skip all sidetable entries from if to else
                            entry.stp = side_table_len as i32 - *side_table_entry as i32;
                        }
                        _ => {
                            return Err(Error::InvalidControlFlow);
                        }
                    }
                }
                GLOBAL_GET => {
                    let idx = self.reader.next_leb128_u32()?;
                    let value = globals
                        .get(idx as usize)
                        .ok_or(Error::UnknownGlobal)?
                        .value_type;
                    self.abstract_value_stack.push(value);
                }
                GLOBAL_SET => {
                    let idx = self.reader.next_leb128_u32()?;
                    let global = globals.get(idx as usize).ok_or(Error::UnknownGlobal)?;
                    if !global.is_mutable {
                        return Err(Error::GlobalIsImmutable);
                    }
                    self.abstract_value_stack.pop_expected(global.value_type)?;
                }
                NOP => {}
                DROP => {
                    self.abstract_value_stack.pop_val()?;
                }
                I32_GE_U | I32_GT_U | I32_LE_S | I32_LT_S | I32_LT_U | I32_LE_U | I32_NE
                | I32_GT_S | I32_GE_S | I32_AND | I32_OR | I32_XOR | I32_EQ | I32_SHL
                | I32_SHR_U | I32_SHR_S | I32_ROTL | I32_ROTR => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_EQ | I64_NE | I64_LT_U | I64_LT_S | I64_LE_S | I64_LE_U | I64_GE_S
                | I64_GT_U | I64_GT_S | I64_GE_U => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_ADD | I64_SUB | I64_MUL | I64_AND | I64_OR | I64_XOR | I64_SHR_U
                | I64_SHR_S | I64_SHL | I64_ROTL | I64_ROTR | I64_DIV_S | I64_DIV_U | I64_REM_S
                | I64_REM_U => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;

                    self.abstract_value_stack.push(ValType::I64);
                }
                F64_ABS | F64_NEG | F64_CEIL | F64_FLOOR | F64_TRUNC | F64_NEAREST | F64_SQRT => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_COPYSIGN => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F64_COPYSIGN => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F64_PROMOTE_F32 => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_DEMOTE_F64 => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::F32);
                }

                I32_REINTERPRET_F32 => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }

                I64_REINTERPRET_F64 => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::I64);
                }

                F64_REINTERPRET_I64 => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_REINTERPRET_I32 => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F32_EQ | F32_NE | F32_LT | F32_GT | F32_LE | F32_GE => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.pop_expected(ValType::F32)?;

                    self.abstract_value_stack.push(ValType::I32);
                }
                F32_ABS | F32_NEG | F32_CEIL | F32_FLOOR | F32_TRUNC | F32_NEAREST | F32_SQRT => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F64_ADD | F64_SUB | F64_MUL | F64_DIV | F64_MIN | F64_MAX => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.pop_expected(ValType::F64)?;

                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_ADD | F32_SUB | F32_MUL | F32_DIV | F32_MIN | F32_MAX => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.pop_expected(ValType::F32)?;

                    self.abstract_value_stack.push(ValType::F32);
                }
                I32_TRUNC_F32_S | I32_TRUNC_F32_U => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_TRUNC_F64_S | I32_TRUNC_F64_U => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::I32);
                }

                I64_TRUNC_F32_S | I64_TRUNC_F32_U => {
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I64_TRUNC_F64_S | I64_TRUNC_F64_U => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.push(ValType::I64);
                }

                F64_EQ | F64_NE | F64_LT | F64_GT | F64_LE | F64_GE => {
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.pop_expected(ValType::F64)?;

                    self.abstract_value_stack.push(ValType::I32);
                }
                F64_CONVERT_I64_U | F64_CONVERT_I64_S => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_CONVERT_I32_U | F32_CONVERT_I32_S => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F64_CONVERT_I32_S | F64_CONVERT_I32_U => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                F32_CONVERT_I64_S | F32_CONVERT_I64_U => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                I32_CTZ | I32_POPCNT | I32_EQZ => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_WRAP_I64 => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_EXTEND_I32_U | I64_EXTEND_I32_S => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I32_EXTEND8_S | I32_EXTEND16_S => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_EQZ => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_CTZ | I64_CLZ | I64_POPCNT | I64_EXTEND8_S | I64_EXTEND16_S
                | I64_EXTEND32_S => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                TABLE_GET => {
                    let table_idx = self.reader.next_leb128_u32()? as usize;

                    if self.module_environment.tablesec.len() <= table_idx {
                        return Err(Error::UndefinedTable);
                    }
                    let ref_type = self
                        .module_environment
                        .tablesec
                        .get(table_idx)
                        .unwrap()
                        .ref_type;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ref_type.to_val_type());
                }
                TABLE_SET => {
                    let table_idx = self.reader.next_leb128_u32()? as usize;

                    if self.module_environment.tablesec.len() <= table_idx {
                        return Err(Error::UndefinedTable);
                    }

                    let ref_type = self
                        .module_environment
                        .tablesec
                        .get(table_idx)
                        .unwrap()
                        .ref_type;

                    self.abstract_value_stack
                        .pop_expected(ref_type.to_val_type())?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                SELECT_T => {
                    let num = self.reader.next_leb128_u32()?;
                    if num != 1 {
                        return Err(Error::InvalidResultArity);
                    }
                    let ty_byte = self.reader.next_byte().ok_or(Error::UnsupportedValueType)?;
                    let ty = ValType::lookup(ty_byte)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ty)?;
                    self.abstract_value_stack.pop_expected(ty)?;
                    self.abstract_value_stack.push(ty);
                }
                SELECT => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    let val1 = self.abstract_value_stack.pop_val()?;
                    if !val1.is_numeric() {
                        return Err(Error::IllegalSelectType);
                    };
                    self.abstract_value_stack.pop_expected(val1)?;
                    self.abstract_value_stack.push(val1);
                }
                RETURN => {
                    let control_type = self.abstract_control_stack.top();
                    match control_type {
                        ControlType::For { func_type, .. }
                        | ControlType::Block { func_type, .. }
                        | ControlType::If { func_type, .. }
                        | ControlType::Func { func_type, .. } => {
                            self.abstract_value_stack
                                .pop_expected_vec(&func_type.result)?;
                        }
                    }
                    self.unreachable();
                }
                BLOCK => {
                    let signature = self.read_block_type();

                    self.abstract_value_stack
                        .pop_expected_vec(&signature.parameter)?;

                    self.push_ctrl(ControlType::Block {
                        func_type: signature.clone(),
                        sidetable_idxs: Vec::new(),
                        reachable: true,
                        height_of_value_stack: self.abstract_value_stack.height(),
                    });
                    self.abstract_value_stack.push_vec(&signature.parameter);
                }
                LOOP => {
                    let index_of_loop_bytecode = self.reader.current_index();
                    let signature = self.read_block_type();
                    self.abstract_value_stack
                        .pop_expected_vec(&signature.parameter)?;

                    self.push_ctrl(ControlType::For {
                        instruction_pointer: index_of_loop_bytecode,
                        func_type: signature.clone(),
                        side_table_entry: self.side_table.len(),
                        reachable: true,
                        height_of_value_stack: self.abstract_value_stack.height(),
                    });
                    self.abstract_value_stack.push_vec(&signature.parameter);
                }
                BR_IF => {
                    let index_of_br_if_bytecode = self.reader.current_index() as i32;

                    self.abstract_value_stack.pop_expected(ValType::I32)?; // bool

                    let depth = self.read_depth();

                    self.create_sidetable_entry(depth, index_of_br_if_bytecode)?;
                }
                BR_TABLE => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?; // index of branch
                    let number_of_branches = self.reader.next_leb128_u32()?;

                    let index_of_br_table_bytecode = self.reader.current_index() as i32; // We jump from here because we need to parse the number of branches during execution (because the default branch can be any i32 and could move the stp to a wrong entry).

                    for _ in 0..=number_of_branches {
                        let depth = self.read_depth();
                        self.create_sidetable_entry(depth, index_of_br_table_bytecode)?;
                    }
                    self.unreachable();
                }
                BR => {
                    let index_of_br_bytecode = self.reader.current_index() as i32;
                    let depth = self.read_depth();
                    self.create_sidetable_entry(depth, index_of_br_bytecode)?;
                    self.unreachable();
                }
                END => {
                    let control_type = self.abstract_control_stack.pop();

                    match control_type {
                        ControlType::Func {
                            func_type,
                            sidetable_idxs,
                            height_of_value_stack,
                            ..
                        } => {
                            self.check_args(&func_type.result, height_of_value_stack)?;
                            self.resolve_sidetable_entries(&sidetable_idxs);
                            break;
                        }
                        ControlType::Block {
                            func_type,
                            sidetable_idxs,
                            height_of_value_stack,
                            ..
                        } => {
                            self.check_args(&func_type.result, height_of_value_stack)?;
                            self.resolve_sidetable_entries(&sidetable_idxs);
                        }
                        ControlType::If {
                            func_type,
                            sidetable_idxs,
                            height_of_value_stack,
                            has_else,
                            side_table_entry,
                            ..
                        } => {
                            if !has_else {
                                self.resolve_sidetable_entries(&[side_table_entry]);
                                // Parameter pass through when there is no else block
                                if func_type.result.len() > func_type.parameter.len() {
                                    return Err(Error::TypeMismatch);
                                }
                            }

                            self.check_args(&func_type.result, height_of_value_stack)?;
                            self.resolve_sidetable_entries(&sidetable_idxs);
                        }
                        ControlType::For {
                            func_type,
                            height_of_value_stack,
                            ..
                        } => self.check_args(&func_type.result, height_of_value_stack)?,
                    }
                    self.abstract_value_stack.update(
                        self.abstract_control_stack.height_of_top(),
                        self.abstract_control_stack.top_is_reachable(),
                    )
                }
                CALL_INDIRECT => {
                    let type_index = self.reader.next_leb128_u32()?;

                    let table_index = self.reader.next_leb128_u32()?;

                    let table = self
                        .module_environment
                        .tablesec
                        .get(table_index as usize)
                        .ok_or(Error::UnknownTable)?;
                    if table.ref_type != RefType::FuncRef {
                        return Err(Error::TypeMismatch);
                    }
                    let func_type = self
                        .module_environment
                        .typesec
                        .get(type_index as usize)
                        .ok_or(Error::UnknownType)?;

                    self.abstract_value_stack.pop_expected(ValType::I32)?; // Table Entry Index
                    self.abstract_value_stack
                        .pop_expected_vec(&func_type.parameter)?;
                    self.abstract_value_stack.push_vec(&func_type.result);
                }
                CALL => {
                    let func_index = self.reader.next_leb128_u32()?;

                    let type_idx = self
                        .module_environment
                        .funcsec
                        .get(func_index as usize)
                        .ok_or(Error::InvalidFunctionIndex)?
                        .sig_index;
                    let sig = self
                        .module_environment
                        .typesec
                        .get(type_idx as usize)
                        .ok_or(Error::InvalidFunctionIndex)?;
                    self.abstract_value_stack.pop_expected_vec(&sig.parameter)?;
                    self.abstract_value_stack.push_vec(&sig.result);
                }
                I32_STORE => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I32_STORE8 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(0)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I32_STORE16 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(3)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE8 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(0)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE16 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE32 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                F32_STORE => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::F32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                F64_STORE => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(3)?;
                    self.abstract_value_stack.pop_expected(ValType::F64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }

                I32_LOAD => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                F32_LOAD => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::F32);
                }
                F64_LOAD => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }

                    self.check_align(3)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::F64);
                }
                I32_LOAD8_U => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(0)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_LOAD8_S | I64_LOAD8_U => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(0)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I64_LOAD16_U | I64_LOAD16_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I64_LOAD32_U | I64_LOAD32_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I32_LOAD8_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }

                    self.check_align(0)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_LOAD16_U | I32_LOAD16_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_CLZ => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_LOAD => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(3)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                REF_IS_NULL => {
                    let value = self.abstract_value_stack.pop_val()?;
                    if !value.is_ref() {
                        return Err(Error::TypeMismatch);
                    }
                    self.abstract_value_stack.push(ValType::I32);
                }
                REF_NULL => {
                    let ref_type = self.reader.next_byte().ok_or(Error::MissingValue)?;
                    self.abstract_value_stack.push(ValType::lookup(ref_type)?);
                }
                MEMORY_GROW => {
                    let idx = *self.reader.next_byte().ok_or(Error::MissingValue)?;
                    if idx != 0 {
                        return Err(Error::MultipleMemoriesNotImplemented);
                    }

                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                MEMORY_SIZE => {
                    let idx = *self.reader.next_byte().ok_or(Error::MissingValue)?;
                    if idx != 0 {
                        return Err(Error::MultipleMemoriesNotImplemented);
                    }

                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.abstract_value_stack.push(ValType::I32);
                }
                REF_FUNC => {
                    let func_idx = self.reader.next_leb128_u32()? as usize;

                    self.module_environment
                        .funcsec
                        .get(func_idx)
                        .ok_or(Error::InvalidFunctionIndex)?;
                    self.abstract_value_stack.push(ValType::FuncRef);
                }
                EXTENSION_PREFIX => {
                    let byte = self.reader.next_leb128_u32()?;
                    match byte {
                        I32_TRUNC_SAT_F32_S | I32_TRUNC_SAT_F32_U => {
                            self.abstract_value_stack.pop_expected(ValType::F32)?;
                            self.abstract_value_stack.push(ValType::I32);
                        }
                        I32_TRUNC_SAT_F64_S | I32_TRUNC_SAT_F64_U => {
                            self.abstract_value_stack.pop_expected(ValType::F64)?;
                            self.abstract_value_stack.push(ValType::I32);
                        }

                        I64_TRUNC_SAT_F32_S | I64_TRUNC_SAT_F32_U => {
                            self.abstract_value_stack.pop_expected(ValType::F32)?;
                            self.abstract_value_stack.push(ValType::I64);
                        }
                        I64_TRUNC_SAT_F64_U | I64_TRUNC_SAT_F64_S => {
                            self.abstract_value_stack.pop_expected(ValType::F64)?;
                            self.abstract_value_stack.push(ValType::I64);
                        }
                        DATA_DROP => {
                            let data_index = self.reader.next_leb128_u32()?;
                            if self.module_environment.datacountsec == 0
                                || self.module_environment.datacountsec <= data_index
                            {
                                return Err(Error::NoDataSegmentFound(None))?;
                            }
                        }
                        MEMORY_INIT => {
                            let data_index = self.reader.next_leb128_u32()?;
                            let idx = *self.reader.next_byte().ok_or(Error::MissingValue)?;
                            if idx != 0 {
                                return Err(Error::MultipleMemoriesNotImplemented);
                            }
                            if self.module_environment.memorysec.is_empty() {
                                return Err(Error::UnknownMemory(idx as usize))?;
                            }
                            if self.module_environment.datacountsec == 0
                                || self.module_environment.datacountsec <= data_index
                            {
                                return Err(Error::NoDataSegmentFound(Some(data_index as usize)))?;
                            }
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                        }
                        MEMORY_COPY => {
                            let (dest, src) = (
                                *self.reader.next_byte().ok_or(Error::MissingValue)?,
                                *self.reader.next_byte().ok_or(Error::MissingValue)?,
                            );
                            if dest != 0 || src != 0 {
                                return Err(Error::MultipleMemoriesNotImplemented);
                            }

                            if self.module_environment.memorysec.is_empty() {
                                return Err(Error::UndefinedMemory)?;
                            }
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                        }
                        MEMORY_FILL => {
                            let idx = *self.reader.next_byte().ok_or(Error::MissingValue)?;
                            if idx != 0 {
                                return Err(Error::MultipleMemoriesNotImplemented);
                            }

                            if self.module_environment.memorysec.is_empty() {
                                return Err(Error::UndefinedMemory)?;
                            }

                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                            self.abstract_value_stack.pop_expected(ValType::I32)?;
                        }
                        _ => return Err(Error::UndefinedOpCode(byte as u8)),
                    }
                }

                UNREACHABLE => self.unreachable(),
                _ => return Err(Error::UndefinedOpCode(byte)),
            };
        }
        let result = self.side_table.clone();
        self.reset();
        Ok(result)
    }
    fn check_align(&mut self, max_align: u32) -> Result<()> {
        let align = self.reader.next_leb128_u32()?;
        let _offset = self.reader.next_leb128_u32()?;

        if align > 31 {
            return Err(Error::MalformedMemOpFlags)?;
        }
        if align > max_align {
            return Err(Error::InvalidAlignment)?;
        }
        Ok(())
    }

    fn read_depth(&mut self) -> u32 {
        self.reader
            .next_leb128_u32()
            .expect("Error parsing break depth")
    }

    fn reset(&mut self) {
        self.abstract_control_stack.reset();
        self.abstract_value_stack.reset();
        self.side_table.reset();
    }

    fn check_args(&mut self, args: &[ValType], height_of_value_stack: u32) -> Result<()> {
        self.abstract_value_stack.pop_expected_vec(args)?;

        if height_of_value_stack != self.abstract_value_stack.height() {
            return Err(Error::TypeMismatch);
        }

        self.abstract_value_stack.push_vec(args);
        Ok(())
    }

    fn resolve_sidetable_entries(&mut self, sidetable_idxs: &[usize]) {
        let abs_target_pos = self.reader.current_index();
        for entry_index in sidetable_idxs.iter() {
            let sidetable_offset = self.side_table.len() as i32 - *entry_index as i32;
            let entry = self.side_table.get_entry(*entry_index);
            let abs_start_pos = entry.ip;
            let rel_offset = abs_target_pos as i32 - abs_start_pos;

            entry.stp = sidetable_offset;
            entry.ip = rel_offset;
        }
    }
    fn create_sidetable_entry(&mut self, depth: u32, index_of_start_bytecode: i32) -> Result<()> {
        let target_control = self.abstract_control_stack.get_control_type(depth)?;
        let index_of_br_sidetable_entry =
            self.side_table
                .create_entry(index_of_start_bytecode, 0, 0, 0);
        match target_control {
            ControlType::Block {
                sidetable_idxs,
                func_type,
                height_of_value_stack,
                ..
            }
            | ControlType::If {
                sidetable_idxs,
                func_type,
                height_of_value_stack,
                ..
            }
            | ControlType::Func {
                sidetable_idxs,
                func_type,
                height_of_value_stack,
                ..
            } => {
                sidetable_idxs.push(index_of_br_sidetable_entry);

                self.abstract_value_stack
                    .pop_expected_vec(&func_type.result)?;

                let entry = self.side_table.get_entry(index_of_br_sidetable_entry);
                entry.pop = (self.abstract_value_stack.height() - *height_of_value_stack) as usize;
                entry.vals = func_type.result.len();
                self.abstract_value_stack.push_vec(&func_type.result);
            }
            ControlType::For {
                func_type,
                side_table_entry,
                instruction_pointer,
                height_of_value_stack,
                ..
            } => {
                self.abstract_value_stack
                    .pop_expected_vec(&func_type.parameter)?;

                let stp = *side_table_entry as i32 - (self.side_table.len() as i32) + 1;
                let ip = *instruction_pointer as i32 - index_of_start_bytecode;
                let entry = self.side_table.get_entry(index_of_br_sidetable_entry);
                entry.pop = (self.abstract_value_stack.height() - *height_of_value_stack) as usize;
                entry.vals = func_type.result.len();
                entry.ip = ip;
                entry.stp = stp;
                self.abstract_value_stack.push_vec(&func_type.parameter);
            }
        };
        Ok(())
    }
    fn read_block_type(&mut self) -> FuncType {
        let func_idx = self
            .reader
            .next_leb128_u32()
            .expect("Error parsing block func idx");
        lookup_signature(func_idx, &self.module_environment.typesec)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{binary::function_section::FuncDecl, validation::side_table::SideTableEntry};
    #[test]
    fn test_validation_undefined_opcode() {
        let bytes = wat::parse_str(
            "(module

             )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![
            FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
        ];
        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        assert!(validator.validate_code(&vec![], 0).is_err());
    }

    #[test]
    fn test_validation_with_blocks() -> Result<()> {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
                 (block $label1 (result i32)
	    	   i32.const 10
		   (block $label2 (param i32) (result i32)
		   	  i32.const 2
			  i32.add
		   )
	         )
               )
             )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![
            FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            },
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
        ];
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(29);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable::new();
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_validation_with_branches() -> Result<()> {
        let bytes = wat::parse_str(
            "(module
                (func
                  (block $label1
                     br $label1
 	            (block $label2
		       (block $label3
		           br $label1
		        )
      		       (block $label4
		           br $label2
		        )
		    )
	          )
                )
             )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];
        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(23);

        let mut validator = CodeValidator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 15,
                    stp: 3,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 9,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 3,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }
    #[test]
    fn test_validation_with_if_else() -> Result<()> {
        let bytes = wat::parse_str(
            "(module
              (func (result i32)
                i32.const 0
                (if (result i32)
                 (then
                  i32.const 1
                 )
                (else
                 i32.const 0
                )
              )
             )
            )",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];
        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(24);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 5,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 3,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }
    #[test]
    fn test_validation_with_if_else_and_branches() -> Result<()> {
        let bytes = wat::parse_str(
            "
(module
  (func (result i32)
    (block $label1 (result i32)
      i32.const 0
      (if $if (result i32)
        (then
          i32.const 1
          br $label1
        )
        (else
	i32.const 3
	br $if
        )
      )
      br $label1
    )
  )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];
        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(24);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 7,
                    stp: 3,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 10,
                    stp: 4,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_validation_with_if_else_and_conditional_branches() -> Result<()> {
        let bytes = wat::parse_str(
            "
(module
  (func)
  (func (param i32) (result i32)
    (block $label1 (result i32)
      (local.get 0)
      (if (result i32)
        (then
          (call 0)
          i32.const 1
        )
        (else
          (call 0)
          i32.const 0
        )
      )
      i32.const 2
      (br_if 0)
      i32.const 3
      return
    )
  )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![
            FuncType {
                parameter: vec![],
                result: vec![],
            },
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            },
        ];

        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }, FuncDecl { sig_index: 1 }],
            ..Default::default()
        };
        reader.seek(32);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 1)?;
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 7,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_validation_with_for_loop() -> Result<()> {
        let bytes = wat::parse_str(
            "
(module

(func
      (local $i i32)
      (loop $my_loop
      local.get $i
      i32.const 1
      i32.add
      local.set $i

      local.get $i
      i32.const 10
      i32.lt_s
      br_if $my_loop
    )
 )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];

        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(25);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![ValType::I32], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![SideTableEntry {
                ip: -14,
                stp: 0,
                vals: 0,
                pop: 0,
            }],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_validation_with_for_loop_and_blocks_with_branches() -> Result<()> {
        let bytes = wat::parse_str(
            "
(module
(func
      (local $i i32)
      (loop $my_loop
      local.get $i
      i32.const 1
      i32.add
      local.set $i
       (block $label1 (result i32)
      i32.const 0
      (if $if (result i32)
        (then
          i32.const 1
          br $label1
        )
        (else
	i32.const 3
	br $if
        )
      )
      br $label1
     )
      i32.const 10
      i32.lt_s
      br_if $my_loop
    )
 )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![],
        }];

        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(25);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![ValType::I32], 0)?;

        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 7,
                    stp: 3,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 10,
                    stp: 4,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 2,
                    stp: 1,
                    vals: 1,
                    pop: 0,
                },
                SideTableEntry {
                    ip: -31,
                    stp: -5,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_validation_with_br_tables() -> Result<()> {
        let bytes = wat::parse_str(
            "
(func (result i32)
  (block
    (block
      (block
        (block
               i32.const 0
               (br_table
                         2
                         1
                         0
                         3)
	)
        i32.const 100
        return
       )
       i32.const 101
       return
      )
    i32.const 102
    return
    )
i32.const 103
return
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];

        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(24);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![
                SideTableEntry {
                    ip: 15,
                    stp: 4,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 10,
                    stp: 3,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 5,
                    stp: 2,
                    vals: 0,
                    pop: 0,
                },
                SideTableEntry {
                    ip: 20,
                    stp: 1,
                    vals: 0,
                    pop: 0,
                },
            ],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }

    #[test]
    fn test_pop_val() -> Result<()> {
        let bytes = wat::parse_str(
            "
(module
   (func (result i32)
       (block $label1 (result i32)
          i32.const 12
          i32.const 10
          i32.const 10
          br $label1
       )
          i32.const 10
          i32.add
   )
)",
        )
        .unwrap();
        let mut reader = WasmReader::new(bytes.to_vec());
        let typesec = vec![FuncType {
            parameter: vec![],
            result: vec![ValType::I32],
        }];
        let module_environment = ModuleEnvironment {
            typesec,
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        reader.seek(24);
        let mut validator = CodeValidator::new(&mut reader, &module_environment);

        let actual_sidetable = validator.validate_code(&vec![], 0)?;
        let expected_sidetable = SideTable {
            entries: vec![SideTableEntry {
                ip: 2,
                stp: 1,
                vals: 1,
                pop: 2,
            }],
        };
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }
}
