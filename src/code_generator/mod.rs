use std::collections::HashMap;

use inkwell::types::AnyTypeEnum;
use inkwell::values::{AnyValueEnum, FunctionValue};

pub use generator::*;

mod generator;
mod optimizer;

pub enum Symbol<'s>{
    Type(AnyTypeEnum<'s>),
    Variable(AnyValueEnum<'s>),
    Function(FunctionValue<'s>)
}

pub type SymbolTable<'s> = HashMap<String,Symbol<'s>>;