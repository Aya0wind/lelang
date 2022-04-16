use enum_dispatch::enum_dispatch;
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicValue, BasicValueEnum};

use crate::code_generator::builder::le_wrapper::*;
use crate::code_generator::builder::LEArrayType;
use crate::error::CompileError;

use super::super::Result;

pub trait LEType<'ctx>: Sized {
    #[allow(non_camel_case_types)]
    type LLVM_Type: BasicType<'ctx>;
    fn get_llvm_type(&self) -> Self::LLVM_Type;
    fn name(&self) -> &'static str;
    fn get_llvm_basic_type(&self) -> BasicTypeEnum<'ctx> {
        self.get_llvm_type().as_basic_type_enum()
    }
}

pub trait LEValue<'ctx>: Sized {
    #[allow(non_camel_case_types)]
    type LLVM_Value_Type: BasicValue<'ctx>;
    type LEType: LEType<'ctx>;
    fn get_llvm_value(&self) -> Self::LLVM_Value_Type;
    fn get_llvm_basic_value(&self) -> BasicValueEnum<'ctx> {
        self.get_llvm_value().as_basic_value_enum()
    }
    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self>;
}

#[enum_dispatch(LEBasicTypeEnum)]
pub trait LEBasicType<'ctx>: LEType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx>;
    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx>;
    fn get_pointer_type(&self) -> LEPointerType<'ctx>;
}

#[enum_dispatch(LEBasicValueEnum)]
pub trait LEBasicValue<'ctx>: LEValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx>;
    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx>;
}




