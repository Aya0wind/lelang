use std::collections::HashMap;

use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicValueEnum, FunctionValue};

pub use generator::*;

mod generator;
mod optimizer;

pub enum Symbol<'s>{
    Type(BasicTypeEnum<'s>),
    Variable(BasicValueEnum<'s>),
    Function(FunctionValue<'s>)
}

pub type SymbolTable<'s> = HashMap<String,Symbol<'s>>;