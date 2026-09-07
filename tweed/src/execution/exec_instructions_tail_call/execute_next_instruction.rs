use crate::core::opcode::*;
use crate::{
    core::error::ExecutionError,
    execution::executor::{ExecutionContext, Executor},
};
use extensions::*;

pub type Result<T> = core::result::Result<T, ExecutionError>;

pub type DispatchFunction =
    fn(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()>;

pub const DISPATCH_TABLE: [DispatchFunction; 256] = const {
    let mut table: [DispatchFunction; 256] = [Executor::execute_unreachable; 256];

    table[UNREACHABLE as usize] = Executor::execute_unreachable;
    table[NOP as usize] = Executor::execute_nop;
    table[BLOCK as usize] = Executor::execute_block;
    table[LOOP as usize] = Executor::execute_loop;
    table[IF as usize] = Executor::execute_if;
    table[ELSE as usize] = Executor::execute_else;
    table[END as usize] = Executor::execute_end;
    table[BR as usize] = Executor::execute_br;
    table[BR_IF as usize] = Executor::execute_br_if;
    table[BR_TABLE as usize] = Executor::execute_br_table;
    table[RETURN as usize] = Executor::execute_return;
    table[CALL as usize] = Executor::execute_call;
    table[CALL_INDIRECT as usize] = Executor::execute_call_indirect;
    table[DROP as usize] = Executor::execute_drop;
    table[SELECT as usize] = Executor::execute_select;
    table[SELECT_T as usize] = Executor::execute_select_t;
    table[LOCAL_GET as usize] = Executor::execute_local_get;
    table[LOCAL_SET as usize] = Executor::execute_local_set;
    table[LOCAL_TEE as usize] = Executor::execute_local_tee;
    table[GLOBAL_GET as usize] = Executor::execute_global_get;
    table[GLOBAL_SET as usize] = Executor::execute_global_set;
    table[TABLE_GET as usize] = Executor::execute_table_get;
    table[TABLE_SET as usize] = Executor::execute_table_set;
    table[I32_LOAD as usize] = Executor::execute_i32_load;
    table[I64_LOAD as usize] = Executor::execute_i64_load;
    table[F32_LOAD as usize] = Executor::execute_f32_load;
    table[F64_LOAD as usize] = Executor::execute_f64_load;
    table[I32_LOAD8_S as usize] = Executor::execute_i32_load8_s;
    table[I32_LOAD8_U as usize] = Executor::execute_i32_load8_u;
    table[I32_LOAD16_S as usize] = Executor::execute_i32_load16_s;
    table[I32_LOAD16_U as usize] = Executor::execute_i32_load16_u;
    table[I64_LOAD8_S as usize] = Executor::execute_i64_load8_s;
    table[I64_LOAD8_U as usize] = Executor::execute_i64_load8_u;
    table[I64_LOAD16_S as usize] = Executor::execute_i64_load16_s;
    table[I64_LOAD16_U as usize] = Executor::execute_i64_load16_u;
    table[I64_LOAD32_S as usize] = Executor::execute_i64_load32_s;
    table[I64_LOAD32_U as usize] = Executor::execute_i64_load32_u;
    table[I32_STORE as usize] = Executor::execute_i32_store;
    table[I64_STORE as usize] = Executor::execute_i64_store;
    table[F32_STORE as usize] = Executor::execute_f32_store;
    table[F64_STORE as usize] = Executor::execute_f64_store;
    table[I32_STORE8 as usize] = Executor::execute_i32_store8;
    table[I32_STORE16 as usize] = Executor::execute_i32_store16;
    table[I64_STORE8 as usize] = Executor::execute_i64_store8;
    table[I64_STORE16 as usize] = Executor::execute_i64_store16;
    table[I64_STORE32 as usize] = Executor::execute_i64_store32;
    table[MEMORY_SIZE as usize] = Executor::execute_memory_size;
    table[MEMORY_GROW as usize] = Executor::execute_memory_grow;
    table[I32_CONST as usize] = Executor::execute_i32_const;
    table[I64_CONST as usize] = Executor::execute_i64_const;
    table[F32_CONST as usize] = Executor::execute_f32_const;
    table[F64_CONST as usize] = Executor::execute_f64_const;
    table[I32_EQZ as usize] = Executor::execute_i32_eqz;
    table[I32_EQ as usize] = Executor::execute_i32_eq;
    table[I32_NE as usize] = Executor::execute_i32_ne;
    table[I32_LT_S as usize] = Executor::execute_i32_lt_s;
    table[I32_LT_U as usize] = Executor::execute_i32_lt_u;
    table[I32_GT_S as usize] = Executor::execute_i32_gt_s;
    table[I32_GT_U as usize] = Executor::execute_i32_gt_u;
    table[I32_LE_S as usize] = Executor::execute_i32_le_s;
    table[I32_LE_U as usize] = Executor::execute_i32_le_u;
    table[I32_GE_S as usize] = Executor::execute_i32_ge_s;
    table[I32_GE_U as usize] = Executor::execute_i32_ge_u;
    table[I32_CLZ as usize] = Executor::execute_i32_clz;
    table[I32_CTZ as usize] = Executor::execute_i32_ctz;
    table[I32_POPCNT as usize] = Executor::execute_i32_popcnt;
    table[I32_ADD as usize] = Executor::execute_i32_add;
    table[I32_SUB as usize] = Executor::execute_i32_sub;
    table[I32_MUL as usize] = Executor::execute_i32_mul;
    table[I32_DIV_S as usize] = Executor::execute_i32_div_s;
    table[I32_DIV_U as usize] = Executor::execute_i32_div_u;
    table[I32_REM_S as usize] = Executor::execute_i32_rem_s;
    table[I32_REM_U as usize] = Executor::execute_i32_rem_u;
    table[I32_AND as usize] = Executor::execute_i32_and;
    table[I32_OR as usize] = Executor::execute_i32_or;
    table[I32_XOR as usize] = Executor::execute_i32_xor;
    table[I32_SHL as usize] = Executor::execute_i32_shl;
    table[I32_SHR_S as usize] = Executor::execute_i32_shr_s;
    table[I32_SHR_U as usize] = Executor::execute_i32_shr_u;
    table[I32_ROTL as usize] = Executor::execute_i32_rotl;
    table[I32_ROTR as usize] = Executor::execute_i32_rotr;
    table[I64_EQZ as usize] = Executor::execute_i64_eqz;
    table[I64_EQ as usize] = Executor::execute_i64_eq;
    table[I64_NE as usize] = Executor::execute_i64_ne;
    table[I64_LT_S as usize] = Executor::execute_i64_lt_s;
    table[I64_LT_U as usize] = Executor::execute_i64_lt_u;
    table[I64_GT_S as usize] = Executor::execute_i64_gt_s;
    table[I64_GT_U as usize] = Executor::execute_i64_gt_u;
    table[I64_LE_S as usize] = Executor::execute_i64_le_s;
    table[I64_LE_U as usize] = Executor::execute_i64_le_u;
    table[I64_GE_S as usize] = Executor::execute_i64_ge_s;
    table[I64_GE_U as usize] = Executor::execute_i64_ge_u;
    table[I64_CLZ as usize] = Executor::execute_i64_clz;
    table[I64_CTZ as usize] = Executor::execute_i64_ctz;
    table[I64_POPCNT as usize] = Executor::execute_i64_popcnt;
    table[I64_ADD as usize] = Executor::execute_i64_add;
    table[I64_SUB as usize] = Executor::execute_i64_sub;
    table[I64_MUL as usize] = Executor::execute_i64_mul;
    table[I64_DIV_S as usize] = Executor::execute_i64_div_s;
    table[I64_DIV_U as usize] = Executor::execute_i64_div_u;
    table[I64_REM_S as usize] = Executor::execute_i64_rem_s;
    table[I64_REM_U as usize] = Executor::execute_i64_rem_u;
    table[I64_AND as usize] = Executor::execute_i64_and;
    table[I64_OR as usize] = Executor::execute_i64_or;
    table[I64_XOR as usize] = Executor::execute_i64_xor;
    table[I64_SHL as usize] = Executor::execute_i64_shl;
    table[I64_SHR_S as usize] = Executor::execute_i64_shr_s;
    table[I64_SHR_U as usize] = Executor::execute_i64_shr_u;
    table[I64_ROTL as usize] = Executor::execute_i64_rotl;
    table[I64_ROTR as usize] = Executor::execute_i64_rotr;
    table[F32_EQ as usize] = Executor::execute_f32_eq;
    table[F32_NE as usize] = Executor::execute_f32_ne;
    table[F32_LT as usize] = Executor::execute_f32_lt;
    table[F32_GT as usize] = Executor::execute_f32_gt;
    table[F32_LE as usize] = Executor::execute_f32_le;
    table[F32_GE as usize] = Executor::execute_f32_ge;
    table[F32_ABS as usize] = Executor::execute_f32_abs;
    table[F32_NEG as usize] = Executor::execute_f32_neg;
    table[F32_CEIL as usize] = Executor::execute_f32_ceil;
    table[F32_FLOOR as usize] = Executor::execute_f32_floor;
    table[F32_TRUNC as usize] = Executor::execute_f32_trunc;
    table[F32_NEAREST as usize] = Executor::execute_f32_nearest;
    table[F32_SQRT as usize] = Executor::execute_f32_sqrt;
    table[F32_ADD as usize] = Executor::execute_f32_add;
    table[F32_SUB as usize] = Executor::execute_f32_sub;
    table[F32_MUL as usize] = Executor::execute_f32_mul;
    table[F32_DIV as usize] = Executor::execute_f32_div;
    table[F32_MIN as usize] = Executor::execute_f32_min;
    table[F32_MAX as usize] = Executor::execute_f32_max;
    table[F32_COPYSIGN as usize] = Executor::execute_f32_copysign;
    table[F64_EQ as usize] = Executor::execute_f64_eq;
    table[F64_NE as usize] = Executor::execute_f64_ne;
    table[F64_LT as usize] = Executor::execute_f64_lt;
    table[F64_GT as usize] = Executor::execute_f64_gt;
    table[F64_LE as usize] = Executor::execute_f64_le;
    table[F64_GE as usize] = Executor::execute_f64_ge;
    table[F64_ABS as usize] = Executor::execute_f64_abs;
    table[F64_NEG as usize] = Executor::execute_f64_neg;
    table[F64_CEIL as usize] = Executor::execute_f64_ceil;
    table[F64_FLOOR as usize] = Executor::execute_f64_floor;
    table[F64_TRUNC as usize] = Executor::execute_f64_trunc;
    table[F64_NEAREST as usize] = Executor::execute_f64_nearest;
    table[F64_SQRT as usize] = Executor::execute_f64_sqrt;
    table[F64_ADD as usize] = Executor::execute_f64_add;
    table[F64_SUB as usize] = Executor::execute_f64_sub;
    table[F64_MUL as usize] = Executor::execute_f64_mul;
    table[F64_DIV as usize] = Executor::execute_f64_div;
    table[F64_MIN as usize] = Executor::execute_f64_min;
    table[F64_MAX as usize] = Executor::execute_f64_max;
    table[F64_COPYSIGN as usize] = Executor::execute_f64_copysign;
    table[I32_WRAP_I64 as usize] = Executor::execute_i32_wrap_i64;
    table[I32_TRUNC_F32_S as usize] = Executor::execute_i32_trunc_f32_s;
    table[I32_TRUNC_F32_U as usize] = Executor::execute_i32_trunc_f32_u;
    table[I32_TRUNC_F64_S as usize] = Executor::execute_i32_trunc_f64_s;
    table[I32_TRUNC_F64_U as usize] = Executor::execute_i32_trunc_f64_u;
    table[I64_EXTEND_I32_S as usize] = Executor::execute_i64_extend_i32_s;
    table[I64_EXTEND_I32_U as usize] = Executor::execute_i64_extend_i32_u;
    table[I64_TRUNC_F32_S as usize] = Executor::execute_i64_trunc_f32_s;
    table[I64_TRUNC_F32_U as usize] = Executor::execute_i64_trunc_f32_u;
    table[I64_TRUNC_F64_S as usize] = Executor::execute_i64_trunc_f64_s;
    table[I64_TRUNC_F64_U as usize] = Executor::execute_i64_trunc_f64_u;
    table[F32_CONVERT_I32_S as usize] = Executor::execute_f32_convert_i32_s;
    table[F32_CONVERT_I32_U as usize] = Executor::execute_f32_convert_i32_u;
    table[F32_CONVERT_I64_S as usize] = Executor::execute_f32_convert_i64_s;
    table[F32_CONVERT_I64_U as usize] = Executor::execute_f32_convert_i64_u;
    table[F32_DEMOTE_F64 as usize] = Executor::execute_f32_demote_f64;
    table[F64_CONVERT_I32_S as usize] = Executor::execute_f64_convert_i32_s;
    table[F64_CONVERT_I32_U as usize] = Executor::execute_f64_convert_i32_u;
    table[F64_CONVERT_I64_S as usize] = Executor::execute_f64_convert_i64_s;
    table[F64_CONVERT_I64_U as usize] = Executor::execute_f64_convert_i64_u;
    table[F64_PROMOTE_F32 as usize] = Executor::execute_f64_promote_f32;
    table[I32_REINTERPRET_F32 as usize] = Executor::execute_i32_reinterpret_f32;
    table[I64_REINTERPRET_F64 as usize] = Executor::execute_i64_reinterpret_f64;
    table[F32_REINTERPRET_I32 as usize] = Executor::execute_f32_reinterpret_i32;
    table[F64_REINTERPRET_I64 as usize] = Executor::execute_f64_reinterpret_i64;
    table[I32_EXTEND8_S as usize] = Executor::execute_i32_extend8_s;
    table[I32_EXTEND16_S as usize] = Executor::execute_i32_extend16_s;
    table[I64_EXTEND8_S as usize] = Executor::execute_i64_extend8_s;
    table[I64_EXTEND16_S as usize] = Executor::execute_i64_extend16_s;
    table[I64_EXTEND32_S as usize] = Executor::execute_i64_extend32_s;
    table[REF_NULL as usize] = Executor::execute_ref_null;
    table[REF_IS_NULL as usize] = Executor::execute_ref_is_null;
    table[REF_FUNC as usize] = Executor::execute_ref_func;
    table[EXTENSION_PREFIX as usize] = Executor::execute_extension;

    table
};

const DISPATCH_EXT_TABLE: [DispatchFunction; 18] = {
    let mut table: [DispatchFunction; 18] = [Executor::execute_unreachable; 18];
    table[ELEM_DROP as usize] = Executor::execute_elem_drop;
    table[I32_TRUNC_SAT_F32_S as usize] = Executor::execute_i32_trunc_sat_f32_s;
    table[I32_TRUNC_SAT_F32_U as usize] = Executor::execute_i32_trunc_sat_f32_u;
    table[I32_TRUNC_SAT_F64_S as usize] = Executor::execute_i32_trunc_sat_f64_s;
    table[I32_TRUNC_SAT_F64_U as usize] = Executor::execute_i32_trunc_sat_f64_u;
    table[I64_TRUNC_SAT_F64_S as usize] = Executor::execute_i64_trunc_sat_f64_s;
    table[I64_TRUNC_SAT_F32_S as usize] = Executor::execute_i64_trunc_sat_f32_s;
    table[I64_TRUNC_SAT_F64_U as usize] = Executor::execute_i64_trunc_sat_f64_u;
    table[I64_TRUNC_SAT_F32_U as usize] = Executor::execute_i64_trunc_sat_f32_u;
    table[DATA_DROP as usize] = Executor::execute_data_drop;
    table[TABLE_INIT as usize] = Executor::execute_table_init;
    table[TABLE_SIZE as usize] = Executor::execute_table_size;
    table[TABLE_COPY as usize] = Executor::execute_table_copy;
    table[TABLE_FILL as usize] = Executor::execute_table_fill;
    table[TABLE_GROW as usize] = Executor::execute_table_grow;
    table[MEMORY_INIT as usize] = Executor::execute_memory_init;
    table[MEMORY_COPY as usize] = Executor::execute_memory_copy;
    table[MEMORY_FILL as usize] = Executor::execute_memory_fill;
    table
};

impl Executor {
    pub fn start_tail_called_execution(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](self, context);
    }

    fn execute_extension(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let byte = context.reader.next_leb128_u32();
        become DISPATCH_EXT_TABLE[byte as usize](executor, context)
    }
}
