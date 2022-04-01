use inkwell::types::AnyTypeEnum;
use inkwell::values::AnyValueEnum;

use crate::code_generator::builder::llvm_wrapper::{LETypeEnum, LEValueEnum};

pub trait LEValue<'s> {
    fn as_le_value_enum(&self) -> LEValueEnum<'s>;
    fn as_any_value_enum(&self) -> AnyValueEnum<'s>;
}

pub trait LEType<'s> {
    fn as_le_type_enum(&self) -> LETypeEnum<'s>;
    fn as_any_type_enum(&self) -> AnyTypeEnum<'s>;
}
