use crate::{
    core::{error::ValidationError, value_types::ValType, wasm_reader::WasmReader},
    validation::validation_stack::ValidationStack,
};

pub type Result<T> = core::result::Result<T, ValidationError>;

/// Function to validate the command v128.const (0x0C)
///
/// This fucntion reads the next 16 bytes (128 bit). If that is possible, a new V128Type is pushed
/// to the validation stack.
///
/// # Arguments
/// * `reader`: a mutable reference to the [WasmReader] to read more bytes.
/// * `stack`: a mutable reference to the [ValidationStack] on which the new ValType will be stored
pub fn v128_const(
    reader: &mut WasmReader,
    abstract_value_stack: &mut ValidationStack,
) -> Result<()> {
    for _ in 0..16 {
        reader.next_byte()?;
    }

    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn v128_vvunop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn v128_vvbinop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn v128_vvternop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn v128_vvtestop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::I32);
    Ok(())
}

pub fn shape_vunop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn shape_vbinop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn shape_vternop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn shape_vtestop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::I32);
    Ok(())
}

pub fn shape_vrelop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn ishape_vshiftop(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::I32)?;
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}

pub fn ishape_bitmask(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::I32);
    Ok(())
}

pub fn shape_vcvtop_half_shape_sx_zero(abstract_value_stack: &mut ValidationStack) -> Result<()> {
    abstract_value_stack.pop_expected(ValType::V128Type)?;
    abstract_value_stack.push(ValType::V128Type);
    Ok(())
}
