use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader},
    execution::{executor::Executor, instances::ModuleInstance, store::GlobalStore},
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_global_get(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        globals: &mut GlobalStore,
    ) -> Result<()> {
        let idx = reader.next_leb128_u32() as usize;
        trace!("GLOBAL_GET({})", idx);
        let global_addr = instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = globals.get_mut(*global_addr)?;
        self.stack.push(global.value)?;
        Ok(())
    }

    pub fn execute_global_set(
        &mut self,
        reader: &mut ValidatedWasmReader,
        instance: &ModuleInstance,
        globals: &mut GlobalStore,
    ) -> Result<()> {
        let idx = reader.next_leb128_u32() as usize;
        let global_addr = instance
            .globaladdr
            .get(idx)
            .ok_or(ExecutionError::UnknownGlobal)?;
        let global = globals.get_mut(*global_addr)?;
        let new_value = self.stack.pop();
        trace!("GLOBAL_SET({}) TO {:?}", idx, new_value);
        global.value = new_value;
        Ok(())
    }
}
