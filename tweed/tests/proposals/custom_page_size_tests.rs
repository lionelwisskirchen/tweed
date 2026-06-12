#[path = "../spec/helper/mod.rs"]
mod helper;
use helper::run::execute_spec_test;
use std::path::Path;
use test_log;

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
fn proposals_custom_page_size() {
    run_spec_test(Path::new(
        "./tests/spec/testsuite/proposals/custom-page-sizes/custom-page-sizes.wast",
    ));
}

#[test_log::test]
fn proposals_custom_page_sizes_invalid() {
    run_spec_test(Path::new(
        "./tests/spec/testsuite/proposals/custom-page-sizes/custom-page-sizes-invalid.wast",
    ));
}

#[test_log::test]
fn proposals_custom_page_sizes_memory_max() {
    run_spec_test(Path::new(
        "./tests/spec/testsuite/proposals/custom-page-sizes/memory_max.wast",
    ));
}
