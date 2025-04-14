use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use inplace_wasm_interpreter::{execution::instance::instantiate, initialize, invoke, Value};

const WASM: &[u8] = include_bytes!("../../esp32c3/src/sum_example.wasm");

pub fn criterion_benchmark(c: &mut Criterion) {
    let test_values = vec![1792938];

    let mut group = c.benchmark_group("Interpreter Benchmark");

    for &val in &test_values {
        group.bench_with_input(BenchmarkId::new("Tweed", val), &val, |b, &_val| {
            b.iter(|| {
                let module = initialize(WASM).expect("Initialization failed");
                let instance = instantiate(module.clone()).expect("Instantiation failed");
                invoke(instance, "sum".to_string(), vec![Value::I32(val)])
                    .expect("Execution failed");
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
