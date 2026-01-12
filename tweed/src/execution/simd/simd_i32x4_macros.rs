use crate::{
    core::{error::ExecutionError, values::V128Shape},
    execution::execution_stack::ExecutionStack,
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Generates a unary SIMD operation function for i32x4 vectors.
///
/// This macro creates a function that applies a unary operation to each lane
/// of an i32x4 vector. The operation should be a method that returns a tuple
/// where the first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Unary method to call on each lane (e.g., `abs`, `overflowing_neg`)
macro_rules! define_i32x4_simd_unary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i32x4 vector.")]
        ///
        /// This Method poppes one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let value = stack.pop_v128()?.to_i32x4();

            let result = std::array::from_fn(|i| value[i].$method().0);
            stack.push(Value::V128(V128Shape::I32x4(result)))?;

            Ok(())
        }
    };
}

/// Generates a binary SIMD operation function for i32x4 vectors.
///
/// This macro creates a function that applies a binary operation to each lane
/// of an i32x4 vector. The given operation should be a method that returns a tuple where the
/// first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Binary method to call on each lane (e.g., `overflowing_add`, `overflowing_sub`)
macro_rules! define_i32x4_simd_binary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i32x4 vector.")]
        ///
        /// This Method poppes two v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let (right, left) = (stack.pop_v128()?.to_i32x4(), stack.pop_v128()?.to_i32x4());

            let mut result = [0i32; 4];

            for (idx, item) in right.iter().enumerate() {
                result[idx] = (left[idx]).$method(*item).0;
            }
            stack.push(Value::V128(V128Shape::I32x4(result)))?;

            Ok(())
        }
    };
}

define_i32x4_simd_binary_op!(i32x4_add, overflowing_add);
define_i32x4_simd_binary_op!(i32x4_sub, overflowing_sub);
define_i32x4_simd_binary_op!(i32x4_mul, overflowing_mul);
define_i32x4_simd_unary_op!(i32x4_neg, overflowing_neg);
