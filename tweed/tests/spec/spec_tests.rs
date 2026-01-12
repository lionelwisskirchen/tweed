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
fn spec_tests_conversions() {
    run_spec_test(Path::new("./tests/spec/testsuite/conversions.wast"));
}

#[test_log::test]
fn spec_tests_const() {
    run_spec_test(Path::new("./tests/spec/testsuite/const.wast"));
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
fn spec_tests_global() {
    run_spec_test(Path::new("./tests/spec/testsuite/global.wast"));
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
fn spec_tests_local_tee() {
    run_spec_test(Path::new("./tests/spec/testsuite/local_tee.wast"));
}

#[test_log::test]
fn spec_tests_load() {
    run_spec_test(Path::new("./tests/spec/testsuite/load.wast"));
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
fn spec_tests_select() {
    run_spec_test(Path::new("./tests/spec/testsuite/select.wast"));
}
#[test_log::test]
fn spec_tests_store() {
    run_spec_test(Path::new("./tests/spec/testsuite/store.wast"));
}

#[test_log::test]
fn spec_tests_table() {
    run_spec_test(Path::new("./tests/spec/testsuite/table.wast"));
}

#[test_log::test]
fn spec_tests_table_get() {
    run_spec_test(Path::new("./tests/spec/testsuite/table_get.wast"));
}

#[test_log::test]
fn spec_tests_binary_leb128() {
    run_spec_test(Path::new("./tests/spec/testsuite/binary-leb128.wast"));
}
