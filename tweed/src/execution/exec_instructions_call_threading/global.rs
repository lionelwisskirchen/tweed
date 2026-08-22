use crate::{
    core::error::ExecutionError,
    execution::executor::{ExecutionContext, Executor},
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_global_get(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        trace!("GLOBAL_GET({})", idx);
        let global_addr = context
            .instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = context.globals.get_mut(*global_addr)?;
        self.stack.push(global.value)?;
        Ok(())
    }

    pub fn execute_global_set(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let global_addr = context
            .instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = context.globals.get_mut(*global_addr)?;
        let new_value = self.stack.pop();
        trace!("GLOBAL_SET({}) TO {:?}", idx, new_value);
        global.value = new_value;
        Ok(())
    }
}
