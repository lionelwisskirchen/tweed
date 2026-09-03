use crate::alloc::borrow::ToOwned;
use crate::alloc::collections::BTreeSet;
use crate::alloc::vec::Vec;
use crate::binary::global_section::GlobalType;
use crate::binary::memory_section::Limits;
use crate::binary::module_environment::ModuleEnvironment;
use crate::binary::type_section::FuncType;
use crate::core::error::ValidationError;
use crate::core::opcode::*;
use crate::core::signature_lookup::lookup_signature;
use crate::core::value_types::{RefType, ValType};
use crate::core::wasm_reader::WasmReader;
use crate::validation::control_stack::ControlStack;
use crate::validation::control_types::ControlType;
use crate::validation::validation_stack::ValidationStack;

use extensions::*;

use super::side_table::SideTable;

pub type Result<T> = core::result::Result<T, ValidationError>;

pub struct CodeValidator<'a, 'b> {
    pub(super) reader: &'a mut WasmReader,
    pub(super) module_environment: &'b ModuleEnvironment,
    pub(super) abstract_control_stack: ControlStack,
    pub(super) abstract_value_stack: ValidationStack,
    pub(super) side_table: SideTable,
    pub(super) refs: &'b BTreeSet<usize>,
}

impl<'a, 'b> CodeValidator<'a, 'b> {
    pub fn new(
        reader: &'a mut WasmReader,
        module_environment: &'b ModuleEnvironment,
        refs: &'b BTreeSet<usize>,
    ) -> Self {
        CodeValidator {
            reader,
            module_environment,
            abstract_control_stack: ControlStack::default(),
            abstract_value_stack: ValidationStack::default(),
            side_table: SideTable::default(),
            refs,
        }
    }

    fn initialize_locals(&mut self, local_vars: &[ValType], signature: &FuncType) -> Vec<ValType> {
        let mut locals = Vec::new();
        locals.append(&mut signature.parameter.clone());
        locals.append(&mut local_vars.to_owned());
        locals
    }
    pub fn push_ctrl(&mut self, ctrl: ControlType) {
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

    pub fn unreachable(&mut self) {
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

    /// Initialize the tables for the validation process
    ///
    /// In this function, the tables are concatenated as follows:
    /// | Tables            |
    /// |-------------------|
    /// | imported table 0  |
    /// | imported table 1  |
    /// | imported table x  |
    /// | local table 0     |
    /// | local table 1     |
    ///
    /// This is needed to be able to validate the module that use imported tables
    ///
    /// # Returns
    /// Returns a list of [`RefType`]s where imported tables appear first, in their original order,
    /// followed by locally defined tables in module order.
    fn initialize_tables(&mut self) -> Vec<RefType> {
        let mut imported_tables: Vec<RefType> = self
            .module_environment
            .importsec
            .tables
            .iter()
            .map(|t| t.context.ref_type)
            .collect();

        let local_tables: Vec<RefType> = self
            .module_environment
            .tablesec
            .iter()
            .map(|g| g.ref_type)
            .collect();
        imported_tables.extend(local_tables);
        imported_tables
    }

    /// Initialize the tables for the validation process
    ///
    /// This is needed to be able to validate the module that use imported tables
    ///
    /// # Returns
    /// Returns a list of [`Limits`]s where imported memories appear first, in their original order,
    /// followed by locally defined memories in module order.
    fn initialize_memories(&mut self) -> Vec<Limits> {
        let mut imported_memories: Vec<Limits> = self
            .module_environment
            .importsec
            .memories
            .iter()
            .map(|t| t.context.limits.clone())
            .collect();

        let local_memories: Vec<Limits> = self
            .module_environment
            .memorysec
            .iter()
            .map(|g| g.limits.clone())
            .collect();
        imported_memories.extend(local_memories);
        imported_memories
    }

    pub fn validate_code(
        &mut self,
        locals: &[ValType],
        index: usize,
        section_end: usize,
    ) -> Result<SideTable> {
        let signature_index = self
            .module_environment
            .funcsec
            .get(index)
            .ok_or(ValidationError::InvalidSignatureIndex)?
            .sig_index;

        let func_signature = lookup_signature(signature_index, &self.module_environment.typesec)?;
        self.push_ctrl(ControlType::Func {
            func_type: func_signature.clone(),
            sidetable_idxs: Vec::new(),
            height_of_value_stack: self.abstract_value_stack.height(),
            reachable: true,
        });

        let locals = self.initialize_locals(locals, &func_signature);
        let globals = self.initialize_globals();
        let tables = self.initialize_tables();
        let memories = self.initialize_memories();

        loop {
            // Reading out of the section size
            if self.reader.current_index() == section_end {
                Err(ValidationError::EndOpCodeExpected)?;
            }
            let byte = self.reader.next_byte()?;
            info!("Validating Byte: {byte:#04X}");
            trace!("Stack: {:?}", self.abstract_value_stack);
            match byte {
                I32_CONST => self.validate_i32_const()?,
                I64_CONST => self.validate_i64_const()?,
                F32_CONST => self.validate_f32_const()?,
                F64_CONST => self.validate_f64_const()?,
                LOCAL_GET => self.validate_local_get(&locals)?,
                LOCAL_SET => self.validate_local_set(&locals)?,
                LOCAL_TEE => self.validate_local_tee(&locals)?,
                I32_ADD | I32_SUB | I32_MUL | I32_DIV_S | I32_DIV_U | I32_REM_S | I32_REM_U
                | I32_AND | I32_OR | I32_XOR | I32_EQ | I32_SHL | I32_SHR_U | I32_SHR_S
                | I32_ROTL | I32_ROTR => self.validate_i32_binop()?,
                IF => self.validate_if()?,
                ELSE => self.validate_else()?,
                GLOBAL_GET => self.validate_global_get(&globals)?,
                GLOBAL_SET => self.validate_global_set(&globals)?,
                NOP => self.validate_nop()?,
                DROP => self.validate_drop()?,
                I32_GE_U | I32_GT_U | I32_LE_S | I32_LT_S | I32_LT_U | I32_LE_U | I32_NE
                | I32_GT_S | I32_GE_S => self.validate_i32_cmp()?,
                I64_EQ | I64_NE | I64_LT_U | I64_LT_S | I64_LE_S | I64_LE_U | I64_GE_S
                | I64_GT_U | I64_GT_S | I64_GE_U => self.validate_i64_cmp()?,
                I64_ADD | I64_SUB | I64_MUL | I64_AND | I64_OR | I64_XOR | I64_SHR_U
                | I64_SHR_S | I64_SHL | I64_ROTL | I64_ROTR | I64_DIV_S | I64_DIV_U | I64_REM_S
                | I64_REM_U => self.validate_i64_binop()?,
                F64_ABS | F64_NEG | F64_CEIL | F64_FLOOR | F64_TRUNC | F64_NEAREST | F64_SQRT => {
                    self.validate_f64_unop()?
                }
                F32_COPYSIGN => self.validate_f32_copysign()?,
                F64_COPYSIGN => self.validate_f64_copysign()?,
                F64_PROMOTE_F32 => self.validate_f64_promote_f32()?,
                F32_DEMOTE_F64 => self.validate_f32_demote_f64()?,
                I32_REINTERPRET_F32 => self.validate_i32_reinterpret_f32()?,
                I64_REINTERPRET_F64 => self.validate_i64_reinterpret_f64()?,
                F64_REINTERPRET_I64 => self.validate_f64_reinterpret_i64()?,
                F32_REINTERPRET_I32 => self.validate_f32_reinterpret_i32()?,
                F32_EQ | F32_NE | F32_LT | F32_GT | F32_LE | F32_GE => self.validate_f32_cmp()?,
                F32_ABS | F32_NEG | F32_CEIL | F32_FLOOR | F32_TRUNC | F32_NEAREST | F32_SQRT => {
                    self.validate_f32_unop()?
                }
                F64_ADD | F64_SUB | F64_MUL | F64_DIV | F64_MIN | F64_MAX => {
                    self.validate_f64_binop()?
                }
                F32_ADD | F32_SUB | F32_MUL | F32_DIV | F32_MIN | F32_MAX => {
                    self.validate_f32_binop()?
                }
                I32_TRUNC_F32_S | I32_TRUNC_F32_U => self.validate_i32_trunc_f32()?,
                I32_TRUNC_F64_S | I32_TRUNC_F64_U => self.validate_i32_trunc_f64()?,
                I64_TRUNC_F32_S | I64_TRUNC_F32_U => self.validate_i64_trunc_f32()?,
                I64_TRUNC_F64_S | I64_TRUNC_F64_U => self.validate_i64_trunc_f64()?,
                F64_EQ | F64_NE | F64_LT | F64_GT | F64_LE | F64_GE => self.validate_f64_cmp()?,
                F64_CONVERT_I64_U | F64_CONVERT_I64_S => self.validate_f64_convert_i64()?,
                F32_CONVERT_I32_U | F32_CONVERT_I32_S => self.validate_f32_convert_i32()?,
                F64_CONVERT_I32_S | F64_CONVERT_I32_U => self.validate_f64_convert_i32()?,
                F32_CONVERT_I64_S | F32_CONVERT_I64_U => self.validate_f32_convert_i64()?,
                I32_CTZ | I32_CLZ | I32_POPCNT | I32_EQZ => self.validate_i32_unop()?,
                I32_WRAP_I64 => self.validate_i32_wrap_i64()?,
                I64_EXTEND_I32_U | I64_EXTEND_I32_S => self.validate_i64_extend_i32()?,
                I32_EXTEND8_S | I32_EXTEND16_S => self.validate_i32_extend()?,
                I64_EQZ => self.validate_i64_eqz()?,
                I64_CTZ | I64_CLZ | I64_POPCNT | I64_EXTEND8_S | I64_EXTEND16_S
                | I64_EXTEND32_S => self.validate_i64_unop()?,
                TABLE_GET => self.validate_table_get(&tables)?,
                TABLE_SET => self.validate_table_set(&tables)?,
                SELECT_T => self.validate_select_t()?,
                SELECT => self.validate_select()?,
                RETURN => self.validate_return(&func_signature)?,
                BLOCK => self.validate_block()?,
                LOOP => self.validate_loop()?,
                BR_IF => self.validate_br_if()?,
                BR_TABLE => self.validate_br_table()?,
                BR => self.validate_br()?,
                END => {
                    if self.validate_end()? == 1 {
                        break;
                    };
                }
                CALL_INDIRECT => self.validate_call_indirect(&tables)?,
                CALL => self.validate_call()?,
                I32_STORE => self.validate_i32_store(&memories, 2)?,
                I32_STORE8 => self.validate_i32_store(&memories, 0)?,
                I32_STORE16 => self.validate_i32_store(&memories, 1)?,
                I64_STORE => self.validate_i64_store(&memories, 3)?,
                I64_STORE8 => self.validate_i64_store(&memories, 0)?,
                I64_STORE16 => self.validate_i64_store(&memories, 1)?,
                I64_STORE32 => self.validate_i64_store(&memories, 2)?,
                F32_STORE => self.validate_f32_store(&memories)?,
                F64_STORE => self.validate_f64_store(&memories)?,
                I32_LOAD => self.validate_i32_load(&memories, 2)?,
                I32_LOAD8_U | I32_LOAD8_S => self.validate_i32_load(&memories, 0)?,
                I32_LOAD16_U | I32_LOAD16_S => self.validate_i32_load(&memories, 1)?,
                I64_LOAD => self.validate_i64_load(&memories, 3)?,
                I64_LOAD8_S | I64_LOAD8_U => self.validate_i64_load(&memories, 0)?,
                I64_LOAD16_U | I64_LOAD16_S => self.validate_i64_load(&memories, 1)?,
                I64_LOAD32_U | I64_LOAD32_S => self.validate_i64_load(&memories, 2)?,
                F32_LOAD => self.validate_f32_load(&memories)?,
                F64_LOAD => self.validate_f64_load(&memories)?,
                REF_IS_NULL => self.validate_ref_is_null()?,
                REF_NULL => self.validate_ref_null()?,
                MEMORY_GROW => self.validate_memory_grow(&memories)?,
                MEMORY_SIZE => self.validate_memory_size(&memories)?,
                REF_FUNC => self.validate_ref_func()?,
                EXTENSION_PREFIX => {
                    let byte = self.reader.next_leb128_u32()?;
                    trace!("Validating Byte: {byte:#04X}");
                    match byte {
                        ELEM_DROP => self.validate_elem_drop()?,
                        TABLE_INIT => self.validate_table_init(&tables)?,
                        TABLE_SIZE => self.validate_table_size(&tables)?,
                        TABLE_COPY => self.validate_table_copy(&tables)?,
                        TABLE_FILL => self.validate_table_fill(&tables)?,
                        TABLE_GROW => self.validate_table_grow(&tables)?,
                        I32_TRUNC_SAT_F32_S | I32_TRUNC_SAT_F32_U => {
                            self.validate_i32_trunc_f32()?
                        }
                        I32_TRUNC_SAT_F64_S | I32_TRUNC_SAT_F64_U => {
                            self.validate_i32_trunc_f64()?;
                        }

                        I64_TRUNC_SAT_F32_S | I64_TRUNC_SAT_F32_U => {
                            self.validate_i64_trunc_f32()?;
                        }
                        I64_TRUNC_SAT_F64_U | I64_TRUNC_SAT_F64_S => {
                            self.validate_i64_trunc_f64()?;
                        }
                        DATA_DROP => self.validate_data_drop()?,
                        MEMORY_INIT => self.validate_memory_init(&memories)?,
                        MEMORY_COPY => self.validate_memory_copy(&memories)?,
                        MEMORY_FILL => self.validate_memory_fill(&memories)?,
                        _ => Err(ValidationError::UndefinedOpCode(byte as u8))?,
                    }
                }

                UNREACHABLE => self.unreachable(),
                _ => Err(ValidationError::UndefinedOpCode(byte))?,
            };
        }
        let result = self.side_table.clone();
        self.reset();
        Ok(result)
    }

    pub fn check_align(&mut self, max_align: u32) -> Result<()> {
        let align = self.reader.next_leb128_u32()?;
        let _offset = self.reader.next_leb128_u32()?;

        if align > 31 {
            Err(ValidationError::MalformedMemOpFlags)?;
        }
        if align > max_align {
            Err(ValidationError::InvalidAlignment)?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.abstract_control_stack.reset();
        self.abstract_value_stack.reset();
        self.side_table.reset();
    }

    pub fn resolve_sidetable_entries(&mut self, sidetable_idxs: &[usize]) {
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

    pub fn create_sidetable_entry(
        &mut self,
        depth: u32,
        index_of_start_bytecode: i32,
    ) -> Result<ControlType> {
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

                let entry = self.side_table.get_entry(index_of_br_sidetable_entry);
                entry.pop = (self.abstract_value_stack.height() - *height_of_value_stack) as usize;
                entry.vals = func_type.result.len();
            }
            ControlType::For {
                func_type,
                side_table_entry,
                instruction_pointer,
                height_of_value_stack,
                ..
            } => {
                let stp = *side_table_entry as i32 - (self.side_table.len() as i32) + 1;
                let ip = *instruction_pointer as i32 - index_of_start_bytecode;
                let entry = self.side_table.get_entry(index_of_br_sidetable_entry);
                entry.pop = (self.abstract_value_stack.height() - *height_of_value_stack) as usize;
                entry.vals = func_type.parameter.len();
                entry.ip = ip;
                entry.stp = stp;
            }
        };

        Ok(target_control.clone())
    }
}
