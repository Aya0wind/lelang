#![allow(unused)]

use std::fs::File;
use std::io::Read;

use anyhow::Result;
use clap::Parser;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::InitializationConfig;

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
mod arg_parser;
mod compiler;

fn main() {
    let args = arg_parser::Args::parse();
    match compiler::compile_with_config(args) {
        Ok(_) => {
            eprintln!("compile success")
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
