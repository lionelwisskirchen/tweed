use crate::{
    binary::type_section::FuncType,
    core::{
        error::ValidationError,
        signature_lookup::lookup_signature,
        value_types::{RefType, ValType},
    },
    validation::{control_types::ControlType, validator::CodeValidator},
};

use alloc::vec::Vec;

pub type Result<T> = core::result::Result<T, ValidationError>;

impl<'a, 'b> CodeValidator<'a, 'b> {
    fn read_depth(&mut self) -> u32 {
        self.reader
            .next_leb128_u32()
            .expect("ValidationError parsing break depth")
    }

    fn check_args(&mut self, args: &[ValType], height_of_value_stack: u32) -> Result<()> {
        self.abstract_value_stack.pop_expected_vec(args)?;

        if height_of_value_stack != self.abstract_value_stack.height() {
            Err(ValidationError::TypeMismatch)?;
        }

        self.abstract_value_stack.push_vec(args);
        Ok(())
    }

    fn read_block_type(&mut self) -> Result<FuncType> {
        let func_idx = self
            .reader
            .next_leb128_u32()
            .expect("ValidationError parsing block func idx");
        lookup_signature(func_idx, &self.module_environment.typesec)
    }

    pub fn validate_if(&mut self) -> Result<()> {
        let index_of_if_bytecode = self.reader.current_index() as i32;
        let signature = self.read_block_type()?;
        self.abstract_value_stack.pop_expected(ValType::I32)?; // bool value
        self.abstract_value_stack
            .pop_expected_vec(&signature.parameter)?;

        let index_of_if_sidetable_entry =
            self.side_table.create_entry(index_of_if_bytecode, 0, 0, 0);

        self.push_ctrl(ControlType::If {
            func_type: signature.clone(),
            sidetable_idxs: Vec::new(),
            side_table_entry: index_of_if_sidetable_entry,
            reachable: true,
            height_of_value_stack: self.abstract_value_stack.height(),
            has_else: false,
        });

        self.abstract_value_stack.push_vec(&signature.parameter);
        Ok(())
    }

    pub fn validate_else(&mut self) -> Result<()> {
        let control_type = self.abstract_control_stack.top();

        let index_of_else_sidetable_entry =
            self.side_table
                .create_entry(self.reader.current_index() as i32, 0, 0, 0);

        match control_type {
            ControlType::If {
                func_type,
                sidetable_idxs,
                side_table_entry,
                height_of_value_stack,
                has_else,
                reachable,
            } => {
                sidetable_idxs.push(index_of_else_sidetable_entry);
                let expected_results = func_type.result.clone();

                self.abstract_value_stack
                    .pop_expected_vec(&expected_results)?;

                if *height_of_value_stack != self.abstract_value_stack.height() {
                    Err(ValidationError::TypeMismatch)?;
                }
                *has_else = true;
                *reachable = true;

                self.abstract_value_stack.push_vec(&func_type.parameter);

                let side_table_len = self.side_table.len();
                let entry = self.side_table.get_entry(*side_table_entry);
                // branch from "if" will go to instruction after else
                entry.ip = (self.reader.current_index() + 1) as i32 - entry.ip;
                // branch if will skip all sidetable entries from if to else
                entry.stp = side_table_len as i32 - *side_table_entry as i32;

                self.abstract_value_stack
                    .update(*height_of_value_stack, true);
            }
            _ => {
                Err(ValidationError::InvalidControlFlow)?;
            }
        }
        Ok(())
    }

    pub fn validate_return(&mut self) -> Result<()> {
        self.abstract_value_stack
            .pop_expected_vec(&self.validation_context.func_signature.result)?;

        self.unreachable();
        Ok(())
    }

    pub fn validate_block(&mut self) -> Result<()> {
        let signature = self.read_block_type()?;

        self.abstract_value_stack
            .pop_expected_vec(&signature.parameter)?;

        self.push_ctrl(ControlType::Block {
            func_type: signature.clone(),
            sidetable_idxs: Vec::new(),
            reachable: true,
            height_of_value_stack: self.abstract_value_stack.height(),
        });
        self.abstract_value_stack.push_vec(&signature.parameter);
        Ok(())
    }

    pub fn validate_loop(&mut self) -> Result<()> {
        let index_of_loop_bytecode = self.reader.current_index();
        let signature = self.read_block_type()?;
        self.abstract_value_stack
            .pop_expected_vec(&signature.parameter)?;

        self.push_ctrl(ControlType::For {
            instruction_pointer: index_of_loop_bytecode,
            func_type: signature.clone(),
            side_table_entry: self.side_table.len(),
            reachable: true,
            height_of_value_stack: self.abstract_value_stack.height(),
        });
        self.abstract_value_stack.push_vec(&signature.parameter);
        Ok(())
    }

    pub fn validate_br_if(&mut self) -> Result<()> {
        let index_of_br_if_bytecode = self.reader.current_index() as i32;

        self.abstract_value_stack.pop_expected(ValType::I32)?; // bool

        let depth = self.read_depth();

        // Pop the expected control types, so the validation stack to get the result types
        // According to ref algorithm: https://webassembly.github.io/spec/versions/core/WebAssembly-2.0.pdf p.184
        let control_types = self.abstract_control_stack.get_control_type(depth)?.types();
        self.abstract_value_stack.pop_expected_vec(control_types)?;

        let control_type = self.create_sidetable_entry(depth, index_of_br_if_bytecode)?;

        self.abstract_value_stack.push_vec(control_type.types());
        Ok(())
    }

    pub fn validate_br_table(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;

        let number_of_branches = self.reader.next_leb128_u32()?;
        let index_of_br_table_bytecode = self.reader.current_index() as i32;

        let target_ctrl_block_idxs = self.reader.next_leb128_u32_vec(number_of_branches)?;
        let target_ctrl_block_default_idx = self.reader.next_leb128_u32()?;

        let arity_len = self
            .abstract_control_stack
            .get_control_type(target_ctrl_block_default_idx)?
            .types()
            .len();

        for ctrl_block_idx in target_ctrl_block_idxs.iter() {
            // Pop the expected control type, so the validation stack to get the result types
            // According to ref algorithm: https://webassembly.github.io/spec/versions/core/WebAssembly-2.0.pdf p.184
            let control_types = self
                .abstract_control_stack
                .get_control_type(*ctrl_block_idx)?
                .types();

            let popped_types = self.abstract_value_stack.pop_expected_vec(control_types)?;

            if control_types.len() != arity_len {
                Err(ValidationError::TypeMismatch)?;
            }

            self.create_sidetable_entry(*ctrl_block_idx, index_of_br_table_bytecode)?;

            self.abstract_value_stack.push_vec(&popped_types);
        }
        let default_control_block = self
            .abstract_control_stack
            .get_control_type(target_ctrl_block_default_idx)?;

        self.abstract_value_stack
            .pop_expected_vec(default_control_block.types())?;

        self.create_sidetable_entry(target_ctrl_block_default_idx, index_of_br_table_bytecode)?;

        self.unreachable();
        Ok(())
    }

    pub fn validate_br(&mut self) -> Result<()> {
        let index_of_br_bytecode = self.reader.current_index() as i32;
        let depth = self.read_depth();

        // Pop the expected control type, so the validation stack to get the result types
        // According to ref algorithm: https://webassembly.github.io/spec/versions/core/WebAssembly-2.0.pdf p.184
        let control_types = self.abstract_control_stack.get_control_type(depth)?.types();

        self.abstract_value_stack.pop_expected_vec(control_types)?;
        self.create_sidetable_entry(depth, index_of_br_bytecode)?;

        self.unreachable();
        Ok(())
    }

    pub fn validate_end(&mut self) -> Result<()> {
        let control_type = self.abstract_control_stack.pop();
        match control_type {
            ControlType::Func {
                func_type,
                sidetable_idxs,
                height_of_value_stack,
                ..
            } => {
                self.check_args(&func_type.result, height_of_value_stack)?;
                self.resolve_sidetable_entries(&sidetable_idxs);
                self.validation_context.should_break = true;
                return Ok(());
            }
            ControlType::Block {
                func_type,
                sidetable_idxs,
                height_of_value_stack,
                ..
            } => {
                self.check_args(&func_type.result, height_of_value_stack)?;
                self.resolve_sidetable_entries(&sidetable_idxs);
            }
            ControlType::If {
                func_type,
                sidetable_idxs,
                height_of_value_stack,
                has_else,
                side_table_entry,
                ..
            } => {
                if !has_else {
                    self.resolve_sidetable_entries(&[side_table_entry]);
                    // Parameter pass through when there is no else block
                    if func_type.result.len() > func_type.parameter.len() {
                        Err(ValidationError::TypeMismatch)?;
                    }
                }

                self.check_args(&func_type.result, height_of_value_stack)?;
                self.resolve_sidetable_entries(&sidetable_idxs);
            }
            ControlType::For {
                func_type,
                height_of_value_stack,
                ..
            } => self.check_args(&func_type.result, height_of_value_stack)?,
        }
        self.abstract_value_stack.update(
            self.abstract_control_stack.height_of_top(),
            self.abstract_control_stack.top_is_reachable(),
        );
        Ok(())
    }

    pub fn validate_call_indirect(&mut self) -> Result<()> {
        let type_index = self.reader.next_leb128_u32()?;
        let table_index = self.reader.next_leb128_u32()?;

        let table = self
            .validation_context
            .tables
            .get(table_index as usize)
            .ok_or(ValidationError::UnknownTable)?;

        // ensure the referenced table contains FuncRefs
        match table {
            RefType::FuncRef => (),
            _ => {
                error!("wrong type: {:?}", table);
                Err(ValidationError::TypeMismatch)?;
            }
        }

        let func_type = self
            .module_environment
            .typesec
            .get(type_index as usize)
            .ok_or(ValidationError::UnknownType)?;

        self.abstract_value_stack.pop_expected(ValType::I32)?; // Table Entry Index
        self.abstract_value_stack
            .pop_expected_vec(&func_type.parameter)?;
        self.abstract_value_stack.push_vec(&func_type.result);
        Ok(())
    }

    pub fn validate_call(&mut self) -> Result<()> {
        let func_index = self.reader.next_leb128_u32()?;

        let type_idx = self
            .module_environment
            .funcsec
            .get(func_index as usize)
            .ok_or(ValidationError::UnknownFunction(None))?
            .sig_index;
        let sig = self
            .module_environment
            .typesec
            .get(type_idx as usize)
            .ok_or(ValidationError::UnknownFunction(None))?;
        self.abstract_value_stack.pop_expected_vec(&sig.parameter)?;
        self.abstract_value_stack.push_vec(&sig.result);
        Ok(())
    }

    pub fn validate_select(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        let val1 = self.abstract_value_stack.pop_val()?;
        if !val1.is_numeric() {
            Err(ValidationError::IllegalSelectType)?;
        };
        self.abstract_value_stack.pop_expected(val1)?;
        self.abstract_value_stack.push(val1);
        Ok(())
    }

    pub fn validate_select_t(&mut self) -> Result<()> {
        let num = self.reader.next_leb128_u32()?;
        if num != 1 {
            Err(ValidationError::InvalidResultArity)?;
        }
        let ty_byte = self.reader.next_byte()?;
        let ty = ValType::lookup(&ty_byte)?;
        self.abstract_value_stack.pop_expected(ValType::I32)?;
        self.abstract_value_stack.pop_expected(ty)?;
        self.abstract_value_stack.pop_expected(ty)?;
        self.abstract_value_stack.push(ty);
        Ok(())
    }

    pub fn validate_nop(&self) -> Result<()> {
        Ok(())
    }

    pub fn validate_drop(&mut self) -> Result<()> {
        self.abstract_value_stack.pop_val()?;
        Ok(())
    }
}
