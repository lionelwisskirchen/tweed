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

/// Computes the bitwise select operation on three v128 values.
///
/// For each bit position, the function selects the corresponding bit from `a` if the corresponding
///  bit in `c` is `1`, or from `b` if it is `0`.
/// *Formula:* output = (input1 AND mask) OR (input2 AND NOT mask).
///
/// This function pops three v128 values from the stack pushes back one.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn v128_bitselect(stack: &mut ExecutionStack) -> Result<()> {
    let (mask, b, a) = (
        stack.pop_v128().to_bytes(),
        stack.pop_v128().to_bytes(),
        stack.pop_v128().to_bytes(),
    );

    let result = core::array::from_fn(|i| (a[i] & mask[i]) | (b[i] & !mask[i]));

    stack.push(Value::V128(V128Shape::Bytes(result)))?;
    Ok(())
}

/// Check if any byte is larger than zero.
///
/// This function pops one v128 values from the stack pushes back one i32.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn v128_any_true(stack: &mut ExecutionStack) -> Result<()> {
    let res = stack.pop_v128().to_bytes().iter().any(|i| *i > 0) as i32;
    stack.push(Value::I32(res))?;
    Ok(())
}

/// Generates a unary SIMD operation function for i8x16 vectors.
///
/// This macro creates a function that applies a unary operation to each lane
/// of an i8x16 vector. The operation should be a method that returns a tuple
/// where the first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Unary method to call on each lane (e.g., `abs`, `overflowing_neg`)
macro_rules! define_v128_unary_op {
    ($func_name:ident => $op:expr) => {
        #[doc = concat!("Computes `", stringify!($op), "` on each lane of an i8x16 vector.")]
        ///
        /// This Method pops one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let value = stack.pop_v128().to_bytes();

            let result = core::array::from_fn(|i| $op(value[i]));

            stack.push(Value::V128(V128Shape::Bytes(result)))?;
            Ok(())
        }
    };
}

/// Generates a binary SIMD operation function for i8x16 vectors.
///
/// This macro creates a function that applies a binary operation to each lane
/// of an i8x16 vector. The given operation should be a method that returns a tuple where the
/// first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Binary method to call on each lane (e.g., `overflowing_add`, `overflowing_sub`)
macro_rules! define_v128_binary_op {
    ($func_name:ident => $op:expr) => {
        #[doc = concat!("Computes `", stringify!($op), "` on each lane of an i8x16 vector.")]
        ///
        /// This Method pops two v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let (right, left) = (stack.pop_v128().to_bytes(), stack.pop_v128().to_bytes());

            let result = core::array::from_fn(|i| $op(left[i], right[i]));

            stack.push(Value::V128(V128Shape::Bytes(result)))?;
            Ok(())
        }
    };
}

define_v128_unary_op!(v128_not => |a: u8| !a);
define_v128_binary_op!(v128_and => |a: u8, b: u8| a & b);
define_v128_binary_op!(v128_andnot => |a: u8, b: u8| a & !b);
define_v128_binary_op!(v128_or => |a: u8, b: u8| a | b);
define_v128_binary_op!(v128_xor => |a: u8, b: u8| a ^ b);
