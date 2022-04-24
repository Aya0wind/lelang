#![allow(dead_code, unused)]

extern crate core;

use std::fs::File;
use std::io::Read;

use ariadne::Source;
use atty::Stream;
use clap::Parser;

use crate::arg_parser::Args;

mod lexer;
mod code_generator;
mod jit;
mod error;
mod ast;
mod optimizer;
// mod analyzer;
mod driver;
mod arg_parser;


fn read_args_and_compile() -> std::io::Result<()> {
    let args: Args = arg_parser::Args::parse();
    let mut input = File::open(&args.input_path)?;
    let mut buffer = String::new();
    let src = args.input_path.to_str().unwrap();
    input.read_to_string(&mut buffer)?;
    match driver::compile_with_config(&args, &buffer) {
        Ok(_) => {}
        Err(err) => {
            // if atty::is(Stream::Stderr){
            //     err.to_error_report_colored(src).eprint((src, Source::from(buffer)))?;
            // }else{
            //     eprintln!("{}",err);
            // }
            err.to_error_report_colored(src).eprint((src, Source::from(buffer)))?;
        }
    }
    Ok(())
}


fn main() {
    match read_args_and_compile() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("error:{}", err);
        }
    };
}
