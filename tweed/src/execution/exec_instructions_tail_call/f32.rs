use crate::{
    core::{error::ExecutionError, values::NaN},
    execution::{
        exec_instructions_tail_call::start_tail_called_execution::DISPATCH_TABLE,
        executor::{ExecutionContext, Executor},
        store::map_memory,
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_f32_const(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = context.reader.next_f32();
        trace!("F32_CONST({})", value);
        executor.stack.push(Value::F32(value))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_store(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_f32()).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("F64_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_load(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: f32 = Self::load(mem, offset, address, 4)?;
        trace!("F32_LOAD({data:?} from {address:?})");

        executor.stack.push(data.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_reinterpret_i32(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        let res = Value::F32(f32::from_bits(value as u32));
        trace!("Instruction: f32.reinterpret_i32 [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_demote_f64(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        let result = value as f32;
        trace!("Instruction: f32.demote_f64 [{value:?}] -> [{result:?}]");
        executor.stack.push(Value::F32(result))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_copysign(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let z_2 = executor.stack.pop_f32();
        let z_1 = executor.stack.pop_f32();
        let res = Value::F32(libm::copysignf(z_1, z_2));
        trace!("Instruction: f32.copysign [{z_1} {z_2}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_convert_i32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        let res = Value::F32(value as f32);
        trace!("Instruction: f32.convert_i32_s [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_convert_i32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        let res = Value::F32(value as u32 as f32);
        trace!("Instruction: f32.convert_i32_u [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_convert_i64_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        let res = Value::F32(value as f32);
        trace!("Instruction: f32.convert_i64_s [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_convert_i64_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        let res = Value::F32(value as u64 as f32);
        trace!("Instruction: f32.convert_i64_u [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_sub(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_SUB({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left - right
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_mul(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_MUL({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left * right
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_div(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_DIV({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left / right
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_min(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_MIN({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left
        } else if right.is_arithmetic_nan() {
            right
        } else if left.is_nan() || right.is_nan() {
            f32::NAN
        } else {
            libm::fminimum_numf(left, right)
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_max(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_MAX({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left
        } else if right.is_arithmetic_nan() {
            right
        } else if left.is_nan() || right.is_nan() {
            f32::NAN
        } else {
            libm::fmaximum_numf(left, right)
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_abs(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::fabsf(value));
        trace!("Instruction: f32_abs [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_neg(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let value = executor.stack.pop_f32();
        let bits = value.to_bits();
        let neg = f32::from_bits(bits ^ 0x80000000);
        let res = Value::F32(neg);
        trace!("Instruction: f32_neg [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_ceil(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::ceilf(value));
        trace!("Instruction: f32_ceil [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_floor(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::floorf(value));
        trace!("Instruction: f32_floor [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_trunc(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::truncf(value));
        trace!("Instruction: f32_trunc [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_nearest(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::rintf(value));
        trace!("Instruction: f32_nearest [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_sqrt(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let value = executor.stack.pop_f32();
        let res = Value::F32(libm::sqrtf(value));
        trace!("Instruction: f32.sqrt [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_add(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_ADD({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left + right
        };
        executor.stack.push(Value::F32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_eq(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_EQ({left}, {right})");
        let res = Value::I32((left == right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_ne(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_NE({left}, {right})");
        let res = Value::I32((left != right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_lt(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_LT({left}, {right})");
        let res = Value::I32((left < right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_gt(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_GT({left}, {right})");
        let res = Value::I32((left > right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_le(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_LE({left}, {right})");
        let res = Value::I32((left <= right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_f32_ge(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_f32(), executor.stack.pop_f32());
        trace!("F32_GE({left}, {right})");
        let res = Value::I32((left >= right) as i32);
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }
}
