extern crate alloc;

use super::execution_stack::{CallStack, ExecutionStack};
use super::instances::function_instance::Internal;
use super::instances::{FuncInstance, MemoryInstance};
use super::store::{HostEnv, Store};
use crate::execution::adresses::{FuncAddr, InstanceAddr};

use crate::core::error::ExecutionError;
use crate::core::opcode::*;
use crate::core::validated_wasm_reader::ValidatedWasmReader;
use crate::core::value_types::ValType;
use crate::execution::instances::ModuleInstance;
use crate::execution::store::{
    DataStore, ElementStore, FunctionStore, GlobalStore, MemoryStore, TableStore,
};
use crate::{core::values::FromLeBytes, validation::side_table::SideTable, Value};
use alloc::vec::Vec;
use extensions::*;
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub reader: &'a mut ValidatedWasmReader,
    pub instance: &'a mut ModuleInstance,
    pub current_frame: &'a mut Frame,
    pub tables: &'a mut TableStore,
    pub memories: &'a mut MemoryStore,
    pub globals: &'a mut GlobalStore,
    pub functions: &'a mut FunctionStore,
    pub elements: &'a mut ElementStore,
    pub datas: &'a mut DataStore,
    pub should_break: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub instance_addr: InstanceAddr,
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub start: usize,
    pub end: usize,
    pub return_value: Vec<ValType>,
    pub locals: Vec<Value>,
    pub side_table: SideTable,
    pub side_table_pointer: usize,
}

#[derive(Debug, Default)]
pub struct Executor {
    pub call_stack: CallStack,
    pub stack: ExecutionStack,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            call_stack: CallStack::default(),
            stack: ExecutionStack::default(),
        }
    }
    pub fn build_frame(&mut self, function: &Internal) -> Result<Frame> {
        let stack = &mut self.stack;
        let mut locals = stack.pop_n(function.signature.parameter.len())?;
        locals.append(&mut function.locals.clone()); // Locals = Params + Locals
        Ok(Frame {
            instance_addr: function.instance,
            program_counter: 0,
            stack_pointer: stack.len(),
            start: function.start,
            end: function.end,

            return_value: function.signature.result.clone(),
            locals,
            side_table: function.side_table.clone(),
            side_table_pointer: 0,
        })
    }
    pub fn invoke(
        &mut self,
        func_addr: FuncAddr,
        args: &[Value],
        store: &mut Store,
    ) -> Result<Vec<Value>> {
        self.call_stack.reset();
        self.stack.reset();
        self.stack.push_vec(args)?;
        let mut func_inst = store.functions.get(func_addr)?.clone();
        let mut env: HostEnv = (store).into();

        match func_inst {
            FuncInstance::Internal(instance) => {
                let frame = self.build_frame(&instance)?;
                self.call_stack.push(frame)?;
                self.execute(store)
            }
            FuncInstance::External(ref mut host_function) => {
                let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                let mut func_ref = host_function.func.borrow_mut();
                let results = func_ref(&mut env, args)?;
                self.stack.push_vec(&results)?;
                Ok(results)
            }
        }
    }
    fn execute(&mut self, store: &mut Store) -> Result<Vec<Value>> {
        loop {
            let Some(mut current_frame) = self.call_stack.pop() else {
                return self.stack.pop_n(self.stack.len());
            };
            self.execute_frame(&mut current_frame, store)?;
        }
    }

    #[cfg(not(feature = "tail-call"))]
    fn execute_frame(&mut self, current_frame: &mut Frame, store: &mut Store) -> Result<()> {
        let instance_addr = &current_frame.instance_addr;
        let instance = store.instances.get_mut(*instance_addr)?;
        let reader_address = instance.reader;
        let reader: &mut ValidatedWasmReader = store.reader.get_mut(reader_address).unwrap();
        reader.seek(current_frame.start + current_frame.program_counter);

        let mut context = ExecutionContext {
            reader,
            instance,
            current_frame,
            tables: &mut store.tables,
            memories: &mut store.memories,
            globals: &mut store.globals,
            functions: &mut store.functions,
            elements: &mut store.elements,
            datas: &mut store.datas,
            should_break: false,
        };

        loop {
            let instruction = context.reader.next_byte();
            match instruction {
                UNREACHABLE => {
                    return Err(ExecutionError::TrapUnreachable);
                }

                I32_CONST => self.execute_i32_const(&mut context)?,
                I64_CONST => self.execute_i64_const(&mut context)?,
                F32_CONST => self.execute_f32_const(&mut context)?,
                F64_CONST => self.execute_f64_const(&mut context)?,
                BLOCK => self.execute_block(&mut context)?,
                LOOP => self.execute_loop(&mut context)?,
                IF => self.execute_if(&mut context)?,
                ELSE => self.execute_else(&mut context)?,
                LOCAL_GET => self.execute_local_get(&mut context)?,
                LOCAL_SET => self.execute_local_set(&mut context)?,
                LOCAL_TEE => self.execute_local_tee(&mut context)?,
                I32_POPCNT => self.execute_i32_popcnt()?,
                GLOBAL_GET => self.execute_global_get(&mut context)?,
                GLOBAL_SET => self.execute_global_set(&mut context)?,
                DROP => self.execute_drop()?,
                NOP => self.execute_nop()?,
                I32_CTZ => self.execute_i32_ctz()?,
                I64_POPCNT => self.execute_i64_popcnt()?,
                I64_CTZ => self.execute_i64_ctz()?,
                I64_ADD => self.execute_i64_add()?,
                I64_SUB => self.execute_i64_sub()?,
                I64_MUL => self.execute_i64_mul()?,
                I64_DIV_S => self.execute_i64_div_s()?,
                I64_DIV_U => self.execute_i64_div_u()?,
                I64_REM_S => self.execute_i64_rem_s()?,
                I64_REM_U => self.execute_i64_rem_u()?,
                I64_AND => self.execute_i64_and()?,
                I64_OR => self.execute_i64_or()?,
                I64_XOR => self.execute_i64_xor()?,
                I64_SHR_U => self.execute_i64_shr_u()?,
                I64_SHR_S => self.execute_i64_shr_s()?,
                I64_SHL => self.execute_i64_shl()?,
                I32_SHR_S => self.execute_i32_shr_s()?,
                I32_SHR_U => self.execute_i32_shr_u()?,
                I32_SHL => self.execute_i32_shl()?,
                I32_ADD => self.execute_i32_add()?,
                I32_MUL => self.execute_i32_mul()?,
                I32_DIV_S => self.execute_i32_div_s()?,
                I32_DIV_U => self.execute_i32_div_u()?,
                I32_REM_S => self.execute_i32_rem_s()?,
                I32_REM_U => self.execute_i32_rem_u()?,
                I32_SUB => self.execute_i32_sub()?,
                I32_EQZ => self.execute_i32_eqz()?,
                I32_EQ => self.execute_i32_eq()?,
                I32_CLZ => self.execute_i32_clz()?,
                I32_LT_S => self.execute_i32_lt_s()?,
                I32_LE_S => self.execute_i32_le_s()?,
                I32_LT_U => self.execute_i32_lt_u()?,
                I32_LE_U => self.execute_i32_le_u()?,
                I32_GT_U => self.execute_i32_gt_u()?,
                I32_GE_U => self.execute_i32_ge_u()?,
                I32_XOR => self.execute_i32_xor()?,
                I32_NE => self.execute_i32_ne()?,
                I32_ROTR => self.execute_i32_rotr()?,
                I32_ROTL => self.execute_i32_rotl()?,
                I64_ROTR => self.execute_i64_rotr()?,
                I64_ROTL => self.execute_i64_rotl()?,
                I64_CLZ => self.execute_i64_clz()?,
                I32_WRAP_I64 => self.execute_i32_wrap_i64()?,
                I32_EXTEND8_S => self.execute_i32_extend8_s()?,
                I32_EXTEND16_S => self.execute_i32_extend16_s()?,
                I64_EXTEND8_S => self.execute_i64_extend8_s()?,
                I64_EXTEND16_S => self.execute_i64_extend16_s()?,
                I64_EXTEND32_S => self.execute_i64_extend32_s()?,
                I64_EXTEND_I32_U => self.execute_i64_extend_i32_u()?,
                I64_EXTEND_I32_S => self.execute_i64_extend_i32_s()?,
                I64_GE_U => self.execute_i64_ge_u()?,
                I64_GT_S => self.execute_i64_gt_s()?,
                I64_GT_U => self.execute_i64_gt_u()?,
                I64_LT_S => self.execute_i64_lt_s()?,
                I64_LT_U => self.execute_i64_lt_u()?,
                I64_GE_S => self.execute_i64_ge_s()?,
                I64_LE_S => self.execute_i64_le_s()?,
                I64_LE_U => self.execute_i64_le_u()?,
                I64_EQZ => self.execute_i64_eqz()?,
                I64_EQ => self.execute_i64_eq()?,
                I64_NE => self.execute_i64_ne()?,
                I32_GT_S => self.execute_i32_gt_s()?,
                I32_GE_S => self.execute_i32_ge_s()?,
                I32_AND => self.execute_i32_and()?,
                I32_OR => self.execute_i32_or()?,
                SELECT => self.execute_select()?,
                BR => self.execute_br(&mut context)?,
                BR_IF => self.execute_br_if(&mut context)?,
                BR_TABLE => self.execute_br_table(&mut context)?,
                SELECT_T => self.execute_select_t(&mut context)?,
                RETURN => {
                    self.execute_return(&mut context)?;
                    break;
                }
                END => {
                    trace!("END");
                    if context.reader.current_index() == context.current_frame.end {
                        break;
                    }
                }
                CALL_INDIRECT => {
                    self.execute_call_indirect(&mut context)?;
                    if context.should_break {
                        break;
                    };
                }
                CALL => {
                    self.execute_call(&mut context)?;
                    if context.should_break {
                        break;
                    };
                }
                I64_STORE => self.execute_i64_store(&mut context)?,
                F32_STORE => self.execute_f32_store(&mut context)?,
                F64_STORE => self.execute_f64_store(&mut context)?,
                I64_STORE8 => self.execute_i64_store8(&mut context)?,
                I64_STORE16 => self.execute_i64_store16(&mut context)?,
                I64_STORE32 => self.execute_i64_store32(&mut context)?,
                I64_LOAD => self.execute_i64_load(&mut context)?,
                F64_LOAD => self.execute_f64_load(&mut context)?,
                I32_STORE => self.execute_i32_store(&mut context)?,
                I32_STORE8 => self.execute_i32_store8(&mut context)?,
                I32_STORE16 => self.execute_i32_store16(&mut context)?,
                I32_LOAD => self.execute_i32_load(&mut context)?,
                F32_LOAD => self.execute_f32_load(&mut context)?,
                I64_LOAD8_S => self.execute_i64_load8_s(&mut context)?,
                I64_LOAD8_U => self.execute_i64_load8_u(&mut context)?,
                I64_LOAD16_S => self.execute_i64_load16_s(&mut context)?,
                I64_LOAD16_U => self.execute_i64_load16_u(&mut context)?,
                I64_LOAD32_S => self.execute_i64_load32_s(&mut context)?,
                I64_LOAD32_U => self.execute_i64_load32_u(&mut context)?,
                I32_LOAD8_U => self.execute_i32_load8_u(&mut context)?,
                I32_LOAD16_U => self.execute_i32_load16_u(&mut context)?,
                I32_LOAD16_S => self.execute_i32_load16_s(&mut context)?,
                I32_LOAD8_S => self.execute_i32_load8_s(&mut context)?,
                TABLE_GET => self.execute_table_get(&mut context)?,
                TABLE_SET => self.execute_table_set(&mut context)?,
                I32_TRUNC_F32_S => self.execute_i32_trunc_f32_s()?,
                I32_TRUNC_F32_U => self.execute_i32_trunc_f32_u()?,
                I32_TRUNC_F64_S => self.execute_i32_trunc_f64_s()?,
                I32_TRUNC_F64_U => self.execute_i32_trunc_f64_u()?,
                I64_TRUNC_F32_S => self.execute_i64_trunc_f32_s()?,
                I64_TRUNC_F32_U => self.execute_i64_trunc_f32_u()?,
                I64_TRUNC_F64_S => self.execute_i64_trunc_f64_s()?,
                I64_TRUNC_F64_U => self.execute_i64_trunc_f64_u()?,
                I32_REINTERPRET_F32 => self.execute_i32_reinterpret_f32()?,
                I64_REINTERPRET_F64 => self.execute_i64_reinterpret_f64()?,
                F64_REINTERPRET_I64 => self.execute_f64_reinterpret_i64()?,
                F32_REINTERPRET_I32 => self.execute_f32_reinterpret_i32()?,
                F32_DEMOTE_F64 => self.execute_f32_demote_f64()?,
                F64_COPYSIGN => self.execute_f64_copysign()?,
                F32_COPYSIGN => self.execute_f32_copysign()?,
                F32_CONVERT_I32_S => self.execute_f32_convert_i32_s()?,
                F32_CONVERT_I32_U => self.execute_f32_convert_i32_u()?,
                F32_CONVERT_I64_S => self.execute_f32_convert_i64_s()?,
                F32_CONVERT_I64_U => self.execute_f32_convert_i64_u()?,
                F64_CONVERT_I32_S => self.execute_f64_convert_i32_s()?,
                F64_CONVERT_I32_U => self.execute_f64_convert_i32_u()?,
                F64_CONVERT_I64_S => self.execute_f64_convert_i64_s()?,
                F64_CONVERT_I64_U => self.execute_f64_convert_i64_u()?,
                F64_PROMOTE_F32 => self.execute_f64_promote_f32()?,
                F64_ABS => self.execute_f64_abs()?,
                F64_NEG => self.execute_f64_neg()?,
                F64_CEIL => self.execute_f64_ceil()?,
                F64_FLOOR => self.execute_f64_floor()?,
                F64_TRUNC => self.execute_f64_trunc()?,
                F64_NEAREST => self.execute_f64_nearest()?,
                F64_SQRT => self.execute_f64_sqrt()?,
                F64_ADD => self.execute_f64_add()?,
                F64_SUB => self.execute_f64_sub()?,
                F64_MUL => self.execute_f64_mul()?,
                F64_DIV => self.execute_f64_div()?,
                F64_MIN => self.execute_f64_min()?,
                F64_MAX => self.execute_f64_max()?,
                F32_SUB => self.execute_f32_sub()?,
                F32_MUL => self.execute_f32_mul()?,
                F32_DIV => self.execute_f32_div()?,
                F32_MIN => self.execute_f32_min()?,
                F32_MAX => self.execute_f32_max()?,
                F32_ABS => self.execute_f32_abs()?,
                F32_NEG => self.execute_f32_neg()?,
                F32_CEIL => self.execute_f32_ceil()?,
                F32_FLOOR => self.execute_f32_floor()?,
                F32_TRUNC => self.execute_f32_trunc()?,
                F32_NEAREST => self.execute_f32_nearest()?,
                F32_SQRT => self.execute_f32_sqrt()?,
                F32_ADD => self.execute_f32_add()?,
                F64_EQ => self.execute_f64_eq()?,
                F64_NE => self.execute_f64_ne()?,
                F64_LT => self.execute_f64_lt()?,
                F64_GT => self.execute_f64_gt()?,
                F64_LE => self.execute_f64_le()?,
                F64_GE => self.execute_f64_ge()?,
                F32_EQ => self.execute_f32_eq()?,
                F32_NE => self.execute_f32_ne()?,
                F32_LT => self.execute_f32_lt()?,
                F32_GT => self.execute_f32_gt()?,
                F32_LE => self.execute_f32_le()?,
                F32_GE => self.execute_f32_ge()?,
                REF_NULL => self.execute_ref_null(&mut context)?,
                REF_IS_NULL => self.execute_ref_is_null()?,
                REF_FUNC => self.execute_ref_func(&mut context)?,
                MEMORY_SIZE => self.execute_memory_size(&mut context)?,
                MEMORY_GROW => self.execute_memory_grow(&mut context)?,
                EXTENSION_PREFIX => {
                    let byte = context.reader.next_leb128_u32();
                    match byte {
                        ELEM_DROP => self.execute_elem_drop(&mut context)?,
                        I32_TRUNC_SAT_F32_S => self.execute_i32_trunc_sat_f32_s()?,
                        I32_TRUNC_SAT_F32_U => self.execute_i32_trunc_sat_f32_u()?,
                        I32_TRUNC_SAT_F64_S => self.execute_i32_trunc_sat_f64_s()?,
                        I32_TRUNC_SAT_F64_U => self.execute_i32_trunc_sat_f64_u()?,
                        I64_TRUNC_SAT_F64_S => self.execute_i64_trunc_sat_f64_s()?,
                        I64_TRUNC_SAT_F32_S => self.execute_i64_trunc_sat_f32_s()?,
                        I64_TRUNC_SAT_F64_U => self.execute_i64_trunc_sat_f64_u()?,
                        I64_TRUNC_SAT_F32_U => self.execute_i64_trunc_sat_f32_u()?,
                        DATA_DROP => self.execute_data_drop(&mut context)?,
                        TABLE_INIT => self.execute_table_init(&mut context)?,
                        TABLE_SIZE => self.execute_table_size(&mut context)?,
                        TABLE_COPY => self.execute_table_copy(&mut context)?,
                        TABLE_FILL => self.execute_table_fill(&mut context)?,
                        TABLE_GROW => self.execute_table_grow(&mut context)?,
                        MEMORY_INIT => self.execute_memory_init(&mut context)?,
                        MEMORY_COPY => self.execute_memory_copy(&mut context)?,
                        MEMORY_FILL => self.execute_memory_fill(&mut context)?,
                        _ => unreachable!("Unreachable due to validation"),
                    }
                }
                _ => unreachable!("Unreachable due to validation"),
            };
        }
        current_frame.program_counter = context.reader.current_index() - current_frame.start + 1;
        Ok(())
    }

    #[cfg(feature = "tail-call")]
    fn execute_frame(&mut self, current_frame: &mut Frame, store: &mut Store) -> Result<()> {
        let instance_addr = &current_frame.instance_addr;
        let instance = store.instances.get_mut(*instance_addr)?;
        let reader_address = instance.reader;
        let reader: &mut ValidatedWasmReader = store.reader.get_mut(reader_address).unwrap();
        reader.seek(current_frame.start + current_frame.program_counter);

        let mut context = ExecutionContext {
            reader,
            instance,
            current_frame,
            tables: &mut store.tables,
            memories: &mut store.memories,
            globals: &mut store.globals,
            functions: &mut store.functions,
            elements: &mut store.elements,
            datas: &mut store.datas,
            should_break: false,
        };

        self.execute_next_instruction(&mut context)?;

        current_frame.program_counter = context.reader.current_index() - current_frame.start + 1;
        Ok(())
    }

    #[cfg(feature = "tail-call")]
    pub fn execute_next_instruction(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let instruction = context.reader.next_byte();

        match instruction {
            UNREACHABLE => {
                return Err(ExecutionError::TrapUnreachable);
            }

            I32_CONST => become self.execute_i32_const(context),
            I64_CONST => become self.execute_i64_const(context),
            F32_CONST => become self.execute_f32_const(context),
            F64_CONST => become self.execute_f64_const(context),
            BLOCK => become self.execute_block(context),
            LOOP => become self.execute_loop(context),
            IF => become self.execute_if(context),
            ELSE => become self.execute_else(context),
            LOCAL_GET => become self.execute_local_get(context),
            LOCAL_SET => become self.execute_local_set(context),
            LOCAL_TEE => become self.execute_local_tee(context),
            I32_POPCNT => become self.execute_i32_popcnt(context),
            GLOBAL_GET => become self.execute_global_get(context),
            GLOBAL_SET => become self.execute_global_set(context),
            DROP => become self.execute_drop(context),
            NOP => become self.execute_nop(context),
            I32_CTZ => become self.execute_i32_ctz(context),
            I64_POPCNT => become self.execute_i64_popcnt(context),
            I64_CTZ => become self.execute_i64_ctz(context),
            I64_ADD => become self.execute_i64_add(context),
            I64_SUB => become self.execute_i64_sub(context),
            I64_MUL => become self.execute_i64_mul(context),
            I64_DIV_S => become self.execute_i64_div_s(context),
            I64_DIV_U => become self.execute_i64_div_u(context),
            I64_REM_S => become self.execute_i64_rem_s(context),
            I64_REM_U => become self.execute_i64_rem_u(context),
            I64_AND => become self.execute_i64_and(context),
            I64_OR => become self.execute_i64_or(context),
            I64_XOR => become self.execute_i64_xor(context),
            I64_SHR_U => become self.execute_i64_shr_u(context),
            I64_SHR_S => become self.execute_i64_shr_s(context),
            I64_SHL => become self.execute_i64_shl(context),
            I32_SHR_S => become self.execute_i32_shr_s(context),
            I32_SHR_U => become self.execute_i32_shr_u(context),
            I32_SHL => become self.execute_i32_shl(context),
            I32_ADD => become self.execute_i32_add(context),
            I32_MUL => become self.execute_i32_mul(context),
            I32_DIV_S => become self.execute_i32_div_s(context),
            I32_DIV_U => become self.execute_i32_div_u(context),
            I32_REM_S => become self.execute_i32_rem_s(context),
            I32_REM_U => become self.execute_i32_rem_u(context),
            I32_SUB => become self.execute_i32_sub(context),
            I32_EQZ => become self.execute_i32_eqz(context),
            I32_EQ => become self.execute_i32_eq(context),
            I32_CLZ => become self.execute_i32_clz(context),
            I32_LT_S => become self.execute_i32_lt_s(context),
            I32_LE_S => become self.execute_i32_le_s(context),
            I32_LT_U => become self.execute_i32_lt_u(context),
            I32_LE_U => become self.execute_i32_le_u(context),
            I32_GT_U => become self.execute_i32_gt_u(context),
            I32_GE_U => become self.execute_i32_ge_u(context),
            I32_XOR => become self.execute_i32_xor(context),
            I32_NE => become self.execute_i32_ne(context),
            I32_ROTR => become self.execute_i32_rotr(context),
            I32_ROTL => become self.execute_i32_rotl(context),
            I64_ROTR => become self.execute_i64_rotr(context),
            I64_ROTL => become self.execute_i64_rotl(context),
            I64_CLZ => become self.execute_i64_clz(context),
            I32_WRAP_I64 => become self.execute_i32_wrap_i64(context),
            I32_EXTEND8_S => become self.execute_i32_extend8_s(context),
            I32_EXTEND16_S => become self.execute_i32_extend16_s(context),
            I64_EXTEND8_S => become self.execute_i64_extend8_s(context),
            I64_EXTEND16_S => become self.execute_i64_extend16_s(context),
            I64_EXTEND32_S => become self.execute_i64_extend32_s(context),
            I64_EXTEND_I32_U => become self.execute_i64_extend_i32_u(context),
            I64_EXTEND_I32_S => become self.execute_i64_extend_i32_s(context),
            I64_GE_U => become self.execute_i64_ge_u(context),
            I64_GT_S => become self.execute_i64_gt_s(context),
            I64_GT_U => become self.execute_i64_gt_u(context),
            I64_LT_S => become self.execute_i64_lt_s(context),
            I64_LT_U => become self.execute_i64_lt_u(context),
            I64_GE_S => become self.execute_i64_ge_s(context),
            I64_LE_S => become self.execute_i64_le_s(context),
            I64_LE_U => become self.execute_i64_le_u(context),
            I64_EQZ => become self.execute_i64_eqz(context),
            I64_EQ => become self.execute_i64_eq(context),
            I64_NE => become self.execute_i64_ne(context),
            I32_GT_S => become self.execute_i32_gt_s(context),
            I32_GE_S => become self.execute_i32_ge_s(context),
            I32_AND => become self.execute_i32_and(context),
            I32_OR => become self.execute_i32_or(context),
            SELECT => become self.execute_select(context),
            BR => become self.execute_br(context),
            BR_IF => become self.execute_br_if(context),
            BR_TABLE => become self.execute_br_table(context),
            SELECT_T => become self.execute_select_t(context),
            RETURN => {
                self.execute_return(context)?;
                return Ok(());
            }
            END => {
                trace!("END");
                if context.reader.current_index() == context.current_frame.end {
                    return Ok(());
                }
                become self.execute_next_instruction(context);
            }
            CALL_INDIRECT => become self.execute_call_indirect(context),
            CALL => become self.execute_call(context),
            I64_STORE => become self.execute_i64_store(context),
            F32_STORE => become self.execute_f32_store(context),
            F64_STORE => become self.execute_f64_store(context),
            I64_STORE8 => become self.execute_i64_store8(context),
            I64_STORE16 => become self.execute_i64_store16(context),
            I64_STORE32 => become self.execute_i64_store32(context),
            I64_LOAD => become self.execute_i64_load(context),
            F64_LOAD => become self.execute_f64_load(context),
            I32_STORE => become self.execute_i32_store(context),
            I32_STORE8 => become self.execute_i32_store8(context),
            I32_STORE16 => become self.execute_i32_store16(context),
            I32_LOAD => become self.execute_i32_load(context),
            F32_LOAD => become self.execute_f32_load(context),
            I64_LOAD8_S => become self.execute_i64_load8_s(context),
            I64_LOAD8_U => become self.execute_i64_load8_u(context),
            I64_LOAD16_S => become self.execute_i64_load16_s(context),
            I64_LOAD16_U => become self.execute_i64_load16_u(context),
            I64_LOAD32_S => become self.execute_i64_load32_s(context),
            I64_LOAD32_U => become self.execute_i64_load32_u(context),
            I32_LOAD8_U => become self.execute_i32_load8_u(context),
            I32_LOAD16_U => become self.execute_i32_load16_u(context),
            I32_LOAD16_S => become self.execute_i32_load16_s(context),
            I32_LOAD8_S => become self.execute_i32_load8_s(context),
            TABLE_GET => become self.execute_table_get(context),
            TABLE_SET => become self.execute_table_set(context),
            I32_TRUNC_F32_S => become self.execute_i32_trunc_f32_s(context),
            I32_TRUNC_F32_U => become self.execute_i32_trunc_f32_u(context),
            I32_TRUNC_F64_S => become self.execute_i32_trunc_f64_s(context),
            I32_TRUNC_F64_U => become self.execute_i32_trunc_f64_u(context),
            I64_TRUNC_F32_S => become self.execute_i64_trunc_f32_s(context),
            I64_TRUNC_F32_U => become self.execute_i64_trunc_f32_u(context),
            I64_TRUNC_F64_S => become self.execute_i64_trunc_f64_s(context),
            I64_TRUNC_F64_U => become self.execute_i64_trunc_f64_u(context),
            I32_REINTERPRET_F32 => become self.execute_i32_reinterpret_f32(context),
            I64_REINTERPRET_F64 => become self.execute_i64_reinterpret_f64(context),
            F64_REINTERPRET_I64 => become self.execute_f64_reinterpret_i64(context),
            F32_REINTERPRET_I32 => become self.execute_f32_reinterpret_i32(context),
            F32_DEMOTE_F64 => become self.execute_f32_demote_f64(context),
            F64_COPYSIGN => become self.execute_f64_copysign(context),
            F32_COPYSIGN => become self.execute_f32_copysign(context),
            F32_CONVERT_I32_S => become self.execute_f32_convert_i32_s(context),
            F32_CONVERT_I32_U => become self.execute_f32_convert_i32_u(context),
            F32_CONVERT_I64_S => become self.execute_f32_convert_i64_s(context),
            F32_CONVERT_I64_U => become self.execute_f32_convert_i64_u(context),
            F64_CONVERT_I32_S => become self.execute_f64_convert_i32_s(context),
            F64_CONVERT_I32_U => become self.execute_f64_convert_i32_u(context),
            F64_CONVERT_I64_S => become self.execute_f64_convert_i64_s(context),
            F64_CONVERT_I64_U => become self.execute_f64_convert_i64_u(context),
            F64_PROMOTE_F32 => become self.execute_f64_promote_f32(context),
            F64_ABS => become self.execute_f64_abs(context),
            F64_NEG => become self.execute_f64_neg(context),
            F64_CEIL => become self.execute_f64_ceil(context),
            F64_FLOOR => become self.execute_f64_floor(context),
            F64_TRUNC => become self.execute_f64_trunc(context),
            F64_NEAREST => become self.execute_f64_nearest(context),
            F64_SQRT => become self.execute_f64_sqrt(context),
            F64_ADD => become self.execute_f64_add(context),
            F64_SUB => become self.execute_f64_sub(context),
            F64_MUL => become self.execute_f64_mul(context),
            F64_DIV => become self.execute_f64_div(context),
            F64_MIN => become self.execute_f64_min(context),
            F64_MAX => become self.execute_f64_max(context),
            F32_SUB => become self.execute_f32_sub(context),
            F32_MUL => become self.execute_f32_mul(context),
            F32_DIV => become self.execute_f32_div(context),
            F32_MIN => become self.execute_f32_min(context),
            F32_MAX => become self.execute_f32_max(context),
            F32_ABS => become self.execute_f32_abs(context),
            F32_NEG => become self.execute_f32_neg(context),
            F32_CEIL => become self.execute_f32_ceil(context),
            F32_FLOOR => become self.execute_f32_floor(context),
            F32_TRUNC => become self.execute_f32_trunc(context),
            F32_NEAREST => become self.execute_f32_nearest(context),
            F32_SQRT => become self.execute_f32_sqrt(context),
            F32_ADD => become self.execute_f32_add(context),
            F64_EQ => become self.execute_f64_eq(context),
            F64_NE => become self.execute_f64_ne(context),
            F64_LT => become self.execute_f64_lt(context),
            F64_GT => become self.execute_f64_gt(context),
            F64_LE => become self.execute_f64_le(context),
            F64_GE => become self.execute_f64_ge(context),
            F32_EQ => become self.execute_f32_eq(context),
            F32_NE => become self.execute_f32_ne(context),
            F32_LT => become self.execute_f32_lt(context),
            F32_GT => become self.execute_f32_gt(context),
            F32_LE => become self.execute_f32_le(context),
            F32_GE => become self.execute_f32_ge(context),
            REF_NULL => become self.execute_ref_null(context),
            REF_IS_NULL => become self.execute_ref_is_null(context),
            REF_FUNC => become self.execute_ref_func(context),
            MEMORY_SIZE => become self.execute_memory_size(context),
            MEMORY_GROW => become self.execute_memory_grow(context),
            EXTENSION_PREFIX => {
                let byte = context.reader.next_leb128_u32();
                match byte {
                    ELEM_DROP => become self.execute_elem_drop(context),
                    I32_TRUNC_SAT_F32_S => become self.execute_i32_trunc_sat_f32_s(context),
                    I32_TRUNC_SAT_F32_U => become self.execute_i32_trunc_sat_f32_u(context),
                    I32_TRUNC_SAT_F64_S => become self.execute_i32_trunc_sat_f64_s(context),
                    I32_TRUNC_SAT_F64_U => become self.execute_i32_trunc_sat_f64_u(context),
                    I64_TRUNC_SAT_F64_S => become self.execute_i64_trunc_sat_f64_s(context),
                    I64_TRUNC_SAT_F32_S => become self.execute_i64_trunc_sat_f32_s(context),
                    I64_TRUNC_SAT_F64_U => become self.execute_i64_trunc_sat_f64_u(context),
                    I64_TRUNC_SAT_F32_U => become self.execute_i64_trunc_sat_f32_u(context),
                    DATA_DROP => become self.execute_data_drop(context),
                    TABLE_INIT => become self.execute_table_init(context),
                    TABLE_SIZE => become self.execute_table_size(context),
                    TABLE_COPY => become self.execute_table_copy(context),
                    TABLE_FILL => become self.execute_table_fill(context),
                    TABLE_GROW => become self.execute_table_grow(context),
                    MEMORY_INIT => become self.execute_memory_init(context),
                    MEMORY_COPY => become self.execute_memory_copy(context),
                    MEMORY_FILL => become self.execute_memory_fill(context),
                    _ => unreachable!("Unreachable due to validation"),
                }
            }
            _ => unreachable!("Unreachable due to validation"),
        };
    }

    pub fn exit_tail_calls(&mut self, _context: &mut ExecutionContext) -> Result<()> {
        Ok(())
    }

    pub fn do_goto(
        stack: &mut ExecutionStack,
        current_frame: &mut Frame,
        reader: &mut ValidatedWasmReader,
    ) -> Result<()> {
        let side_table_entry = current_frame
            .side_table
            .entries
            .get(current_frame.side_table_pointer)
            .ok_or(ExecutionError::InvalidSidetableIndex)?;
        let pos = reader.current_index() as i32 + side_table_entry.ip;
        current_frame.side_table_pointer = current_frame
            .side_table_pointer
            .saturating_add_signed(side_table_entry.stp as isize);
        reader.seek(pos as usize);
        let saved_vals = stack.pop_n(side_table_entry.vals)?;
        stack.pop_n(side_table_entry.pop)?;
        stack.push_vec(&saved_vals)?;
        Ok(())
    }

    pub fn load<T>(
        memory: &mut MemoryInstance,
        offset: u32,
        address: u32,
        bytes: usize,
    ) -> Result<T>
    where
        T: FromLeBytes,
    {
        let address = offset
            .checked_add(address)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)? as usize;
        memory.load(address, bytes)
    }
}
