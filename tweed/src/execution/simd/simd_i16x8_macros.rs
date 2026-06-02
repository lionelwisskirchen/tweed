use crate::{
    core::{error::ExecutionError, values::V128Shape},
    execution::execution_stack::ExecutionStack,
    Value,
};

pub type Result<T> = core::result::Result<T, ExecutionError>;

/// Generates a unary SIMD operation function for i16x8 vectors.
///
/// This macro creates a function that applies a unary operation to each lane
/// of an i16x8 vector. The operation should be a method that returns a tuple
/// where the first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Unary method to call on each lane (e.g., `abs`, `overflowing_neg`)
macro_rules! define_i16x8_simd_unary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i16x8 vector.")]
        ///
        /// This Method poppes one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let value = stack.pop_v128().to_i16x8();

            let result = core::array::from_fn(|i| value[i].$method().0);

            stack.push(Value::V128(V128Shape::I16x8(result)))?;

            Ok(())
        }
    };
}

/// Generates a binary SIMD operation function for i16x8 vectors.
///
/// This macro creates a function that applies a binary operation to each lane
/// of an i16x8 vector. The given operation should be a method that returns a tuple where the
/// first element is the result (e.g., `overflowing_*` methods).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$method` - Binary method to call on each lane (e.g., `overflowing_add`, `overflowing_sub`)
macro_rules! define_i16x8_simd_binary_op {
    ($func_name:ident, $method:ident) => {
        #[doc = concat!("Computes `", stringify!($method), "` on each lane of an i16x8 vector.")]
        ///
        /// This Method poppes two v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let (right, left) = (stack.pop_v128().to_i16x8(), stack.pop_v128().to_i16x8());

            let mut result = [0i16; 8];

            for (idx, item) in right.iter().enumerate() {
                result[idx] = (left[idx]).$method(*item).0;
            }
            stack.push(Value::V128(V128Shape::I16x8(result)))?;

            Ok(())
        }
    };
}

/// Generates a shift SIMD operation function for i16x8 vectors.
///
/// This macro creates a function that applies a shift operation to each lane
/// of an i16x8 vector. The given operation should be a method that shift a given i16 to the left or
/// the right (e.g., `wrapping_shl`, ..).
///
/// # Arguments
/// * `$func_name` - Name of the generated function
/// * `$op` - The shift operation to call on each lane
/// * `$shift_type` - An intermediate type in which the shift should take place
macro_rules! define_i16x8_simd_shift_op {
    ($func_name:ident => $op:ident, $shift_type:ty) => {
        #[doc = concat!("Computes `", stringify!($op), "` on each lane of an i8x16 vector.")]
        ///
        /// This Method pops one i32 and one v128 of the given [ExecutionStack]. After executing
        /// the operation, the result is pushed back to the stack.
        ///
        /// # Arguments
        /// * `stack` - a mutable reference to the [ExecutionStack] to work with
        pub fn $func_name(stack: &mut ExecutionStack) -> Result<()> {
            let rhs = (stack.pop_i32() % 16) as u32;
            let mut v128_value = stack.pop_v128().to_i16x8();

            v128_value = v128_value.map(|v| (v as $shift_type).$op(rhs) as i16);

            stack.push(Value::V128(V128Shape::I16x8(v128_value)))?;
            Ok(())
        }
    };
}

define_i16x8_simd_binary_op!(i16x8_add, overflowing_add);
define_i16x8_simd_binary_op!(i16x8_sub, overflowing_sub);
define_i16x8_simd_binary_op!(i16x8_mul, overflowing_mul);
define_i16x8_simd_unary_op!(i16x8_neg, overflowing_neg);

define_i16x8_simd_shift_op!(i16x8_shl => wrapping_shl, i16);
define_i16x8_simd_shift_op!(i16x8_shr_s => wrapping_shr, i16);
define_i16x8_simd_shift_op!(i16x8_shr_u => wrapping_shr, u16);

/// Check if all lanes are larger than zero.
///
/// This function pops one v128 values from the stack pushes back one i32.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn i16x8_all_true(stack: &mut ExecutionStack) -> Result<()> {
    let res = stack.pop_v128().to_i16x8().iter().all(|&i| i != 0) as i32;
    stack.push(Value::I32(res))?;
    Ok(())
}

/// Computes the `i8x16.bitmask` operation on a v128 value.
///
/// Checks which lane is positive and which lane is negative.
/// For lane[0] = -1, result:0 = 1 => result = 0x00000001
///
/// This method pops one v128 from the given [ExecutionStack]. After executing
/// the operation, the result is pushed back to the stack as an i32.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn i16x8_bitmask(stack: &mut ExecutionStack) -> Result<()> {
    let v = stack.pop_v128().to_i16x8();
    let mut result = 0;

    for (idx, &lane) in v.iter().enumerate() {
        if lane < 0 {
            result = result | (1 << idx);
        }
    }

    stack.push(Value::I32(result))?;
    Ok(())
}

/// Takes two i32x4 and combines them in a i16x8 vec
///
/// This function fills the first 4 lanes of an i8x16 vec with one i32x4 and the second 4 lanes
/// with the other i32x4. Each i32 lane is clamped to the u16 range (-327680 – 32768) before truncation.
///
/// This method pops one v128 from the given [ExecutionStack]. After executing
/// the operation, the result is pushed back to the stack as an v128.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn i16_8_narrow_i32_s(stack: &mut ExecutionStack) -> Result<()> {
    let v = stack.pop_v128().to_i32x4();
    let v2 = stack.pop_v128().to_i32x4();
    let mut result = [0i16; 8];

    for idx in 0..4 {
        result[idx] = v2[idx].clamp(-32768, 32767) as i16;
        result[idx + 4] = v[idx].clamp(-32768, 32767) as i16;
    }

    stack.push(Value::V128(V128Shape::I16x8(result)))?;

    Ok(())
}

/// Takes two i32x4 and combines them in a i16x8 vec
///
/// This function fills the first 4 lanes of an i8x16 vec with one i32x4 and the second 4 lanes
/// with the other i32x4. Each i32 lane is clamped to the u16 range (0–65535) before truncation.
///
/// This method pops one v128 from the given [ExecutionStack]. After executing
/// the operation, the result is pushed back to the stack as an v128.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
pub fn i16_8_narrow_i32_u(stack: &mut ExecutionStack) -> Result<()> {
    let v = stack.pop_v128().to_i32x4();
    let v2 = stack.pop_v128().to_i32x4();
    let mut result = [0i16; 8];
    for idx in 0..4 {
        result[idx] = (v2[idx].clamp(0, 65535) as u16) as i16;
        result[idx + 4] = (v[idx].clamp(0, 65535) as u16) as i16;
    }

    stack.push(Value::V128(V128Shape::I16x8(result)))?;

    Ok(())
}

/// Extends a i8x16 vec to a i16x8 vec
///
/// This function takes the first 8 lanes of an i8x16 vec and stores them as i16 in a new v128
///
/// This method pops one v128 from the given [ExecutionStack]. After executing
/// the operation, the result is pushed back to the stack as an v128.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
///
/// # Examples
/// ```
/// // [-1, 127, -128, 0, ...ignored...]
/// //  ↓
/// // [-1, 127, -128, 0]  (sign-extended to i16)
/// ```
pub fn i16_8_extend_low_i8x16_s(stack: &mut ExecutionStack) -> Result<()> {
    let v = stack.pop_v128().to_i8x16();
    let mut result = [0i16; 8];
    for idx in 0..8 {
        result[idx] = v[idx] as i16;
    }

    stack.push(Value::V128(V128Shape::I16x8(result)))?;

    Ok(())
}

/// Extends a i8x16 vec to a i16x8 vec
///
/// This function takes the first 8 lanes of an i8x16 vec and stores them as i16 in a new v128
///
/// This method pops one v128 from the given [ExecutionStack]. After executing
/// the operation, the result is pushed back to the stack as an v128.
///
/// # Arguments
/// * `stack`: a mutable reference to the [ExecutionStack] on which the new const will be stored
///
/// # Examples
/// ```
/// // [-1, 127, -128, 0, ...ignored...]
/// //  ↓
/// // [255, 127, 128, 0]  (zero-extended, i8 bits reinterpreted as u8)
pub fn i16_8_extend_low_i8x16_u(stack: &mut ExecutionStack) -> Result<()> {
    let v = stack.pop_v128().to_i8x16();
    let mut result = [0i16; 8];
    for idx in 0..8 {
        result[idx] = (v[idx] as u8) as i16;
    }

    stack.push(Value::V128(V128Shape::I16x8(result)))?;

    Ok(())
}
