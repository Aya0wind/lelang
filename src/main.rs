#![allow(unused)]
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::InitializationConfig;

use crate::ast::Ast;
use crate::code_generator::CodeGenerator;
use crate::jit::JITCompiler;
use crate::optimizer::Optimizer;

mod lexer;
mod code_generator;
mod jit;
mod error;
mod ast;
mod optimizer;
mod target;
pub fn compile_with_error_handling(code: &str) -> Result<()> {
    let lexer = lexer::LELexer::new(code).unwrap();
    let ast = Ast::from_lexer(lexer)?;
    let context = Context::create();
    let mut code_generator = CodeGenerator::create(&context);
    let module = context.create_module("main");
    code_generator.compile(&module, ast)?;
    let optimizer = Optimizer::new(&module, OptimizationLevel::None);
    //optimizer.run_on_module(&module);
    module.print_to_file("out.ll").unwrap();
    // let jit_compiler = JITCompiler::new(&module);
    // jit_compiler.run_main()?;
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
