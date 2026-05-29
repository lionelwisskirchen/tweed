#[path = "helper/mod.rs"]
mod helper;
use std::path::Path;
use test_log;

use helper::run::execute_spec_test;
fn run_spec_test(path: &Path) {
    let result = execute_spec_test(path.to_str().expect("Malformed path"));
    match result {
        Err(e) => {
            println!("{e}");
            assert!(false)
        }
        Ok(report) => {
            println!("{report}");
            assert_eq!(report.failure, vec![]);
        }
    }
}

#[test_log::test]
fn spec_tests_address() {
    run_spec_test(Path::new("./tests/spec/testsuite/address.wast"));
}
#[test_log::test]
fn spec_tests_align() {
    run_spec_test(Path::new("./tests/spec/testsuite/align.wast"));
}

#[test_log::test]
fn spec_tests_br() {
    run_spec_test(Path::new("./tests/spec/testsuite/br.wast"));
}

#[test_log::test]
fn spec_tests_br_if() {
    run_spec_test(Path::new("./tests/spec/testsuite/br_if.wast"));
}

#[test_log::test]
fn spec_tests_block() {
    run_spec_test(Path::new("./tests/spec/testsuite/block.wast"));
}

#[test_log::test]
fn spec_tests_call() {
    run_spec_test(Path::new("./tests/spec/testsuite/call.wast"));
}

#[test_log::test]
fn spec_tests_call_indirect() {
    run_spec_test(Path::new("./tests/spec/testsuite/call_indirect.wast"));
}

#[test_log::test]
fn spec_tests_comments() {
    run_spec_test(Path::new("./tests/spec/testsuite/comments.wast"));
}

#[test_log::test]
fn spec_tests_const() {
    run_spec_test(Path::new("./tests/spec/testsuite/const.wast"));
}

#[test_log::test]
fn spec_tests_conversions() {
    run_spec_test(Path::new("./tests/spec/testsuite/conversions.wast"));
}

#[test_log::test]
fn spec_tests_endianess() {
    run_spec_test(Path::new("./tests/spec/testsuite/endianness.wast"));
}

#[test_log::test]
fn spec_tests_f32() {
    run_spec_test(Path::new("./tests/spec/testsuite/f32.wast"));
}

#[test_log::test]
fn spec_tests_f32_bitwise() {
    run_spec_test(Path::new("./tests/spec/testsuite/f32_bitwise.wast"));
}

#[test_log::test]
fn spec_tests_f32_cmp() {
    run_spec_test(Path::new("./tests/spec/testsuite/f32_cmp.wast"));
}

#[test_log::test]
fn spec_tests_f64() {
    run_spec_test(Path::new("./tests/spec/testsuite/f64.wast"));
}

#[test_log::test]
fn spec_tests_f64_bitwise() {
    run_spec_test(Path::new("./tests/spec/testsuite/f64_bitwise.wast"));
}

#[test_log::test]
fn spec_tests_f64_cmp() {
    run_spec_test(Path::new("./tests/spec/testsuite/f64_cmp.wast"));
}

#[test_log::test]
fn spec_tests_float_exprs() {
    run_spec_test(Path::new("./tests/spec/testsuite/float_exprs.wast"));
}

#[test_log::test]
fn spec_tests_float_literals() {
    run_spec_test(Path::new("./tests/spec/testsuite/float_literals.wast"));
}

#[test_log::test]
fn spec_tests_float_memory() {
    run_spec_test(Path::new("./tests/spec/testsuite/float_memory.wast"));
}

#[test_log::test]
fn spec_tests_float_misc() {
    run_spec_test(Path::new("./tests/spec/testsuite/float_misc.wast"));
}

#[test_log::test]
fn spec_tests_forward() {
    run_spec_test(Path::new("./tests/spec/testsuite/forward.wast"));
}

#[test_log::test]
fn spec_tests_global() {
    run_spec_test(Path::new("./tests/spec/testsuite/global.wast"));
}

#[test_log::test]
fn spec_tests_i32() {
    run_spec_test(Path::new("./tests/spec/testsuite/i32.wast"));
}

#[test_log::test]
fn spec_tests_i64() {
    run_spec_test(Path::new("./tests/spec/testsuite/i64.wast"));
}

#[test_log::test]
fn spec_tests_if() {
    run_spec_test(Path::new("./tests/spec/testsuite/if.wast"));
}

#[test_log::test]
fn spec_tests_inline_module() {
    run_spec_test(Path::new("./tests/spec/testsuite/inline-module.wast"));
}

#[test_log::test]
fn spec_tests_binary_leb128() {
    run_spec_test(Path::new("./tests/spec/testsuite/binary-leb128.wast"));
}

#[test_log::test]
fn spec_tests_int_exprs() {
    run_spec_test(Path::new("./tests/spec/testsuite/int_exprs.wast"));
}

#[test_log::test]
fn spec_tests_int_literals() {
    run_spec_test(Path::new("./tests/spec/testsuite/int_literals.wast"));
}

#[test_log::test]
fn spec_tests_left_to_right() {
    run_spec_test(Path::new("./tests/spec/testsuite/left-to-right.wast"));
}

#[test_log::test]
fn spec_tests_linker() {
    run_spec_test(Path::new("./tests/spec/testsuite/linking.wast"));
}

#[test_log::test]
fn spec_tests_load() {
    run_spec_test(Path::new("./tests/spec/testsuite/load.wast"));
}

#[test_log::test]
fn spec_tests_local_get() {
    run_spec_test(Path::new("./tests/spec/testsuite/local_get.wast"));
}

#[test_log::test]
fn spec_tests_local_set() {
    run_spec_test(Path::new("./tests/spec/testsuite/local_set.wast"));
}

#[test_log::test]
fn spec_tests_local_tee() {
    run_spec_test(Path::new("./tests/spec/testsuite/local_tee.wast"));
}

#[test_log::test]
fn spec_tests_loop() {
    run_spec_test(Path::new("./tests/spec/testsuite/loop.wast"));
}

#[test_log::test]
fn spec_tests_memory() {
    run_spec_test(Path::new("./tests/spec/testsuite/memory.wast"));
}

#[test_log::test]
fn spec_tests_memory_init() {
    run_spec_test(Path::new("./tests/spec/testsuite/memory_init.wast"));
}

#[test_log::test]
fn spec_tests_names() {
    run_spec_test(Path::new("./tests/spec/testsuite/names.wast"));
}

#[test_log::test]
fn spec_tests_nop() {
    run_spec_test(Path::new("./tests/spec/testsuite/nop.wast"));
}

#[test_log::test]
fn spec_tests_return() {
    run_spec_test(Path::new("./tests/spec/testsuite/return.wast"));
}

#[test_log::test]
fn spec_tests_start() {
    run_spec_test(Path::new("./tests/spec/testsuite/start.wast"));
}

#[test_log::test]
fn spec_tests_store() {
    run_spec_test(Path::new("./tests/spec/testsuite/store.wast"));
}

#[test_log::test]
fn spec_tests_switch() {
    run_spec_test(Path::new("./tests/spec/testsuite/switch.wast"));
}

#[test_log::test]
fn spec_tests_table() {
    run_spec_test(Path::new("./tests/spec/testsuite/table.wast"));
}

#[test_log::test]
fn spec_tests_simd_address() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_address.wast"));
}

#[test_log::test]
fn spec_tests_simd_align() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_align.wast"));
}

#[test_log::test]
fn spec_tests_simd_bit_shift() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_bit_shift.wast"));
}

#[test_log::test]
fn spec_tests_simd_bitwise() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_bitwise.wast"));
}

#[test_log::test]
fn spec_tests_simd_const() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_const.wast"));
}

#[test_log::test]
fn spec_tests_simd_i8x16_arith() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_i8x16_arith.wast"));
}

#[test_log::test]
fn spec_tests_simd_i16x8_arith() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_i16x8_arith.wast"));
}

#[test_log::test]
fn spec_tests_simd_i32x4_arith() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_i32x4_arith.wast"));
}

#[test_log::test]
fn spec_tests_simd_i64x2_arith() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_i64x2_arith.wast"));
}

#[test_log::test]
fn spec_tests_simd_linking() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_linking.wast"));
}

// #[test_log::test]
// fn spec_tests_simd_load() {
//     // depends on:
//     // - simd_extract_land
//     // - eq
//     // - not
//     // - all-true
//     // - bitselect
//     // - shl
//     run_spec_test(Path::new("./tests/spec/testsuite/simd_load.wast"));
// }

#[test_log::test]
fn spec_tests_simd_store() {
    run_spec_test(Path::new("./tests/spec/testsuite/simd_store.wast"));
}

#[test_log::test]
fn spec_tests_table_get() {
    run_spec_test(Path::new("./tests/spec/testsuite/table_get.wast"));
}

#[test_log::test]
fn spec_tests_table_set() {
    run_spec_test(Path::new("./tests/spec/testsuite/table_set.wast"));
}

#[test_log::test]
fn spec_tests_token() {
    run_spec_test(Path::new("./tests/spec/testsuite/token.wast"));
}

#[test_log::test]
fn spec_tests_traps() {
    run_spec_test(Path::new("./tests/spec/testsuite/traps.wast"));
}

#[test_log::test]
fn spec_tests_type() {
    run_spec_test(Path::new("./tests/spec/testsuite/type.wast"));
}

#[test_log::test]
fn spec_tests_unreachable() {
    run_spec_test(Path::new("./tests/spec/testsuite/unreachable.wast"));
}

#[test_log::test]
fn spec_tests_unwind() {
    run_spec_test(Path::new("./tests/spec/testsuite/unwind.wast"));
}

#[test_log::test]
fn spec_tests_utf8_invalid_encoding() {
    run_spec_test(Path::new(
        "./tests/spec/testsuite/utf8-invalid-encoding.wast",
    ));
}
