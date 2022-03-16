use std::collections::HashMap;

use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicValueEnum, FunctionValue, PointerValue};

pub use generator::*;

mod generator;
mod optimizer;

pub type VariableTable<'s> = HashMap<String,PointerValue<'s>>;
pub type TypeTable<'s> = HashMap<String,BasicTypeEnum<'s>>;
pub type FunctionTable<'s> = HashMap<String,FunctionValue<'s>>;

#[derive(Debug,Default,PartialEq,Clone)]
pub struct SymbolTable<'s>{
    variables:VariableTable<'s>,
    types:TypeTable<'s>,
    functions:FunctionTable<'s>,
}



