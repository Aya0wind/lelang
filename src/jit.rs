use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub struct JITCompiler<'source> {
    engine: ExecutionEngine<'source>,
}

type MainFunc = unsafe extern "C" fn() -> i32;

impl<'source> JITCompiler<'source> {
    pub fn new(module: &'source Module) -> Self {
        Self { engine: module.create_jit_execution_engine(OptimizationLevel::None).unwrap() }
    }
    pub fn run_main(&self) -> i32 {
        unsafe {
            let main = self.engine.get_function::<MainFunc>("main").unwrap();
            main.call()
        }
    }
}
