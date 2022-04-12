#![allow(unused)]

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
