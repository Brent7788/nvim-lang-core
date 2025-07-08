use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nvim_lang_core::{
    common::test::{get_bench_path, get_test_comment_path},
    nvim_lang_core::NvimLangCore,
    nvim_lang_dictionary::NvimLanguageReadonlyDictionary,
    nvim_language::{self, core::NvimLanguageCore},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let core = NvimLangCore::new(None, None);

    let mut group = c.benchmark_group("Nvim Lang Bench");
    let file_path = get_bench_path();

    println!("BENCH=== {}", file_path);
    group.measurement_time(Duration::new(80, 0));
    group.sample_size(10);

    group.bench_function("Programming Lang bench", |b| {
        b.iter(|| core.process_file(file_path.clone(), None))
    });

    let file_path = "/home/brent/Documents/pojects/nvim-lang-core/src/lib.rs".to_owned();

    println!("BENCH=== {}", file_path);
    group.measurement_time(Duration::new(30, 0));
    group.sample_size(10);

    group.bench_function("Bench src/lib.rs", |b| {
        b.iter(|| core.process_file(file_path.clone(), None))
    });

    let file_path = "/home/brent/Documents/pojects/nvim-lang-core/tests/file_test_cases/rust/codes/multiple_code.rs".to_owned();

    println!("BENCH=== {}", file_path);
    group.sample_size(20);

    group.bench_function("Bench Multiple Code Test", |b| {
        b.iter(|| core.process_file(file_path.clone(), None))
    });

    let core = NvimLanguageCore::new(None, None);

    let file_path = get_bench_path();

    println!("BENCH=== {}", file_path);
    group.measurement_time(Duration::new(80, 0));
    group.sample_size(10);

    group.bench_function("Programming Lang bench V2", |b| {
        b.iter(|| core.process_file(file_path.clone(), NvimLanguageReadonlyDictionary::new()))
    });

    let file_path = "/home/brent/Documents/pojects/nvim-lang-core/src/lib.rs".to_owned();

    println!("BENCH=== {}", file_path);
    group.measurement_time(Duration::new(30, 0));
    group.sample_size(10);

    group.bench_function("Bench src/lib.rs V2", |b| {
        b.iter(|| core.process_file(file_path.clone(), NvimLanguageReadonlyDictionary::new()))
    });

    let file_path = "/home/brent/Documents/pojects/nvim-lang-core/tests/file_test_cases/rust/codes/multiple_code.rs".to_owned();

    println!("BENCH=== {}", file_path);
    group.sample_size(20);

    group.bench_function("Bench Multiple Code Test V2", |b| {
        b.iter(|| core.process_file(file_path.clone(), NvimLanguageReadonlyDictionary::new()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
