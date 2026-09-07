use crate::{
    core::error::ExecutionError,
    execution::{
        adresses::Address,
        exec_instructions_tail_call::execute_next_instruction::DISPATCH_TABLE,
        executor::{ExecutionContext, Executor},
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_table_get(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32() as usize;
        trace!("TABLE_GET '{table_idx:?}'");

        let table_addr = context
            .instance
            .tableaddr
            .get(table_idx)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        let offset = executor.stack.pop_i32() as usize;
        let table = context.tables.get_mut(*table_addr).unwrap(); // validated
        let value = table.get(offset)?;
        executor.stack.push(value)?;
        trace!("TABLE_GET '{}' [{}] -> [{:?}]", table_idx, offset, value);

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_set(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32() as usize;

        let table_addr = context
            .instance
            .tableaddr
            .get(table_idx)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        let table = context.tables.get_mut(*table_addr).unwrap(); // validated
        let val = executor.stack.pop();
        let index = executor.stack.pop_i32() as usize;
        table.insert(index, &[val])?;
        trace!("TABLE_SET '{}' [{} {:?}] -> []", table_idx, index, val);

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_init(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let element_idx = context.reader.next_leb128_u32() as usize;
        let table_idx = context.reader.next_leb128_u32();

        // 2. Assert: due to validation, 𝐹.module.tableaddrs[𝑥] exists.
        // 3. Let ta be the table address 𝐹.module.tableaddrs[𝑥]
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap();
        // 4. Assert: due to validation, 𝑆.tables[ta] exists.
        // 5. Let tab be the table instance 𝑆.tables[ta]
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated

        // 6. Assert: due to validation, 𝐹.module.elemaddrs[𝑦] exists
        // 7. Let ea be the element address 𝐹.module.elemaddrs[𝑦].
        let element_addr = context.instance.elemaddr.get(element_idx).unwrap();
        // 8. Assert: due to validation, 𝑆.elems[ea] exists.
        // 9. Let elem be the element instance 𝑆.elems[ea]
        let elem = context.elements.get(*element_addr).unwrap(); // validated

        // 10. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 11. Pop the value i32.const 𝑛 from the stack.
        let n = executor.stack.pop_i32() as usize;
        // 12. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 13. Pop the value i32.const 𝑠 from the stack.
        let s = executor.stack.pop_i32() as usize;
        // 14. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 15. Pop the value i32.const 𝑑 from the stack.
        let d = executor.stack.pop_i32() as usize;

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

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_size(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated
        info!("TABLE_SIZE {:?}", tab.reference_count());

        executor
            .stack
            .push(Value::I32(tab.reference_count() as i32))?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_copy(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx_x = context.reader.next_leb128_u32(); // Destination
        let table_idx_y = context.reader.next_leb128_u32(); // Source
        let table_x_addr = context
            .instance
            .tableaddr
            .get(table_idx_x as usize)
            .unwrap();
        let table_y_addr = context
            .instance
            .tableaddr
            .get(table_idx_y as usize)
            .unwrap();

        let n = executor.stack.pop_i32() as usize; // size
        let s = executor.stack.pop_i32() as usize; // source
        let d = executor.stack.pop_i32() as usize; // destination

        let src_end = s
            .checked_add(n)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;

        let dst_end = d
            .checked_add(n)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        let src_data = {
            let table_y = context.tables.get(*table_y_addr).unwrap();
            let len = table_y.reference_values.len();
            if src_end > len {
                return Err(ExecutionError::TableAccessOutOfBounds);
            };

            let data = table_y.get_slice(s, n)?.to_vec();
            data
        };

        let table_x = context.tables.get_mut(*table_x_addr).unwrap(); // validated
        if dst_end > table_x.reference_values.len() {
            return Err(ExecutionError::TableAccessOutOfBounds);
        }
        table_x.insert(d, &src_data)?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_fill(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated
        let n = executor.stack.pop_i32() as usize; // size
        let val = executor.stack.pop();
        let i = executor.stack.pop_i32() as usize; // dest
        let end = i
            .checked_add(n)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        if end > tab.reference_count() {
            return Err(ExecutionError::TableAccessOutOfBounds);
        }
        tab.fill(i, end, val)?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_table_grow(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated

        let sz = tab.reference_count();

        let n = executor.stack.pop_i32() as usize; // size
        let val = executor.stack.pop();

        let lens = sz.checked_add(n);
        let Some(len) = lens else {
            executor.stack.push(Value::I32(u32::MAX as i32))?;

            let instruction = context.reader.next_byte() as usize;
            become DISPATCH_TABLE[instruction](executor, context);
        };

        if let Some(max) = tab.table_type.limit.max {
            if len > max as usize {
                executor.stack.push(Value::I32(u32::MAX as i32))?;

                let instruction = context.reader.next_byte() as usize;
                become DISPATCH_TABLE[instruction](executor, context);
            }
        }
        tab.grow(n, val)?;
        trace!("GROW n:{:?}, val:{:?}", n, val);

        tab.table_type.limit.min = len as u32;
        executor.stack.push(Value::I32(sz as i32))?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_ref_null(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let ref_type = crate::core::value_types::RefType::from_byte(context.reader.next_byte())
            .ok_or(ExecutionError::UnexpectedValueType)?;
        executor.stack.push(Value::RefNull(ref_type))?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }
    pub fn execute_ref_func(executor: &mut Executor, context: &mut ExecutionContext) -> Result<()> {
        let func_idx = context.reader.next_leb128_u32() as usize;
        let store_addr = context.instance.funcaddr.get(func_idx).unwrap(); // Validated
        executor.stack.push(Value::FuncRef(store_addr.as_usize()))?;

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_ref_is_null(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let value = executor.stack.pop();
        match value {
            Value::RefNull(_) => executor.stack.push(Value::I32(1))?,
            _ => executor.stack.push(Value::I32(0))?,
        }

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }

    pub fn execute_elem_drop(
        executor: &mut Executor,
        context: &mut ExecutionContext,
    ) -> Result<()> {
        let element_idx = context.reader.next_leb128_u32() as usize;
        // 2. Assert: due to validation, 𝐹.module.elemaddrs[𝑥] exists.
        // 3. Let 𝑎 be the element address 𝐹.module.elemaddrs[𝑥].
        let element_addr = context.instance.elemaddr.get(element_idx).unwrap();
        // 4. Assert: due to validation, 𝑆.elems[𝑎] exists.
        let elem = context.elements.get_mut(*element_addr).unwrap();
        // Replace 𝑆.elems[𝑎].elem with 𝜖.
        elem.references.clear();

        let instruction = context.reader.next_byte() as usize;
        become DISPATCH_TABLE[instruction](executor, context);
    }
}
