pub mod simd_execute;
mod simd_i16x8_macros;
mod simd_i32x4_macros;
mod simd_i64x2_macros;
mod simd_i8x16_macros;

#[allow(ambiguous_glob_reexports)]
pub use simd_i16x8_macros::*;
pub use simd_i32x4_macros::*;
pub use simd_i64x2_macros::*;
pub use simd_i8x16_macros::*;
