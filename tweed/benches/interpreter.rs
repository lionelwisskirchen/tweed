use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tweed::{initialize, instantiate, invoke, Value};

const WASM: &[u8] = include_bytes!("../../esp32c3/src/sum_example.wasm");

pub fn criterion_benchmark(c: &mut Criterion) {
    let test_values = vec![1792938];

    let mut group = c.benchmark_group("Interpreter Benchmark");

    for &val in &test_values {
        group.bench_with_input(BenchmarkId::new("Tweed", val), &val, |b, &_val| {
            b.iter(|| {
                let module = initialize(WASM).expect("Initialization failed");
                let mut instance = instantiate(module.clone()).expect("Instantiation failed");
                invoke(&mut instance, "sum".to_string(), vec![Value::I32(val)])
                    .expect("Execution failed");
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
