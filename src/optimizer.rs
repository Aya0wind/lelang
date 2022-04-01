use anyhow::Result;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::values::FunctionValue;

pub struct Optimizer<'s> {
    pass_manager: PassManager<FunctionValue<'s>>,
}

impl<'s> Optimizer<'s> {
    pub fn new(module: &Module<'s>, level: OptimizationLevel) -> Self {
        let pass_manager = PassManager::create(module);
        let builder = PassManagerBuilder::create();
        builder.set_optimization_level(level);
        builder.populate_function_pass_manager(&pass_manager);
        Self { pass_manager }
    }
    pub fn run_on_function(&self, function: &FunctionValue) -> bool {
        self.pass_manager.run_on(function)
    }

    pub fn run_on_module(&self,module:&Module)->Result<()>{
        let mut current_function = module.get_first_function();
        while let Some(f) = current_function {
            self.run_on_function(&f);
            current_function = f.get_next_function();
        }
        Ok(())
    }
}