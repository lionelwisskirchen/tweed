use crate::{
    core::error::ExecutionError,
    execution::{
        exec_instructions_tail_call::execute_next_instruction::DISPATCH_TABLE,
        executor::{ExecutionContext, Executor},
    },
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_global_get(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        trace!("GLOBAL_GET({})", idx);
        let global_addr = context
            .instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = context.globals.get_mut(*global_addr)?;
        executor.stack.push(global.value)?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_global_set(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let global_addr = context
            .instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = context.globals.get_mut(*global_addr)?;
        let new_value = executor.stack.pop();
        trace!("GLOBAL_SET({}) TO {:?}", idx, new_value);
        global.value = new_value;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }
}
