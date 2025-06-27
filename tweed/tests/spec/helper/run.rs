use log::info;
use std::collections::HashMap;
use std::{self, fs};
use tweed::binary::type_section::FuncType;
use tweed::core::value_types::ValType;
use tweed::execution::linker::Linker;
use tweed::execution::store::Store;
use tweed::{initialize, invoke, Value};
use wast::core::{HeapType, WastArgCore, WastRetCore};
use wast::token::Index;
use wast::{WastArg, WastRet};

use super::wast_report::{WastReport, WastResult};

#[derive(Default)]
struct Environment {
    modules: HashMap<String, String>,
    last_module: Option<String>,
}

impl Environment {
    fn register(&mut self, name: &str, addr: &str) {
        self.modules.insert(name.to_string(), addr.to_string());
        self.last_module = Some(addr.to_string());
    }
    fn last(&self) -> Option<String> {
        self.last_module.clone()
    }
    fn get<'a>(&self, module_id: Option<wast::token::Id<'_>>) -> Option<String> {
        match module_id {
            Some(n) => self.modules.get(n.name()).cloned(),
            None => self.last_module.clone(),
        }
    }
    fn update_last(&mut self, name: &str) {
        self.last_module = Some(name.to_string());
    }
}

pub fn convert_name<'a>(name: Option<wast::token::Id<'_>>) -> Option<String> {
    match name {
        Some(n) => Some(n.name().to_string()),
        None => None,
    }
}
pub fn execute_spec_test(path: &str) -> Result<WastReport, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(path).unwrap();
    let buf = wast::parser::ParseBuffer::new(&input)?;
    let wast = wast::parser::parse::<wast::Wast>(&buf)?;
    let mut report = WastReport {
        file: path.to_string(),
        ..Default::default()
    };
    let mut module_registry = Environment::default();
    let mut store: Store = Default::default();
    let mut linker: Linker = Linker::default();
    linker
        .add_global_import(
            "spectest".to_string(),
            "global_i32".to_string(),
            Value::I32(42),
            true,
        )
        .unwrap();
    linker
        .add_global_import(
            "spectest".to_string(),
            "global_i64".to_string(),
            Value::I64(42),
            true,
        )
        .unwrap();

    linker
        .add_function(
            "spectest".to_string(),
            "print_i32".to_string(),
            FuncType {
                parameter: vec![ValType::I32],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();

    for directive in wast.directives {
        use wast::WastDirective::{
            AssertExhaustion, AssertInvalid, AssertMalformed, AssertReturn, AssertTrap,
            AssertUnlinkable, Invoke, Module, Register,
        };

        match directive {
            Register { name, module, .. } => match module {
                None => {
                    let Some(last) = module_registry.last() else {
                        unreachable!("No Module registered");
                    };
                    module_registry.register(name, &last);
                    store.rename_instance(&last, name).unwrap();
                    let instance = store.get_instance(&name).unwrap();
                    linker.instance(instance).unwrap();
                }
                Some(id) => {
                    let name = module_registry.get(Some(id)).unwrap();
                    let instance = store.get_instance(&name).unwrap();
                    linker.instance(instance).unwrap()
                }
            },
            Module(mut module) => {
                let module_binary = module.encode()?;
                let mut test_module = initialize(&module_binary).unwrap();
                let name = convert_name(module.name());
                let instance = linker
                    .instantiate(&mut test_module, &mut store, name.as_deref())
                    .unwrap();

                if let Some(name) = module.name() {
                    module_registry.register(name.name(), &instance);
                } else {
                    module_registry.update_last(&instance);
                }
            }
            Invoke(invoke_confs) => {
                let args: Vec<Value> = invoke_confs
                    .args
                    .iter()
                    .map(args_to_runtime_value)
                    .collect();
                let module = module_registry.get(invoke_confs.module).unwrap();
                let function = invoke_confs.name.to_string();
                invoke(module, function, args, &mut store).unwrap();
            }
            AssertUnlinkable {
                span,
                module: mut module @ wast::Wat::Module(_),
                message,
            } => {
                let module_binary = module.encode()?;
                let mut test_module = initialize(&module_binary).unwrap();
                let result = WastResult::AssertResult {
                    name: "Invalid test".to_string(),
                    line: span.linecol_in(&input).0 + 1,
                };
                if let Err(e) = linker.instantiate(&mut test_module, &mut store, None) {
                    if format!("{e}") == message {
                        report.success.push(result);
                    } else {
                        report.failure.push(result);
                    }
                } else {
                    info!("No error");
                    report.failure.push(result);
                }
            }
            AssertInvalid {
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
                _ => panic!("Testsuite: Error encoding the test wat."),
            },
            AssertReturn {
                exec,
                results,
                span,
            } => {
                let expected_results: Vec<Value> =
                    results.iter().map(result_to_runtime_value).collect();
                match exec {
                    wast::WastExecute::Invoke(invoke_confs) => {
                        let args: Vec<Value> = invoke_confs
                            .args
                            .iter()
                            .map(args_to_runtime_value)
                            .collect();

                        let module = module_registry.get(invoke_confs.module).unwrap();
                        let actual_results =
                            invoke(module, invoke_confs.name.to_string(), args, &mut store)
                                .unwrap();
                        let result = WastResult::AssertResult {
                            name: invoke_confs.name.to_string(),
                            line: span.linecol_in(&input).0 + 1,
                        };
                        if expected_results == actual_results {
                            report.success.push(result);
                        } else {
                            println!("expected {expected_results:?} actual {actual_results:?}");
                            report.failure.push(result);
                        };
                    }
                    wast::WastExecute::Get {
                        span,
                        module,
                        global,
                    } => {
                        let name = module_registry.get(module).unwrap();
                        let instance = store.get_instance(&name).unwrap();
                        let actual_results =
                            vec![instance.get_global_by_name(global, &store).unwrap().value];

                        let result = WastResult::AssertResult {
                            name: global.to_string(),
                            line: span.linecol_in(&input).0 + 1,
                        };
                        if expected_results == actual_results {
                            report.success.push(result);
                        } else {
                            info!("expected {expected_results:?} actual {actual_results:?}");
                            report.failure.push(result);
                        };
                    }
                    _ => todo!(),
                };
            }
            AssertMalformed {
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

            AssertTrap {
                span,
                exec,
                message,
            } => match exec {
                wast::WastExecute::Invoke(invoke_confs) => {
                    let args: Vec<Value> = invoke_confs
                        .args
                        .iter()
                        .map(args_to_runtime_value)
                        .collect();
                    let name = module_registry.get(invoke_confs.module).unwrap();
                    let result = invoke(
                        name.to_string(),
                        invoke_confs.name.to_string(),
                        args,
                        &mut store,
                    );
                    let report_result = WastResult::AssertResult {
                        name: invoke_confs.name.to_string(),
                        line: span.linecol_in(&input).0 + 1,
                    };
                    match result {
                        Err(e) => {
                            info!("{e} == {message}");
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
            AssertExhaustion {
                span,
                call,
                message,
            } => {
                let args: Vec<Value> = call.args.iter().map(args_to_runtime_value).collect();
                let module = module_registry.get(call.module).unwrap();
                let function = call.name.to_string();
                let result = invoke(module, function, args, &mut store);
                let report_result = WastResult::AssertResult {
                    name: call.name.to_string(),
                    line: span.linecol_in(&input).0 + 1,
                };
                match result {
                    Err(e) => {
                        info!("{e} == {message}");
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
        WastRet::Core(WastRetCore::F64(x)) => match x {
            wast::core::NanPattern::CanonicalNan => {
                Value::F64(f64::from_bits(0x7ff8_0000_0000_0000))
            }
            wast::core::NanPattern::ArithmeticNan => {
                Value::F64(f64::from_bits(0x7ff0_0000_0000_0001))
            }
            wast::core::NanPattern::Value(val) => Value::F64(f64::from_bits(val.bits)),
        },
        WastRet::Core(WastRetCore::F32(x)) => match x {
            wast::core::NanPattern::CanonicalNan => Value::F32(f32::from_bits(0x7fc0_0000)),
            wast::core::NanPattern::ArithmeticNan => Value::F32(f32::from_bits(0x7f80_0001)),
            wast::core::NanPattern::Value(val) => Value::F32(f32::from_bits(val.bits)),
        },
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
        _ => todo!(),
    }
}

fn args_to_runtime_value(expr: &WastArg) -> Value {
    match &expr {
        WastArg::Core(WastArgCore::I32(x)) => Value::I32(*x),
        WastArg::Core(WastArgCore::I64(x)) => Value::I64(*x),
        WastArg::Core(WastArgCore::F64(x)) => Value::F64(f64::from_bits(x.bits)),
        WastArg::Core(WastArgCore::F32(x)) => Value::F32(f32::from_bits(x.bits)),
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
        _ => unreachable!("{:?}", expr),
    }
}
