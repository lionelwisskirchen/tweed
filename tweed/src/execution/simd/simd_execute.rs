use crate::{
    core::{error::ExecutionError, validated_wasm_reader::ValidatedWasmReader, values::V128Shape},
    execution::execution_stack::ExecutionStack,
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Function to execute the command v128.const (0x0C)
///
/// This fucntion reads the next 16 bytes (128 bit) and creates a new bytewise Shape of it.
/// This v128 value is then pushed to the execution stack
///
/// # Arguments
/// * `reader`: a mutable reference to the [ValidatedWasmReader] to read more bytes.
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn v128_const(reader: &mut ValidatedWasmReader, stack: &mut ExecutionStack) -> Result<()> {
    let mut vec = [0; 16];
    for item in &mut vec {
        *item = reader.next_byte();
    }
    trace!("V128_CONST({:?})", vec);

    let v128 = V128Shape::Bytes(vec);
    stack.push(Value::V128(v128))?;
    Ok(())
}
