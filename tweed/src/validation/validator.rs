use crate::binary::global_section::GlobalType;
use crate::binary::module_environment::ModuleEnvironment;
use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;

use crate::core::signature_lookup::lookup_signature;

use crate::core::opcode::*;

use crate::binary::{code_section::FuncCode, type_section::FuncType};

use crate::core::wasm_reader::WasmReader;
use crate::validation::control_stack::ControlStack;
use crate::validation::control_types::ControlType;
use crate::validation::validation_stack::ValidationStack;
use alloc::vec::Vec;
use extensions::{MEMORY_COPY, MEMORY_FILL};

use super::side_table::SideTable;

pub struct Validator<'a, 'b> {
    reader: &'a mut WasmReader,
    module_environment: &'b ModuleEnvironment,
    abstract_control_stack: ControlStack,
    abstract_value_stack: ValidationStack,
    side_table: SideTable,
}

impl<'a, 'b> Validator<'a, 'b> {
    pub fn new(reader: &'a mut WasmReader, module_environment: &'b ModuleEnvironment) -> Self {
        Validator {
            reader,
            module_environment,
            abstract_control_stack: ControlStack::default(),
            abstract_value_stack: ValidationStack::default(),
            side_table: SideTable::default(),
        }
    }

    fn initialize_locals(&mut self, function: &FuncCode, signature: &FuncType) -> Vec<ValType> {
        let mut locals = Vec::new();
        locals.append(&mut signature.parameter.clone());
        locals.append(&mut function.locals.clone());
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

    pub fn validate(&mut self) -> Result<Vec<SideTable>> {
        let mut side_tables = Vec::new();
        for (index, fun) in self.module_environment.codesec.iter().enumerate() {
            let signature_index = self
                .module_environment
                .funcsec
                .get(index)
                .ok_or(Error::InvalidSignatureIndex)?
                .sig_index;

            let signature = lookup_signature(signature_index, &self.module_environment.typesec);

            let side_table = self.validate_code(fun, &signature)?;
            side_tables.push(side_table);
        }
        Ok(side_tables)
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
    fn validate_code(&mut self, function: &FuncCode, signature: &FuncType) -> Result<SideTable> {
        let locals = self.initialize_locals(function, signature);
        let globals = self.initialize_globals();
        self.reader.seek(function.start);
        self.push_ctrl(ControlType::Func {
            func_type: signature.clone(),
            sidetable_idxs: Vec::new(),
            height_of_value_stack: self.abstract_value_stack.height(),
            reachable: true,
        });

        loop {
            let byte = *self.reader.next_byte().ok_or(Error::InvalidControlFlow)?;
            trace!("Validating Byte: {byte:#04X}");
            trace!("Stack: {:?}", self.abstract_value_stack);
            match byte {
                I32_CONST => {
                    self.reader.next_leb128_i32().ok_or(Error::MissingValue)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_CONST => {
                    self.reader.next_leb128_i64().ok_or(Error::MissingValue)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                LOCAL_GET => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidLocalIndex)?;
                    let value = locals.get(idx as usize).ok_or(Error::InvalidLocalIndex)?;
                    self.abstract_value_stack.push(*value);
                }
                LOCAL_SET => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidLocalIndex)?;
                    let value = locals.get(idx as usize).ok_or(Error::InvalidLocalIndex)?;
                    self.abstract_value_stack.pop_expected(*value)?;
                }
                LOCAL_TEE => {
                    let idx = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidLocalIndex)?;
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
                    let idx = self.reader.next_leb128_u32().ok_or(Error::UnknownGlobal)?;
                    let value = globals
                        .get(idx as usize)
                        .ok_or(Error::UnknownGlobal)?
                        .value_type;
                    self.abstract_value_stack.push(value);
                }
                GLOBAL_SET => {
                    let idx = self.reader.next_leb128_u32().ok_or(Error::UnknownGlobal)?;
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
                I64_EQ | I64_NE | I64_LT_U | I64_GT_U | I64_GE_U => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I64_ADD | I64_SUB | I64_MUL | I64_AND | I64_OR | I64_XOR | I64_SHR_U
                | I64_SHR_S | I64_SHL => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;

                    self.abstract_value_stack.push(ValType::I64);
                }
                I32_CTZ | I32_EQZ => {
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
                I64_CTZ => {
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                SELECT => {
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    let val1 = self.abstract_value_stack.pop_val()?;
                    if !val1.is_numeric() {
                        return Err(Error::IllegalSelectType(val1));
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
                    let number_of_branches = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidSectionBytecode)?;

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
                CALL => {
                    let func_index = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidFunctionIndex)?;

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
                    self.check_align(4)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I32_STORE8 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I32_STORE16 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(8)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE8 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE16 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I64_STORE32 => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(4)?;
                    self.abstract_value_stack.pop_expected(ValType::I64)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                }
                I32_LOAD => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(4)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_LOAD8_U => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_LOAD8_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(1)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I32);
                }
                I32_LOAD16_U | I32_LOAD16_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(2)?;
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
                    self.check_align(8)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                I64_LOAD32_S => {
                    if self.module_environment.memorysec.is_empty() {
                        return Err(Error::UndefinedMemory)?;
                    }
                    self.check_align(4)?;
                    self.abstract_value_stack.pop_expected(ValType::I32)?;
                    self.abstract_value_stack.push(ValType::I64);
                }
                EXTENSION_PREFIX => {
                    let byte = *self.reader.next_byte().ok_or(Error::InvalidControlFlow)?;
                    match byte {
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
                        _ => return Err(Error::UndefinedOpCode(byte)),
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
        let align = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
        let _offset = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
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
        let code = FuncCode {
            locals: vec![],
            start: 0,
            end: 40,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        assert!(validator.validate().is_err());
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
        let code = FuncCode {
            locals: vec![],
            start: 29,
            end: 40,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable::new()];
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
        let code = FuncCode {
            locals: vec![],
            start: 23,
            end: 41,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
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
        }];
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
        let code = FuncCode {
            locals: vec![],
            start: 24,
            end: 34,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
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
        }];
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
        let code = FuncCode {
            locals: vec![],
            start: 24,
            end: 34,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
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
        }];
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

        let code = vec![
            FuncCode {
                locals: vec![],
                start: 29,
                end: 29,
            },
            FuncCode {
                locals: vec![],
                start: 32,
                end: 56,
            },
        ];
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: code,
            funcsec: vec![FuncDecl { sig_index: 0 }, FuncDecl { sig_index: 1 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![
            SideTable { entries: vec![] },
            SideTable {
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
            },
        ];
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

        let code = FuncCode {
            locals: vec![ValType::I32],
            start: 25,
            end: 42,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
            entries: vec![SideTableEntry {
                ip: -14,
                stp: 0,
                vals: 0,
                pop: 0,
            }],
        }];
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

        let code = FuncCode {
            locals: vec![ValType::I32],
            start: 25,
            end: 42,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
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
        }];
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

        let code = FuncCode {
            locals: vec![],
            start: 24,
            end: 60,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
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
        }];
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
        let code = FuncCode {
            locals: vec![],
            start: 24,
            end: 38,
        };
        let module_environment = ModuleEnvironment {
            typesec,
            codesec: vec![code],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            ..Default::default()
        };
        let mut validator = Validator::new(&mut reader, &module_environment);
        let actual_sidetable = validator.validate()?;
        let expected_sidetable = vec![SideTable {
            entries: vec![SideTableEntry {
                ip: 2,
                stp: 1,
                vals: 1,
                pop: 2,
            }],
        }];
        assert_eq!(actual_sidetable, expected_sidetable);
        Ok(())
    }
}
