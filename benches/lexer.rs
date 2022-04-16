#![allow(unused)]

use std::fs::File;
use std::io::Read;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use inkwell::context::Context;

use lelang::arg_parser::{Args, OutputFormatEnum};
use lelang::ast::Ast;
use lelang::code_generator::generator::CodeGenerator;
use lelang::lexer::LELexer;

fn bench_lexer(c: &mut Criterion) {
    let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    c.bench_function("llvm", |b| b.iter(|| {
        let le_lexer = LELexer::new(buffer.as_str()).unwrap();
        for token in black_box(le_lexer) {}
    }));
}

fn bench_parser(c: &mut Criterion) {
    let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    c.bench_function("llvm", |b| b.iter(|| {
        let le_lexer = LELexer::new(buffer.as_str()).unwrap();
        let ast = Ast::from_lexer(black_box(le_lexer)).unwrap();
    }));
}

fn bench_codegen(c: &mut Criterion) {
    let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let context = Context::create();
    let le_lexer = LELexer::new(buffer.as_str()).unwrap();
    let ast = Ast::from_lexer(le_lexer).unwrap();
    let module = context.create_module("main");
    c.bench_function("llvm", |b| b.iter(|| {
        let mut code_generator = CodeGenerator::create(&context);
        code_generator.compile(&module, &ast).unwrap();
    }));
}

// fn bench_workflow(c: &mut Criterion) {
//     let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
//     let mut buffer = String::new();
//     f.read_to_string(&mut buffer).unwrap();
//     let le_lexer = LELexer::new(buffer.as_str()).unwrap();
//     c.bench_function("llvm", |b| b.iter(|| {
//         for token in black_box(le_lexer){}
//     }));
// }

criterion_group!(benches, bench_lexer,bench_parser,bench_codegen);
criterion_main!(benches);