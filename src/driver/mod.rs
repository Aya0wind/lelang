use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use ariadne::{Report, Source};
use inkwell::context::Context;
use inkwell::data_layout::DataLayout;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::OptimizationLevel;
use inkwell::targets::FileType;

use crate::arg_parser::{Args, OutputFormatEnum};
use crate::ast::Ast;
use crate::code_generator::generator::CodeGenerator;
// use crate::code_generator::generator::CodeGenerator;
use crate::driver::target::{initialize_target_machine, optimize_number_to_level};
use crate::error::{LEError, Result};
use crate::lexer;
use crate::optimizer::Optimizer;

// use crate::optimizer::Optimizer;

mod target;

pub fn compile_with_config(config: &Args, source: &str) -> Result<()> {
    //词法分析
    let mut lexer = lexer::LELexer::new(source);
    if let Some(lexer) = lexer {
        //语法分析
        let ast = Ast::from_lexer(lexer)?;
        //类型检查和LLVM IR生成
        let context = Context::create();
        let mut code_generator = CodeGenerator::create(&context);
        let module = context.create_module("main");
        code_generator.compile(&module, &ast)?;

        //前端优化
        let optimizer = Optimizer::new(&module, optimize_number_to_level(config.optimization));
        optimizer.run_on_module(&module);

        //后端优化与目标机器码生成
        let target_machine = initialize_target_machine(&config);
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
                    .spawn().unwrap();
                link_process.wait().unwrap();
            }
        }
    }
    Ok(())
}
