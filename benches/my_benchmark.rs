// use std::time::Duration;
//
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use nvim_lang_core::{
//     common::test::{get_bench_path, get_test_comment_path},
//     nvim_lang_core::NvimLangCore,
// };
//
// pub fn criterion_benchmark(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Nvim Lang Bench");
//     let core = NvimLangCore::new(None, None);
//
//     let file_path = get_bench_path();
//
//     println!("BENCH=== {}", file_path);
//     group.measurement_time(Duration::new(40, 0));
//     group.sample_size(10);
//
//     group.bench_function("Comment bench", |b| {
//         b.iter(|| core.process_file(file_path.clone()))
//     });
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
