use log::{error, info};
use std::collections::HashMap;
use std::{self, f32, fs};
use tweed::binary::table_section::Table;
use tweed::binary::type_section::FuncType;
use tweed::core::value_types::ValType;
use tweed::core::values::NaN;
use tweed::execution::engine::Engine;

use tweed::{initialize, Value};
use wast::core::{HeapType, WastArgCore, WastRetCore};
use wast::token::Index;
use wast::{WastArg, WastRet, Wat};

use super::wast_report::{WastReport, WastResult};

#[derive(Default, Debug)]
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
    fn count(&self) -> usize {
        self.modules.len()
    }
}

/// Clean the input from confusing unicode
///
/// This is needed to protect against a problem where unicode characters can cause editors to
/// render source code differently for humans than the compiler itself sees. The detection of
/// confusing uincode characters is inspired by the [wast::lexer::LexError].
///
/// To mitigate this issue, and because it's relatively rare in practice,
/// this values are simply overwritten by the "_" character.
pub fn clean_input(input: &String) -> String {
    let result = input
        .replace('\u{202a}', "_")
        .replace('\u{202b}', "_")
        .replace('\u{202d}', "_")
        .replace('\u{202e}', "_")
        .replace('\u{2066}', "_")
        .replace('\u{2067}', "_")
        .replace('\u{2068}', "_")
        .replace('\u{206c}', "_")
        .replace('\u{2069}', "_");

    result
}

pub fn execute_spec_test(path: &str) -> Result<WastReport, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(path).unwrap();
    let clean_input = clean_input(&input);
    let buf = wast::parser::ParseBuffer::new(&clean_input)?;
    let wast = wast::parser::parse::<wast::Wast>(&buf)?;
    let mut report = WastReport {
        file: path.to_string(),
        ..Default::default()
    };
    let mut module_registry = Environment::default();
    let mut engine: Engine = Default::default();
    engine
        .add_memory_import("spectest".to_string(), "memory".to_string(), 1, Some(2))
        .unwrap();
    engine
        .add_global_import(
            "spectest".to_string(),
            "global_i32".to_string(),
            Value::I32(666),
            false,
        )
        .unwrap();
    engine
        .add_global_import(
            "spectest".to_string(),
            "global_i64".to_string(),
            Value::I64(666),
            false,
        )
        .unwrap();
    engine
        .add_global_import(
            "spectest".to_string(),
            "global_f32".to_string(),
            Value::F32(666.6),
            false,
        )
        .unwrap();
    engine
        .add_global_import(
            "spectest".to_string(),
            "global_f64".to_string(),
            Value::F64(666.6),
            false,
        )
        .unwrap();
    engine
        .add_function_import(
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
    engine
        .add_function_import(
            "spectest".to_string(),
            "print_i64".to_string(),
            FuncType {
                parameter: vec![ValType::I64],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();
    engine
        .add_function_import(
            "spectest".to_string(),
            "print_f32".to_string(),
            FuncType {
                parameter: vec![ValType::F32],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();
    engine
        .add_function_import(
            "spectest".to_string(),
            "print_f64".to_string(),
            FuncType {
                parameter: vec![ValType::F64],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();

    engine
        .add_function_import(
            "spectest".to_string(),
            "print_i32_f32".to_string(),
            FuncType {
                parameter: vec![ValType::I32, ValType::F32],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();

    engine
        .add_function_import(
            "spectest".to_string(),
            "print_f64_f64".to_string(),
            FuncType {
                parameter: vec![ValType::F64, ValType::F64],
                result: vec![],
            },
            |_, args| {
                println!("{:?}", args);
                Ok(vec![])
            },
        )
        .unwrap();

    engine
        .add_function_import(
            "spectest".to_string(),
            "print".to_string(),
            FuncType {
                parameter: vec![],
                result: vec![],
            },
            |_, _| Ok(vec![]),
        )
        .unwrap();

    engine
        .add_table_import(
            "spectest".to_string(),
            "table".to_string(),
            Table {
                ref_type: tweed::core::value_types::RefType::FuncRef,
                limit: tweed::binary::memory_section::Limits {
                    min: 10,
                    max: Some(20),
                },
            },
        )
        .unwrap();

    for directive in wast.directives {
        use wast::WastDirective::{
            AssertExhaustion, AssertInvalid, AssertMalformed, AssertReturn, AssertTrap,
            AssertUnlinkable, Invoke, Module, Register,
        };

        println!(
            "Running Directive: {}",
            directive.span().linecol_in(&input).0 + 1
        );

        match directive {
            Register { name, module, .. } => match module {
                None => {
                    let Some(last) = module_registry.last() else {
                        unreachable!("No Module registered");
                    };
                    module_registry.register(name, &last);
                    let instance_ref = engine
                        .register(
                            &name,
                            tweed::execution::engine::InstanceRef::Name(last.to_string()),
                        )
                        .unwrap();
                    engine.register_exports(instance_ref).unwrap();
                }
                Some(id) => {
                    let inst = tweed::execution::engine::InstanceRef::Name(
                        module_registry.get(Some(id)).unwrap().to_string(),
                    );
                    let new_name = engine.register(name, inst).unwrap();
                    engine.register_exports(new_name).unwrap();
                }
            },
            Module(mut module) => {
                let module_binary = module.encode()?;
                let test_module = initialize(&module_binary).unwrap();
                let instance_addr = engine.instantiate(test_module).unwrap();

                if let Some(name) = module.name() {
                    engine.register(name.name(), instance_addr).unwrap();
                    module_registry.register(name.name(), name.name());
                } else {
                    let instance_default_name = format!(
                        "{}#{}",
                        "__module_instance_default__",
                        module_registry.count()
                    );
                    engine
                        .register(&instance_default_name, instance_addr)
                        .unwrap();
                    module_registry.update_last(&instance_default_name);
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
                engine
                    .invoke(
                        tweed::execution::engine::InstanceRef::Name(module),
                        function,
                        args,
                    )
                    .unwrap();
            }
            AssertUnlinkable {
                span,
                module: mut module @ wast::Wat::Module(_),
                message,
            } => {
                let module_binary = module.encode()?;
                let test_module = initialize(&module_binary).unwrap();
                let result = WastResult::AssertResult {
                    name: "Invalid test".to_string(),
                    line: span.linecol_in(&input).0 + 1,
                };
                if let Err(e) = engine.instantiate(test_module) {
                    if format!("{e}") == message {
                        report.success.push(result);
                    } else {
                        info!("Error msg doesn't match: {e} == {message}");
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
                            error!("Error msg doesn't match: {e} == {message}");
                            report.failure.push(result);
                        }
                    } else {
                        info!("No error");
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
                        let actual_results = engine
                            .invoke(
                                tweed::execution::engine::InstanceRef::Name(module),
                                invoke_confs.name.to_string(),
                                args,
                            )
                            .unwrap();
                        let result = WastResult::AssertResult {
                            name: invoke_confs.name.to_string(),
                            line: span.linecol_in(&input).0 + 1,
                        };
                        if compare_results(actual_results.clone(), expected_results.clone()) {
                            report.success.push(result);
                        } else {
                            info!("{actual_results:?} != {expected_results:?}");
                            report.failure.push(result);
                        };
                    }
                    wast::WastExecute::Get {
                        span,
                        module,
                        global,
                    } => {
                        let name = tweed::execution::engine::InstanceRef::Name(
                            module_registry.get(module).unwrap(),
                        );

                        let actual_results =
                            vec![engine.get_global(name, global.to_string()).unwrap().value];
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
                    let result = engine.invoke(
                        tweed::execution::engine::InstanceRef::Name(name.to_string()),
                        invoke_confs.name.to_string(),
                        args,
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
                wast::WastExecute::Wat(Wat::Module(mut module)) => {
                    let report_result = WastResult::AssertResult {
                        name: "Trap Test".to_string(),
                        line: span.linecol_in(&input).0 + 1,
                    };
                    let module_binary = module.encode()?;
                    let test_module = initialize(&module_binary).unwrap();
                    let result = engine.instantiate(test_module);
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
                let result = engine.invoke(
                    tweed::execution::engine::InstanceRef::Name(module),
                    function,
                    args,
                );
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
            wast::core::NanPattern::CanonicalNan => Value::CanonicalNan,
            wast::core::NanPattern::ArithmeticNan => Value::ArithmeticNan,
            wast::core::NanPattern::Value(val) => Value::F64(f64::from_bits(val.bits)),
        },
        WastRet::Core(WastRetCore::F32(x)) => match x {
            wast::core::NanPattern::CanonicalNan => Value::CanonicalNan,
            wast::core::NanPattern::ArithmeticNan => Value::ArithmeticNan,
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
fn compare_results(actual: Vec<Value>, expected: Vec<Value>) -> bool {
    // TODO maybe extract to types
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        match (a, e) {
            (Value::F32(x), Value::F32(y)) => {
                let xb = x.to_bits();
                let yb = y.to_bits();

                return xb == yb;
            }
            (Value::F64(x), Value::F64(y)) => {
                let xb = x.to_bits();
                let yb = y.to_bits();
                return xb == yb;
            }
            (Value::F64(x), Value::ArithmeticNan) => {
                return x.is_arithmetic_nan();
            }
            (Value::F64(x), Value::CanonicalNan) => {
                return x.is_nan(); // TODO correct?
            }
            (Value::F32(x), Value::ArithmeticNan) => {
                return x.is_arithmetic_nan();
            }
            (Value::F32(x), Value::CanonicalNan) => {
                return x.is_nan(); // TODO correct?
            }
            (Value::I32(x), Value::I32(y)) => {
                return x.eq(y);
            }
            (Value::I64(x), Value::I64(y)) => {
                println!("Actual: {:x} vs Expected {:x}", x, y);
                return x.eq(y);
            }
            _ => assert_eq!(a, e, "Mismatch at index {}", i),
        }
    }
    true
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
