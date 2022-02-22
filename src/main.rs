use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use inkwell::values::AnyValue;
use inkwell::passes::{PassManager, PassManagerBuilder};
use crate::lexer::Token;

mod lexer;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn() -> i32;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        // let i64_type = self.context.i64_type();
        // let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        // let function = self.module.add_function("sum", fn_type, None);
        // let basic_block = self.context.append_basic_block(function, "entry");
        //
        // self.builder.position_at_end(basic_block);
        //
        // let x = function.get_nth_param(0)?.into_int_value();
        // let y = function.get_nth_param(1)?.into_int_value();
        // let z = function.get_nth_param(2)?.into_int_value();
        //
        // let sum = self.builder.build_int_add(x, y, "sum");
        // let sum = self.builder.build_int_add(sum, z, "sum");
        // self.builder.build_return(Some(&sum));
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[],false);
        let main_fn = self.module.add_function("main",main_fn_type,None);
        let basic_block = self.context.append_basic_block(main_fn, "entry");

        self.builder.position_at_end(basic_block);
        self.builder.build_return(Some(&i32_type.const_zero()));
        let pass_manager_builder = PassManagerBuilder::create();

        let fpm = PassManager::create(&self.module);

        pass_manager_builder.populate_function_pass_manager(&fpm);
        fpm.add_instruction_combining_pass();
        fpm.run_on(&main_fn);
        self.module.print_to_file("out.ll");
        File::open("out.ll").unwrap().write_all(main_fn.print_to_string().to_bytes());
        unsafe { self.execution_engine.get_function("main").ok() }

    }
}

use logos::Logos;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lex = Token::lexer(include_str!("../Cargo.toml"));
    for token in lex{
        eprintln!("{:?}",token);
    }
    // let context = Context::create();
    // let module = context.create_module("main");
    // let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    // let codegen = CodeGen {
    //     context: &context,
    //     module,
    //     builder: context.create_builder(),
    //     execution_engine,
    // };
    //
    // let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;
    //
    // unsafe {
    //     println!("{}",sum.call() );
    // }

    Ok(())
}