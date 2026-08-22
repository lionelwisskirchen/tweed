use crate::{
    core::{error::ExecutionError, values::NaN},
    execution::{
        executor::{ExecutionContext, Executor},
        store::map_memory,
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_f64_const(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = context.reader.next_f64();
        trace!("F64_CONST({})", value);
        self.stack.push(Value::F64(value))?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_store(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let store_data = (self.stack.pop_f64()).to_le_bytes();
        let rel_address = self.stack.pop_i32() as u32;
        trace!("F64_STORE({store_data:?} to {rel_address:?})");
        let mem = map_memory(0, context.instance, context.memories)?;
        mem.store(rel_address as usize, offset as usize, &store_data)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_load(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (_, offset) = context.reader.read_mem_args();
        let address = self.stack.pop_i32() as u32;
        let mem = map_memory(0, context.instance, context.memories)?;
        let data: f64 = Self::load(mem, offset, address, 8)?;
        trace!("F64_LOAD({data:?} from {address:?})");
        self.stack.push(data.into())?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_reinterpret_i64(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_i64();
        let bits = value as u64;
        let f_value = f64::from_bits(bits);
        let res = Value::F64(f_value);
        info!("Instruction: f64.reinterpret_i64 [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_copysign(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let z_2 = self.stack.pop_f64();
        let z_1 = self.stack.pop_f64();
        let res = Value::F64(libm::copysign(z_1, z_2));
        trace!("Instruction: f64.copysign [{z_1} {z_2}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_convert_i32_s(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = Value::F64(value as f64);
        trace!("Instruction: f64.convert_i32_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_convert_i32_u(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_i32();
        let res = Value::F64(value as u32 as f64);
        trace!("Instruction: f64.convert_i32_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_convert_i64_s(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_i64();
        let res = Value::F64(value as f64);
        trace!("Instruction: f64.convert_i64_s [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_convert_i64_u(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_i64();
        let res = Value::F64(value as u64 as f64);
        trace!("Instruction: f64.convert_i64_u [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_promote_f32(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f32();
        let res = value as f64;
        trace!("Instruction: f64.promote_f32 [{value:?}] -> [{res:?}]");
        if log::log_enabled!(log::Level::Info) {
            let quiet_bit = (res.to_bits() >> 51) & 0x1;
            info!("is arith? {}", res.is_nan() && quiet_bit == 1);
        }
        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_abs(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::fabs(value));
        trace!("Instruction: f64_abs [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_neg(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let bits = value.to_bits();
        let neg = f64::from_bits(bits ^ 0x8000000000000000);
        let res = Value::F64(neg);
        trace!("Instruction: f64_neg [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_ceil(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::ceil(value));
        trace!("Instruction: f64_ceil [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_floor(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::floor(value));
        trace!("Instruction: f64_floor [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_trunc(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::trunc(value));
        trace!("Instruction: f64_trunc [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_nearest(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::rint(value));
        trace!("Instruction: f64_nearest [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_sqrt(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let value = self.stack.pop_f64();
        let res = Value::F64(libm::sqrt(value));
        trace!("Instruction: f64.sqrt [{value:?}] -> [{res:?}]");
        self.stack.push(res)?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_add(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_ADD({left}, {right})");

        // if an arithmetic NaN is used, no add is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left + right
        };

        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_sub(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_ADD({left}, {right})");

        // if an arithmetic NaN is used, no add is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left - right
        };

        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_mul(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_ADD({left}, {right})");

        // if an arithmetic NaN is used, no add is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left * right
        };

        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_div(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_ADD({left}, {right})");

        // if an arithmetic NaN is used, no add is done
        let res = if left.is_arithmetic_nan() {
            left.with_quiet_nan_bit()
        } else if right.is_arithmetic_nan() {
            right.with_quiet_nan_bit()
        } else {
            left / right
        };

        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_min(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_MIN({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left
        } else if right.is_arithmetic_nan() {
            right
        } else if left.is_nan() || right.is_nan() {
            f64::NAN
        } else {
            libm::fminimum_num(left, right)
        };
        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_max(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_MAX({left}, {right})");
        let res = if left.is_arithmetic_nan() {
            left
        } else if right.is_arithmetic_nan() {
            right
        } else if left.is_nan() || right.is_nan() {
            f64::NAN
        } else {
            libm::fmaximum_num(left, right)
        };
        self.stack.push(Value::F64(res))?;
        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_eq(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_EQ({left}, {right})");
        let res = Value::I32((left == right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_ne(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_NE({left}, {right})");
        let res = Value::I32((left != right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_lt(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_LT({left}, {right})");
        let res = Value::I32((left < right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_gt(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_GT({left}, {right})");
        let res = Value::I32((left > right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_f64_le(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_LE({left}, {right})");
        let res = Value::I32((left <= right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }
    pub fn execute_f64_ge(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
        trace!("F64_GE({left}, {right})");
        let res = Value::I32((left >= right) as i32);
        self.stack.push(res)?;

        become self.execute_next_instruction(context)
    }
}
