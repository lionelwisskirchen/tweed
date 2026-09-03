use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader},
    execution::{
        executor::Executor,
        instances::ModuleInstance,
        store::{map_memory, MemoryStore},
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_i32_const(&mut self, reader: &mut ValidatedWasmReader) -> Result<()> {
        let value = reader.next_leb128_i32();
        trace!("I32_CONST({})", value);
        self.stack.push(Value::I32(value))?;
        Ok(())
    }
    pub fn execute_i32_popcnt(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = value.count_ones() as i32;
        trace!("I32_POPCNT({}) TO {:?}", value, res);
        self.stack.push(Value::I32(res))?;
        Ok(())
    }

    pub fn execute_i32_ctz(&mut self) -> Result<()> {
        let val = self.stack.pop_i32();
        trace!("I32_CTZ({val})");
        let result = val.trailing_zeros() as i32;
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_shr_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_SHR_S({left}, {right})");
        let result = left.wrapping_shr((right) as u32);
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_shr_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_SHR_U({left}, {right})");
        let result = (left as u32).wrapping_shr((right) as u32) as i32;
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_shl(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_SHL({left}, {right})");
        let result = left.wrapping_shl((right) as u32);
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_add(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_ADD({left}, {right})");
        let result = right.wrapping_add(left);
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_mul(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_MUL({left}, {right})");
        let result = right.wrapping_mul(left);
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_div_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_DIV_S({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        if left == i32::MIN && right == -1 {
            return Err(ExecutionError::IntegerOverflow);
        }
        self.stack.push((left / right).into())?;
        Ok(())
    }

    pub fn execute_i32_div_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_DIV_U({left}, {right})");
        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        self.stack.push(((left as u32) / (right as u32)).into())?;
        Ok(())
    }

    pub fn execute_i32_rem_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_REM_S({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = left.checked_rem(right);
        self.stack.push(result.unwrap_or_default().into())?;
        Ok(())
    }

    pub fn execute_i32_rem_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_REM_U({left}, {right})");

        if right == 0 {
            return Err(ExecutionError::DivisionByZero);
        }
        let result = (left as u32).checked_rem(right as u32);
        self.stack.push(result.unwrap_or_default().into())?;
        Ok(())
    }

    pub fn execute_i32_sub(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_SUB({left}, {right})");
        let result = left.wrapping_sub(right);
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_eqz(&mut self) -> Result<()> {
        let val = self.stack.pop_i32();
        trace!("I32_EQZ({val})");
        self.stack.push(Value::I32((val == 0) as i32))?;
        Ok(())
    }

    pub fn execute_i32_eq(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_EQ({left}, {right})");
        self.stack.push(Value::I32((left == right) as i32))?;
        Ok(())
    }

    pub fn execute_i32_clz(&mut self) -> Result<()> {
        let val = self.stack.pop_i32();
        trace!("I32_CLZ({val})");
        self.stack.push(val.leading_zeros().into())?;
        Ok(())
    }

    pub fn execute_i32_lt_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_LT_S({left}, {right})");
        self.stack.push(Value::I32((left < right) as i32))?;

        Ok(())
    }

    pub fn execute_i32_le_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_LE_S({left}, {right})");
        self.stack.push(Value::I32((left <= right) as i32))?;
        Ok(())
    }

    pub fn execute_i32_lt_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_LT_U({left}, {right})");
        self.stack
            .push(Value::I32(((left as u32) < (right as u32)) as i32))?;
        Ok(())
    }

    pub fn execute_i32_le_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_LE_U({left}, {right})");
        self.stack
            .push(Value::I32(((left as u32) <= (right as u32)) as i32))?;
        Ok(())
    }

    pub fn execute_i32_gt_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_GT_U({left}, {right})");
        self.stack
            .push(Value::I32(((left as u32) > (right as u32)) as i32))?;
        Ok(())
    }

    pub fn execute_i32_ge_u(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_GE_U({left}, {right})");
        self.stack
            .push(Value::I32(((left as u32) >= (right as u32)) as i32))?;
        Ok(())
    }

    pub fn execute_i32_xor(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_XOR({left}, {right})");
        let result = right ^ left;
        self.stack.push(result.into())?;
        Ok(())
    }

    pub fn execute_i32_ne(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_NE({left}, {right})");
        self.stack.push(Value::I32((left != right) as i32))?;
        Ok(())
    }

    pub fn execute_i32_rotr(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        let result = left.rotate_right(right as u32);
        trace!("I32_ROTR [{left} {right}] -> [{result}]");
        self.stack.push(Value::I32(result))?;
        Ok(())
    }

    pub fn execute_i32_rotl(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        let result = left.rotate_left(right as u32);
        trace!("I32_ROTL [{left} {right}] -> [{result}]");
        self.stack.push(Value::I32(result))?;
        Ok(())
    }

    pub fn execute_i32_wrap_i64(&mut self) -> Result<()> {
        let value = self.stack.pop_i64();
        trace!("I32_WRAP_I64({value})");
        self.stack.push((value as i32).into())?;
        Ok(())
    }

    pub fn execute_i32_extend8_s(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        trace!("I32_EXTEND8_S({value})");
        let extended = (value as i8) as i32;
        self.stack.push(extended.into())?;
        Ok(())
    }

    pub fn execute_i32_extend16_s(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        trace!("I32_EXTEND16_S({value})");
        let extended = (value as i16) as i32;
        self.stack.push(extended.into())?;
        Ok(())
    }

    pub fn execute_i32_gt_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_GT_S({left}, {right})");
        self.stack.push(Value::I32((left > right) as i32))?;
        Ok(())
    }

    pub fn execute_i32_ge_s(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_GE_S({left}, {right})");
        self.stack.push(Value::I32((left >= right) as i32))?;
        Ok(())
    }

    pub fn execute_i32_and(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_AND({left}, {right})");
        self.stack.push(Value::I32(right & left))?;
        Ok(())
    }

    pub fn execute_i32_or(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
        trace!("I32_OR({left}, {right})");
        self.stack.push(Value::I32(right | left))?;
        Ok(())
    }

    pub fn execute_i32_store(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let store_data = (self.stack.pop_i32() as u32).to_le_bytes();
        let rel_address = self.stack.pop_i32() as u32;
        trace!("I32_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, instance, &mut *memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;
        Ok(())
    }

    pub fn execute_i32_store8(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let store_data = (self.stack.pop_i32() as u8).to_le_bytes();
        let rel_address = self.stack.pop_i32() as u32;
        trace!("I32_STORE8({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, instance, &mut *memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;
        Ok(())
    }

    pub fn execute_i32_store16(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let store_data = (self.stack.pop_i32() as u16).to_le_bytes();
        let rel_address = self.stack.pop_i32() as u32;
        trace!("I32_STORE16({store_data:?} to {rel_address:?})");

        let mem = map_memory(0, instance, &mut *memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;
        Ok(())
    }

    pub fn execute_i32_load(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, &mut *memories)?;
        let data: i32 = Self::load(mem, offset, address, 4)?;
        trace!("I32_LOAD({data:?} from {address:?})");
        self.stack.push(data.into())?;
        Ok(())
    }

    pub fn execute_i32_load8_u(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, &mut *memories)?;
        let data: u8 = Self::load(mem, offset, address, 1)?;
        trace!("I32_LOAD8_U({data:?} from {address:?})");
        self.stack.push((data as u32).into())?;
        Ok(())
    }

    pub fn execute_i32_load8_s(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, &mut *memories)?;
        let data: i8 = Self::load(mem, offset, address, 1)?;
        trace!("I32_LOAD8_S({data:?} from {address:?})");
        self.stack.push((data as u32).into())?;
        Ok(())
    }

    pub fn execute_i32_load16_u(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, &mut *memories)?;
        let data: u16 = Self::load(mem, offset, address, 2)?;
        trace!("I32_LOAD16_U({data:?} from {address:?})");
        self.stack.push((data as u32).into())?;
        Ok(())
    }

    pub fn execute_i32_load16_s(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, &mut *memories)?;
        let data: i16 = Self::load(mem, offset, address, 2)?;
        trace!("I32_LOAD16_S({data:?} from {address:?})");
        self.stack.push((data as u32).into())?;
        Ok(())
    }

    pub fn execute_i32_trunc_f32_s(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        // -2147483904.0 because -2147483647.0 is not representable as f32
        if value >= 2147483648.0 || value <= -2147483904.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as i32);
        trace!("Instruction: i32.trunc_f32_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_i32_trunc_f32_u(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }

        if value >= 4294967296.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as u32 as i32);
        trace!("Instruction: i32.trunc_f32_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_i32_trunc_f64_s(&mut self) -> Result<()> {
        let value = self.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }

        if value >= 2147483648.0 || value <= -2147483649.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as i32);
        trace!("Instruction: i32.trunc_f64_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_i32_trunc_f64_u(&mut self) -> Result<()> {
        let value = self.stack.pop_f64();
        if value.is_nan() {
            return Err(ExecutionError::InvalidConversionToInt);
        }
        if value >= 4294967296.0 || value <= -1.0 {
            return Err(ExecutionError::IntegerOverflow);
        }
        let res = Value::I32(value as u32 as i32);
        trace!("Instruction: i32.trunc_f64_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_i32_reinterpret_f32(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let bits = value.to_bits();
        let res = Value::I32(bits as i32);
        trace!("Instruction: i32.reinterpret_f32 [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_i32_trunc_sat_f32_s(&mut self) -> Result<()> {
        let val = self.stack.pop_f32();
        let res = val as i32; // Since Rust 1.45 f32->i32 is saturating
        trace!("I32_TRUNC_SAT_F32_S: {val} -> {res}");
        self.stack.push(Value::I32(res))?;
        Ok(())
    }

    pub fn execute_i32_trunc_sat_f32_u(&mut self) -> Result<()> {
        let val = self.stack.pop_f32();
        let res = (val as u32) as i32; // Since Rust 1.45 f32->u32 is saturating
        trace!("I32_TRUNC_SAT_F32_U: {val} -> {res}");
        self.stack.push(Value::I32(res))?;
        Ok(())
    }

    pub fn execute_i32_trunc_sat_f64_s(&mut self) -> Result<()> {
        let val = self.stack.pop_f64();
        let res = val as i32; // Since Rust 1.45 f32->i32 is saturating
        trace!("I32_TRUNC_SAT_F64_S: {val} -> {res}");
        self.stack.push(Value::I32(res))?;
        Ok(())
    }

    pub fn execute_i32_trunc_sat_f64_u(&mut self) -> Result<()> {
        let val = self.stack.pop_f64();
        let res = (val as u32) as i32; // Since Rust 1.45 f32->u32 is saturating
        trace!("I32_TRUNC_SAT_F64_U: {val} -> {res}");
        self.stack.push(Value::I32(res))?;
        Ok(())
    }
}
