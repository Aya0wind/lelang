use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use ariadne::{Report, Source};
use inkwell::context::Context;
use inkwell::data_layout::DataLayout;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::targets::FileType;
use inkwell::OptimizationLevel;
use nom::error::context;
use nom::Parser;

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
    let output_path = &config.output_path;

    let context = Context::create();
    let module = context.create_module("main");
    //词法分析
    let lexer = lexer::LELexer::new(source);
    if let Some(lexer) = lexer {
        //如果只需要打印token，可以直接跳过后续阶段
        if let OutputFormatEnum::TOKENS = config.output_format {
            let tokens = lexer.map(|s| format!("{:?}\n", s)).collect::<Vec<_>>();
            std::fs::write(
                output_path,
                tokens.iter().flat_map(|s| s.chars()).collect::<String>(),
            )
            .map_err(|e| LEError::IOError { other: Box::new(e) })?;
        } else {
            //语法分析
            let ast = Ast::from_lexer(lexer)?;
            //如果只需要打印ast，可以直接跳过后续阶段
            if let OutputFormatEnum::AST = config.output_format {
                let mut output_file = File::create(output_path).unwrap();
                return {
                    ast.print_with_root_name(
                        output_file,
                        config.input_path.to_str().unwrap().to_string(),
                    )
                    .map_err(|e| LEError::IOError { other: Box::new(e) })?;
                    Ok(())
                };
            }
            //类型检查和LLVM IR生成
            let mut code_generator = CodeGenerator::create(&context);
            code_generator.compile(&module, &ast)?;

            //前端优化
            let optimizer = Optimizer::new(&module, optimize_number_to_level(config.optimization));
            optimizer.run_on_module(&module);

            //后端优化与目标代码生成的设置
            let target_machine = initialize_target_machine(config);
            module.set_triple(&target_machine.get_triple());

            //运行LLVM后端并输出编译结果
            match config.output_format {
                OutputFormatEnum::IR => {
                    module
                        .print_to_file(output_path.with_extension("ll").as_path())
                        .unwrap();
                }
                OutputFormatEnum::ASM => {
                    target_machine
                        .write_to_file(
                            &module,
                            FileType::Assembly,
                            config.output_path.with_extension("S").as_path(),
                        )
                        .unwrap();
                }
                OutputFormatEnum::OBJ => {
                    target_machine
                        .write_to_file(
                            &module,
                            FileType::Object,
                            config.output_path.with_extension("o").as_path(),
                        )
                        .unwrap();
                }
                OutputFormatEnum::EXE => {
                    target_machine
                        .write_to_file(
                            &module,
                            FileType::Object,
                            config.output_path.with_extension("out").as_path(),
                        )
                        .unwrap();

                    //由于没有暂时没有考虑如何打包平台对应的链接工具链，所以目前直接使用llvm的工具链进行链接
                    let mut cmd = Command::new("clang");
                    let mut link_process = cmd
                        .args([
                            config.output_path.to_str().unwrap(),
                            "print.c",
                            "-o",
                            config
                                .output_path
                                .as_path()
                                .with_extension("out")
                                .to_str()
                                .unwrap(),
                        ])
                        .spawn()
                        .unwrap();
                    link_process.wait().unwrap();
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
