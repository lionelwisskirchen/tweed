#[path = "helper/mod.rs"]
mod helper;
use std::path::Path;
use test_log;

use helper::run::execute_spec_test;
fn run_spec_test(path: &Path) {
    let result = execute_spec_test(path.to_str().expect("Malformed path"));
    match result {
        Err(_) => {
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
    run_spec_test(Path::new("./tests/spec/testsuite/linking.wast"));
    run_spec_test(Path::new("./tests/spec/br.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/local_tee.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/load.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/i32.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/global.wast"));
    run_spec_test(Path::new("./tests/spec/testsuite/table_get.wast"));
}
