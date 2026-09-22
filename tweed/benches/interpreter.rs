use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tweed::{initialize, Value};
use wat;

#[cfg(feature = "std")]
pub fn criterion_benchmark(c: &mut Criterion) {
    use std::time::Duration;

    let test_values = vec![
        (
            "AHA_MONT64",
            wat::parse_file("benches/aha-mont64.wat").unwrap(),
        ),
        ("CRC32", wat::parse_file("benches/crc32.wat").unwrap()),
        (
            "DEPTHCONV",
            wat::parse_file("benches/depthconv.wat").unwrap(),
        ),
        ("EDN", wat::parse_file("benches/edn.wat").unwrap()),
        (
            "HUFFBENCH",
            wat::parse_file("benches/huffbench.wat").unwrap(),
        ),
        (
            "MATMULT_INT",
            wat::parse_file("benches/matmult-int.wat").unwrap(),
        ),
        ("MD5SUM", wat::parse_file("benches/md5sum.wat").unwrap()),
        (
            "NETTLE_AES",
            wat::parse_file("benches/nettle-aes.wat").unwrap(),
        ),
        (
            "NETTLE_SHA256",
            wat::parse_file("benches/nettle-sha256.wat").unwrap(),
        ),
        ("NSICHNEU", wat::parse_file("benches/nsichneu.wat").unwrap()),
        ("QRDUINO", wat::parse_file("benches/qrduino.wat").unwrap()),
        ("PICOJPEG", wat::parse_file("benches/picojpeg.wat").unwrap()),
        (
            "SGLIB_COMBINED",
            wat::parse_file("benches/sglib-combined.wat").unwrap(),
        ),
        ("SLRE", wat::parse_file("benches/slre.wat").unwrap()),
        (
            "STATEMATE",
            wat::parse_file("benches/statemate.wat").unwrap(),
        ),
        ("TARFIND", wat::parse_file("benches/tarfind.wat").unwrap()),
        ("WIKISORT", wat::parse_file("benches/wikisort.wat").unwrap()),
        ("UD", wat::parse_file("benches/ud.wat").unwrap()),
        ("XGBOOST", wat::parse_file("benches/xgboost.wat").unwrap()),
    ];

    let mut group = c.benchmark_group("Interpreter Benchmark");
    group
        .sample_size(1000)
        .measurement_time(Duration::from_secs(30));

    for val in test_values {
        group.bench_with_input(BenchmarkId::new("Tweed", &val.0), &val, |b, _val| {
            b.iter(|| {
                use tweed::execution::engine::Engine;

                let module = initialize(&val.1).unwrap_or_else(|e| {
                    panic!("Initialization failed: {e}");
                });
                let mut engine = Engine::default();
                let instance = engine.instantiate(module).unwrap_or_else(|e| {
                    panic!("Instantiation failed: {e}");
                });
                let result = engine
                    .invoke(&instance, "_run".to_string(), vec![])
                    .expect("Execution failed");
                if result != vec![Value::I32(0)] {
                    panic!("Embench program failed!");
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
