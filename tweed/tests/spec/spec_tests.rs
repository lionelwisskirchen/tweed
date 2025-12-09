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
fn spec_tests() {
    run_spec_test(Path::new("./tests/spec/testsuite/start.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/store.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/address.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/align.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/block.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/memory_init.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/conversions.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/float_misc.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/call_indirect.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/f64_cmp.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/i32.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/br.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/table_get.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/local_tee.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/i64.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/global.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/select.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/load.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/if.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/linking.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/binary-leb128.wast"));
}
