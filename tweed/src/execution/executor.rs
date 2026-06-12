extern crate alloc;

use super::execution_stack::{CallStack, ExecutionStack};
use super::instances::function_instance::Internal;
use super::instances::{FuncInstance, MemoryInstance};
use super::store::{map_memory, HostEnv, Store};
use crate::execution::adresses::{Address, FuncAddr, InstanceAddr};

use crate::core::error::ExecutionError;
use crate::core::opcode::*;
use crate::core::validated_wasm_reader::ValidatedWasmReader;
use crate::core::value_types::ValType;
use crate::core::values::NaN;
use crate::{core::values::FromLeBytes, validation::side_table::SideTable, Value};
use alloc::vec::Vec;
use extensions::*;
use libm;
pub type Result<T> = core::result::Result<T, ExecutionError>;

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
    fn build_frame(&mut self, function: &Internal) -> Result<Frame> {
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

    fn execute_frame(&mut self, current_frame: &mut Frame, store: &mut Store) -> Result<()> {
        let instance_addr = &current_frame.instance_addr;
        let instance = store.instances.get_mut(*instance_addr)?;
        let reader_address = instance.reader;
        let reader = store.reader.get_mut(reader_address).unwrap();
        reader.seek(current_frame.start + current_frame.program_counter);

        loop {
            let instruction = reader.next_byte();
            match instruction {
                UNREACHABLE => {
                    return Err(ExecutionError::TrapUnreachable);
                }

                I32_CONST => {
                    let value = reader.next_leb128_i32();
                    trace!("I32_CONST({})", value);
                    self.stack.push(Value::I32(value))?;
                }

                I64_CONST => {
                    let value = reader.next_leb128_i64();
                    trace!("I64_CONST({})", value);
                    self.stack.push(Value::I64(value))?;
                }
                F32_CONST => {
                    let value = reader.next_f32();
                    trace!("F32_CONST({})", value);
                    self.stack.push(Value::F32(value))?;
                }
                F64_CONST => {
                    let value = reader.next_f64();
                    trace!("F64_CONST({})", value);
                    self.stack.push(Value::F64(value))?;
                }
                BLOCK => {
                    trace!("BLOCK");
                    reader.next_leb128_u32(); // Skip Block type
                }
                LOOP => {
                    trace!("LOOP");
                    reader.next_leb128_u32(); // Skip Block type
                }
                IF => {
                    let bool_val = self.stack.pop();
                    trace!("IF({:?})", bool_val);
                    if bool_val == Value::I32(0) {
                        Self::do_goto(&mut self.stack, current_frame, reader)?;
                    } else {
                        reader.next_leb128_u32(); // Skip Block type
                        current_frame.side_table_pointer += 1;
                    }
                }
                ELSE => {
                    trace!("ELSE");
                    Self::do_goto(&mut self.stack, current_frame, reader)?;
                }
                LOCAL_GET => {
                    let idx = reader.next_leb128_u32() as usize;
                    trace!("LOCAL_GET({})", idx);
                    let local = current_frame
                        .locals
                        .get(idx)
                        .ok_or(ExecutionError::InvalidLocalIndex)?;
                    self.stack.push(*local)?;
                }
                I32_POPCNT => {
                    let value = self.stack.pop_i32();
                    let res = value.count_ones() as i32;
                    trace!("I32_POPCNT({}) TO {:?}", value, res);
                    self.stack.push(Value::I32(res))?;
                }
                LOCAL_SET => {
                    let idx = reader.next_leb128_u32() as usize;
                    let local = current_frame
                        .locals
                        .get_mut(idx)
                        .ok_or(ExecutionError::InvalidLocalIndex)?;
                    let new_value = self.stack.pop();
                    trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
                    *local = new_value;
                }
                GLOBAL_GET => {
                    let idx = reader.next_leb128_u32() as usize;
                    trace!("GLOBAL_GET({})", idx);
                    let global_addr = instance
                        .globaladdr
                        .get(idx)
                        .ok_or(ExecutionError::UnknownGlobal)?;
                    let global = store.globals.get_mut(*global_addr)?;
                    self.stack.push(global.value)?;
                }
                GLOBAL_SET => {
                    let idx = reader.next_leb128_u32() as usize;
                    let global_addr = instance
                        .globaladdr
                        .get(idx)
                        .ok_or(ExecutionError::UnknownGlobal)?;
                    let global = store.globals.get_mut(*global_addr)?;
                    let new_value = self.stack.pop();
                    trace!("GLOBAL_SET({}) TO {:?}", idx, new_value);
                    global.value = new_value;
                }
                LOCAL_TEE => {
                    let idx = reader.next_leb128_u32() as usize;
                    let local = current_frame
                        .locals
                        .get_mut(idx)
                        .ok_or(ExecutionError::InvalidLocalIndex)?;
                    let new_value = self.stack.pop();
                    trace!("LOCAL_TEE({}) TO {:?}", idx, new_value);
                    *local = new_value;
                    self.stack.push(new_value)?;
                }
                DROP => {
                    let val = self.stack.pop();
                    trace!("DROP {:?}", val);
                }
                NOP => {
                    trace!("NOP");
                }
                I32_CTZ => {
                    let val = self.stack.pop_i32();
                    trace!("I32_CTZ({val})");
                    let result = val.trailing_zeros() as i32;
                    self.stack.push(result.into())?;
                }
                I64_POPCNT => {
                    let value = self.stack.pop_i64();
                    let res = value.count_ones() as i64;
                    trace!("I64_POPCNT({}) TO {:?}", value, res);
                    self.stack.push(Value::I64(res))?;
                }
                I64_CTZ => {
                    let val = self.stack.pop_i64();
                    trace!("I64_CTZ({val})");
                    let result = val.trailing_zeros() as i64;
                    self.stack.push(result.into())?;
                }
                I64_ADD => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_ADD({left}, {right})");
                    let result = right.wrapping_add(left);
                    self.stack.push(result.into())?;
                }
                I64_SUB => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_SUB({left}, {right})");
                    let result = left.wrapping_sub(right);
                    self.stack.push(result.into())?;
                }
                I64_MUL => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_MUL({left}, {right})");
                    let result = left.wrapping_mul(right);
                    self.stack.push(result.into())?;
                }

                I64_DIV_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_DIV_S({left}, {right})");
                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    if left == i64::MIN && right == -1 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    self.stack.push((left / right).into())?;
                }
                I64_DIV_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_DIV_U({left}, {right})");
                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    self.stack.push(((left as u64) / (right as u64)).into())?;
                }
                I64_REM_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_REM_S({left}, {right})");

                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    let result = left.checked_rem(right);
                    self.stack.push(result.unwrap_or_default().into())?;
                }
                I64_REM_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_REM_U({left}, {right})");

                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    let result = (left as u64).checked_rem(right as u64);
                    self.stack.push(result.unwrap_or_default().into())?;
                }

                I64_AND => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_AND({left}, {right})");

                    let result = right & left;
                    self.stack.push(result.into())?;
                }
                I64_OR => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_OR({left}, {right})");

                    let result = right | left;
                    self.stack.push(result.into())?;
                }
                I64_XOR => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_XOR({left}, {right})");

                    let result = right ^ left;
                    self.stack.push(result.into())?;
                }
                I64_SHR_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_SHR_U({left}, {right})");

                    let result = (left as u64).wrapping_shr((right & 63) as u32) as i64;
                    self.stack.push(result.into())?;
                }
                I64_SHR_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_SHR_S({left}, {right})");
                    let result = left.wrapping_shr((right & 63) as u32);
                    self.stack.push(result.into())?;
                }
                I64_SHL => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_SHL({left}, {right})");
                    let result = left.wrapping_shl((right & 63) as u32);
                    self.stack.push(result.into())?;
                }
                I32_SHR_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_SHR_S({left}, {right})");
                    let result = left.wrapping_shr((right) as u32);
                    self.stack.push(result.into())?;
                }
                I32_SHR_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_SHR_U({left}, {right})");
                    let result = (left as u32).wrapping_shr((right) as u32) as i32;
                    self.stack.push(result.into())?;
                }
                I32_SHL => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_SHL({left}, {right})");
                    let result = left.wrapping_shl((right) as u32);
                    self.stack.push(result.into())?;
                }
                I32_ADD => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_ADD({left}, {right})");
                    let result = right.wrapping_add(left);
                    self.stack.push(result.into())?;
                }
                I32_MUL => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_MUL({left}, {right})");
                    let result = right.wrapping_mul(left);
                    self.stack.push(result.into())?;
                }
                I32_DIV_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_DIV_S({left}, {right})");
                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    if left == i32::MIN && right == -1 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    self.stack.push((left / right).into())?;
                }
                I32_DIV_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_DIV_U({left}, {right})");
                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    self.stack.push(((left as u32) / (right as u32)).into())?;
                }
                I32_REM_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_REM_S({left}, {right})");

                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    let result = left.checked_rem(right);
                    self.stack.push(result.unwrap_or_default().into())?;
                }
                I32_REM_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_REM_U({left}, {right})");

                    if right == 0 {
                        return Err(ExecutionError::DivisionByZero);
                    }
                    let result = (left as u32).checked_rem(right as u32);
                    self.stack.push(result.unwrap_or_default().into())?;
                }
                I32_SUB => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_SUB({left}, {right})");
                    let result = left.wrapping_sub(right);
                    self.stack.push(result.into())?;
                }
                I32_EQZ => {
                    let val = self.stack.pop_i32();
                    trace!("I32_EQZ({val})");
                    if val == 0 {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }

                I32_EQ => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_EQ({left}, {right})");
                    if right == left {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }

                I32_CLZ => {
                    let val = self.stack.pop_i32();
                    trace!("I32_CLZ({val})");
                    self.stack.push(val.leading_zeros().into())?;
                }
                I32_LT_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_LT_S({left}, {right})");
                    if left < right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_LE_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_LE_S({left}, {right})");
                    if left <= right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_LT_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_LT_U({left}, {right})");
                    if (left as u32) < (right as u32) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_LE_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_LE_U({left}, {right})");
                    if (left as u32) <= (right as u32) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_GT_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_GT_U({left}, {right})");
                    if (left as u32) > (right as u32) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_GE_U => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_GT_U({left}, {right})");
                    if (left as u32) >= (right as u32) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_XOR => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_XOR({left}, {right})");
                    let result = right ^ left;
                    self.stack.push(result.into())?;
                }

                I32_NE => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_NE({left}, {right})");
                    if left != right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_ROTR => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    let result = left.rotate_right(right as u32);
                    trace!("I32_ROTR [{left} {right}] -> [{result}]");
                    self.stack.push(Value::I32(result))?;
                }
                I32_ROTL => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    let result = left.rotate_left(right as u32);
                    trace!("I32_ROTL [{left} {right}] -> [{result}]");
                    self.stack.push(Value::I32(result))?;
                }
                I64_ROTR => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    let result = left.rotate_right(right as u32);
                    trace!("I64_ROTR [{left} {right}] -> [{result}]");
                    self.stack.push(Value::I64(result))?;
                }
                I64_ROTL => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    let result = left.rotate_left(right as u32);
                    trace!("I64_ROTL [{left} {right}] -> [{result}]");
                    self.stack.push(Value::I64(result))?;
                }
                I64_CLZ => {
                    let val = self.stack.pop_i64();
                    trace!("I64_CLZ({val})");
                    let result = val.leading_zeros() as u64;
                    self.stack.push(result.into())?;
                }
                I32_WRAP_I64 => {
                    let value = self.stack.pop_i64();
                    trace!("I32_WRAP_I64({value})");
                    self.stack.push((value as i32).into())?
                }
                I32_EXTEND8_S => {
                    let value = self.stack.pop_i32();
                    trace!("I32_EXTEND8_S({value})");
                    let extended = (value as i8) as i32;
                    self.stack.push(extended.into())?
                }
                I32_EXTEND16_S => {
                    let value = self.stack.pop_i32();
                    trace!("I32_EXTEND16_S({value})");
                    let extended = (value as i16) as i32;
                    self.stack.push(extended.into())?
                }
                I64_EXTEND8_S => {
                    let value = self.stack.pop_i64();
                    trace!("I64_EXTEND8_S({value})");
                    let extended = (value as i8) as i64;
                    self.stack.push(extended.into())?
                }
                I64_EXTEND16_S => {
                    let value = self.stack.pop_i64();
                    trace!("I64_EXTEND16_S({value})");
                    let extended = (value as i16) as i64;
                    self.stack.push(extended.into())?
                }
                I64_EXTEND32_S => {
                    let value = self.stack.pop_i64();
                    trace!("I64_EXTEND32_S({value})");
                    let extended = (value as i32) as i64;
                    self.stack.push(extended.into())?
                }
                I64_EXTEND_I32_U => {
                    let value = self.stack.pop_i32();
                    trace!("I64_EXTEND_I32_U({value})");
                    self.stack.push((value as u32 as i64).into())?
                }
                I64_EXTEND_I32_S => {
                    let value = self.stack.pop_i32();
                    trace!("I64_EXTEND_I32_S({value})");
                    self.stack.push((value as i64).into())?
                }
                I64_GE_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_GE_U({left}, {right})");
                    if (left as u64) >= (right as u64) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_GT_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_GT_S({left}, {right})");
                    if (left) > (right) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_GT_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_GT_U({left}, {right})");
                    if (left as u64) > (right as u64) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_LT_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_LT_S({left}, {right})");
                    if (left) < (right) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_LT_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_LT_U({left}, {right})");
                    if (left as u64) < (right as u64) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_GE_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_GE_S({left}, {right})");
                    if (left) >= (right) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_LE_S => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_LE_S({left}, {right})");
                    if (left) <= (right) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_LE_U => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_GE_U({left}, {right})");
                    if (left as u64) <= (right as u64) {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_EQZ => {
                    let val = self.stack.pop_i64();
                    trace!("I64_EQZ({val})");
                    if val == 0 {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }

                I64_EQ => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_EQ({left}, {right})");
                    if left == right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I64_NE => {
                    let (right, left) = (self.stack.pop_i64(), self.stack.pop_i64());
                    trace!("I64_NE({left}, {right})");
                    if left != right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_GT_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_GT_S({left}, {right})");
                    if left > right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }

                I32_GE_S => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_GE_S({left}, {right})");
                    if left >= right {
                        self.stack.push(Value::I32(1))?;
                    } else {
                        self.stack.push(Value::I32(0))?;
                    }
                }
                I32_AND => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_AND({left}, {right})");
                    let result = right & left;
                    self.stack.push(Value::I32(result))?;
                }
                I32_OR => {
                    let (right, left) = (self.stack.pop_i32(), self.stack.pop_i32());
                    trace!("I32_OR({left}, {right})");
                    let result = right | left;
                    self.stack.push(Value::I32(result))?;
                }
                SELECT => {
                    let (cond, b, a) = (self.stack.pop_i32(), self.stack.pop(), self.stack.pop());
                    trace!("SELECT({cond} ? {a:?} : {b:?})");
                    if cond == 0 {
                        self.stack.push(b)?;
                    } else {
                        self.stack.push(a)?;
                    }
                }
                BR => {
                    trace!("BR");
                    Self::do_goto(&mut self.stack, current_frame, reader)?;
                }
                BR_IF => {
                    let bool_val = self.stack.pop_i32();
                    trace!("BR_IF({bool_val})");
                    if bool_val == 0 {
                        reader.next_leb128_u32(); // skip depth
                        current_frame.side_table_pointer += 1;
                    } else {
                        Self::do_goto(&mut self.stack, current_frame, reader)?;
                    }
                }
                SELECT_T => {
                    reader.next_leb128_u32();

                    reader.next_byte();
                    let c = self.stack.pop_i32();
                    let val_2 = self.stack.pop(); // TODO Type check?
                    let val_1 = self.stack.pop();
                    trace!("SELECT_T({c})");
                    if c == 0 {
                        self.stack.push(val_2)?;
                    } else {
                        self.stack.push(val_1)?;
                    }
                }
                BR_TABLE => {
                    // TODO br_table vec[Branch index] not += branch_index!
                    let branch_index = self.stack.pop_i32();
                    trace!("BR_TABLE({branch_index})");
                    let number_of_branches = reader.next_leb128_u32() as usize;
                    if branch_index as usize >= number_of_branches {
                        current_frame.side_table_pointer += number_of_branches;
                    } else {
                        current_frame.side_table_pointer += branch_index as usize;
                    }
                    Self::do_goto(&mut self.stack, current_frame, reader)?;
                }
                RETURN => {
                    trace!("RETURN");
                    let count = current_frame.return_value.len();
                    let results = self.stack.pop_n(count)?;
                    self.stack.unwind_to(current_frame.stack_pointer);
                    self.stack.push_vec(&results)?;
                    break;
                }
                END => {
                    trace!("END");
                    if reader.current_index() == current_frame.end {
                        break;
                    }
                }
                CALL_INDIRECT => {
                    let type_index = reader.next_leb128_u32();
                    let table_index = reader.next_leb128_u32();
                    let table_addr = instance
                        .tableaddr
                        .get(table_index as usize)
                        .ok_or(ExecutionError::UnknownTable)?;

                    let table = store.tables.get(*table_addr)?;

                    let func_type = instance
                        .types
                        .get(type_index as usize)
                        .ok_or(ExecutionError::InvalidFunctionIndex)?;
                    let entry_index = self.stack.pop_i32() as usize;
                    let Ok(entry) = table.get(entry_index) else {
                        return Err(ExecutionError::UndefinedElement);
                    };
                    let index = match entry {
                        Value::RefNull(_) => {
                            return Err(ExecutionError::UninitializedElement(None))
                        }
                        Value::FuncRef(idx) => FuncAddr(idx),
                        Value::ExternRef(_) => return Err(ExecutionError::TypeMismatch),
                        _ => unreachable!(),
                    };
                    let function = store
                        .functions
                        .get_mut(index)
                        .map_err(|_| ExecutionError::InvalidFunctionIndex)?
                        .clone();
                    if function.get_signature() != func_type {
                        return Err(ExecutionError::IndirectCallTypeMismatch);
                    };

                    match function {
                        FuncInstance::Internal(instance) => {
                            let frame = self.build_frame(&instance)?;
                            current_frame.program_counter =
                                reader.current_index() - current_frame.start + 1;

                            self.call_stack.push(current_frame.clone())?;
                            self.call_stack.push(frame)?;
                            break;
                        }

                        FuncInstance::External(host_function) => {
                            let mut env: HostEnv = HostEnv {
                                memories: store.memories.items(),
                                globals: store.globals.items(),
                            };
                            let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                            let mut func_ref = host_function.func.borrow_mut();
                            let results = func_ref(&mut env, args)?;
                            self.stack.push_vec(&results)?;
                        }
                    }
                }
                CALL => {
                    let index = reader.next_leb128_u32();
                    trace!("CALL({index})");

                    // funcaddr contains the indices of the functions on the store
                    // | [0] StoreIdx = 2 | [1] StoreIdx = 3 | [2] StoreIdx = 6 | [3] StoreIdx = x |
                    // so if want to call the function with index 0 we actually call the second
                    // function in the store
                    let function_addr = instance
                        .funcaddr
                        .get(index as usize)
                        .ok_or(ExecutionError::InvalidFunctionIndex)?;

                    let function = store.functions.get_mut(*function_addr)?.clone();

                    match function {
                        FuncInstance::Internal(instance) => {
                            let frame = self.build_frame(&instance)?;
                            current_frame.program_counter =
                                reader.current_index() - current_frame.start + 1;
                            self.call_stack.push(current_frame.clone())?;
                            self.call_stack.push(frame)?;
                            break;
                        }

                        FuncInstance::External(host_function) => {
                            let mut env: HostEnv = HostEnv {
                                memories: store.memories.items(),
                                globals: store.globals.items(),
                            };
                            let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                            let mut func_ref = host_function.func.borrow_mut();
                            let results = func_ref(&mut env, args)?;
                            self.stack.push_vec(&results)?;
                        }
                    }
                }
                I64_STORE => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i64()).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I64_STORE({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                F32_STORE => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_f32()).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("F64_STORE({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                F64_STORE => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_f64()).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("F64_STORE({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I64_STORE8 => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i64() as u8).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I64_STORE8({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I64_STORE16 => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i64() as u16).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I64_STORE16({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I64_STORE32 => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i64() as u32).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I64_STORE32({store_data:?} to {rel_address:?})");

                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I64_LOAD => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i64 = Self::load(mem, offset, address, 8)?;
                    trace!("I64_LOAD({data:?} from {address:?})");
                    self.stack.push(data.into())?;
                }
                F64_LOAD => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: f64 = Self::load(mem, offset, address, 8)?;
                    trace!("F64_LOAD({data:?} from {address:?})");
                    self.stack.push(data.into())?;
                }
                I32_STORE => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i32() as u32).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I32_STORE({store_data:?} to {rel_address:?})");
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I32_STORE8 => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i32() as u8).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I32_STORE8({store_data:?} to {rel_address:?})");

                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I32_STORE16 => {
                    let (_, offset) = reader.read_mem_args();
                    let store_data = (self.stack.pop_i32() as u16).to_le_bytes();
                    let rel_address = self.stack.pop_i32() as u32;
                    trace!("I32_STORE16({store_data:?} to {rel_address:?})");

                    let mem = map_memory(0, instance, &mut store.memories)?;
                    mem.store(rel_address as usize, offset as usize, &store_data)?;
                }
                I32_LOAD => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i32 = Self::load(mem, offset, address, 4)?;
                    trace!("I32_LOAD({data:?} from {address:?})");

                    self.stack.push(data.into())?;
                }
                F32_LOAD => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: f32 = Self::load(mem, offset, address, 4)?;
                    trace!("F32_LOAD({data:?} from {address:?})");

                    self.stack.push(data.into())?;
                }
                I64_LOAD8_S => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i8 = Self::load(mem, offset, address, 1)?;
                    trace!("I64_LOAD8_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I64_LOAD8_U => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: u8 = Self::load(mem, offset, address, 1)?;
                    trace!("I64_LOAD8_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I64_LOAD16_S => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i16 = Self::load(mem, offset, address, 2)?;
                    trace!("I64_LOAD16_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I64_LOAD16_U => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: u16 = Self::load(mem, offset, address, 2)?;
                    trace!("I64_LOAD16_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I64_LOAD32_S => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i32 = Self::load(mem, offset, address, 4)?;
                    trace!("I64_LOAD32_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I64_LOAD32_U => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: u32 = Self::load(mem, offset, address, 4)?;
                    trace!("I64_LOAD32_S({data:?} from {address:?})");
                    self.stack.push((data as u64).into())?;
                }
                I32_LOAD8_U => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: u8 = Self::load(mem, offset, address, 1)?;
                    trace!("I32_LOAD8_U({data:?} from {address:?})");
                    self.stack.push((data as u32).into())?;
                }
                I32_LOAD16_U => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: u16 = Self::load(mem, offset, address, 2)?;
                    trace!("I32_LOAD16_U({data:?} from {address:?})");
                    self.stack.push((data as u32).into())?;
                }
                I32_LOAD16_S => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i16 = Self::load(mem, offset, address, 2)?;
                    trace!("I32_LOAD16_S({data:?} from {address:?})");
                    self.stack.push((data as u32).into())?;
                }
                I32_LOAD8_S => {
                    let (_, offset) = reader.read_mem_args();
                    let address = self.stack.pop_i32() as u32;
                    let mem = map_memory(0, instance, &mut store.memories)?;
                    let data: i8 = Self::load(mem, offset, address, 1)?;
                    trace!("I32_LOAD8_S({data:?} from {address:?})");
                    self.stack.push((data as u32).into())?;
                }
                TABLE_GET => {
                    let table_idx = reader.next_leb128_u32() as usize;
                    trace!("TABLE_GET '{table_idx:?}'");

                    let table_addr = instance
                        .tableaddr
                        .get(table_idx)
                        .ok_or(ExecutionError::TableAccessOutOfBounds)?;
                    let offset = self.stack.pop_i32() as usize;
                    let table = store.tables.get_mut(*table_addr).unwrap(); // validated
                    let value = table.get(offset)?;
                    self.stack.push(value)?;
                    trace!("TABLE_GET '{}' [{}] -> [{:?}]", table_idx, offset, value);
                }
                TABLE_SET => {
                    let table_idx = reader.next_leb128_u32() as usize;

                    let table_addr = instance
                        .tableaddr
                        .get(table_idx)
                        .ok_or(ExecutionError::TableAccessOutOfBounds)?;
                    let table = store.tables.get_mut(*table_addr).unwrap(); // validated
                    let val = self.stack.pop();
                    let index = self.stack.pop_i32() as usize;
                    table.insert(index, &[val])?;
                    trace!("TABLE_SET '{}' [{} {:?}] -> []", table_idx, index, val)
                }

                I32_TRUNC_F32_S => {
                    let value = self.stack.pop_f32();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    // -2147483904.0 Because -2147483647.0 is not representable as f32
                    if value >= 2147483648.0 || value <= -2147483904.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I32(value as i32);
                    trace!("Instruction: i32.trunc_f32_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I32_TRUNC_F32_U => {
                    let value = self.stack.pop_f32();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    trace!(
                        "Instruction: i32.trunc_f32_u [{value:?}] {}",
                        u32::MAX as f32
                    );

                    if value >= 4294967296.0 || value <= -1.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I32(value as u32 as i32);
                    trace!("Instruction: i32.trunc_f32_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I32_TRUNC_F64_S => {
                    let value = self.stack.pop_f64();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }

                    if value >= 2147483648.0 || value <= -2147483649.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I32(value as i32);
                    trace!("Instruction: i32.trunc_f64_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I32_TRUNC_F64_U => {
                    let value = self.stack.pop_f64();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    if value >= 4294967296.0 || value <= -1.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I32(value as u32 as i32);
                    trace!("Instruction: i32.trunc_f64_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I64_TRUNC_F32_S => {
                    let value = self.stack.pop_f32();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    if value >= 9223372036854775808.0 || value <= -9223373136366403584.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I64(value as i64);
                    trace!("Instruction: i64.trunc_f32_s  [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I64_TRUNC_F32_U => {
                    let value = self.stack.pop_f32();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    // 2^64
                    if value >= 18446744073709551616.0 || value <= -1.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I64(value as u64 as i64);
                    trace!("Instruction: i64.trunc_f32_u  [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }

                I64_TRUNC_F64_S => {
                    let value = self.stack.pop_f64();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    if value >= 9223372036854775808.0 || value <= -9223372036854777856.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I64(value as i64);
                    trace!("Instruction: i64.trunc_f64_s  [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I64_TRUNC_F64_U => {
                    let value = self.stack.pop_f64();
                    if value.is_nan() {
                        return Err(ExecutionError::InvalidConversionToInt);
                    }
                    if value.is_infinite() {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    if value >= 18446744073709551616.0 || value <= -1.0 {
                        return Err(ExecutionError::IntegerOverflow);
                    }
                    let res = Value::I64(value as u64 as i64);
                    trace!("Instruction: i64.trunc_f64_u  [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I32_REINTERPRET_F32 => {
                    let value = self.stack.pop_f32();
                    let bits = value.to_bits();
                    let res = Value::I32(bits as i32);
                    trace!("Instruction: i32.reinterpret_f32 [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                I64_REINTERPRET_F64 => {
                    let value = self.stack.pop_f64();
                    let bits = value.to_bits();
                    let res = Value::I64(bits as i64);
                    trace!("Instruction: i64.reinterpret_f64 [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_REINTERPRET_I64 => {
                    let value = self.stack.pop_i64();
                    let bits = value as u64;
                    let f_value = f64::from_bits(bits);
                    let res = Value::F64(f_value);
                    info!("Instruction: f64.reinterpret_i64 [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_REINTERPRET_I32 => {
                    let value = self.stack.pop_i32();
                    let bits = value as u32;
                    let f_value = f32::from_bits(bits);
                    let res = Value::F32(f_value);
                    trace!("Instruction: f32.reinterpret_i32 [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_DEMOTE_F64 => {
                    let value = self.stack.pop_f64();
                    let result = value as f32;
                    trace!("Instruction: f32.demote_f64 [{value:?}] -> [{result:?}]");
                    self.stack.push(Value::F32(result))?;
                }
                F64_COPYSIGN => {
                    let z_2 = self.stack.pop_f64();
                    let z_1 = self.stack.pop_f64();
                    let res = Value::F64(libm::copysign(z_1, z_2));
                    trace!("Instruction: f64.copysign [{z_1} {z_2}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_COPYSIGN => {
                    let z_2 = self.stack.pop_f32();
                    let z_1 = self.stack.pop_f32();
                    let res = Value::F32(libm::copysignf(z_1, z_2));
                    trace!("Instruction: f32.copysign [{z_1} {z_2}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_CONVERT_I32_S => {
                    let value = self.stack.pop_i32();
                    let res = Value::F32(value as f32);
                    trace!("Instruction: f32.convert_i32_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_CONVERT_I32_U => {
                    let value = self.stack.pop_i32();
                    let res = Value::F32(value as u32 as f32);
                    trace!("Instruction: f32.convert_i32_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_CONVERT_I64_S => {
                    let value = self.stack.pop_i64();
                    let res = Value::F32(value as f32);
                    trace!("Instruction: f32.convert_i64_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_CONVERT_I64_U => {
                    let value = self.stack.pop_i64();
                    let res = Value::F32(value as u64 as f32);
                    trace!("Instruction: f32.convert_i64_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_CONVERT_I32_S => {
                    let value = self.stack.pop_i32();
                    let res = Value::F64(value as f64);
                    trace!("Instruction: f64.convert_i32_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_CONVERT_I32_U => {
                    let value = self.stack.pop_i32();
                    let res = Value::F64(value as u32 as f64);
                    trace!("Instruction: f64.convert_i32_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_CONVERT_I64_S => {
                    let value = self.stack.pop_i64();
                    let res = Value::F64(value as f64);
                    trace!("Instruction: f64.convert_i64_s [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_CONVERT_I64_U => {
                    let value = self.stack.pop_i64();
                    let res = Value::F64(value as u64 as f64);
                    trace!("Instruction: f64.convert_i64_u [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_PROMOTE_F32 => {
                    let value = self.stack.pop_f32();
                    let res = value as f64;
                    trace!("Instruction: f64.promote_f32 [{value:?}] -> [{res:?}]");
                    let quiet_bit = (res.to_bits() >> 51) & 0x1;
                    info!("is arith? {}", res.is_nan() && quiet_bit == 1);
                    self.stack.push(Value::F64(res))?;
                }

                F64_ABS => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::fabs(value));
                    trace!("Instruction: f64_abs [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_NEG => {
                    let value = self.stack.pop_f64();
                    let bits = value.to_bits();
                    let neg = f64::from_bits(bits ^ 0x8000000000000000);
                    let res = Value::F64(neg);
                    trace!("Instruction: f64_neg [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_CEIL => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::ceil(value));
                    trace!("Instruction: f64_ceil [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_FLOOR => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::floor(value));
                    trace!("Instruction: f64_floor [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_TRUNC => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::trunc(value));
                    trace!("Instruction: f64_trunc [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_NEAREST => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::rint(value));
                    trace!("Instruction: f64_nearest [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_SQRT => {
                    let value = self.stack.pop_f64();
                    let res = Value::F64(libm::sqrt(value));
                    trace!("Instruction: f64.sqrt [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F64_ADD => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_ADD({left}, {right})");

                    // if an arithmetic NaN is used, no add is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F64(left.with_quiet_nan_bit()),
                        (_, true) => Value::F64(right.with_quiet_nan_bit()),
                        _ => Value::F64(left + right),
                    };

                    self.stack.push(res)?;
                }
                F64_SUB => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_SUB({left}, {right})");

                    // if an arithmetic NaN is used, no sub is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F64(left.with_quiet_nan_bit()),
                        (_, true) => Value::F64(right.with_quiet_nan_bit()),
                        _ => Value::F64(left - right),
                    };

                    self.stack.push(res)?;
                }
                F64_MUL => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_MUL({left}, {right})");

                    // if an arithmetic NaN is used, no mul is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F64(left.with_quiet_nan_bit()),
                        (_, true) => Value::F64(right.with_quiet_nan_bit()),
                        _ => Value::F64(left * right),
                    };

                    self.stack.push(res)?;
                }
                F64_DIV => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_DIV({left}, {right})");

                    // if an arithmetic NaN is used, no div is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F64(left.with_quiet_nan_bit()),
                        (_, true) => Value::F64(right.with_quiet_nan_bit()),
                        _ => Value::F64(left / right),
                    };

                    self.stack.push(res)?;
                }
                F64_MIN => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_MIN({left}, {right})");

                    // Handling the NaN Edge Cases so the right errors are returned
                    // As soon as an arithmetic nan is used, its value is returned so an Value::ArithmeticNan is detected
                    let res = if left.is_nan() || right.is_nan() {
                        match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                            (true, _) => left,
                            (_, true) => right,
                            _ => f64::NAN,
                        }
                    } else if left == right {
                        if left.is_sign_negative() {
                            left
                        } else {
                            right
                        }
                    } else {
                        left.min(right)
                    };

                    self.stack.push(Value::F64(res))?;
                }
                F64_MAX => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_MAX({left}, {right})");

                    // Handling the NaN Edge Cases so the right errors are returned
                    // As soon as an arithmetic nan is used, its value is returned so an Value::ArithmeticNan is detected
                    let res = if left.is_nan() || right.is_nan() {
                        match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                            (true, _) => left,
                            (_, true) => right,
                            _ => f64::NAN,
                        }
                    } else if left == right {
                        if left.is_sign_negative() {
                            right
                        } else {
                            left
                        }
                    } else {
                        left.max(right)
                    };

                    // let res = Value::F64(left.max(right));
                    self.stack.push(Value::F64(res))?;
                }

                F32_SUB => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_SUB({left}, {right})");

                    // if an arithmetic NaN is used, no sub is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F32(left.with_quiet_nan_bit()),
                        (_, true) => Value::F32(right.with_quiet_nan_bit()),
                        _ => Value::F32(left - right),
                    };

                    // let res = Value::F32(left - right);
                    self.stack.push(res)?;
                }
                F32_MUL => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_MUL({left}, {right})");

                    // if an arithmetic NaN is used, no mul is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F32(left.with_quiet_nan_bit()),
                        (_, true) => Value::F32(right.with_quiet_nan_bit()),
                        _ => Value::F32(left * right),
                    };

                    self.stack.push(res)?;
                }
                F32_DIV => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_DIV({left}, {right})");

                    // if an arithmetic NaN is used, no div is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F32(left.with_quiet_nan_bit()),
                        (_, true) => Value::F32(right.with_quiet_nan_bit()),
                        _ => Value::F32(left / right),
                    };

                    self.stack.push(res)?;
                }
                F32_MIN => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_MIN({left}, {right})");

                    // Handling the NaN Edge Cases so the right errors are returned
                    // As soon as an arithmetic nan is used, its value is returned so an Value::ArithmeticNan is detected
                    let res = if left.is_nan() || right.is_nan() {
                        match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                            (true, _) => left,
                            (_, true) => right,
                            _ => f32::NAN,
                        }
                    } else if left == right {
                        if left.is_sign_negative() {
                            left
                        } else {
                            right
                        }
                    } else {
                        left.min(right)
                    };

                    self.stack.push(Value::F32(res))?;
                }
                F32_MAX => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_MAX({left}, {right})");

                    // Handling the NaN Edge Cases so the right errors are returned
                    // As soon as an arithmetic nan is used, its value is returned so an Value::ArithmeticNan is detected
                    let res = if left.is_nan() || right.is_nan() {
                        match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                            (true, _) => left,
                            (_, true) => right,
                            _ => f32::NAN,
                        }
                    } else if left == right {
                        if left.is_sign_negative() {
                            right
                        } else {
                            left
                        }
                    } else {
                        left.max(right)
                    };

                    self.stack.push(Value::F32(res))?;
                }

                F32_ABS => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::fabsf(value));
                    trace!("Instruction: f32_abs [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_NEG => {
                    let value = self.stack.pop_f32();
                    let bits = value.to_bits();
                    let neg = f32::from_bits(bits ^ 0x80000000);
                    let res = Value::F32(neg);
                    trace!("Instruction: f32_neg [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_CEIL => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::ceilf(value));
                    trace!("Instruction: f32_ceil [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_FLOOR => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::floorf(value));
                    trace!("Instruction: f32_floor [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_TRUNC => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::truncf(value));
                    trace!("Instruction: f32_trunc [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_NEAREST => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::rintf(value));
                    trace!("Instruction: f32_nearest [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_SQRT => {
                    let value = self.stack.pop_f32();
                    let res = Value::F32(libm::sqrtf(value));
                    trace!("Instruction: f32.sqrt [{value:?}] -> [{res:?}]");
                    self.stack.push(res)?;
                }
                F32_ADD => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_ADD({left}, {right})");

                    // if an arithmetic NaN is used, no add is done
                    let res = match (left.is_arithmetic_nan(), right.is_arithmetic_nan()) {
                        (true, _) => Value::F32(left.with_quiet_nan_bit()),
                        (_, true) => Value::F32(right.with_quiet_nan_bit()),
                        _ => Value::F32(left + right),
                    };

                    self.stack.push(res)?;
                }

                F64_EQ => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_EQ({left}, {right})");
                    let res = Value::I32(if left == right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F64_NE => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_NE({left}, {right})");
                    let res = Value::I32(if left != right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F64_LT => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_LT({left}, {right})");
                    let res = Value::I32(if left < right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F64_GT => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_GT({left}, {right})");
                    let res = Value::I32(if left > right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F64_LE => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_LE({left}, {right})");
                    let res = Value::I32(if left <= right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F64_GE => {
                    let (right, left) = (self.stack.pop_f64(), self.stack.pop_f64());
                    trace!("F64_GE({left}, {right})");
                    let res = Value::I32(if left >= right { 1 } else { 0 });
                    self.stack.push(res)?;
                }

                F32_EQ => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_EQ({left}, {right})");
                    let res = Value::I32(if left == right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F32_NE => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_NE({left}, {right})");
                    let res = Value::I32(if left != right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F32_LT => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_LT({left}, {right})");
                    let res = Value::I32(if left < right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F32_GT => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_GT({left}, {right})");
                    let res = Value::I32(if left > right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F32_LE => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_LE({left}, {right})");
                    let res = Value::I32(if left <= right { 1 } else { 0 });
                    self.stack.push(res)?;
                }
                F32_GE => {
                    let (right, left) = (self.stack.pop_f32(), self.stack.pop_f32());
                    trace!("F32_GE({left}, {right})");
                    let res = Value::I32(if left >= right { 1 } else { 0 });
                    self.stack.push(res)?;
                }

                REF_NULL => {
                    let ref_type = crate::core::value_types::RefType::from_byte(reader.next_byte())
                        .ok_or(ExecutionError::UnexpectedValueType)?;
                    self.stack.push(Value::RefNull(ref_type))?;
                }
                REF_IS_NULL => {
                    let value = self.stack.pop();
                    match value {
                        Value::RefNull(_) => self.stack.push(Value::I32(1))?,
                        _ => self.stack.push(Value::I32(0))?,
                    }
                }
                REF_FUNC => {
                    let func_idx = reader.next_leb128_u32() as usize;
                    let store_addr = instance.funcaddr.get(func_idx).unwrap(); // Validated
                    self.stack.push(Value::FuncRef(store_addr.as_usize()))?;
                }
                MEMORY_SIZE => {
                    let idx = reader.next_byte();
                    let mem = map_memory(idx as usize, instance, &mut store.memories).unwrap();
                    let size_in_pages = mem.pages() as i32;

                    self.stack.push(Value::I32(size_in_pages))?;
                }
                MEMORY_GROW => {
                    let idx = reader.next_byte();
                    let mem = map_memory(idx as usize, instance, &mut store.memories).unwrap();
                    let size_in_pages = mem.pages() as i32;

                    let amount = self.stack.pop_i32() as u32;
                    trace!("Instruction: memory.grow {}", size_in_pages);

                    if mem.grow(amount).is_ok() {
                        self.stack.push(Value::I32(size_in_pages))?;
                    } else {
                        self.stack.push(Value::I32(-1))?;
                    }
                }
                EXTENSION_PREFIX => {
                    let byte = reader.next_leb128_u32();
                    match byte {
                        ELEM_DROP => {
                            let element_idx = reader.next_leb128_u32() as usize;
                            // 2. Assert: due to validation, 𝐹.module.elemaddrs[𝑥] exists.
                            // 3. Let 𝑎 be the element address 𝐹.module.elemaddrs[𝑥].
                            let element_addr = instance.elemaddr.get(element_idx).unwrap();
                            // 4. Assert: due to validation, 𝑆.elems[𝑎] exists.
                            let elem = store.elements.get_mut(*element_addr).unwrap();
                            // Replace 𝑆.elems[𝑎].elem with 𝜖.
                            elem.references.clear();
                        }
                        I32_TRUNC_SAT_F32_S => {
                            let val = self.stack.pop_f32();
                            let res = if val.is_nan() {
                                0
                            } else if val <= (i32::MIN as f32) {
                                i32::MIN
                            } else if val >= (i32::MAX as f32) {
                                i32::MAX
                            } else {
                                val as i32
                            };
                            trace!("I32_TRUNC_SAT_F32_S: {val} -> {res}");
                            self.stack.push(Value::I32(res))?;
                        }
                        I32_TRUNC_SAT_F32_U => {
                            let val = self.stack.pop_f32();
                            let res = if val.is_nan() || val <= 0.0 {
                                0
                            } else if val >= u32::MAX as f32 {
                                u32::MAX
                            } else {
                                val as u32
                            };
                            trace!("I32_TRUNC_SAT_F32_U: {val} -> {res}");
                            self.stack.push(Value::I32(res as i32))?;
                        }
                        I32_TRUNC_SAT_F64_S => {
                            let val = self.stack.pop_f64();
                            let res = if val.is_nan() {
                                0
                            } else if val <= (i32::MIN as f64) {
                                i32::MIN
                            } else if val >= (i32::MAX as f64) {
                                i32::MAX
                            } else {
                                val as i32
                            };
                            trace!("I32_TRUNC_SAT_F64_S: {val} -> {res}");
                            self.stack.push(Value::I32(res))?;
                        }
                        I32_TRUNC_SAT_F64_U => {
                            let val = self.stack.pop_f64();
                            let res = if val.is_nan() || val <= 0.0 {
                                0
                            } else if val >= (u32::MAX as f64) {
                                u32::MAX
                            } else {
                                val as u32
                            };
                            trace!("I32_TRUNC_SAT_F64_U: {val} -> {res}");
                            self.stack.push(Value::I32(res as i32))?;
                        }
                        I64_TRUNC_SAT_F64_S => {
                            let val = self.stack.pop_f64();
                            let res = if val.is_nan() {
                                0
                            } else if val <= (i64::MIN as f64) {
                                i64::MIN
                            } else if val >= (i64::MAX as f64) {
                                i64::MAX
                            } else {
                                val as i64
                            };
                            trace!("I64_TRUNC_SAT_F64_S: {val} -> {res}");
                            self.stack.push(Value::I64(res))?;
                        }
                        I64_TRUNC_SAT_F32_S => {
                            let val = self.stack.pop_f32();
                            let res = if val.is_nan() {
                                0
                            } else if val <= (i64::MIN as f32) {
                                i64::MIN
                            } else if val >= (i64::MAX as f32) {
                                i64::MAX
                            } else {
                                val as i64
                            };
                            trace!("I64_TRUNC_SAT_F32_S: {val} -> {res}");
                            self.stack.push(Value::I64(res))?;
                        }
                        I64_TRUNC_SAT_F64_U => {
                            let val = self.stack.pop_f64();
                            let res = if val.is_nan() || val <= 0.0 {
                                0
                            } else if val >= (u64::MAX as f64) {
                                u64::MAX
                            } else {
                                val as u64
                            };
                            trace!("I64_TRUNC_SAT_F64_U: {val} -> {res}");
                            self.stack.push(Value::I64(res as i64))?;
                        }
                        I64_TRUNC_SAT_F32_U => {
                            let val = self.stack.pop_f32();
                            let res = if val.is_nan() || val <= 0.0 {
                                0
                            } else if val >= (u64::MAX as f32) {
                                u64::MAX
                            } else {
                                val as u64
                            };
                            trace!("I64_TRUNC_SAT_F32_U: {val} -> {res}");
                            self.stack.push(Value::I64(res as i64))?;
                        }
                        DATA_DROP => {
                            // 2. Assert: due to validation, 𝐹.module.dataaddrs[𝑥] exists.
                            // 3. Let 𝑎 be the data address 𝐹.module.dataaddrs[𝑥].
                            let data_index = reader.next_leb128_u32() as usize;
                            let data_addr = instance.dataddr.get(data_index).unwrap();
                            // 4. Assert: due to validation, 𝑆.datas[𝑎] exists.
                            let data_inst = store.datas.get_mut(*data_addr).unwrap();
                            // 5. Replace 𝑆.datas[𝑎] with the data instance {data 𝜖}.
                            data_inst.clear();
                        }
                        TABLE_INIT => {
                            let element_idx = reader.next_leb128_u32() as usize;
                            let table_idx = reader.next_leb128_u32();

                            // 2. Assert: due to validation, 𝐹.module.tableaddrs[𝑥] exists.
                            // 3. Let ta be the table address 𝐹.module.tableaddrs[𝑥]
                            let table_addr = instance.tableaddr.get(table_idx as usize).unwrap();
                            // 4. Assert: due to validation, 𝑆.tables[ta] exists.
                            // 5. Let tab be the table instance 𝑆.tables[ta]
                            let tab = store.tables.get_mut(*table_addr).unwrap(); // validated

                            // 6. Assert: due to validation, 𝐹.module.elemaddrs[𝑦] exists
                            // 7. Let ea be the element address 𝐹.module.elemaddrs[𝑦].
                            let element_addr = instance.elemaddr.get(element_idx).unwrap();
                            // 8. Assert: due to validation, 𝑆.elems[ea] exists.
                            // 9. Let elem be the element instance 𝑆.elems[ea]
                            let elem = store.elements.get(*element_addr).unwrap(); // validated

                            // 10. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 11. Pop the value i32.const 𝑛 from the stack.
                            let n = self.stack.pop_i32() as usize;
                            // 12. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 13. Pop the value i32.const 𝑠 from the stack.
                            let s = self.stack.pop_i32() as usize;
                            // 14. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 15. Pop the value i32.const 𝑑 from the stack.
                            let d = self.stack.pop_i32() as usize;

                            let src_end = s
                                .checked_add(n)
                                .ok_or(ExecutionError::TableAccessOutOfBounds)?;
                            let dst_end = d
                                .checked_add(n)
                                .ok_or(ExecutionError::TableAccessOutOfBounds)?;

                            // If 𝑠 + 𝑛 is larger than the length of elem.elem or 𝑑 + 𝑛 is larger
                            // than the length of tab.elem, then:
                            // a. Trap.
                            if src_end > elem.references.len() || dst_end > tab.reference_count() {
                                return Err(ExecutionError::TableAccessOutOfBounds);
                            }

                            let elems_to_copy = &elem.references[s..src_end];
                            tab.insert(d, elems_to_copy)?;
                        }
                        TABLE_SIZE => {
                            let table_idx = reader.next_leb128_u32();
                            let table_addr = instance.tableaddr.get(table_idx as usize).unwrap(); // validated
                            let tab = store.tables.get_mut(*table_addr).unwrap(); // validated
                            info!("TABLE_SIZE {:?}", tab.reference_count());

                            self.stack.push(Value::I32(tab.reference_count() as i32))?;
                        }
                        TABLE_COPY => {
                            let table_idx_x = reader.next_leb128_u32(); // Destination
                            let table_idx_y = reader.next_leb128_u32(); // Source
                            let table_x_addr =
                                instance.tableaddr.get(table_idx_x as usize).unwrap();
                            let table_y_addr =
                                instance.tableaddr.get(table_idx_y as usize).unwrap();

                            let n = self.stack.pop_i32() as usize; // size
                            let s = self.stack.pop_i32() as usize; // source
                            let d = self.stack.pop_i32() as usize; // destination

                            let src_end = s
                                .checked_add(n)
                                .ok_or(ExecutionError::TableAccessOutOfBounds)?;

                            let dst_end = d
                                .checked_add(n)
                                .ok_or(ExecutionError::TableAccessOutOfBounds)?;
                            let src_data = {
                                let table_y = store.tables.get(*table_y_addr).unwrap();
                                let len = table_y.reference_values.len();
                                if src_end > len {
                                    return Err(ExecutionError::TableAccessOutOfBounds);
                                };

                                let data = table_y.get_slice(s, n)?.to_vec();
                                data
                            };

                            let table_x = store.tables.get_mut(*table_x_addr).unwrap(); // validated
                            if dst_end > table_x.reference_values.len() {
                                return Err(ExecutionError::TableAccessOutOfBounds);
                            }
                            table_x.insert(d, &src_data)?;
                        }
                        TABLE_FILL => {
                            let table_idx = reader.next_leb128_u32();
                            let table_addr = instance.tableaddr.get(table_idx as usize).unwrap(); // validated
                            let tab = store.tables.get_mut(*table_addr).unwrap(); // validated
                            let n = self.stack.pop_i32() as usize; // size
                            let val = self.stack.pop();
                            let i = self.stack.pop_i32() as usize; // dest
                            let end = i
                                .checked_add(n)
                                .ok_or(ExecutionError::TableAccessOutOfBounds)?;
                            if end > tab.reference_count() {
                                return Err(ExecutionError::TableAccessOutOfBounds);
                            }
                            tab.fill(i, end, val)?;
                        }
                        TABLE_GROW => {
                            let table_idx = reader.next_leb128_u32();
                            let table_addr = instance.tableaddr.get(table_idx as usize).unwrap(); // validated
                            let tab = store.tables.get_mut(*table_addr).unwrap(); // validated

                            let sz = tab.reference_count();

                            let n = self.stack.pop_i32() as usize; // size
                            let val = self.stack.pop();

                            let lens = sz.checked_add(n);
                            let Some(len) = lens else {
                                self.stack.push(Value::I32(u32::MAX as i32))?;
                                continue;
                            };

                            if let Some(max) = tab.table_type.limit.max {
                                if len > max as usize {
                                    self.stack.push(Value::I32(u32::MAX as i32))?;
                                    continue;
                                }
                            }
                            tab.grow(n, val)?;
                            trace!("GROW n:{:?}, val:{:?}", n, val);

                            tab.table_type.limit.min = len as u32;
                            self.stack.push(Value::I32(sz as i32))?;
                        }
                        MEMORY_INIT => {
                            let data_index = reader.next_leb128_u32() as usize;
                            let memory_index = reader.next_byte() as usize;
                            // 2. Assert: due to validation, 𝐹.module.memaddrs[0] exists.
                            // 3. Let ma be the memory address 𝐹.module.memaddrs[0].
                            let mem_addr = instance.memaddr.get(memory_index).unwrap();

                            // 4. Assert: due to validation, 𝑆.mems[ma] exists.
                            // 5. Let mem be the memory instance 𝑆.mems[ma]
                            let mem = store.memories.get_mut(*mem_addr).unwrap();
                            // 6. Assert: due to validation, 𝐹.module.dataaddrs[𝑥] exists.
                            // 7. Let da be the data address 𝐹.module.dataaddrs[𝑥].
                            let data_addr = instance.dataddr.get(data_index).unwrap();
                            // 8. Assert: due to validation, 𝑆.datas[da] exists.
                            // 9. Let data be the data instance 𝑆.datas[da].
                            let data = store.datas.get_mut(*data_addr).unwrap();

                            // 10. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 11. Pop the value i32.const 𝑛 from the stack.
                            let n = self.stack.pop_i32() as usize;
                            // 12. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 13. Pop the value i32.const 𝑠 from the stack
                            let s = self.stack.pop_i32() as usize;
                            // 14. Assert: due to validation, a value of value type i32 is on the top of the stack.
                            // 15. Pop the value i32.const 𝑑 from the stack
                            let d = self.stack.pop_i32() as usize;
                            let src_end = s
                                .checked_add(n)
                                .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
                            let dst_end = d
                                .checked_add(n)
                                .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
                            // 16. If 𝑠 + 𝑛 is larger than the length of data.data or 𝑑 + 𝑛 is larger than the length of mem.data, then:
                            // a. Trap.
                            if src_end > data.data.len() || dst_end > mem.bytes.len() {
                                return Err(ExecutionError::MemoryAccessOutOfBounds);
                            };
                            let data = data.load_slice(s, n)?;
                            mem.store(d, 0, &data)?;
                        }

                        MEMORY_COPY => {
                            let (dest, src) = (reader.next_byte(), reader.next_byte());

                            let number_of_bytes = self.stack.pop_i32() as usize;
                            let src_address = self.stack.pop_i32() as usize;
                            let dest_address = self.stack.pop_i32() as usize;

                            let data = {
                                let src_mem =
                                    map_memory(src as usize, instance, &mut store.memories)
                                        .unwrap();
                                src_mem.load_slice(src_address, number_of_bytes)?.to_vec()
                            };
                            let dest_mem =
                                map_memory(dest as usize, instance, &mut store.memories).unwrap();
                            dest_mem.store(dest_address, 0, &data)?;
                            trace!("MEMORY_COPY: {number_of_bytes} bytes from {src}:{src_address} to {dest}:{dest_address}");
                        }
                        MEMORY_FILL => {
                            let idx = reader.next_byte();
                            let mem =
                                map_memory(idx as usize, instance, &mut store.memories).unwrap(); // always 0
                            let number_of_bytes = self.stack.pop_i32() as usize;
                            let val = self.stack.pop_i32() as u8;

                            if number_of_bytes > isize::MAX as usize {
                                return Err(ExecutionError::MemoryAccessOutOfBounds);
                            }
                            let data = vec![val; number_of_bytes];

                            let address = self.stack.pop_i32() as usize;
                            mem.store(address, 0, &data)?;
                            trace!("MEMORY_FILL: {number_of_bytes} bytes from {address} to {val}");
                        }
                        _ => unreachable!("Unreachable due to validation"),
                    }
                }
                _ => unreachable!("Unreachable due to validation"),
            };
        }
        current_frame.program_counter = reader.current_index() - current_frame.start + 1;
        Ok(())
    }

    fn do_goto(
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

    fn load<T>(memory: &mut MemoryInstance, offset: u32, address: u32, bytes: usize) -> Result<T>
    where
        T: FromLeBytes,
    {
        let address = offset
            .checked_add(address)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)? as usize;
        memory.load(address, bytes)
    }
}
