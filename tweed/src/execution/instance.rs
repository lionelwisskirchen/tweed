use crate::binary::data_section::SegmentMode;
use crate::binary::export_section::ExportContext;
use crate::core::signature_lookup::lookup_signature;
use crate::core::values::{FromLeBytes, Value};
use crate::validation::module::Module;
use crate::{
    core::{
        error::{Error, Result},
        opcode::*,
        value_types::ValType,
        wasm_reader::WasmReader,
    },
    validation::side_table::SideTable,
};

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::zip;

use super::constant_expression_executor::execute_const_expr;
use super::execution_stack::ExecutionStack;
use super::store::{DataInstance, FunctionInstance, GlobalInstance, MemoryInstance, Store};

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub start: usize,
    pub end: usize,
    pub return_value: Vec<ValType>,
    pub locals: Vec<Value>,
    pub side_table: SideTable,
    pub side_table_pointer: usize,
}

// An instantiated Module
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Instance {
    store: Store,
    reader: WasmReader,
    call_stack: Vec<Frame>,
    stack: ExecutionStack,
}

impl Instance {
    pub fn call_by_name(&mut self, name: &str, args: Vec<Value>) -> Result<Vec<Value>> {
        let Some(export_instance) = self.store.exports.get(name) else {
            return Err(Error::UnknownExportCall);
        };
        match export_instance {
            ExportContext::FunctionExport(index) => self.call(*index as usize, args),
        }
    }
    pub fn call(&mut self, index: usize, args: Vec<Value>) -> Result<Vec<Value>> {
        self.stack.reset(); // Reset stack. Maybe not the perfect location
        self.stack.push_vec(&args);
        self.invoke(index)
    }
    fn build_frame(&mut self, function: FunctionInstance) -> Result<Frame> {
        let mut locals = self.stack.pop_n(function.signature.parameter.len())?;
        locals.append(&mut function.locals.clone()); // Locals = Params + Locals
        Ok(Frame {
            program_counter: 0,
            stack_pointer: self.stack.len(),
            start: function.start,
            end: function.end,
            return_value: function.signature.result.clone(),
            locals,
            side_table: function.side_table.clone(),
            side_table_pointer: 0,
        })
    }
    fn invoke(&mut self, index: usize) -> Result<Vec<Value>> {
        let function = self
            .store
            .functions
            .get(index)
            .ok_or(Error::InvalidFunctionIndex)?;
        let frame = self.build_frame(function.clone())?;
        self.call_stack.push(frame);
        self.execute()
    }
    fn execute(&mut self) -> Result<Vec<Value>> {
        loop {
            let Some(mut current_frame) = self.call_stack.pop() else {
                return self.stack.pop_n(self.stack.len());
            };

            self.execute_frame(&mut current_frame)?;
        }
    }
    fn skip_block_type(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }
    fn skip_depth(&mut self) -> Result<u32> {
        self.reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)
    }

    fn do_goto(&mut self, current_frame: &mut Frame) -> Result<()> {
        let side_table_entry = current_frame
            .side_table
            .entries
            .get(current_frame.side_table_pointer)
            .ok_or(Error::InvalidSidetableIndex)?;
        let pos = self.reader.current_index() as i32 + side_table_entry.ip;
        current_frame.side_table_pointer = current_frame
            .side_table_pointer
            .saturating_add_signed(side_table_entry.stp as isize);
        self.reader.seek(pos as usize);
        let saved_vals = self.stack.pop_n(side_table_entry.vals)?;
        self.stack.pop_n(side_table_entry.pop)?;
        self.stack.push_vec(&saved_vals);
        Ok(())
    }

    fn execute_frame(&mut self, current_frame: &mut Frame) -> Result<()> {
        self.reader
            .seek(current_frame.start + current_frame.program_counter);
        while self.reader.current_index() < current_frame.end {
            let instruction = self
                .reader
                .next_byte()
                .ok_or(Error::InvalidSectionBytecode)?;
            match *instruction {
                I32_CONST => {
                    let value = Value::I32(self.reader.next_leb128_i32().unwrap());
                    self.stack.push(value);
                }
                I64_CONST => {
                    let value = Value::I64(self.reader.next_leb128_i64().unwrap());
                    self.stack.push(value);
                }
                BLOCK | LOOP => {
                    self.skip_block_type()?;
                }
                IF => {
                    let bool_val = self.stack.pop()?;
                    if bool_val == Value::I32(0) {
                        self.do_goto(current_frame)?;
                    } else {
                        self.skip_block_type()?;
                        current_frame.side_table_pointer += 1;
                    }
                }
                ELSE => {
                    self.do_goto(current_frame)?;
                }
                LOCAL_GET => {
                    let local = current_frame
                        .locals
                        .get(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::InvalidLocalIndex)?;
                    self.stack.push(*local);
                }
                LOCAL_SET => {
                    let local = current_frame
                        .locals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::InvalidLocalIndex)?;
                    let new_value = self.stack.pop()?;
                    *local = new_value;
                }
                GLOBAL_GET => {
                    let global = self
                        .store
                        .globals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::UnknownGlobal)?;

                    self.stack.push(global.value);
                }
                GLOBAL_SET => {
                    let global = self
                        .store
                        .globals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::UnknownGlobal)?;
                    let new_value = self.stack.pop()?;
                    global.value = new_value;
                }
                LOCAL_TEE => {
                    let local = current_frame
                        .locals
                        .get_mut(self.reader.next_leb128_u32().unwrap() as usize)
                        .ok_or(Error::InvalidLocalIndex)?;
                    let new_value = self.stack.pop()?;
                    *local = new_value;
                    self.stack.push(new_value);
                }
                DROP => {
                    self.stack.pop()?;
                }
                NOP => {}
                I32_CTZ => {
                    let val = self.stack.pop_i32()?;
                    let result = val.trailing_zeros() as i32;
                    self.stack.push(result.into());
                }
                I64_CTZ => {
                    let val = self.stack.pop_i64()?;
                    let result = val.trailing_zeros() as i64;
                    self.stack.push(result.into());
                }
                I64_ADD => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = right.wrapping_add(left);
                    self.stack.push(result.into());
                }
                I64_SUB => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = left.wrapping_sub(right);
                    self.stack.push(result.into());
                }
                I64_MUL => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = left.wrapping_mul(right);
                    self.stack.push(result.into());
                }
                I64_AND => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);

                    let result = right & left;
                    self.stack.push(result.into());
                }
                I64_OR => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);

                    let result = right | left;
                    self.stack.push(result.into());
                }
                I64_XOR => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);

                    let result = right ^ left;
                    self.stack.push(result.into());
                }
                I64_SHR_U => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = (left as u64).wrapping_shr((right & 63) as u32);
                    self.stack.push(result.into());
                }
                I64_SHR_S => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = left.wrapping_shr((right & 63) as u32);
                    self.stack.push(result.into());
                }
                I64_SHL => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    let result = left.wrapping_shl((right & 63) as u32);
                    self.stack.push(result.into());
                }
                I32_ADD => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    let result = right.wrapping_add(left);
                    self.stack.push(result.into());
                }
                I32_MUL => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    let result = right.wrapping_mul(left);
                    self.stack.push(result.into());
                }
                I32_DIV_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    if left == i32::MIN && left == -1 {
                        return Err(Error::UnrepresentableResult);
                    }
                    self.stack.push((left / right).into());
                }
                I32_DIV_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(((left as u32) / (right as u32)).into());
                }
                I32_REM_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let result = left.checked_rem(right);
                    self.stack.push(result.unwrap_or_default().into());
                }
                I32_REM_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if right == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let result = (left as u32).checked_rem(right as u32);
                    self.stack.push(result.unwrap_or_default().into());
                }
                I32_SUB => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    let result = left.wrapping_sub(right);
                    self.stack.push(result.into());
                }
                I32_EQZ => {
                    let val = self.stack.pop_i32()?;
                    if val == 0 {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }

                I32_EQ => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if right == left {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }

                I32_CLZ => {
                    let val = self.stack.pop_i32()?;
                    self.stack.push(val.leading_zeros().into());
                }
                I32_LT_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if left < right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_LE_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if left <= right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_LT_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if (left as u32) < (right as u32) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_LE_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if (left as u32) <= (right as u32) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GT_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if (left as u32) > (right as u32) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GE_U => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if (left as u32) >= (right as u32) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_XOR => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    let result = right ^ left;
                    self.stack.push(result.into());
                }

                I32_NE => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);
                    if left != right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_WRAP_I64 => {
                    let value = self.stack.pop_i64()?;
                    self.stack.push((value as i32).into())
                }
                I64_EXTEND_I32_U => {
                    let value = self.stack.pop_i32()?;
                    self.stack.push((value as u32 as i64).into())
                }
                I64_LT_U => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    if (left as u64) < (right as u64) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I64_GT_U => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    if (left as u64) > (right as u64) {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I64_EQ => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    if left == right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I64_NE => {
                    let (right, left) = (self.stack.pop_i64()?, self.stack.pop_i64()?);
                    if left != right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GT_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    if left > right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_GE_S => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    if left >= right {
                        self.stack.push(Value::I32(1));
                    } else {
                        self.stack.push(Value::I32(0));
                    }
                }
                I32_AND => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    let result = right & left;
                    self.stack.push(Value::I32(result));
                }
                I32_OR => {
                    let (right, left) = (self.stack.pop_i32()?, self.stack.pop_i32()?);

                    let result = right | left;
                    self.stack.push(Value::I32(result));
                }
                SELECT => {
                    let (cond, b, a) =
                        (self.stack.pop_i32()?, self.stack.pop()?, self.stack.pop()?);
                    if cond == 0 {
                        self.stack.push(b);
                    } else {
                        self.stack.push(a);
                    }
                }
                BR => {
                    self.do_goto(current_frame)?;
                }
                BR_IF => {
                    let bool_val = self.stack.pop_i32()?;
                    if bool_val == 0 {
                        self.skip_depth()?;
                        current_frame.side_table_pointer += 1;
                    } else {
                        self.do_goto(current_frame)?;
                    }
                }
                BR_TABLE => {
                    let branch_index = self.stack.pop_i32()?;
                    let number_of_branches =
                        self.reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)? as usize;
                    if branch_index as usize >= number_of_branches {
                        current_frame.side_table_pointer += number_of_branches;
                    } else {
                        current_frame.side_table_pointer += branch_index as usize;
                    }
                    self.do_goto(current_frame)?;
                    #[cfg(test)]
                    println!("{:?}", self.reader.rest());
                }
                RETURN => {
                    let count = current_frame.return_value.len();
                    let results = self.stack.pop_n(count)?;
                    self.stack.unwind_to(current_frame.stack_pointer);
                    self.stack.push_vec(&results);
                    break;
                }
                END => {
                    if self.reader.current_index() == current_frame.end {
                        break;
                    }
                }
                CALL => {
                    let index = self
                        .reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidFunctionIndex)?;
                    let function = self
                        .store
                        .functions
                        .get(index as usize)
                        .ok_or(Error::InvalidFunctionIndex)?;

                    // TODO External/internal functions
                    let frame = self.build_frame(function.clone())?;
                    current_frame.program_counter =
                        self.reader.current_index() - current_frame.start + 1;
                    self.call_stack.push(current_frame.clone());
                    self.call_stack.push(frame);
                    break;
                }
                I64_STORE => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.stack.pop_i64()? as u64).to_le_bytes();
                    let rel_address = self.stack.pop_i32()? as u32;
                    self.store(offset, &store_data, rel_address)?;
                }
                I64_LOAD => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.stack.pop_i32()? as u32;
                    let data: i64 = self.load(offset, address, 8)?;
                    self.stack.push(data.into());
                }

                I32_STORE => {
                    let (_, offset) = self.read_mem_args()?;
                    let store_data = (self.stack.pop_i32()? as u32).to_le_bytes();
                    let rel_address = self.stack.pop_i32()? as u32;
                    self.store(offset, &store_data, rel_address)?;
                }
                I32_LOAD => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.stack.pop_i32()? as u32;
                    let data: i32 = self.load(offset, address, 4)?;
                    self.stack.push(data.into());
                }
                I32_LOAD8_U => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.stack.pop_i32()? as u32;
                    let data: u8 = self.load(offset, address, 4)?;
                    self.stack.push((data as u32).into());
                }
                I32_LOAD8_S => {
                    let (_, offset) = self.read_mem_args()?;
                    let address = self.stack.pop_i32()? as u32;
                    let data: i8 = self.load(offset, address, 4)?;
                    self.stack.push((data as u32).into());
                }
                _ => return Err(Error::UndefinedInstruction(*instruction)),
            };
        }
        current_frame.program_counter = self.reader.current_index() - current_frame.start + 1;
        Ok(())
    }
    fn store(&mut self, offset: u32, data: &[u8], address: u32) -> Result<()> {
        let mem = self.store.memories.get_mut(0).unwrap();
        let address = offset
            .checked_add(address)
            .ok_or(Error::MemoryAccessOutOfBounds)? as usize;
        let end = address + data.len();
        mem.data[address..end].copy_from_slice(data);
        Ok(())
    }

    fn load<T>(&mut self, offset: u32, address: u32, bytes: usize) -> Result<T>
    where
        T: FromLeBytes,
    {
        let mem = self.store.memories.get_mut(0).unwrap();
        let address = offset
            .checked_add(address)
            .ok_or(Error::MemoryAccessOutOfBounds)? as usize;
        let data_bytes = mem
            .data
            .get(address..address + bytes)
            .ok_or(Error::MemoryAccessOutOfBounds)?;
        let data = T::from_le_bytes(data_bytes);
        Ok(data)
    }
    fn read_mem_args(&mut self) -> Result<(u32, u32)> {
        let _align = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
        let offset = self
            .reader
            .next_leb128_u32()
            .ok_or(Error::UndefinedMemory)?;
        Ok((_align, offset))
    }
}
pub const PAGE_SIZE: u32 = 65536;

pub struct Linker {
    module: Module,
    imported_globals: BTreeMap<ImportName, GlobalInstance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ImportName {
    pub module: String,
    pub name: String,
}

impl Linker {
    pub fn new(module: Module) -> Self {
        Linker {
            module,
            imported_globals: BTreeMap::new(),
        }
    }

    pub fn auto_link_spectest_globals(&mut self) -> Result<()> {
        for import in &self.module.importsec.globals.clone() {
            if import.module == "spectest" {
                self.add_global_import(
                    import.module.clone(),
                    import.name.clone(),
                    match import.context.value_type {
                        ValType::I32 => Value::I32(42),
                        ValType::I64 => Value::I64(42),
                        _ => panic!("Unknown value type cant be converted to value."),
                    },
                )?;
            }
        }
        Ok(())
    }
    pub fn add_global_import(&mut self, module: String, name: String, value: Value) -> Result<()> {
        let key = ImportName {
            module: module.clone(),
            name: name.clone(),
        };
        if let Some(import) = self
            .module
            .importsec
            .globals
            .iter()
            .find(|g| g.module == module && g.name == name)
        {
            let global_type = import.context.clone();
            if !global_type.value_type.is_type(value) {
                return Err(Error::TypeMismatch);
            }
            let instance = GlobalInstance { global_type, value };
            self.imported_globals.insert(key, instance);
            Ok(())
        } else {
            Err(Error::UnknownGlobal)
        }
    }
    pub fn instantiate(&mut self) -> Result<Instance> {
        let mut imported_globals = self.instantiate_global_imports()?;
        let exports = self.instantiate_export()?;
        let functions = self.instantiate_functions()?;
        let mut memories = self.instantiate_memories()?;
        let data = self.instantiate_datas(&mut memories, &imported_globals)?;
        imported_globals.extend(self.instantiate_globals(&imported_globals)?);

        let store = Store {
            exports,
            functions,
            memories,
            data,
            globals: imported_globals,
        };
        Ok(Instance {
            store,
            reader: self.module.reader.clone(),
            call_stack: vec![],
            stack: ExecutionStack::default(),
        })
    }
    fn instantiate_global_imports(&mut self) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();
        for import in self.module.importsec.globals.iter() {
            let key = ImportName {
                module: import.module.clone(),
                name: import.name.clone(),
            };
            let global = self
                .imported_globals
                .get(&key)
                .ok_or(Error::MissingGlobalImport)?;
            globals.push(global.clone())
        }
        Ok(globals)
    }
    fn instantiate_globals(
        &mut self,
        imported_globals: &[GlobalInstance],
    ) -> Result<Vec<GlobalInstance>> {
        let mut globals = Vec::new();

        for global in self.module.globalsec.iter() {
            let expr = {
                self.module.reader.seek(global.start + 1);
                let return_value = execute_const_expr(
                    &mut self.module.reader,
                    imported_globals,
                    &[global.global_type.value_type],
                )?;
                *return_value.first().ok_or(Error::TypeMismatch)?
            };
            globals.push(GlobalInstance {
                global_type: global.global_type.clone(),
                value: expr,
            });
        }
        Ok(globals)
    }
    fn instantiate_export(&mut self) -> Result<BTreeMap<String, ExportContext>> {
        let mut exports = BTreeMap::new();
        for export in &self.module.exportsec {
            exports.insert(export.name.clone(), export.context.clone());
        }
        Ok(exports)
    }
    fn instantiate_functions(&mut self) -> Result<Vec<FunctionInstance>> {
        let function_to_sidetable = zip(self.module.codesec.iter(), self.module.sidetables.iter());
        let mut functions = Vec::new();
        for (index, (function, side_table)) in function_to_sidetable.enumerate() {
            let locals: Vec<Value> = function
                .locals
                .iter()
                .map(|val_type| (*val_type).into())
                .collect::<Vec<Value>>();

            let signature_index = self
                .module
                .funcsec
                .get(index)
                .ok_or(Error::InvalidFunctionIndex)?
                .sig_index;
            let signature = lookup_signature(signature_index, &self.module.typesec);

            functions.push(FunctionInstance {
                signature: signature.clone(),
                locals,
                start: function.start,
                end: function.end,
                side_table: side_table.clone(),
            });
        }
        Ok(functions)
    }

    fn instantiate_memories(&mut self) -> Result<Vec<MemoryInstance>> {
        let mut memories = Vec::new();
        for memory in self.module.memorysec.iter() {
            let data = vec![0; (memory.min * PAGE_SIZE) as usize];
            memories.push(MemoryInstance {
                data,
                max: memory.max,
            });
        }
        Ok(memories)
    }
    fn instantiate_datas(
        &mut self,
        memories: &mut [MemoryInstance],
        imported_globals: &[GlobalInstance],
    ) -> Result<Vec<DataInstance>> {
        let mut data = Vec::new();
        for data_segment in self.module.datasec.iter() {
            if let SegmentMode::Active {
                memory,
                offset_start,
                offset_end: _,
            } = data_segment.mode
            {
                let offset = {
                    self.module.reader.seek(offset_start + 1);
                    let return_values = execute_const_expr(
                        &mut self.module.reader,
                        imported_globals,
                        &[ValType::I32],
                    )?;
                    let Value::I32(value) = return_values.first().ok_or(Error::TypeMismatch)?
                    else {
                        return Err(Error::TypeMismatch);
                    };
                    *value
                };

                let mem_inst = memories
                    .get_mut(memory as usize)
                    .ok_or(Error::InvalidMemoryIndex)?;

                mem_inst.data[offset as usize..offset as usize + data_segment.init.len()]
                    .copy_from_slice(&data_segment.init);
            };
            data.push(DataInstance {
                data: data_segment.init.clone(),
            });
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        binary::{code_section::FuncCode, function_section::FuncDecl, type_section::FuncType},
        validation::side_table::SideTableEntry,
    };

    use super::*;
    #[test]
    fn test_simple_add() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 29,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
            ..Default::default()
        };

        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[should_panic]
    #[test]
    fn test_invalid_instruction() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 1,
                end: 29,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
            ..Default::default()
        };

        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_no_end_instruction() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   i32.add
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 28,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
            ..Default::default()
        };

        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(22)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_return() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
	    	   i32.const 10
	    	   i32.const 12
               	   return
                   i32.const 12
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 24,
                end: 31,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
            ..Default::default()
        };

        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_blocks() {
        let bytes = wat::parse_str(
            "(module
               (func (result i32)
                 (block $label1 (result i32)
	    	   i32.const 10
		   (block $label2 (param i32) (result i32)
		   	  i32.const 2
			  i32.add
		   )
	         )
               )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![
                FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![ValType::I32],
                },
            ],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 29,
                end: 40,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable { entries: vec![] }],
            ..Default::default()
        };
        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(12)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_blocks_with_branches() {
        let bytes = wat::parse_str(
            "(module
                (func (result i32)
                  (block $label1 (result i32)
                     i32.const 4
                     i32.const 5
                     i32.add
                     br $label1
	          )
                  (block $label3 (param i32)
                        i32.const 7                        
                     (block $label2 (param i32 i32)
                        i32.add
                        br_if $label3
                     )
                  )
		  i32.const 3
                )
             )",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![
                FuncType {
                    parameter: vec![],
                    result: vec![ValType::I32],
                },
                FuncType {
                    parameter: vec![ValType::I32],
                    result: vec![],
                },
                FuncType {
                    parameter: vec![ValType::I32, ValType::I32],
                    result: vec![],
                },
            ],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 33,
                end: 56,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 3,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
            ..Default::default()
        };
        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(3)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_for_loop() {
        let bytes = wat::parse_str(
            "(module
                 (func (result i32)
                   (local $i i32)
                   (loop $my_loop
                       local.get $i
                       i32.const 1
                       i32.add
                       local.set $i
                       local.get $i
                       i32.const 10
                       i32.lt_s
                       br_if $my_loop
                   )
                    local.get $i
                )
             )
",
        )
        .unwrap();

        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![ValType::I32],
                start: 26,
                end: 45,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![SideTableEntry {
                    ip: -14,
                    stp: 0,
                    vals: 0,
                    pop: 0,
                }],
            }],
            ..Default::default()
        };
        let mut instance = Linker::new(module).instantiate().unwrap();
        let expected_result = vec![Value::I32(10)];
        assert_eq!(instance.call(0, vec![]).unwrap(), expected_result);
    }

    #[test]
    fn test_if_else() {
        let bytes = wat::parse_str(
            "
(module
  (func (param i32) (result i32)
    (block $label1 (result i32)
      local.get 0
      (if $if (result i32)
        (then
          i32.const 1
          br $label1
        )
        (else
	i32.const 3
	br $if
        )
      )
      br $label1
    )
  )
)",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 25,
                end: 44,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 7,
                        stp: 3,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 10,
                        stp: 4,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 5,
                        stp: 2,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 2,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
            ..Default::default()
        };
        let mut instance = Linker::new(module).instantiate().unwrap();
        assert_eq!(
            instance.call(0, vec![Value::I32(1)]).unwrap(),
            vec![Value::I32(1)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(0)]).unwrap(),
            vec![Value::I32(3)]
        );
    }

    #[test]
    fn test_br_table() {
        let bytes = wat::parse_str(
            "
(func (param $p i32) (result i32)
  (block
    (block
      (block
        (block (local.get $p)
               (br_table
                         2   ;; p == 0 => (br 2)
                         1   ;; p == 1 => (br 1)
                         0   ;; p == 2 => (br 0)
                         3)) ;; else => (br 3)
        ;; Target for (br 0)
        (i32.const 100)
        (return))
      ;; Target for (br 1)
      (i32.const 101)
      (return))
    ;; Target for (br 2)
    (i32.const 102)
    (return))
  ;; Target for (br 3)
  (i32.const 103)
  (return))",
        )
        .unwrap();
        let module = Module {
            reader: WasmReader::new(bytes.to_vec()),
            typesec: vec![FuncType {
                parameter: vec![ValType::I32],
                result: vec![ValType::I32],
            }],
            funcsec: vec![FuncDecl { sig_index: 0 }],
            codesec: vec![FuncCode {
                locals: vec![],
                start: 25,
                end: 61,
            }],
            exportsec: vec![],
            sidetables: vec![SideTable {
                entries: vec![
                    SideTableEntry {
                        ip: 15,
                        stp: 4,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 10,
                        stp: 3,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 5,
                        stp: 2,
                        vals: 0,
                        pop: 0,
                    },
                    SideTableEntry {
                        ip: 20,
                        stp: 1,
                        vals: 0,
                        pop: 0,
                    },
                ],
            }],
            ..Default::default()
        };
        let mut instance = Linker::new(module).instantiate().unwrap();
        assert_eq!(
            instance.call(0, vec![Value::I32(0)]).unwrap(),
            vec![Value::I32(102)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(1)]).unwrap(),
            vec![Value::I32(101)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(2)]).unwrap(),
            vec![Value::I32(100)]
        );
        assert_eq!(
            instance.call(0, vec![Value::I32(10)]).unwrap(),
            vec![Value::I32(103)]
        );
    }
}
