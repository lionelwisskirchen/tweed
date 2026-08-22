use crate::{
    core::error::ExecutionError,
    execution::executor::{ExecutionContext, Executor},
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_local_get(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        trace!("LOCAL_GET({})", idx);
        let local = context
            .current_frame
            .locals
            .get(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        self.stack.push(*local)?;
        Ok(())
    }

    pub fn execute_local_set(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let local = context
            .current_frame
            .locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = self.stack.pop();
        trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
        *local = new_value;
        Ok(())
    }

    pub fn execute_local_tee(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_leb128_u32() as usize;
        let local = context
            .current_frame
            .locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = self.stack.pop();
        trace!("LOCAL_TEE({}) TO {:?}", idx, new_value);
        *local = new_value;
        self.stack.push(new_value)?;
        Ok(())
    }
}
