#![allow(dead_code, unused)]

use clap::Parser;

mod lexer;
mod code_generator;
mod jit;
mod error;
mod ast;
mod optimizer;
mod driver;
mod arg_parser;


fn main() {
    use inkwell::context::Context;

    let context = Context::create();
    let f32_type = context.f32_type();
    let f32_array_type = f32_type.array_type(3);
    let f32_array_val = f32_array_type.const_zero();
    let f32_array_array = f32_array_type.const_array(&[f32_array_val, f32_array_val]);

    let args = arg_parser::Args::parse();

    match driver::compile_with_config(args) {
        Ok(_) => {
            eprintln!("compile success")
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
