use crate::core::opcode::*;
use crate::{
    core::error::ExecutionError,
    execution::executor::{ExecutionContext, Executor},
};
use extensions::*;

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_next_instruction(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let instruction = context.reader.next_byte();

        match instruction {
            UNREACHABLE => {
                return Err(ExecutionError::TrapUnreachable);
            }

            I32_CONST => become Executor::execute_i32_const(self, context),
            I64_CONST => become Executor::execute_i64_const(self, context),
            F32_CONST => become Executor::execute_f32_const(self, context),
            F64_CONST => become Executor::execute_f64_const(self, context),
            BLOCK => become Executor::execute_block(self, context),
            LOOP => become Executor::execute_loop(self, context),
            IF => become Executor::execute_if(self, context),
            ELSE => become Executor::execute_else(self, context),
            LOCAL_GET => become Executor::execute_local_get(self, context),
            LOCAL_SET => become Executor::execute_local_set(self, context),
            LOCAL_TEE => become Executor::execute_local_tee(self, context),
            I32_POPCNT => become Executor::execute_i32_popcnt(self, context),
            GLOBAL_GET => become Executor::execute_global_get(self, context),
            GLOBAL_SET => become Executor::execute_global_set(self, context),
            DROP => become Executor::execute_drop(self, context),
            NOP => become Executor::execute_nop(self, context),
            I32_CTZ => become Executor::execute_i32_ctz(self, context),
            I64_POPCNT => become Executor::execute_i64_popcnt(self, context),
            I64_CTZ => become Executor::execute_i64_ctz(self, context),
            I64_ADD => become Executor::execute_i64_add(self, context),
            I64_SUB => become Executor::execute_i64_sub(self, context),
            I64_MUL => become Executor::execute_i64_mul(self, context),
            I64_DIV_S => become Executor::execute_i64_div_s(self, context),
            I64_DIV_U => become Executor::execute_i64_div_u(self, context),
            I64_REM_S => become Executor::execute_i64_rem_s(self, context),
            I64_REM_U => become Executor::execute_i64_rem_u(self, context),
            I64_AND => become Executor::execute_i64_and(self, context),
            I64_OR => become Executor::execute_i64_or(self, context),
            I64_XOR => become Executor::execute_i64_xor(self, context),
            I64_SHR_U => become Executor::execute_i64_shr_u(self, context),
            I64_SHR_S => become Executor::execute_i64_shr_s(self, context),
            I64_SHL => become Executor::execute_i64_shl(self, context),
            I32_SHR_S => become Executor::execute_i32_shr_s(self, context),
            I32_SHR_U => become Executor::execute_i32_shr_u(self, context),
            I32_SHL => become Executor::execute_i32_shl(self, context),
            I32_ADD => become Executor::execute_i32_add(self, context),
            I32_MUL => become Executor::execute_i32_mul(self, context),
            I32_DIV_S => become Executor::execute_i32_div_s(self, context),
            I32_DIV_U => become Executor::execute_i32_div_u(self, context),
            I32_REM_S => become Executor::execute_i32_rem_s(self, context),
            I32_REM_U => become Executor::execute_i32_rem_u(self, context),
            I32_SUB => become Executor::execute_i32_sub(self, context),
            I32_EQZ => become Executor::execute_i32_eqz(self, context),
            I32_EQ => become Executor::execute_i32_eq(self, context),
            I32_CLZ => become Executor::execute_i32_clz(self, context),
            I32_LT_S => become Executor::execute_i32_lt_s(self, context),
            I32_LE_S => become Executor::execute_i32_le_s(self, context),
            I32_LT_U => become Executor::execute_i32_lt_u(self, context),
            I32_LE_U => become Executor::execute_i32_le_u(self, context),
            I32_GT_U => become Executor::execute_i32_gt_u(self, context),
            I32_GE_U => become Executor::execute_i32_ge_u(self, context),
            I32_XOR => become Executor::execute_i32_xor(self, context),
            I32_NE => become Executor::execute_i32_ne(self, context),
            I32_ROTR => become Executor::execute_i32_rotr(self, context),
            I32_ROTL => become Executor::execute_i32_rotl(self, context),
            I64_ROTR => become Executor::execute_i64_rotr(self, context),
            I64_ROTL => become Executor::execute_i64_rotl(self, context),
            I64_CLZ => become Executor::execute_i64_clz(self, context),
            I32_WRAP_I64 => become Executor::execute_i32_wrap_i64(self, context),
            I32_EXTEND8_S => become Executor::execute_i32_extend8_s(self, context),
            I32_EXTEND16_S => become Executor::execute_i32_extend16_s(self, context),
            I64_EXTEND8_S => become Executor::execute_i64_extend8_s(self, context),
            I64_EXTEND16_S => become Executor::execute_i64_extend16_s(self, context),
            I64_EXTEND32_S => become Executor::execute_i64_extend32_s(self, context),
            I64_EXTEND_I32_U => become Executor::execute_i64_extend_i32_u(self, context),
            I64_EXTEND_I32_S => become Executor::execute_i64_extend_i32_s(self, context),
            I64_GE_U => become Executor::execute_i64_ge_u(self, context),
            I64_GT_S => become Executor::execute_i64_gt_s(self, context),
            I64_GT_U => become Executor::execute_i64_gt_u(self, context),
            I64_LT_S => become Executor::execute_i64_lt_s(self, context),
            I64_LT_U => become Executor::execute_i64_lt_u(self, context),
            I64_GE_S => become Executor::execute_i64_ge_s(self, context),
            I64_LE_S => become Executor::execute_i64_le_s(self, context),
            I64_LE_U => become Executor::execute_i64_le_u(self, context),
            I64_EQZ => become Executor::execute_i64_eqz(self, context),
            I64_EQ => become Executor::execute_i64_eq(self, context),
            I64_NE => become Executor::execute_i64_ne(self, context),
            I32_GT_S => become Executor::execute_i32_gt_s(self, context),
            I32_GE_S => become Executor::execute_i32_ge_s(self, context),
            I32_AND => become Executor::execute_i32_and(self, context),
            I32_OR => become Executor::execute_i32_or(self, context),
            SELECT => become Executor::execute_select(self, context),
            BR => become Executor::execute_br(self, context),
            BR_IF => become Executor::execute_br_if(self, context),
            BR_TABLE => become Executor::execute_br_table(self, context),
            SELECT_T => become Executor::execute_select_t(self, context),
            RETURN => {
                Executor::execute_return(self, context)?;
                return Ok(());
            }
            END => {
                trace!("END");
                if context.reader.current_index() == context.current_frame.end {
                    return Ok(());
                }
                become Executor::execute_next_instruction(self, context);
            }
            CALL_INDIRECT => become Executor::execute_call_indirect(self, context),
            CALL => become Executor::execute_call(self, context),
            I64_STORE => become Executor::execute_i64_store(self, context),
            F32_STORE => become Executor::execute_f32_store(self, context),
            F64_STORE => become Executor::execute_f64_store(self, context),
            I64_STORE8 => become Executor::execute_i64_store8(self, context),
            I64_STORE16 => become Executor::execute_i64_store16(self, context),
            I64_STORE32 => become Executor::execute_i64_store32(self, context),
            I64_LOAD => become Executor::execute_i64_load(self, context),
            F64_LOAD => become Executor::execute_f64_load(self, context),
            I32_STORE => become Executor::execute_i32_store(self, context),
            I32_STORE8 => become Executor::execute_i32_store8(self, context),
            I32_STORE16 => become Executor::execute_i32_store16(self, context),
            I32_LOAD => become Executor::execute_i32_load(self, context),
            F32_LOAD => become Executor::execute_f32_load(self, context),
            I64_LOAD8_S => become Executor::execute_i64_load8_s(self, context),
            I64_LOAD8_U => become Executor::execute_i64_load8_u(self, context),
            I64_LOAD16_S => become Executor::execute_i64_load16_s(self, context),
            I64_LOAD16_U => become Executor::execute_i64_load16_u(self, context),
            I64_LOAD32_S => become Executor::execute_i64_load32_s(self, context),
            I64_LOAD32_U => become Executor::execute_i64_load32_u(self, context),
            I32_LOAD8_U => become Executor::execute_i32_load8_u(self, context),
            I32_LOAD16_U => become Executor::execute_i32_load16_u(self, context),
            I32_LOAD16_S => become Executor::execute_i32_load16_s(self, context),
            I32_LOAD8_S => become Executor::execute_i32_load8_s(self, context),
            TABLE_GET => become Executor::execute_table_get(self, context),
            TABLE_SET => become Executor::execute_table_set(self, context),
            I32_TRUNC_F32_S => become Executor::execute_i32_trunc_f32_s(self, context),
            I32_TRUNC_F32_U => become Executor::execute_i32_trunc_f32_u(self, context),
            I32_TRUNC_F64_S => become Executor::execute_i32_trunc_f64_s(self, context),
            I32_TRUNC_F64_U => become Executor::execute_i32_trunc_f64_u(self, context),
            I64_TRUNC_F32_S => become Executor::execute_i64_trunc_f32_s(self, context),
            I64_TRUNC_F32_U => become Executor::execute_i64_trunc_f32_u(self, context),
            I64_TRUNC_F64_S => become Executor::execute_i64_trunc_f64_s(self, context),
            I64_TRUNC_F64_U => become Executor::execute_i64_trunc_f64_u(self, context),
            I32_REINTERPRET_F32 => become Executor::execute_i32_reinterpret_f32(self, context),
            I64_REINTERPRET_F64 => become Executor::execute_i64_reinterpret_f64(self, context),
            F64_REINTERPRET_I64 => become Executor::execute_f64_reinterpret_i64(self, context),
            F32_REINTERPRET_I32 => become Executor::execute_f32_reinterpret_i32(self, context),
            F32_DEMOTE_F64 => become Executor::execute_f32_demote_f64(self, context),
            F64_COPYSIGN => become Executor::execute_f64_copysign(self, context),
            F32_COPYSIGN => become Executor::execute_f32_copysign(self, context),
            F32_CONVERT_I32_S => become Executor::execute_f32_convert_i32_s(self, context),
            F32_CONVERT_I32_U => become Executor::execute_f32_convert_i32_u(self, context),
            F32_CONVERT_I64_S => become Executor::execute_f32_convert_i64_s(self, context),
            F32_CONVERT_I64_U => become Executor::execute_f32_convert_i64_u(self, context),
            F64_CONVERT_I32_S => become Executor::execute_f64_convert_i32_s(self, context),
            F64_CONVERT_I32_U => become Executor::execute_f64_convert_i32_u(self, context),
            F64_CONVERT_I64_S => become Executor::execute_f64_convert_i64_s(self, context),
            F64_CONVERT_I64_U => become Executor::execute_f64_convert_i64_u(self, context),
            F64_PROMOTE_F32 => become Executor::execute_f64_promote_f32(self, context),
            F64_ABS => become Executor::execute_f64_abs(self, context),
            F64_NEG => become Executor::execute_f64_neg(self, context),
            F64_CEIL => become Executor::execute_f64_ceil(self, context),
            F64_FLOOR => become Executor::execute_f64_floor(self, context),
            F64_TRUNC => become Executor::execute_f64_trunc(self, context),
            F64_NEAREST => become Executor::execute_f64_nearest(self, context),
            F64_SQRT => become Executor::execute_f64_sqrt(self, context),
            F64_ADD => become Executor::execute_f64_add(self, context),
            F64_SUB => become Executor::execute_f64_sub(self, context),
            F64_MUL => become Executor::execute_f64_mul(self, context),
            F64_DIV => become Executor::execute_f64_div(self, context),
            F64_MIN => become Executor::execute_f64_min(self, context),
            F64_MAX => become Executor::execute_f64_max(self, context),
            F32_SUB => become Executor::execute_f32_sub(self, context),
            F32_MUL => become Executor::execute_f32_mul(self, context),
            F32_DIV => become Executor::execute_f32_div(self, context),
            F32_MIN => become Executor::execute_f32_min(self, context),
            F32_MAX => become Executor::execute_f32_max(self, context),
            F32_ABS => become Executor::execute_f32_abs(self, context),
            F32_NEG => become Executor::execute_f32_neg(self, context),
            F32_CEIL => become Executor::execute_f32_ceil(self, context),
            F32_FLOOR => become Executor::execute_f32_floor(self, context),
            F32_TRUNC => become Executor::execute_f32_trunc(self, context),
            F32_NEAREST => become Executor::execute_f32_nearest(self, context),
            F32_SQRT => become Executor::execute_f32_sqrt(self, context),
            F32_ADD => become Executor::execute_f32_add(self, context),
            F64_EQ => become Executor::execute_f64_eq(self, context),
            F64_NE => become Executor::execute_f64_ne(self, context),
            F64_LT => become Executor::execute_f64_lt(self, context),
            F64_GT => become Executor::execute_f64_gt(self, context),
            F64_LE => become Executor::execute_f64_le(self, context),
            F64_GE => become Executor::execute_f64_ge(self, context),
            F32_EQ => become Executor::execute_f32_eq(self, context),
            F32_NE => become Executor::execute_f32_ne(self, context),
            F32_LT => become Executor::execute_f32_lt(self, context),
            F32_GT => become Executor::execute_f32_gt(self, context),
            F32_LE => become Executor::execute_f32_le(self, context),
            F32_GE => become Executor::execute_f32_ge(self, context),
            REF_NULL => become Executor::execute_ref_null(self, context),
            REF_IS_NULL => become Executor::execute_ref_is_null(self, context),
            REF_FUNC => become Executor::execute_ref_func(self, context),
            MEMORY_SIZE => become Executor::execute_memory_size(self, context),
            MEMORY_GROW => become Executor::execute_memory_grow(self, context),
            EXTENSION_PREFIX => {
                let byte = context.reader.next_leb128_u32();
                match byte {
                    ELEM_DROP => become Executor::execute_elem_drop(self, context),
                    I32_TRUNC_SAT_F32_S => {
                        become Executor::execute_i32_trunc_sat_f32_s(self, context)
                    }
                    I32_TRUNC_SAT_F32_U => {
                        become Executor::execute_i32_trunc_sat_f32_u(self, context)
                    }
                    I32_TRUNC_SAT_F64_S => {
                        become Executor::execute_i32_trunc_sat_f64_s(self, context)
                    }
                    I32_TRUNC_SAT_F64_U => {
                        become Executor::execute_i32_trunc_sat_f64_u(self, context)
                    }
                    I64_TRUNC_SAT_F64_S => {
                        become Executor::execute_i64_trunc_sat_f64_s(self, context)
                    }
                    I64_TRUNC_SAT_F32_S => {
                        become Executor::execute_i64_trunc_sat_f32_s(self, context)
                    }
                    I64_TRUNC_SAT_F64_U => {
                        become Executor::execute_i64_trunc_sat_f64_u(self, context)
                    }
                    I64_TRUNC_SAT_F32_U => {
                        become Executor::execute_i64_trunc_sat_f32_u(self, context)
                    }
                    DATA_DROP => become Executor::execute_data_drop(self, context),
                    TABLE_INIT => become Executor::execute_table_init(self, context),
                    TABLE_SIZE => become Executor::execute_table_size(self, context),
                    TABLE_COPY => become Executor::execute_table_copy(self, context),
                    TABLE_FILL => become Executor::execute_table_fill(self, context),
                    TABLE_GROW => become Executor::execute_table_grow(self, context),
                    MEMORY_INIT => become Executor::execute_memory_init(self, context),
                    MEMORY_COPY => become Executor::execute_memory_copy(self, context),
                    MEMORY_FILL => become Executor::execute_memory_fill(self, context),
                    _ => unreachable!("Unreachable due to validation"),
                }
            }
            _ => unreachable!("Unreachable due to validation"),
        };
    }
}
