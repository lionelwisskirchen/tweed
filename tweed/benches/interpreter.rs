use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tweed::{initialize, instantiate, invoke};

const AHA_MONT64: &[u8] = include_bytes!("./aha-mont64.wasm");
const CRC32: &[u8] = include_bytes!("./crc32.wasm");

#[cfg(feature = "std")]
pub fn criterion_benchmark(c: &mut Criterion) {
    let test_values = vec![("AHA_MONT64", AHA_MONT64), ("CRC32", CRC32)];

    let mut group = c.benchmark_group("Interpreter Benchmark");

    for &val in &test_values {
        group.bench_with_input(BenchmarkId::new("Tweed", &val.0), &val, |b, &_val| {
            b.iter(|| {
                let module = initialize(val.1).unwrap_or_else(|e| {
                    panic!("Initialization failed: {e}");
                });

                let mut instance = instantiate(module).unwrap_or_else(|e| {
                    panic!("Instantiation failed: {e}");
                });
                invoke(&mut instance, "_run".to_string(), vec![]).expect("Execution failed");
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
