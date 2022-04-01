use std::fs::File;
use anyhow::Result;
use std::io::Read;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::{FileType, TargetTriple};
use crate::{Ast, CodeGenerator, lexer, Optimizer};
use crate::arg_parser::{Args, OutputFormatEnum};
use crate::target::initialize_target_machine;


pub fn optimize_number_to_level(number:usize)->OptimizationLevel{
    match number {
        0 => {OptimizationLevel::None}
        1 => {OptimizationLevel::Less}
        2 => {OptimizationLevel::Default}
        3 => {OptimizationLevel::Aggressive}
        _=>{unreachable!()}
    }
}


pub fn compile_with_config(config:Args) ->Result<()>{
    let mut input = File::open(&config.input_path)?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer);

    //词法分析
    let lexer = lexer::LELexer::new(&buffer).unwrap();
    //语法分析
    let ast = Ast::from_lexer(lexer)?;
    //LLVM IR生成
    let context = Context::create();
    let mut code_generator = CodeGenerator::create(&context);
    let module = context.create_module("main");
    code_generator.compile(&module, ast)?;

    //优化
    let optimizer = Optimizer::new(&module, optimize_number_to_level(config.optimization));
    optimizer.run_on_module(&module);

    //目标机器码生成
    let target_machine = initialize_target_machine(&config)?;
    module.set_triple(&target_machine.get_triple());
    let output_path = &config.output_path;
    match config.output_format{
        OutputFormatEnum::LLVMIR => {    module.print_to_file(output_path.as_path()).unwrap();}
        OutputFormatEnum::ASM => { target_machine.write_to_file(&module, FileType::Assembly, config.output_path.as_path()).unwrap();}
        OutputFormatEnum::Object=>{target_machine.write_to_file(&module, FileType::Object, config.output_path.as_path()).unwrap();}
    }
    Ok(())
}
