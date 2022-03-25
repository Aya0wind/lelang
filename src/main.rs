#![allow(unused)]

use std::fs::File;
use std::io::Read;

use anyhow::Result;
use inkwell::context::Context;
use inkwell::OptimizationLevel;

use crate::code_gen::ModuleCodeGenerator;
use crate::jit::JITCompiler;

mod lexer;
mod code_gen;
mod jit;
mod error;
mod ast;

pub fn compile_with_error_handling(code: &str) -> Result<()> {
    let tokens = lexer::LELexer::new(code).unwrap();
    // for token in tokens {
    //     eprintln!("{:?}",token);
    // }
    let context = Context::create();
    let mut code_generator = ModuleCodeGenerator::create(&context);
    let module = context.create_module("main");
    code_generator.compile_module(&module, tokens, OptimizationLevel::None)?;
    module.print_to_file("out.ll").unwrap();
    Ok(())
}


fn main() {
    let mut file = File::open("main.le").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    match compile_with_error_handling(&buffer) {
        Ok(_) => { eprintln!("compile success") }
        Err(err) => { eprintln!("error occurred:{}", err) }
    }
}
