use crate::{
    core::error::ExecutionError,
    execution::{
        adresses::FuncAddr,
        executor::{ExecutionContext, Executor},
        instances::FuncInstance,
        store::HostEnv,
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_block(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("BLOCK");
        context.reader.next_leb128_u32(); // Skip Block type

        become self.execute_next_instruction(context)
    }

    pub fn execute_loop(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("LOOP");
        context.reader.next_leb128_u32(); // Skip Block type

        become self.execute_next_instruction(context)
    }

    pub fn execute_if(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let bool_val = self.stack.pop();
        trace!("IF({:?})", bool_val);
        if bool_val == Value::I32(0) {
            Self::do_goto(&mut self.stack, context.current_frame, context.reader)?;
        } else {
            context.reader.next_leb128_u32(); // Skip Block type
            context.current_frame.side_table_pointer += 1;
        }

        become self.execute_next_instruction(context)
    }

    pub fn execute_else(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("ELSE");
        Self::do_goto(&mut self.stack, context.current_frame, context.reader)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_select(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (cond, b, a) = (self.stack.pop_i32(), self.stack.pop(), self.stack.pop());
        trace!("SELECT({cond} ? {a:?} : {b:?})");
        if cond == 0 {
            self.stack.push(b)?;
        } else {
            self.stack.push(a)?;
        }

        become self.execute_next_instruction(context)
    }

    pub fn execute_select_t(&mut self, context: &mut ExecutionContext) -> Result<()> {
        context.reader.next_leb128_u32();

        context.reader.next_byte();
        let c = self.stack.pop_i32();
        let val_2 = self.stack.pop(); // TODO Type check?
        let val_1 = self.stack.pop();
        trace!("SELECT_T({c})");
        if c == 0 {
            self.stack.push(val_2)?;
        } else {
            self.stack.push(val_1)?;
        }

        become self.execute_next_instruction(context)
    }

    pub fn execute_br(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("BR");
        Self::do_goto(&mut self.stack, context.current_frame, context.reader)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_br_if(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let bool_val = self.stack.pop_i32();
        trace!("BR_IF({bool_val})");

        if bool_val == 0 {
            context.reader.next_leb128_u32(); // skip depth
            context.current_frame.side_table_pointer += 1;
        } else {
            Self::do_goto(&mut self.stack, context.current_frame, context.reader)?;
        }

        become self.execute_next_instruction(context)
    }

    pub fn execute_br_table(&mut self, context: &mut ExecutionContext) -> Result<()> {
        // TODO br_table vec[Branch index] not += branch_index!
        let branch_index = self.stack.pop_i32() as usize;
        let number_of_branches = context.reader.next_leb128_u32() as usize;
        trace!("BR_TABLE({branch_index})");
        let offset = branch_index.min(number_of_branches);
        context.current_frame.side_table_pointer += offset;

        Self::do_goto(&mut self.stack, context.current_frame, context.reader)?;

        become self.execute_next_instruction(context)
    }

    pub fn execute_return(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("RETURN");
        let count = context.current_frame.return_value.len();
        let results = self.stack.pop_n(count)?;
        self.stack.unwind_to(context.current_frame.stack_pointer);
        self.stack.push_vec(&results)?;

        Ok(())
    }

    pub fn execute_call_indirect(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let type_index = context.reader.next_leb128_u32();
        let table_index = context.reader.next_leb128_u32();
        let table_addr = context
            .instance
            .tableaddr
            .get(table_index as usize)
            .ok_or(ExecutionError::UnknownTable)?;
        let table = context.tables.get(*table_addr)?;

        let func_type = context
            .instance
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
        let function = context
            .functions
            .get(index)
            .map_err(|_| ExecutionError::InvalidFunctionIndex)?;
        if function.get_signature() != func_type {
            return Err(ExecutionError::IndirectCallTypeMismatch);
        };

        match function {
            FuncInstance::Internal(instance) => {
                let frame = self.build_frame(instance)?;
                context.current_frame.program_counter =
                    context.reader.current_index() - context.current_frame.start + 1;

                self.call_stack.push(context.current_frame.clone())?;
                self.call_stack.push(frame)?;
                context.should_break = true;
                become self.exit_tail_calls(context)
            }

            FuncInstance::External(host_function) => {
                let mut env: HostEnv = HostEnv {
                    memories: context.memories.items(),
                    globals: context.globals.items(),
                };
                let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                let mut func_ref = host_function.func.borrow_mut();
                let results = func_ref(&mut env, args)?;
                self.stack.push_vec(&results)?;
            }
        }
        Ok(())
    }

    pub fn execute_call(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let index = context.reader.next_leb128_u32();
        trace!("CALL({index})");

        // funcaddr contains the indices of the functions on the store
        // | [0] StoreIdx = 2 | [1] StoreIdx = 3 | [2] StoreIdx = 6 | [3] StoreIdx = x |
        // so if want to call the function with index 0 we actually call the second
        // function in the store
        let function_addr = context
            .instance
            .funcaddr
            .get(index as usize)
            .ok_or(ExecutionError::InvalidFunctionIndex)?;

        let function = context
            .functions
            .get(*function_addr)
            .map_err(|_| ExecutionError::InvalidFunctionIndex)?;

        match function {
            FuncInstance::Internal(instance) => {
                let frame = self.build_frame(instance)?;
                context.current_frame.program_counter =
                    context.reader.current_index() - context.current_frame.start + 1;
                self.call_stack.push(context.current_frame.clone())?;
                self.call_stack.push(frame)?;
                context.should_break = true;
                become self.exit_tail_calls(context)
            }

            FuncInstance::External(host_function) => {
                let mut env: HostEnv = HostEnv {
                    memories: context.memories.items(),
                    globals: context.globals.items(),
                };
                let args = self.stack.pop_n(host_function.signature.parameter.len())?;
                let mut func_ref = host_function.func.borrow_mut();
                let results = func_ref(&mut env, args)?;
                self.stack.push_vec(&results)?;
            }
        }

        become self.execute_next_instruction(context)
    }

    pub fn execute_nop(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("NOP");

        become self.execute_next_instruction(context)
    }

    pub fn execute_end(&mut self, context: &mut ExecutionContext) -> Result<()> {
        trace!("END");
        if context.reader.current_index() == context.current_frame.end {
            return Ok(());
        }
        become self.execute_next_instruction(context)
    }

    pub fn execute_drop(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let val = self.stack.pop();
        trace!("DROP {:?}", val);

        become self.execute_next_instruction(context)
    }

    pub fn execute_unreachable(
        _executor: &mut Executor,
        _context: &mut ExecutionContext,
    ) -> Result<()> {
        return Err(ExecutionError::TrapUnreachable);
    }
}
