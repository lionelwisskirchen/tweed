use crate::{
    core::error::ExecutionError,
    execution::{
        exec_instructions_tail_call::execute_next_instruction::DISPATCH_TABLE,
        executor::{ExecutionContext, Executor},
    },
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_local_get(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        trace!("LOCAL_GET({})", idx);
        let local = context
            .current_frame
            .locals
            .get(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;

        executor.stack.push(*local)?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_local_set(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let local = context
            .current_frame
            .locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = executor.stack.pop();
        trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
        *local = new_value;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_local_tee(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let local = context
            .current_frame
            .locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = executor.stack.pop();
        trace!("LOCAL_TEE({}) TO {:?}", idx, new_value);
        *local = new_value;
        executor.stack.push(new_value)?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }
}
