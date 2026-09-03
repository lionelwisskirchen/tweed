use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader, values::NaN},
    execution::{
        executor::Executor,
        instances::ModuleInstance,
        store::{map_memory, MemoryStore},
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_f32_const(&mut self, reader: &mut ValidatedWasmReader) -> Result<()> {
        let value = reader.next_f32();
        trace!("F32_CONST({})", value);
        self.stack.push(Value::F32(value))?;
        Ok(())
    }

    pub fn execute_f32_store(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let store_data = (self.stack.pop_f32()).to_le_bytes();
        let rel_address = self.stack.pop_i32() as u32;
        trace!("F64_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, instance, memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;
        Ok(())
    }

    pub fn execute_f32_load(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        memories: &mut MemoryStore,
    ) -> Result<()> {
        let (_, offset) = reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, instance, memories)?;
        let data: f32 = Self::load(mem, offset, address, 4)?;
        trace!("F32_LOAD({data:?} from {address:?})");

        self.stack.push(data.into())?;
        Ok(())
    }

    pub fn execute_f32_reinterpret_i32(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = Value::F32(f32::from_bits(value as u32));
        trace!("Instruction: f32.reinterpret_i32 [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_demote_f64(&mut self) -> Result<()> {
        let value = self.stack.pop_f64();
        let result = value as f32;
        trace!("Instruction: f32.demote_f64 [{value:?}] -> [{result:?}]");
        self.stack.push(Value::F32(result))?;
        Ok(())
    }

    pub fn execute_f32_copysign(&mut self) -> Result<()> {
        let z_2 = self.stack.pop_f32();
        let z_1 = self.stack.pop_f32();
        let res = Value::F32(libm::copysignf(z_1, z_2));
        trace!("Instruction: f32.copysign [{z_1} {z_2}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_convert_i32_s(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = Value::F32(value as f32);
        trace!("Instruction: f32.convert_i32_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_convert_i32_u(&mut self) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = Value::F32(value as u32 as f32);
        trace!("Instruction: f32.convert_i32_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_convert_i64_s(&mut self) -> Result<()> {
        let value = self.stack.pop_i64();
        let res = Value::F32(value as f32);
        trace!("Instruction: f32.convert_i64_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_convert_i64_u(&mut self) -> Result<()> {
        let value = self.stack.pop_i64();
        let res = Value::F32(value as u64 as f32);
        trace!("Instruction: f32.convert_i64_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_sub(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_SUB({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left - right
        };
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_mul(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_MUL({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left * right
        };
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_div(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_DIV({left}, {right})");
        // if an arithmetic NaN is used, no sub is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left / right
        };
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_min(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
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
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_max(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
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
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_abs(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::fabsf(value));
        trace!("Instruction: f32_abs [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_neg(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let bits = value.to_bits();
        let neg = f32::from_bits(bits ^ 0x80000000);
        let res = Value::F32(neg);
        trace!("Instruction: f32_neg [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_ceil(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::ceilf(value));
        trace!("Instruction: f32_ceil [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_floor(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::floorf(value));
        trace!("Instruction: f32_floor [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_trunc(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::truncf(value));
        trace!("Instruction: f32_trunc [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_nearest(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::rintf(value));
        trace!("Instruction: f32_nearest [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_sqrt(&mut self) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = Value::F32(libm::sqrtf(value));
        trace!("Instruction: f32.sqrt [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_add(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_ADD({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left + right
        };
        self.stack.push(Value::F32(res))?;
        Ok(())
    }

    pub fn execute_f32_eq(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_EQ({left}, {right})");
        let res = Value::I32((left == right) as i32);
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_ne(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_NE({left}, {right})");
        let res = Value::I32((left != right) as i32);
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_lt(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_LT({left}, {right})");
        let res = Value::I32((left < right) as i32);
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_gt(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_GT({left}, {right})");
        let res = Value::I32((left > right) as i32);
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_le(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_LE({left}, {right})");
        let res = Value::I32((left <= right) as i32);
        self.stack.push(res)?;
        Ok(())
    }

    pub fn execute_f32_ge(&mut self) -> Result<()> {
        let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
        trace!("F32_GE({left}, {right})");
        let res = Value::I32((left >= right) as i32);
        self.stack.push(res)?;
        Ok(())
    }
}
