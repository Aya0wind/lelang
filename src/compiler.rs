use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::OptimizationLevel;
use inkwell::targets::FileType;

use crate::arg_parser::{Args, OutputFormatEnum};
use crate::ast::Ast;
use crate::code_generator::CodeGenerator;
use crate::lexer;
use crate::optimizer::Optimizer;
use crate::target::{initialize_target_machine, optimize_number_to_level};

pub fn compile_with_config(config: Args) -> Result<()> {
    let mut input = File::open(&config.input_path)?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    //词法分析
    let lexer = lexer::LELexer::new(&buffer).unwrap();

    //语法分析
    let ast = Ast::from_lexer(lexer)?;

    //LLVM IR生成
    let context = Context::create();
    let mut code_generator = CodeGenerator::create(&context);
    let module = context.create_module("main");
    code_generator.compile(&module, &ast)?;

    //前端优化
    let optimizer = Optimizer::new(&module, optimize_number_to_level(config.optimization));
    optimizer.run_on_module(&module)?;

    //后端优化与目标机器码生成
    let target_machine = initialize_target_machine(&config)?;
    module.set_triple(&target_machine.get_triple());
    let output_path = &config.output_path;
    match config.output_format {
        OutputFormatEnum::IR => { module.print_to_file(output_path.as_path()).unwrap(); }
        OutputFormatEnum::ASM => { target_machine.write_to_file(&module, FileType::Assembly, config.output_path.as_path()).unwrap(); }
        OutputFormatEnum::OBJ => { target_machine.write_to_file(&module, FileType::Object, config.output_path.as_path()).unwrap(); }
        OutputFormatEnum::EXE => {
            target_machine.write_to_file(&module, FileType::Object, config.output_path.as_path()).unwrap();
            let mut cmd = Command::new("clang");
            let mut link_process = cmd.args([config.output_path.to_str().unwrap(), "print.o", "-o", config.output_path.as_path().with_extension("out").to_str().unwrap()])
                .spawn()?;
            link_process.wait()?;
        }
    }
    Ok(())
}
