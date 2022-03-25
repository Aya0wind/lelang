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
        builder.populate_function_pass_manager(&pass_manager);
        builder.set_optimization_level(level);
        Self { pass_manager }
    }
    pub fn run(&self, function: &FunctionValue) -> bool {
        self.pass_manager.run_on(function)
    }
}