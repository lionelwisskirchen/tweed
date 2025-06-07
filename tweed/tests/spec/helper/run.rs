use std::{self, fs};
use tweed::core::wasm_reader::WasmReader;
use tweed::execution::instance::Instance;
use tweed::execution::linker::Linker;
use tweed::execution::store::Store;
use tweed::{initialize, Value};
use wast::core::{HeapType, WastArgCore, WastRetCore};
use wast::token::Index;
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
    let mut store: Store = Default::default();
    let mut reader: WasmReader = Default::default();
    for directive in wast.directives {
        match directive {
            WastDirective::Module(mut module) => {
                let module_binary = module.encode()?;
                let mut test_module = initialize(&module_binary).unwrap();
                let mut linker = Linker::new(&mut test_module);
                linker.auto_link_spectest_globals().unwrap();
                let name = match module.name() {
                    Some(n) => Some(n.name()),
                    None => None,
                };

                instance = linker.instantiate(&mut store, name).unwrap();
                reader = test_module.reader;
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
                let actual_results = instance
                    .call_by_name(name, args, &mut store, &mut reader)
                    .unwrap();
                let result = WastResult::AssertResult {
                    name: name.to_string(),
                    line: span.linecol_in(&input).0 + 1,
                };
                if expected_results == actual_results {
                    report.success.push(result);
                } else {
                    println!("expectd {expected_results:?} actual {actual_results:?}");
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
            WastDirective::Invoke(invoke) => {
                let args: Vec<Value> = invoke.args.iter().map(args_to_runtime_value).collect();
                let name = match invoke.module {
                    Some(n) => Some(n.name()),
                    None => None,
                };
                let mut instance = store.get_instance(name).unwrap();
                instance
                    .call_by_name(invoke.name, args, &mut store, &mut reader)
                    .unwrap();
            }

            WastDirective::AssertTrap {
                span,
                exec,
                message,
            } => match exec {
                wast::WastExecute::Invoke(invoke) => {
                    let args = invoke.args.iter().map(args_to_runtime_value).collect();
                    let name = match invoke.module {
                        Some(n) => Some(n.name()),
                        None => None,
                    };
                    let mut instance = store.get_instance(name).unwrap();
                    let result = instance.call_by_name(invoke.name, args, &mut store, &mut reader);
                    let report_result = WastResult::AssertResult {
                        name: invoke.name.to_string(),
                        line: span.linecol_in(&input).0 + 1,
                    };
                    match result {
                        Err(e) => {
                            if format!("{e}") == message {
                                report.success.push(report_result);
                            } else {
                                report.failure.push(report_result);
                            }
                        }
                        Ok(_) => {
                            report.failure.push(report_result);
                        }
                    };
                }
                _ => todo!(),
            },
            _ => {
                unreachable!()
            }
        }
    }
    Ok(report)
}

fn result_to_runtime_value(expr: &WastRet) -> Value {
    match &expr {
        WastRet::Core(WastRetCore::I32(x)) => Value::I32(*x),
        WastRet::Core(WastRetCore::I64(x)) => Value::I64(*x),
        WastRet::Core(WastRetCore::RefFunc(Some(x))) => {
            let Index::Num(i, _) = *x else {
                unreachable!("Access to human readable id");
            };
            Value::FuncRef(i as usize)
        }
        WastRet::Core(WastRetCore::RefFunc(None)) => {
            unreachable!("Expected a non-null func reference");
        }
        WastRet::Core(WastRetCore::RefNull(Some(HeapType::Concrete(_)))) => {
            unreachable!("Null refs don't point to any specific reference")
        }
        WastRet::Core(WastRetCore::RefNull(Some(HeapType::Abstract { shared: _, ty }))) => match ty
        {
            wast::core::AbstractHeapType::Func => {
                Value::RefNull(tweed::core::value_types::RefType::FuncRef)
            }
            wast::core::AbstractHeapType::Extern => {
                Value::RefNull(tweed::core::value_types::RefType::ExternRef)
            }
            _ => todo!("`GC` proposal not yet implemented"),
        },
        WastRet::Core(WastRetCore::RefNull(None)) => {
            unreachable!("Not implemented")
        }

        WastRet::Core(WastRetCore::RefExtern(x)) => {
            return match *x {
                Some(index) => Value::ExternRef(index as usize),
                None => unreachable!("Expected a non-null extern reference"),
            }
        }
        //      wast::Instruction::F32Const(x) => RuntimeValue::F32(f32::from_bits(x.bits)),
        //        wast::Instruction::F64Const(x) => RuntimeValue::F64(f64::from_bits(x.bits)),
        _ => unreachable!("{:?}", expr),
    }
}

fn args_to_runtime_value(expr: &WastArg) -> Value {
    match &expr {
        WastArg::Core(WastArgCore::I32(x)) => Value::I32(*x),
        WastArg::Core(WastArgCore::I64(x)) => Value::I64(*x),
        WastArg::Core(WastArgCore::RefExtern(x)) => Value::ExternRef(*x as usize),

        WastArg::Core(WastArgCore::RefHost(_)) => {
            todo!("`RefHost` value arguments not yet implemented")
        }
        WastArg::Core(WastArgCore::RefNull(rref)) => match rref {
            HeapType::Concrete(_) => {
                unreachable!("Null refs don't point to any specific reference")
            }
            HeapType::Abstract { shared: _, ty } => match ty {
                wast::core::AbstractHeapType::Func => {
                    Value::RefNull(tweed::core::value_types::RefType::FuncRef)
                }
                wast::core::AbstractHeapType::Extern => {
                    Value::RefNull(tweed::core::value_types::RefType::ExternRef)
                }
                _ => todo!("`GC` proposal not yet implemented"),
            },
        },

        //      wast::Instruction::F32Const(x) => RuntimeValue::F32(f32::from_bits(x.bits)),
        //        wast::Instruction::F64Const(x) => RuntimeValue::F64(f64::from_bits(x.bits)),
        _ => unreachable!("{:?}", expr),
    }
}
