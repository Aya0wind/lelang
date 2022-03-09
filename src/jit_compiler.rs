use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use anyhow::Result;



pub struct JITCompiler<'source>{
    engine:ExecutionEngine<'source>,
}
/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type MainFunc = unsafe extern "C" fn() -> i32;
impl<'source> JITCompiler<'source> {
    pub fn new(module:&'source Module)->Self{
        Self{engine:module.create_jit_execution_engine(OptimizationLevel::None).unwrap()}
    }
    pub fn run_main(&self)->Result<i32>{
        unsafe{
            let main = self.engine.get_function::<MainFunc>("main")?;
            Ok(main.call())
        }
    }
}
