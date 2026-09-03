use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader},
    execution::executor::Executor,
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_local_get(
        &mut self,
        reader: &mut ValidatedWasmReader,
        locals: &[Value],
    ) -> Result<()> {
        let idx = reader.next_leb128_u32() as usize;
        trace!("LOCAL_GET({})", idx);
        let local = locals.get(idx).ok_or(ExecutionError::InvalidLocalIndex)?;
        self.stack.push(*local)?;
        Ok(())
    }

    pub fn execute_local_set(
        &mut self,
        reader: &mut ValidatedWasmReader,
        locals: &mut [Value],
    ) -> Result<()> {
        let idx = reader.next_leb128_u32() as usize;
        let local = locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = self.stack.pop();
        trace!("LOCAL_SET({}) TO {:?}", idx, new_value);
        *local = new_value;
        Ok(())
    }

    pub fn execute_local_tee(
        &mut self,
        reader: &mut ValidatedWasmReader,
        locals: &mut [Value],
    ) -> Result<()> {
        let idx = reader.next_leb128_u32() as usize;
        let local = locals
            .get_mut(idx)
            .ok_or(ExecutionError::InvalidLocalIndex)?;
        let new_value = self.stack.pop();
        trace!("LOCAL_TEE({}) TO {:?}", idx, new_value);
        *local = new_value;
        self.stack.push(new_value)?;
        Ok(())
    }
}
