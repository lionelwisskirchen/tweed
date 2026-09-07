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
    pub fn execute_i64_const(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = context.reader.next_leb128_i64();
        trace!("I64_CONST({})", value);
        executor.stack.push(Value::I64(value))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_popcnt(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        let res = value.count_ones() as i64;
        trace!("I64_POPCNT({}) TO {:?}", value, res);
        executor.stack.push(Value::I64(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_ctz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i64();
        trace!("I64_CTZ({val})");
        let result = val.trailing_zeros() as i64;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_shr_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_SHR_S({left}, {right})");
        let result = left.wrapping_shr((right) as u32);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_shr_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_SHR_U({left}, {right})");
        let result = (left as u64).wrapping_shr((right) as u32) as i64;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_shl(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_SHL({left}, {right})");
        let result = left.wrapping_shl((right) as u32);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_add(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_ADD({left}, {right})");
        let result = right.wrapping_add(left);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }
    pub fn execute_i64_sub(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_SUB({left}, {right})");
        let result = left.wrapping_sub(right);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_mul(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_MUL({left}, {right})");
        let result = right.wrapping_mul(left);
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_div_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_DIV_S({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        if left == i64::MIN && right == -1 {
            return Err(ExecutionError::IntegerOverflow);
        }
        executor.stack.push((left / right).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_div_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_DIV_U({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        executor
            .stack
            .push(((left as u64) / (right as u64)).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_rem_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_REM_S({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = left.checked_rem(right);
        executor.stack.push(result.unwrap_or_default().into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_rem_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_REM_U({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = (left as u64).checked_rem(right as u64);
        executor.stack.push(result.unwrap_or_default().into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_and(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_AND({left}, {right})");

        let result = right & left;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_or(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_OR({left}, {right})");

        let result = right | left;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_xor(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_XOR({left}, {right})");

        let result = right ^ left;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_rotr(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        let result = left.rotate_right(right as u32);
        trace!("I64_ROTR [{left} {right}] -> [{result}]");
        executor.stack.push(Value::I64(result))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_rotl(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        let result = left.rotate_left(right as u32);
        trace!("I64_ROTL [{left} {right}] -> [{result}]");
        executor.stack.push(Value::I64(result))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_clz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i64();
        trace!("I64_CLZ({val})");
        let result = val.leading_zeros() as u64;
        executor.stack.push(result.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_extend8_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        trace!("I64_EXTEND8_S({value})");
        let extended = (value as i8) as i64;
        executor.stack.push(extended.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_extend16_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        trace!("I64_EXTEND16_S({value})");
        let extended = (value as i16) as i64;
        executor.stack.push(extended.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_extend32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i64();
        trace!("I64_EXTEND32_S({value})");
        let extended = (value as i32) as i64;
        executor.stack.push(extended.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_extend_i32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        trace!("I64_EXTEND_I32_U({value})");
        executor.stack.push((value as u32 as i64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_extend_i32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_i32();
        trace!("I64_EXTEND_I32_S({value})");
        executor.stack.push((value as i64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_ge_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_GE_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u64) >= (right as u64)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_ge_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_GE_U({left}, {right})");
        executor.stack.push(Value::I32((left >= right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_gt_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_GT_U({left}, {right})");
        executor.stack.push(Value::I32((left > right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_gt_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_GT_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u64) > (right as u64)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_lt_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_LT_U({left}, {right})");
        executor.stack.push(Value::I32((left < right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_lt_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_LT_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u64) < (right as u64)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_le_s(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_LE_S({left}, {right})");
        executor.stack.push(Value::I32((left <= right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_le_u(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_LE_U({left}, {right})");
        executor
            .stack
            .push(Value::I32(((left as u64) <= (right as u64)) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_eqz(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let val = executor.stack.pop_i64();
        trace!("I64_EQZ({val})");
        executor.stack.push(Value::I32((val == 0) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_eq(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_EQ({left}, {right})");
        executor.stack.push(Value::I32((left == right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_ne(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (executor.stack.pop_i64(), executor.stack.pop_i64());
        trace!("I64_NE({left}, {right})");
        executor.stack.push(Value::I32((left != right) as i32))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_store(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i64()).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I64_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_store8(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i64() as u8).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I64_STORE8({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_store16(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i64() as u16).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I64_STORE16({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_store32(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (executor.stack.pop_i64() as u32).to_le_bytes();
        let rel_address = executor.stack.pop_i32() as u32;
        trace!("I64_STORE32({store_data:?} to {rel_address:?})");

        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i64 = Self::load(mem, offset, address, 8)?;
        trace!("I64_LOAD({data:?} from {address:?})");
        executor.stack.push(data.into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load8_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i8 = Self::load(mem, offset, address, 1)?;
        trace!("I64_LOAD8_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load8_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: u8 = Self::load(mem, offset, address, 1)?;
        trace!("I64_LOAD8_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load16_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i16 = Self::load(mem, offset, address, 2)?;
        trace!("I64_LOAD16_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load16_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: u16 = Self::load(mem, offset, address, 2)?;
        trace!("I64_LOAD16_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: i32 = Self::load(mem, offset, address, 4)?;
        trace!("I64_LOAD32_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_load32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = executor.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: u32 = Self::load(mem, offset, address, 4)?;
        trace!("I64_LOAD32_S({data:?} from {address:?})");
        executor.stack.push((data as u64).into())?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_f32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        if value >= 9223372036854775808.0 || value <= -9223373136366403584.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I64(value as i64);
        trace!("Instruction: i64.trunc_f32_s  [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_f32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        // 2^64
        if value >= 18446744073709551616.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I64(value as u64 as i64);
        trace!("Instruction: i64.trunc_f32_u  [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_f64_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }

        if value >= 9223372036854775808.0 || value <= -9223372036854777856.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I64(value as i64);
        trace!("Instruction: i64.trunc_f64_s  [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_f64_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        if value >= 18446744073709551616.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I64(value as u64 as i64);
        trace!("Instruction: i64.trunc_f64_u  [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_reinterpret_f64(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop_f64();
        let bits = value.to_bits();
        let res = Value::I64(bits as i64);
        trace!("Instruction: i64.reinterpret_f64 [{value:?}] -> [{res:?}]");
        executor.stack.push(res)?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_sat_f64_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f64();
        let res = val as i64; // Since Rust 1.45 f64->i64 is saturating
        trace!("I64_TRUNC_SAT_F64_S: {val} -> {res}");
        executor.stack.push(Value::I64(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_sat_f32_s(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f32();
        let res = val as i64; // Since Rust 1.45 f64->i64 is saturating
        trace!("I64_TRUNC_SAT_F32_S: {val} -> {res}");
        executor.stack.push(Value::I64(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_sat_f64_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f64();
        let res = (val as u64) as i64; // Since Rust 1.45 f64->i64 is saturating
        trace!("I64_TRUNC_SAT_F64_U: {val} -> {res}");
        executor.stack.push(Value::I64(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }

    pub fn execute_i64_trunc_sat_f32_u(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let val = executor.stack.pop_f32();
        let res = (val as u64) as i64; // Since Rust 1.45 f64->i64 is saturating
        trace!("I64_TRUNC_SAT_F32_U: {val} -> {res}");
        executor.stack.push(Value::I64(res))?;

        let instruction = context.reader.next_byte() as usize;
        let handler = unsafe { *DISPATCH_TABLE.get_unchecked(instruction as usize) };
        become handler(executor, context)
    }
}
