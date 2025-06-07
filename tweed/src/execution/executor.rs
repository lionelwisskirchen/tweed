extern crate alloc;

use alloc::vec::Vec;
use extensions::*;

use crate::core::error::{Error, Result};
use crate::core::opcode::*;
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::{core::values::FromLeBytes, validation::side_table::SideTable, Value};

use super::instance::Instance;
use super::store::{FunctionInstance, Store};

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub start: usize,
    pub end: usize,
    pub return_value: Vec<ValType>,
    pub locals: Vec<Value>,
    pub side_table: SideTable,
    pub side_table_pointer: usize,
}
#[derive(Debug)]
pub struct Executor<'store, 'reader, 'instance> {
    instance: &'instance Instance,
    store: &'store mut Store,
    reader: &'reader mut WasmReader,
}

impl<'store, 'reader, 'instance> Executor<'store, 'reader, 'instance> {
    pub fn new(
        instance: &'instance Instance,
        store: &'store mut Store,
        reader: &'reader mut WasmReader,
    ) -> Self {
        Self {
            instance,
            store,
            reader,
        }
    }
    pub fn invoke(&mut self, index: usize) -> Result<Vec<Value>> {
        let function_idx = self
            .instance
            .funcaddr
            .get(index)
            .ok_or(Error::InvalidFunctionIndex)?;
        let function = self
            .store
            .functions
            .get(*function_idx)
            .ok_or(Error::InvalidFunctionIndex)?;
        let frame = self.build_frame(function.clone())?;
        self.store.call_stack.push(frame);
        self.execute()
    }
    fn build_frame(&mut self, function: FunctionInstance) -> Result<Frame> {
        let mut locals = self.store.stack.pop_n(function.signature.parameter.len())?;
        locals.append(&mut function.locals.clone()); // Locals = Params + Locals
        Ok(Frame {
            program_counter: 0,
            stack_pointer: self.store.stack.len(),
            start: function.start,
            end: function.end,
            return_value: function.signature.result.clone(),
            locals,
            side_table: function.side_table.clone(),
            side_table_pointer: 0,
        })
    }
    fn execute(&mut self) -> Result<Vec<Value>> {
        loop {
            let Some(mut current_frame) = self.store.call_stack.pop() else {
                return self.store.stack.pop_n(self.store.stack.len());
            };
            self.execute_frame(&mut current_frame)?;
        }
    }

    fn skip_block_type(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }
    fn skip_depth(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }

    fn do_goto(&mut self, current_frame: &mut Frame) -> Result<()> {
        let side_table_entry = current_frame
            .side_table
            .entries
            .get(current_frame.side_table_pointer)
            .ok_or(Error::InvalidSidetableIndex)?;
        let pos = self.reader.current_index() as i32 + side_table_entry.ip;
        current_frame.side_table_pointer = current_frame
            .side_table_pointer
            .saturating_add_signed(side_table_entry.stp as isize);
        self.reader.seek(pos as usize);
        let saved_vals = self.store.stack.pop_n(side_table_entry.vals)?;
        self.store.stack.pop_n(side_table_entry.pop)?;
        self.store.stack.push_vec(&saved_vals);
        Ok(())
    }

    fn execute_frame(&mut self, current_frame: &mut Frame) -> Result<()> {
        self.reader
            .seek(current_frame.start + current_frame.program_counter);
        while self.reader.current_index() < current_frame.end {
            let instruction = self
                .reader
                .next_byte()
                .ok_or(Error::InvalidSectionBytecode)?;
            match *instruction {
                I32_CONST => {
                    let value = self.reader.next_leb128_i32().unwrap();
                    trace!("I32_CONST({})", value);
                    self.store.stack.push(Value::I32(value));
                }
                I64_CONST => {
                    let value = self.reader.next_leb128_i64().unwrap();
                    trace!("I64_CONST({})", value);
                    self.store.stack.push(Value::I64(value));
                }
                BLOCK => {
                    trace!("BLOCK");
                    self.skip_block_type()?;
                }
                LOOP => {
                    trace!("LOOP");
                    self.skip_block_type()?;
                }
                IF => {
                    let bool_val = self.store.stack.pop()?;
                    trace!("IF({:?})", bool_val);
                    if bool_val == Value::I32(0) {
                        self.do_goto(current_frame)?;
                    } else {
                        self.skip_block_type()?;
                        current_frame.side_table_pointer += 1;
                    }
                }
                ELSE => {
                    trace!("ELSE");
                    self.do_goto(current_frame)?;
                }
                LOCAL_GET => {
                    let idx = self.reader.next_leb128_u32().unwrap() as usize;
                    trace!("LOCAL_GET({})", idx);
                    let local = current_frame
                        .locals
                        .get(idx)
                        .ok_or(Error::InvalidLocalIndex)?;
                    self.store.stack.push(*local);
                }
                LOCAL_SET => {
                    let idx = self.reader.next_leb128_u32().unwrap() as usize;
                    let local = current_frame
                        .locals
                        .get_mut(idx)
                        .ok_or(Error::InvalidLocalIndex)?;
                    let new_value = self.store.stack.pop()?;
                    trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
                    *local = new_value;
                }
                GLOBAL_GET => {
                    let idx = self.reader.next_leb128_u32().unwrap() as usize;
                    trace!("GLOBAL_GET({})", idx);
                    let global_addr = self
                        .instance
                        .globaladdr
                        .get(idx)
                        .ok_or(Error::UnknownGlobal)?;
                    let global = self
                        .store
                        .globals
                        .get_mut(*global_addr)
                        .ok_or(Error::UnknownGlobal)?;
                    self.store.stack.push(global.value);
                }
                GLOBAL_SET => {
                    let idx = self.reader.next_leb128_u32().unwrap() as usize;
                    let global_addr = self
                        .instance
                        .globaladdr
                        .get(idx)
                        .ok_or(Error::UnknownGlobal)?;
                    let global = self
                        .store
                        .globals
                        .get_mut(*global_addr)
                        .ok_or(Error::UnknownGlobal)?;
                    let new_value = self.store.stack.pop()?;
                    trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
                    global.value = new_value;
                }
                LOCAL_TEE => {
                    let idx = self.reader.next_leb128_u32().unwrap() as usize;
                    let local = current_frame
                        .locals
                        .get_mut(idx)
                        .ok_or(Error::InvalidLocalIndex)?;
                    let new_value = self.store.stack.pop()?;
                    trace!("LOCAL_TEE({}) TO {:?}", idx, new_value);
                    *local = new_value;
                    self.store.stack.push(new_value);
                }
                DROP => {
                    let val = self.store.stack.pop()?;
                    trace!("DROP {:?}", val);
                }
                NOP => {
                    trace!("NOP");
                }
                I32_CTZ => {
                    let val = self.store.stack.pop_i32()?;
                    trace!("I32_CTZ({val})");
                    let result = val.trailing_zeros() as i32;
                    self.store.stack.push(result.into());
                }
                I64_CTZ => {
                    let val = self.store.stack.pop_i64()?;
                    trace!("I64_CTZ({val})");
                    let result = val.trailing_zeros() as i64;
                    self.store.stack.push(result.into());
                }
                I64_ADD => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_ADD({left}, {right})");
                    let result = right.wrapping_add(left);
                    self.store.stack.push(result.into());
                }
                I64_SUB => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_SUB({left}, {right})");
                    let result = left.wrapping_sub(right);
                    self.store.stack.push(result.into());
                }
                I64_MUL => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_MUL({left}, {right})");
                    let result = left.wrapping_mul(right);
                    self.store.stack.push(result.into());
                }
                I64_AND => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_AND({left}, {right})");

                    let result = right & left;
                    self.store.stack.push(result.into());
                }
                I64_OR => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_OR({left}, {right})");

                    let result = right | left;
                    self.store.stack.push(result.into());
                }
                I64_XOR => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_XOR({left}, {right})");

                    let result = right ^ left;
                    self.store.stack.push(result.into());
                }
                I64_SHR_U => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_SHR_U({left}, {right})");

                    let result = (left as u64).wrapping_shr((right & 63) as u32) as i64;
                    self.store.stack.push(result.into());
                }
                I64_SHR_S => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_SHR_S({left}, {right})");
                    let result = left.wrapping_shr((right & 63) as u32);
                    self.store.stack.push(result.into());
                }
                I64_SHL => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_SHL({left}, {right})");
                    let result = left.wrapping_shl((right & 63) as u32);
                    self.store.stack.push(result.into());
                }
                I32_SHR_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_SHR_S({left}, {right})");
                    let result = left.wrapping_shr((right) as u32);
                    self.store.stack.push(result.into());
                }
                I32_SHR_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_SHR_U({left}, {right})");
                    let result = (left as u32).wrapping_shr((right) as u32) as i32;
                    self.store.stack.push(result.into());
                }
                I32_SHL => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_SHL({left}, {right})");
                    let result = left.wrapping_shl((right) as u32);
                    self.store.stack.push(result.into());
                }
                I32_ADD => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_ADD({left}, {right})");
                    let result = right.wrapping_add(left);
                    self.store.stack.push(result.into());
                }
                I32_MUL => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_MUL({left}, {right})");
                    let result = right.wrapping_mul(left);
                    self.store.stack.push(result.into());
                }
                I32_DIV_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_DIV_S({left}, {right})");
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    if left == i32::MIN && right == -1 {
                        return Err(Error::IntegerOverflow);
                    }
                    self.store.stack.push((left / right).into());
                }
                I32_DIV_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_DIV_U({left}, {right})");
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.store
                        .stack
                        .push(((left as u32) / (right as u32)).into());
                }
                I32_REM_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_REM_S({left}, {right})");

                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let result = left.checked_rem(right);
                    self.store.stack.push(result.unwrap_or_default().into());
                }
                I32_REM_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_REM_U({left}, {right})");

                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let result = (left as u32).checked_rem(right as u32);
                    self.store.stack.push(result.unwrap_or_default().into());
                }
                I32_SUB => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_SUB({left}, {right})");
                    let result = left.wrapping_sub(right);
                    self.store.stack.push(result.into());
                }
                I32_EQZ => {
                    let val = self.store.stack.pop_i32()?;
                    trace!("I32_EQZ({val})");
                    if val == 0 {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }

                I32_EQ => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_EQ({left}, {right})");
                    if right == left {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }

                I32_CLZ => {
                    let val = self.store.stack.pop_i32()?;
                    trace!("I32_CLZ({val})");
                    self.store.stack.push(val.leading_zeros().into());
                }
                I32_LT_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_LT_S({left}, {right})");
                    if left < right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_LE_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_LE_S({left}, {right})");
                    if left <= right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_LT_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_LT_U({left}, {right})");
                    if (left as u32) < (right as u32) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_LE_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_LE_U({left}, {right})");
                    if (left as u32) <= (right as u32) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_GT_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_GT_U({left}, {right})");
                    if (left as u32) > (right as u32) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_GE_U => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_GT_U({left}, {right})");
                    if (left as u32) >= (right as u32) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_XOR => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_XOR({left}, {right})");
                    let result = right ^ left;
                    self.store.stack.push(result.into());
                }

                I32_NE => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_NE({left}, {right})");
                    if left != right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_ROTR => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    let result = left.rotate_right(right as u32);
                    trace!("I32_ROTR [{left} {right}] -> [{result}]");
                    self.store.stack.push(Value::I32(result));
                }
                I32_ROTL => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    let result = left.rotate_left(right as u32);
                    trace!("I32_ROTL [{left} {right}] -> [{result}]");
                    self.store.stack.push(Value::I32(result));
                }
                I32_WRAP_I64 => {
                    let value = self.store.stack.pop_i64()?;
                    trace!("I32_WRAP_I64({value})");
                    self.store.stack.push((value as i32).into())
                }
                I32_EXTEND8_S => {
                    let value = self.store.stack.pop_i32()?;
                    trace!("I32_EXTEND8_S({value})");
                    let extended = (value as i8) as i32;
                    self.store.stack.push(extended.into())
                }
                I32_EXTEND16_S => {
                    let value = self.store.stack.pop_i32()?;
                    trace!("I32_EXTEND16_S({value})");
                    let extended = (value as i16) as i32;
                    self.store.stack.push(extended.into())
                }
                I64_EXTEND_I32_U => {
                    let value = self.store.stack.pop_i32()?;
                    trace!("I64_EXTEND_I32_U({value})");
                    self.store.stack.push((value as u32 as i64).into())
                }
                I64_EXTEND_I32_S => {
                    let value = self.store.stack.pop_i32()?;
                    trace!("I64_EXTEND_I32_S({value})");
                    self.store.stack.push((value as i64).into())
                }
                I64_GE_U => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_GE_U({left}, {right})");
                    if (left as u64) >= (right as u64) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I64_LT_U => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_LT_U({left}, {right})");
                    if (left as u64) < (right as u64) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I64_GT_U => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_GT_U({left}, {right})");
                    if (left as u64) > (right as u64) {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I64_EQ => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_EQ({left}, {right})");
                    if left == right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I64_NE => {
                    let (right, left) = (self.store.stack.pop_i64()?, self.store.stack.pop_i64()?);
                    trace!("I64_NE({left}, {right})");
                    if left != right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_GT_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I64_GT_S({left}, {right})");
                    if left > right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_GE_S => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_GE_S({left}, {right})");
                    if left >= right {
                        self.store.stack.push(Value::I32(1));
                    } else {
                        self.store.stack.push(Value::I32(0));
                    }
                }
                I32_AND => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_AND({left}, {right})");
                    let result = right & left;
                    self.store.stack.push(Value::I32(result));
                }
                I32_OR => {
                    let (right, left) = (self.store.stack.pop_i32()?, self.store.stack.pop_i32()?);
                    trace!("I32_OR({left}, {right})");
                    let result = right | left;
                    self.store.stack.push(Value::I32(result));
                }
                SELECT => {
                    let (cond, b, a) = (
                        self.store.stack.pop_i32()?,
                        self.store.stack.pop()?,
                        self.store.stack.pop()?,
                    );
                    trace!("SELECT({cond} ? {a:?} : {b:?})");
                    if cond == 0 {
                        self.store.stack.push(b);
                    } else {
                        self.store.stack.push(a);
                    }
                }
                BR => {
                    trace!("BR");
                    self.do_goto(current_frame)?;
                }
                BR_IF => {
                    let bool_val = self.store.stack.pop_i32()?;
                    trace!("BR_IF({bool_val})");
                    if bool_val == 0 {
                        self.skip_depth()?;
                        current_frame.side_table_pointer += 1;
                    } else {
                        self.do_goto(current_frame)?;
                    }
                }
                BR_TABLE => {
                    let branch_index = self.store.stack.pop_i32()?;
                    trace!("BR_TABLE({branch_index})");
                    let number_of_branches =
                        self.reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)? as usize;
                    if branch_index as usize >= number_of_branches {
                        current_frame.side_table_pointer += number_of_branches;
                    } else {
                        current_frame.side_table_pointer += branch_index as usize;
                    }
                    self.do_goto(current_frame)?;
                }
                RETURN => {
                    trace!("RETURN");
                    let count = current_frame.return_value.len();
                    let results = self.store.stack.pop_n(count)?;
                    self.store.stack.unwind_to(current_frame.stack_pointer);
                    self.store.stack.push_vec(&results);
                    break;
                }
                END => {
                    trace!("END");
                    if self.reader.current_index() == current_frame.end {
                        break;
                    }
                }
                CALL => {
                    let index = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidFunctionIndex)?;
                    trace!("CALL({index})");
                    let function_addr = self
                        .instance
                        .funcaddr
                        .get(index as usize)
                        .ok_or(Error::InvalidFunctionIndex)?;
                    let function = self
                        .store
                        .functions
                        .get(*function_addr)
                        .ok_or(Error::InvalidFunctionIndex)?;

                    // TODO External/internal functions
                    let frame = self.build_frame(function.clone())?;
                    current_frame.program_counter =
                        self.reader.current_index() - current_frame.start + 1;
                    self.store.call_stack.push(current_frame.clone());
                    self.store.call_stack.push(frame);
                    break;
                }
                I64_STORE => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i64()? as u64).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I64_STORE({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I64_STORE8 => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i64()? as u8).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I64_STORE8({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I64_STORE16 => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i64()? as u16).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I64_STORE16({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I64_STORE32 => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i64()? as u32).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I64_STORE32({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I64_LOAD => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: i64 = self.load(offset, address, 8)?;
                    trace!("I64_LOAD({data:?} from {address:?})");

                    self.store.stack.push(data.into());
                }

                I32_STORE => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i32()? as u32).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I32_STORE({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I32_STORE8 => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i32()? as u8).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I32_STORE8({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I32_STORE16 => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.store.stack.pop_i32()? as u16).to_le_bytes();
                    let rel_address = self.store.stack.pop_i32()? as u32;
                    trace!("I32_STORE16({store_data:?} to {rel_address:?})");

                    self.store(offset, &store_data, rel_address)?;
                }
                I32_LOAD => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: i32 = self.load(offset, address, 4)?;
                    trace!("I32_LOAD({data:?} from {address:?})");

                    self.store.stack.push(data.into());
                }
                I32_LOAD8_U => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: u8 = self.load(offset, address, 1)?;
                    trace!("I32_LOAD8_U({data:?} from {address:?})");
                    self.store.stack.push((data as u32).into());
                }
                I32_LOAD16_U => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: u16 = self.load(offset, address, 2)?;
                    trace!("I32_LOAD16_U({data:?} from {address:?})");
                    self.store.stack.push((data as u32).into());
                }
                I32_LOAD16_S => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: i16 = self.load(offset, address, 2)?;
                    trace!("I32_LOAD16_S({data:?} from {address:?})");
                    self.store.stack.push((data as u32).into());
                }
                I32_LOAD8_S => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: i8 = self.load(offset, address, 1)?;
                    trace!("I32_LOAD8_S({data:?} from {address:?})");
                    self.store.stack.push((data as u32).into());
                }
                I64_LOAD32_S => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.store.stack.pop_i32()? as u32;
                    let data: i32 = self.load(offset, address, 4)?;
                    trace!("I64_LOAD32_S({data:?} from {address:?})");
                    self.store.stack.push((data as i64).into());
                }
                TABLE_GET => {
                    let table_idx =
                        self.reader.next_leb128_u32().ok_or(Error::MissingValue)? as usize;
                    trace!("TABLE_GET '{table_idx:?}'");
                    trace!("Instance: '{:?}'", self.instance);

                    let table_addr = self
                        .instance
                        .tableaddr
                        .get(table_idx)
                        .ok_or(Error::TableAccessOutOfBounds)?;
                    let offset = self.store.stack.pop_i32()? as usize;
                    let table = self.store.tables.get_mut(*table_addr).unwrap(); // validated
                    let value = table.get(offset)?;
                    self.store.stack.push(value);
                    trace!("TABLE_GET '{}' [{}] -> [{:?}]", table_idx, offset, value);
                }
                TABLE_SET => {
                    let table_idx =
                        self.reader.next_leb128_u32().ok_or(Error::MissingValue)? as usize;
                    let table_addr = self
                        .instance
                        .tableaddr
                        .get(table_idx)
                        .ok_or(Error::TableAccessOutOfBounds)?;
                    let table = self.store.tables.get_mut(*table_addr).unwrap(); // validated
                    let val = self.store.stack.pop()?;
                    let index = self.store.stack.pop_i32()? as usize;
                    table.insert(index, &[val])?;
                    trace!("TABLE_SET '{}' [{} {:?}] -> []", table_idx, index, val)
                }

                REF_NULL => {
                    let ref_type = crate::core::value_types::RefType::from_byte(
                        *self.reader.next_byte().ok_or(Error::MissingValue)?,
                    )
                    .ok_or(Error::UnexpectedValueType)?;
                    self.store.stack.push(Value::RefNull(ref_type));
                }
                REF_IS_NULL => {
                    let value = self.store.stack.pop()?;
                    match value {
                        Value::RefNull(_) => self.store.stack.push(Value::I32(1)),
                        _ => self.store.stack.push(Value::I32(0)),
                    }
                }
                REF_FUNC => {
                    let func_idx =
                        self.reader.next_leb128_u32().ok_or(Error::MissingValue)? as usize;
                    self.store.stack.push(Value::FuncRef(func_idx));
                }
                EXTENSION_PREFIX => {
                    let byte = *self.reader.next_byte().ok_or(Error::InvalidControlFlow)?;
                    match byte {
                        MEMORY_COPY => {
                            let (dest, src) = (
                                *self.reader.next_byte().unwrap(), // validated and 0
                                *self.reader.next_byte().unwrap(), // validated and 0
                            );
                            let src_mem_addr = self
                                .instance
                                .memaddr
                                .get(src as usize)
                                .ok_or(Error::InvalidMemoryIndex)?;
                            let dest_mem_addr = self
                                .instance
                                .memaddr
                                .get(dest as usize)
                                .ok_or(Error::InvalidMemoryIndex)?;

                            let number_of_bytes = self.store.stack.pop_i32()? as usize;
                            let src_address = self.store.stack.pop_i32()? as usize;
                            let dest_address = self.store.stack.pop_i32()? as usize;
                            let data = {
                                let src_mem = self.store.memories.get_mut(*src_mem_addr).unwrap();
                                src_mem.load_slice(src_address, number_of_bytes)?.to_vec()
                            };
                            let dest_mem = self.store.memories.get_mut(*dest_mem_addr).unwrap();
                            dest_mem.store(dest_address, &data)?;
                            trace!("MEMORY_COPY: {number_of_bytes} bytes from {src}:{src_address} to {dest}:{dest_address}");
                        }
                        MEMORY_FILL => {
                            let idx = *self.reader.next_byte().unwrap(); // validated
                            let mem_addr = self
                                .instance
                                .memaddr
                                .get(idx as usize)
                                .ok_or(Error::InvalidMemoryIndex)?;
                            let mem = self.store.memories.get_mut(*mem_addr).unwrap(); // always 0
                            let number_of_bytes = self.store.stack.pop_i32()? as usize;
                            let val = self.store.stack.pop_i32()? as u8;
                            let data = vec![val; number_of_bytes];
                            let address = self.store.stack.pop_i32()? as usize;
                            mem.store(address, &data)?;
                            trace!("MEMORY_FILL: {number_of_bytes} bytes from {address} to {val}");
                        }
                        _ => return Err(Error::UndefinedOpCode(byte)),
                    }
                }
                _ => return Err(Error::UndefinedInstruction(*instruction)),
            };
        }
        current_frame.program_counter = self.reader.current_index() - current_frame.start + 1;
        Ok(())
    }
    fn store(&mut self, offset: u32, data: &[u8], address: u32) -> Result<()> {
        let mem_addr = self
            .instance
            .memaddr
            .first() // only one memory allowed
            .ok_or(Error::InvalidMemoryIndex)?;
        let mem = self.store.memories.get_mut(*mem_addr).unwrap();
        let address = offset
            .checked_add(address)
            .ok_or(Error::MemoryAccessOutOfBounds)? as usize;
        mem.store(address, data)?;
        Ok(())
    }

    fn load<T>(&mut self, offset: u32, address: u32, bytes: usize) -> Result<T>
    where
        T: FromLeBytes,
    {
        let mem_addr = self
            .instance
            .memaddr
            .first() // only one memory allowed
            .ok_or(Error::InvalidMemoryIndex)?;
        let mem = self.store.memories.get_mut(*mem_addr).unwrap();
        let address = offset
            .checked_add(address)
            .ok_or(Error::MemoryAccessOutOfBounds)? as usize;
        mem.load(address, bytes)
    }
    fn read_mem_args(&mut self) -> Result<(u32, u32)> {
        let _align = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
        let offset = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
        Ok((_align, offset))
    }
}
