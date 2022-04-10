use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::code_generator::builder::llvm_type_wrapper::{LEBasicValue, LEIntegerValue};

use super::super::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperator {
    Equal,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,
}


pub trait BinaryOpBuilder<'ctx,'a>: LEBasicValue<'ctx,'a>{
    fn build_add(&self,llvm_builder: &Builder<'ctx>, llvm_context: &Context,rhs:&Self)->Result<Self>;
    fn build_sub(&self,llvm_builder: &Builder<'ctx>, llvm_context: &Context,rhs:&Self)->Result<Self>;
    fn build_mul(&self,llvm_builder: &Builder<'ctx>, llvm_context: &Context,rhs:&Self)->Result<Self>;
    fn build_div(&self,llvm_builder: &Builder<'ctx>, llvm_context: &Context,rhs:&Self)->Result<Self>;
    fn build_cmp(&self,llvm_builder: &Builder<'ctx>, llvm_context: &Context,rhs:&Self,op:CompareOperator)->Result<LEIntegerValue>;
}
