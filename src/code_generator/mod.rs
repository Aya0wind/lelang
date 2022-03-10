use std::collections::HashMap;

use inkwell::types::{AnyTypeEnum};
use inkwell::values::AnyValueEnum;

pub use generator::*;

mod generator;

pub type VariableTable<'s> = HashMap<String, AnyValueEnum<'s>>;
pub type TypeTable<'s> = HashMap<String, AnyTypeEnum<'s>>;

#[derive(Default)]
struct GlobalSymbolTable<'s> {
    variables: VariableTable<'s>,
    types: TypeTable<'s>,
    function_table: TypeTable<'s>,
}

