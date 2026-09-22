use crate::{
    core::error::ExecutionError,
    execution::{
        adresses::Address,
        executor::{ExecutionContext, Executor},
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_table_get(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32() as usize;
        trace!("TABLE_GET '{table_idx:?}'");

        let table_addr = context
            .instance
            .tableaddr
            .get(table_idx)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        let offset = self.stack.pop_i32() as usize;
        let table = context.tables.get_mut(*table_addr).unwrap(); // validated
        let value = table.get(offset)?;
        self.stack.push(value)?;
        trace!("TABLE_GET '{}' [{}] -> [{:?}]", table_idx, offset, value);
        Ok(())
    }

    pub fn execute_table_set(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32() as usize;

        let table_addr = context
            .instance
            .tableaddr
            .get(table_idx)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        let table = context.tables.get_mut(*table_addr).unwrap(); // validated
        let val = self.stack.pop();
        let index = self.stack.pop_i32() as usize;
        table.insert(index, &[val])?;
        trace!("TABLE_SET '{}' [{} {:?}] -> []", table_idx, index, val);
        Ok(())
    }

    pub fn execute_table_init(&mut self, context: &mut ExecutionContext) -> Result<()> {
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
        let n = self.stack.pop_i32() as usize;
        // 12. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 13. Pop the value i32.const 𝑠 from the stack.
        let s = self.stack.pop_i32() as usize;
        // 14. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 15. Pop the value i32.const 𝑑 from the stack.
        let d = self.stack.pop_i32() as usize;

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

        Ok(())
    }

    pub fn execute_table_size(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated
        info!("TABLE_SIZE {:?}", tab.reference_count());

        self.stack.push(Value::I32(tab.reference_count() as i32))?;
        Ok(())
    }

    pub fn execute_table_copy(&mut self, context: &mut ExecutionContext) -> Result<()> {
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

        let n = self.stack.pop_i32() as usize; // size
        let s = self.stack.pop_i32() as usize; // source
        let d = self.stack.pop_i32() as usize; // destination

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
        Ok(())
    }

    pub fn execute_table_fill(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated
        let n = self.stack.pop_i32() as usize; // size
        let val = self.stack.pop();
        let i = self.stack.pop_i32() as usize; // dest
        let end = i
            .checked_add(n)
            .ok_or(ExecutionError::TableAccessOutOfBounds)?;
        if end > tab.reference_count() {
            return Err(ExecutionError::TableAccessOutOfBounds);
        }
        tab.fill(i, end, val)?;
        Ok(())
    }

    pub fn execute_table_grow(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let table_idx = context.reader.next_leb128_u32();
        let table_addr = context.instance.tableaddr.get(table_idx as usize).unwrap(); // validated
        let tab = context.tables.get_mut(*table_addr).unwrap(); // validated

        let sz = tab.reference_count();

        let n = self.stack.pop_i32() as usize; // size
        let val = self.stack.pop();

        let lens = sz.checked_add(n);
        let Some(len) = lens else {
            self.stack.push(Value::I32(u32::MAX as i32))?;
            return Ok(());
        };

        if let Some(max) = tab.table_type.limit.max {
            if len > max as usize {
                self.stack.push(Value::I32(u32::MAX as i32))?;
                return Ok(());
            }
        }
        tab.grow(n, val)?;
        trace!("GROW n:{:?}, val:{:?}", n, val);

        tab.table_type.limit.min = len as u32;
        self.stack.push(Value::I32(sz as i32))?;

        Ok(())
    }

    pub fn execute_ref_null(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let ref_type = crate::core::value_types::RefType::from_byte(context.reader.next_byte())
            .ok_or(ExecutionError::UnexpectedValueType)?;
        self.stack.push(Value::RefNull(ref_type))?;
        Ok(())
    }
    pub fn execute_ref_func(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let func_idx = context.reader.next_leb128_u32() as usize;
        let store_addr = context.instance.funcaddr.get(func_idx).unwrap(); // Validated
        self.stack.push(Value::FuncRef(store_addr.as_usize()))?;
        Ok(())
    }

    pub fn execute_ref_is_null(&mut self) -> Result<()> {
        let value = self.stack.pop();
        match value {
            Value::RefNull(_) => self.stack.push(Value::I32(1))?,
            _ => self.stack.push(Value::I32(0))?,
        }
        Ok(())
    }

    pub fn execute_elem_drop(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let element_idx = context.reader.next_leb128_u32() as usize;
        // 2. Assert: due to validation, 𝐹.module.elemaddrs[𝑥] exists.
        // 3. Let 𝑎 be the element address 𝐹.module.elemaddrs[𝑥].
        let element_addr = context.instance.elemaddr.get(element_idx).unwrap();
        // 4. Assert: due to validation, 𝑆.elems[𝑎] exists.
        let elem = context.elements.get_mut(*element_addr).unwrap();
        // Replace 𝑆.elems[𝑎].elem with 𝜖.
        elem.references.clear();
        Ok(())
    }
}
