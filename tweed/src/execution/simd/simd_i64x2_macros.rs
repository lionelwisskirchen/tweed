use crate::{
    core::{error::ExecutionError, values::V128Shape},
    execution::execution_stack::ExecutionStack,
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Generates a unary SIMD operation function for i64x2 vectors.
///
/// This macro creates a function that applies a unary operation to each lane
/// of an i64x2 vector. The operation should be a method that returns a tuple
/// where the first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Unary method to call on each lane (e.g., `abs`, `overflowing_neg`)
macro_rules! define_i64x2_simd_unary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i64x2 vector.")]
        ///
        /// This Method poppes one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let value = stack.pop_v128().to_i64x2();

            let result = std::array::from_fn(|i| value[i].$method().0);
            stack.push(Value::V128(V128Shape::I64x2(result)))?;

            Ok(())
        }
    };
}

/// Generates a binary SIMD operation function for i64x2 vectors.
///
/// This macro creates a function that applies a binary operation to each lane
/// of an i64x2 vector. The given operation should be a method that returns a tuple where the
/// first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Binary method to call on each lane (e.g., `overflowing_add`, `overflowing_sub`)
macro_rules! define_i64x2_simd_binary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i64x2 vector.")]
        ///
        /// This Method poppes two v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let (right, left) = (stack.pop_v128().to_i64x2(), stack.pop_v128().to_i64x2());

            let mut result = [0i64; 2];

            for (idx, item) in right.iter().enumerate() {
                result[idx] = (left[idx]).$method(*item).0;
            }
            stack.push(Value::V128(V128Shape::I64x2(result)))?;

            Ok(())
        }
    };
}

/// Generates a shift SIMD operation function for i64x2 vectors.
///
/// This macro creates a function that applies a shift operation to each lane
/// of an i64x2 vector. The given operation should be a method that shift a given i64 to the left or
/// the right (e.g., `wrapping_shl`, ..).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$op` - The shift operation to call on each lane
/// * `$shift_type` - An intermediate type in which the shift should take place
macro_rules! define_i64x2_simd_shift_op {
    ($func_name:ident => $op:ident, $shift_type:ty) => {
        #[doc = concat!("Computes `", stringify!($op), "` on each lane of an i8x16 vector.")]
        ///
        /// This Method pops one i32 and one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let rhs = (stack.pop_i32() % 64) as u32;
            let mut v128_value = stack.pop_v128().to_i64x2();

            for idx in 0..v128_value.len() {
                v128_value[idx] = (v128_value[idx] as $shift_type).$op(rhs) as i64;
            }
            stack.push(Value::V128(V128Shape::I64x2(v128_value)))?;
            Ok(())
        }
    };
}

define_i64x2_simd_binary_op!(i64x2_add, overflowing_add);
define_i64x2_simd_binary_op!(i64x2_sub, overflowing_sub);
define_i64x2_simd_binary_op!(i64x2_mul, overflowing_mul);
define_i64x2_simd_unary_op!(i64x2_neg, overflowing_neg);

define_i64x2_simd_shift_op!(i64x2_shl => wrapping_shl, i64);
define_i64x2_simd_shift_op!(i64x2_shr_s => wrapping_shr, i64);
define_i64x2_simd_shift_op!(i64x2_shr_u => wrapping_shr, u64);
