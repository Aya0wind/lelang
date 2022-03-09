use inkwell::OptimizationLevel;
use inkwell::passes::{PassManager, PassManagerBuilder};
use std::default::Default;
pub trait Optimizer{

}

struct OptimizerBuilder{
    optimize_passes:Vec<String>,
    builder:PassManagerBuilder,
}

impl Default for OptimizerBuilder {
    fn default() -> Self {
        Self{
            optimize_passes: vec![],
            builder: PassManagerBuilder::create()
        }
    }
}

pub enum OptimizeLevel{
    Zero,
    One,
    Two,
    Three,
    Size,
}

impl OptimizerBuilder{
    pub fn new()->Self{
        Self::default()
    }
    pub fn with_pass(mut self,pass_name:String)->Self{
        self.optimize_passes.push(pass_name);
        self
    }

    pub fn with_pass_function(mut self,pass_name:String)->Self{
        self.optimize_passes.push(pass_name);
        self
    }

    pub fn with_optimize_level(mut self,level:OptimizeLevel)->Self{
        self.builder.set_optimization_level(OptimizationLevel::Default);
        self
    }


}

struct PassContainer<M>{
    pass_manager:PassManager<M>,
}

#[allow(unused)]
struct OptimizerLevel0;
#[allow(unused)]
struct OptimizerLevel1;
#[allow(unused)]
struct OptimizerLevel2;
#[allow(unused)]
struct OptimizerLevel3;