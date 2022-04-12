use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::code_generator::builder::le_type::{LEBasicTypeEnum, LEBasicValueEnum, LEPointerType, LEPointerValue};
use crate::error::CompileError;

use super::super::Result;

//
// pub trait LEType<'ctx>{
//     #[allow(non_camel_case_types)]
//     type LLVM_Type;
//     fn as_le_type_enum(&self) -> LEBasicTypeEnum<'ctx>;
//     fn get_llvm_type(&self)->Self::LLVM_Type;
// }
//
// pub trait LEValue<'ctx>{
//     #[allow(non_camel_case_types)]
//     type LLVM_Type;
//     fn as_le_type_enum(&self) -> LEBasicTypeEnum<'ctx>;
//     fn get_llvm_type(&self)->Self::LLVM_Type;
// }


pub trait LEBasicType<'ctx>: TryFrom<LEBasicTypeEnum<'ctx>, Error=CompileError> {
    #[allow(non_camel_case_types)]
    type LLVM_Type;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx>;
    fn get_llvm_type(&self) -> Self::LLVM_Type;
    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx>;
}


pub trait LEBasicValue<'ctx>: TryFrom<LEBasicValueEnum<'ctx>, Error=CompileError> {
    #[allow(non_camel_case_types)]
    type LEType: LEBasicType<'ctx>;
    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx>;
    fn get_le_type(&self) -> Self::LEType;
    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx>;
    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self>;
}




