use std::{self, fs};
use tweed::execution::instance::Instance;
use tweed::execution::linker::Linker;
use tweed::{initialize, Value};
use wast::core::{WastArgCore, WastRetCore};
use wast::{WastArg, WastDirective, WastRet};

use super::wast_report::{WastReport, WastResult};

pub fn execute_spec_test(path: &str) -> Result<WastReport, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(path).unwrap();
    let buf = wast::parser::ParseBuffer::new(&input)?;
    let wast = wast::parser::parse::<wast::Wast>(&buf)?;
    let mut report = WastReport {
        file: path.to_string(),
        ..Default::default()
    };
    let mut instance: Instance = Default::default();

    for directive in wast.directives {
        match directive {
            WastDirective::Module(mut module) => {
                let module_binary = module.encode()?;
                let test_module = initialize(&module_binary).unwrap();
                let mut linker = Linker::new(test_module);
                linker.auto_link_spectest_globals().unwrap();
                instance = linker.instantiate().unwrap();
            }
            WastDirective::AssertInvalid {
                span,
                mut module,
                message,
            } => match module.encode() {
                Ok(bytes) => {
                    let result = WastResult::AssertResult {
                        name: "Invalid test".to_string(),
                        line: span.linecol_in(&input).0 + 1,
                    };
                    if let Err(e) = initialize(&bytes) {
                        if format!("{e}") == message {
                            report.success.push(result);
                        } else {
                            report.failure.push(result);
                        }
                    } else {
                        report.failure.push(result);
                    }
                }
                _ => panic!("Testsuit: Error encoding the test wat."),
            },
            WastDirective::AssertReturn {
                exec,
                results,
                span,
            } => {
                let (name, args) = match exec {
                    wast::WastExecute::Invoke(invoke) => (invoke.name, invoke.args),
                    _ => unreachable!(),
                };

                let args: Vec<Value> = args.iter().map(args_to_runtime_value).collect();
                let expected_results: Vec<Value> =
                    results.iter().map(result_to_runtime_value).collect();
                let actual_results = instance.call_by_name(name, args).unwrap();
                let result = WastResult::AssertResult {
                    name: name.to_string(),
                    line: span.linecol_in(&input).0 + 1,
                };
                if expected_results == actual_results {
                    report.success.push(result);
                } else {
                    report.failure.push(result);
                };
            }
            WastDirective::AssertMalformed {
                span,
                mut module,
                message,
            } => match module {
                wast::QuoteWat::QuoteModule(_, _) => {
                    // Ignore this branch as it is only relevant for wat parsing
                }
                _ => match module.encode() {
                    Ok(bytes) => {
                        let result = WastResult::AssertResult {
                            name: "Invalid test".to_string(),
                            line: span.linecol_in(&input).0 + 1,
                        };
                        if let Err(e) = initialize(&bytes) {
                            if format!("{e}") == message {
                                report.success.push(result);
                            } else {
                                report.failure.push(result);
                                println!("{e}");
                            }
                        } else {
                            report.failure.push(result);
                        }
                    }
                    _ => panic!("Testsuit: Error encoding the test wat."),
                },
            },

            _ => {}
        }
    }
    Ok(report)
}

fn result_to_runtime_value(expr: &WastRet) -> Value {
    match &expr {
        WastRet::Core(WastRetCore::I32(x)) => Value::I32(*x),
        WastRet::Core(WastRetCore::I64(x)) => Value::I64(*x),
        //      wast::Instruction::F32Const(x) => RuntimeValue::F32(f32::from_bits(x.bits)),
        //        wast::Instruction::F64Const(x) => RuntimeValue::F64(f64::from_bits(x.bits)),
        _ => unreachable!("{:?}", expr),
    }
}

fn args_to_runtime_value(expr: &WastArg) -> Value {
    match &expr {
        WastArg::Core(WastArgCore::I32(x)) => Value::I32(*x),
        WastArg::Core(WastArgCore::I64(x)) => Value::I64(*x),
        //      wast::Instruction::F32Const(x) => RuntimeValue::F32(f32::from_bits(x.bits)),
        //        wast::Instruction::F64Const(x) => RuntimeValue::F64(f64::from_bits(x.bits)),
        _ => unreachable!("{:?}", expr),
    }
}
