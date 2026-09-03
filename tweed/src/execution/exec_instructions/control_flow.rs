use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader},
    execution::{
        adresses::FuncAddr,
        executor::{Executor, Frame},
        instances::{FuncInstance, ModuleInstance},
        store::{FunctionStore, GlobalStore, HostEnv, MemoryStore, TableStore},
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_block(&mut self, reader: &mut ValidatedWasmReader) -> Result<()> {
        trace!("BLOCK");
        reader.next_leb128_u32(); // Skip Block type
        Ok(())
    }

    pub fn execute_loop(&mut self, reader: &mut ValidatedWasmReader) -> Result<()> {
        trace!("LOOP");
        reader.next_leb128_u32(); // Skip Block type
        Ok(())
    }

    pub fn execute_if(
        &mut self,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
    ) -> Result<()> {
        let bool_val = self.stack.pop();
        trace!("IF({:?})", bool_val);
        if bool_val == Value::I32(0) {
            Self::do_goto(&mut self.stack, current_frame, reader)?;
        } else {
            reader.next_leb128_u32(); // Skip Block type
            current_frame.side_table_pointer += 1;
        }
        Ok(())
    }

    pub fn execute_else(
        &mut self,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
    ) -> Result<()> {
        trace!("ELSE");
        Self::do_goto(&mut self.stack, current_frame, reader)?;
        Ok(())
    }

    pub fn execute_select(&mut self) -> Result<()> {
        let (cond, b, a) = (self.stack.pop_i32(), self.stack.pop(), self.stack.pop());
        trace!("SELECT({cond} ? {a:?} : {b:?})");
        if cond == 0 {
            self.stack.push(b)?;
        } else {
            self.stack.push(a)?;
        }

        Ok(())
    }

    pub fn execute_select_t(&mut self, reader: &mut ValidatedWasmReader) -> Result<()> {
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

        Ok(())
    }

    pub fn execute_br(
        &mut self,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
    ) -> Result<()> {
        trace!("BR");
        Self::do_goto(&mut self.stack, current_frame, reader)?;
        Ok(())
    }

    pub fn execute_br_if(
        &mut self,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
    ) -> Result<()> {
        let bool_val = self.stack.pop_i32();
        trace!("BR_IF({bool_val})");

        if bool_val == 0 {
            reader.next_leb128_u32(); // skip depth
            current_frame.side_table_pointer += 1;
        } else {
            Self::do_goto(&mut self.stack, current_frame, reader)?;
        }
        Ok(())
    }

    pub fn execute_br_table(
        &mut self,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
    ) -> Result<()> {
        // TODO br_table vec[Branch index] not += branch_index!
        let branch_index = self.stack.pop_i32() as usize;
        let number_of_branches = reader.next_leb128_u32() as usize;
        trace!("BR_TABLE({branch_index})");
        let offset = branch_index.min(number_of_branches);
        current_frame.side_table_pointer += offset;

        Self::do_goto(&mut self.stack, current_frame, reader)?;
        Ok(())
    }

    pub fn execute_return(&mut self, current_frame: &mut Frame) -> Result<()> {
        trace!("RETURN");
        let count = current_frame.return_value.len();
        let results = self.stack.pop_n(count)?;
        self.stack.unwind_to(current_frame.stack_pointer);
        self.stack.push_vec(&results)?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute_call_indirect(
        &mut self,
        instance: &mut ModuleInstance,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
        tables: &TableStore,
        functions: &mut FunctionStore,
        memories: &MemoryStore,
        globals: &GlobalStore,
    ) -> Result<i32> {
        let type_index = reader.next_leb128_u32();
        let table_index = reader.next_leb128_u32();
        let table_addr = instance
            .tableaddr
            .get(table_index as usize)
            .ok_or(ExecutionError::UnknownTable)?;

        let table = tables.get(*table_addr)?;

        let func_type = instance
            .types
            .get(type_index as usize)
            .ok_or(ExecutionError::InvalidFunctionIndex)?;
        let entry_index = self.stack.pop_i32() as usize;
        let Ok(entry) = table.get(entry_index) else {
            return Err(ExecutionError::UndefinedElement);
        };
        let index = match entry {
            Value::RefNull(_) => return Err(ExecutionError::UninitializedElement(None)),
            Value::FuncRef(idx) => FuncAddr(idx),
            Value::ExternRef(_) => return Err(ExecutionError::TypeMismatch),
            _ => unreachable!(),
        };
        let function = functions
            .get(index)
            .map_err(|_| ExecutionError::InvalidFunctionIndex)?;
        if function.get_signature() != func_type {
            return Err(ExecutionError::IndirectCallTypeMismatch);
        };

        match function {
            FuncInstance::Internal(instance) => {
                let frame = self.build_frame(instance)?;
                current_frame.program_counter = reader.current_index() - current_frame.start + 1;

                self.call_stack.push(current_frame.clone())?;
                self.call_stack.push(frame)?;
                return Ok(1);
            }

            FuncInstance::External(host_function) => {
                let mut env: HostEnv = HostEnv {
                    memories: memories.items(),
                    globals: globals.items(),
                };
                let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                let mut func_ref = host_function.func.borrow_mut();
                let results = func_ref(&mut env, args)?;
                self.stack.push_vec(&results)?;
            }
        }
        Ok(0)
    }

    pub fn execute_call(
        &mut self,
        instance: &mut ModuleInstance,
        reader: &mut ValidatedWasmReader,
        current_frame: &mut Frame,
        functions: &mut FunctionStore,
        memories: &MemoryStore,
        globals: &GlobalStore,
    ) -> Result<i32> {
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

        let function = functions
            .get(*function_addr)
            .map_err(|_| ExecutionError::InvalidFunctionIndex)?;

        match function {
            FuncInstance::Internal(instance) => {
                let frame = self.build_frame(instance)?;
                current_frame.program_counter = reader.current_index() - current_frame.start + 1;
                self.call_stack.push(current_frame.clone())?;
                self.call_stack.push(frame)?;
                return Ok(1);
            }

            FuncInstance::External(host_function) => {
                let mut env: HostEnv = HostEnv {
                    memories: memories.items(),
                    globals: globals.items(),
                };
                let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                let mut func_ref = host_function.func.borrow_mut();
                let results = func_ref(&mut env, args)?;
                self.stack.push_vec(&results)?;
            }
        }
        Ok(0)
    }

    pub fn execute_nop(&self) -> Result<()> {
        trace!("NOP");
        Ok(())
    }

    pub fn execute_drop(&mut self) -> Result<()> {
        let val = self.stack.pop();
        trace!("DROP {:?}", val);
        Ok(())
    }
}
