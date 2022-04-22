#![allow(dead_code, unused)]

extern crate core;

use std::fs::File;
use std::io::Read;

use ariadne::Source;
use clap::Parser;

use crate::arg_parser::Args;
use crate::ast::Ast;

mod lexer;
mod code_generator;
mod jit;
mod error;
mod ast;
mod optimizer;
// mod analyzer;
mod driver;
mod arg_parser;



fn main() {
    let args: Args = arg_parser::Args::parse();
    let input_path_str = &args.input_path.to_str().unwrap();
    let mut input = File::open(&args.input_path).unwrap();
    let mut buffer = String::new();
    let src = args.input_path.to_str().unwrap();
    input.read_to_string(&mut buffer).unwrap();
    match driver::compile_with_config(&args, &buffer) {
        Ok(_) => {}
        Err(err) => {
            err.to_error_report_colored(src).eprint((src, Source::from(buffer))).unwrap();
        }
    }
}
