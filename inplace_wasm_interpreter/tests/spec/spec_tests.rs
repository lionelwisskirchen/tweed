#[path = "helper/mod.rs"]
mod helper;
use std::path::Path;

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

#[test]
fn br() {
    run_spec_test(Path::new("./tests/spec/br.wast"));
}
