use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::code_generator::builder::llvm_type_wrapper::{LEBasicTypeGenericRef, LEBasicValueEnum, LEIntegerType};
use crate::error::CompileError;

pub trait LEBasicType<'ctx>{
    #[allow(non_camel_case_types)]
    type LLVM_Type;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx>;
    fn get_llvm_type(&self)->Self::LLVM_Type;
    fn get_basic_llvm_type(&self) ->BasicTypeEnum<'ctx>;
}


pub trait LEBasicValue<'ctx, 'a> {
    #[allow(non_camel_case_types)]
    type LEType: LEBasicType<'ctx>;
    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx,'a>;
    fn get_le_type(&self)->&'a Self::LEType;
    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx>;
}




