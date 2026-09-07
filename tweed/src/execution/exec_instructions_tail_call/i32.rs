use crate::{
    core::error::ExecutionError,
    execution::{
        exec_instructions_tail_call::execute_next_instruction::DISPATCH_TABLE,
        executor::{ExecutionContext, Executor},
        store::map_memory,
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_i32_const(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = context.reader.next_leb128_i32();
        trace!("I32_CONST({})", value);
        executor.stack.push(Value::I32(value))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }
    pub fn execute_i32_popcnt(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        let res = value.count_ones() as i32;
        trace!("I32_POPCNT({}) TO {:?}", value, res);
        executor.stack.push(Value::I32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_ctz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i32();
        trace!("I32_CTZ({val})");
        let result = val.trailing_zeros() as i32;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_shr_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_SHR_S({left}, {right})");
        let result = left.wrapping_shr((right) as u32);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_shr_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_SHR_U({left}, {right})");
        let result = (left as u32).wrapping_shr((right) as u32) as i32;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_shl(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_SHL({left}, {right})");
        let result = left.wrapping_shl((right) as u32);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_add(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_ADD({left}, {right})");
        let result = right.wrapping_add(left);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_mul(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_MUL({left}, {right})");
        let result = right.wrapping_mul(left);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_div_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_DIV_S({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        if left == i32::MIN && right == -1 {
            return Err(ExecutionError::IntegerOverflow);
        }
        executor.stack.push((left / right).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_div_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_DIV_U({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        executor
            .stack
            .push(((left as u32) / (right as u32)).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_rem_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_REM_S({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = left.checked_rem(right);
        executor.stack.push(result.unwrap_or_default().into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_rem_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_REM_U({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = (left as u32).checked_rem(right as u32);
        executor.stack.push(result.unwrap_or_default().into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_sub(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_SUB({left}, {right})");
        let result = left.wrapping_sub(right);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_eqz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i32();
        trace!("I32_EQZ({val})");
        executor.stack.push(Value::I32((val == 0) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_eq(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_EQ({left}, {right})");
        executor.stack.push(Value::I32((left == right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_clz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i32();
        trace!("I32_CLZ({val})");
        executor.stack.push(val.leading_zeros().into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_lt_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_LT_S({left}, {right})");
        executor.stack.push(Value::I32((left < right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_le_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_LE_S({left}, {right})");
        executor.stack.push(Value::I32((left <= right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_lt_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_LT_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u32) < (right as u32)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_le_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_LE_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u32) <= (right as u32)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_gt_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_GT_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u32) > (right as u32)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_ge_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_GE_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u32) >= (right as u32)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_xor(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_XOR({left}, {right})");
        let result = right ^ left;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_ne(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_NE({left}, {right})");
        executor.stack.push(Value::I32((left != right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_rotr(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        let result = left.rotate_right(right as u32);
        trace!("I32_ROTR [{left} {right}] -> [{result}]");
        executor.stack.push(Value::I32(result))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_rotl(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        let result = left.rotate_left(right as u32);
        trace!("I32_ROTL [{left} {right}] -> [{result}]");
        executor.stack.push(Value::I32(result))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_wrap_i64(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        trace!("I32_WRAP_I64({value})");
        executor.stack.push((value as i32).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_extend8_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        trace!("I32_EXTEND8_S({value})");
        let extended = (value as i8) as i32;
        executor.stack.push(extended.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_extend16_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        trace!("I32_EXTEND16_S({value})");
        let extended = (value as i16) as i32;
        executor.stack.push(extended.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_gt_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_GT_S({left}, {right})");
        executor.stack.push(Value::I32((left > right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_ge_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_GE_S({left}, {right})");
        executor.stack.push(Value::I32((left >= right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_and(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_AND({left}, {right})");
        executor.stack.push(Value::I32(right & left))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_or(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i32(), executor.stack.pop_i32());
        trace!("I32_OR({left}, {right})");
        executor.stack.push(Value::I32(right | left))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_store(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i32() as u32).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I32_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_store8(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i32() as u8).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I32_STORE8({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_store16(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i32() as u16).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I32_STORE16({store_data:?} to {rel_address:?})");

        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_load(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i32 = Self::load(mem, offset, address, 4)?;
        trace!("I32_LOAD({data:?} from {address:?})");
        executor.stack.push(data.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_load8_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: u8 = Self::load(mem, offset, address, 1)?;
        trace!("I32_LOAD8_U({data:?} from {address:?})");
        executor.stack.push((data as u32).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_load8_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i8 = Self::load(mem, offset, address, 1)?;
        trace!("I32_LOAD8_S({data:?} from {address:?})");
        executor.stack.push((data as u32).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_load16_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: u16 = Self::load(mem, offset, address, 2)?;
        trace!("I32_LOAD16_U({data:?} from {address:?})");
        executor.stack.push((data as u32).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_load16_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i16 = Self::load(mem, offset, address, 2)?;
        trace!("I32_LOAD16_S({data:?} from {address:?})");
        executor.stack.push((data as u32).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_f32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        // -2147483904.0 because -2147483647.0 is not representable as f32
        if value >= 2147483648.0 || value <= -2147483904.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as i32);
        trace!("Instruction: i32.trunc_f32_s [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_f32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }

        if value >= 4294967296.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as u32 as i32);
        trace!("Instruction: i32.trunc_f32_u [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_f64_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }

        if value >= 2147483648.0 || value <= -2147483649.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as i32);
        trace!("Instruction: i32.trunc_f64_s [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_f64_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        if value >= 4294967296.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as u32 as i32);
        trace!("Instruction: i32.trunc_f64_u [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_reinterpret_f32(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        let bits = value.to_bits();
        let res = Value::I32(bits as i32);
        trace!("Instruction: i32.reinterpret_f32 [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_sat_f32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f32();
        let res = val as i32; // Since Rust 1.45 f32->i32 is saturating
        trace!("I32_TRUNC_SAT_F32_S: {val} -> {res}");
        executor.stack.push(Value::I32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_sat_f32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f32();
        let res = (val as u32) as i32; // Since Rust 1.45 f32->u32 is saturating
        trace!("I32_TRUNC_SAT_F32_U: {val} -> {res}");
        executor.stack.push(Value::I32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_sat_f64_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f64();
        let res = val as i32; // Since Rust 1.45 f32->i32 is saturating
        trace!("I32_TRUNC_SAT_F64_S: {val} -> {res}");
        executor.stack.push(Value::I32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i32_trunc_sat_f64_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f64();
        let res = (val as u32) as i32; // Since Rust 1.45 f32->u32 is saturating
        trace!("I32_TRUNC_SAT_F64_U: {val} -> {res}");
        executor.stack.push(Value::I32(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }
}
