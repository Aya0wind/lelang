use std::fs::File;
use std::io::Read;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lelang::compile_with_error_handling;

// let mut f = File::open("/Users/li/CLionProject/llvm-prc/src/main.cpp").unwrap();
// let mut b = String::new();
// f.read_to_string(&mut b);
// let tokens = lexer::TokenIterator::new(&b);
// let v = tokens.into_iter().count();
// eprintln!("{}",v);
fn criterion_benchmark(c: &mut Criterion) {
    let mut f = File::open("/Users/li/CLionProject/lelang/main.le").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    c.bench_function("llvm", |b| b.iter(|| black_box(compile_with_error_handling(black_box(buffer.as_str())))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);