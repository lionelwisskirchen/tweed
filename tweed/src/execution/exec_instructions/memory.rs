use crate::{
    core::error::ExecutionError,
    execution::{
        executor::{ExecutionContext, Executor},
        store::map_memory,
    },
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

impl Executor {
    pub fn execute_memory_size(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_byte();
        let mem = map_memory(idx as usize, context.instance, context.memories).unwrap();
        let size_in_pages = mem.pages() as i32;

        self.stack.push(Value::I32(size_in_pages))?;
        Ok(())
    }

    pub fn execute_memory_grow(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_byte();
        let mem = map_memory(idx as usize, context.instance, context.memories).unwrap();
        let size_in_pages = mem.pages() as i32;

        let amount = self.stack.pop_i32() as u32;
        trace!("Instruction: memory.grow {}", size_in_pages);

        if mem.grow(amount).is_ok() {
            self.stack.push(Value::I32(size_in_pages))?;
        } else {
            self.stack.push(Value::I32(-1))?;
        }
        Ok(())
    }

    pub fn execute_data_drop(&mut self, context: &mut ExecutionContext) -> Result<()> {
        // 2. Assert: due to validation, 𝐹.module.dataaddrs[𝑥] exists.
        // 3. Let 𝑎 be the data address 𝐹.module.dataaddrs[𝑥].
        let data_index = context.reader.next_leb128_u32() as usize;
        let data_addr = context.instance.dataddr.get(data_index).unwrap();
        // 4. Assert: due to validation, 𝑆.datas[𝑎] exists.
        let data_inst = context.datas.get_mut(*data_addr).unwrap();
        // 5. Replace 𝑆.datas[𝑎] with the data instance {data 𝜖}.
        data_inst.clear();
        Ok(())
    }

    pub fn execute_memory_init(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let data_index = context.reader.next_leb128_u32() as usize;
        let memory_index = context.reader.next_byte() as usize;
        // 2. Assert: due to validation, 𝐹.module.memaddrs[0] exists.
        // 3. Let ma be the memory address 𝐹.module.memaddrs[0].
        let mem_addr = context.instance.memaddr.get(memory_index).unwrap();

        // 4. Assert: due to validation, 𝑆.mems[ma] exists.
        // 5. Let mem be the memory instance 𝑆.mems[ma]
        let mem = context.memories.get_mut(*mem_addr).unwrap();
        // 6. Assert: due to validation, 𝐹.module.dataaddrs[𝑥] exists.
        // 7. Let da be the data address 𝐹.module.dataaddrs[𝑥].
        let data_addr = context.instance.dataddr.get(data_index).unwrap();
        // 8. Assert: due to validation, 𝑆.datas[da] exists.
        // 9. Let data be the data instance 𝑆.datas[da].
        let data = context.datas.get_mut(*data_addr).unwrap();

        // 10. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 11. Pop the value i32.const 𝑛 from the stack.
        let n = self.stack.pop_i32() as usize;
        // 12. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 13. Pop the value i32.const 𝑠 from the stack
        let s = self.stack.pop_i32() as usize;
        // 14. Assert: due to validation, a value of value type i32 is on the top of the stack.
        // 15. Pop the value i32.const 𝑑 from the stack
        let d = self.stack.pop_i32() as usize;
        let src_end = s
            .checked_add(n)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
        let dst_end = d
            .checked_add(n)
            .ok_or(ExecutionError::MemoryAccessOutOfBounds)?;
        // 16. If 𝑠 + 𝑛 is larger than the length of data.data or 𝑑 + 𝑛 is larger than the length of mem.data, then:
        // a. Trap.
        if src_end > data.data.len() || dst_end > mem.bytes.len() {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        };
        let data = data.load_slice(s, n)?;
        mem.store(d, 0, &data)?;

        Ok(())
    }

    pub fn execute_memory_copy(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let (dest, src) = (context.reader.next_byte(), context.reader.next_byte());

        let number_of_bytes = self.stack.pop_i32() as usize;
        let src_address = self.stack.pop_i32() as usize;
        let dest_address = self.stack.pop_i32() as usize;

        let data = {
            let src_mem = map_memory(src as usize, context.instance, context.memories).unwrap();
            src_mem.load_slice(src_address, number_of_bytes)?.to_vec()
        };
        let dest_mem = map_memory(dest as usize, context.instance, context.memories).unwrap();
        dest_mem.store(dest_address, 0, &data)?;
        trace!("MEMORY_COPY: {number_of_bytes} bytes from {src}:{src_address} to {dest}:{dest_address}");

        Ok(())
    }

    pub fn execute_memory_fill(&mut self, context: &mut ExecutionContext) -> Result<()> {
        let idx = context.reader.next_byte();
        let mem = map_memory(idx as usize, context.instance, context.memories).unwrap(); // always 0
        let number_of_bytes = self.stack.pop_i32() as usize;
        let val = self.stack.pop_i32() as u8;

        if number_of_bytes > isize::MAX as usize {
            return Err(ExecutionError::MemoryAccessOutOfBounds);
        }
        let data = vec![val; number_of_bytes];

        let address = self.stack.pop_i32() as usize;
        mem.store(address, 0, &data)?;
        trace!("MEMORY_FILL: {number_of_bytes} bytes from {address} to {val}");
        Ok(())
    }
}
