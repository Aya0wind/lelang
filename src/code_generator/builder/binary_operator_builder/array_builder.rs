use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{ArrayValue, StructValue};

use crate::code_generator::builder::binary_operator_builder::traits::{BinaryOpBuilder, CompareOperator};
use crate::code_generator::builder::llvm_type_wrapper::{LEArrayValue, LEIntegerValue};

use super::super::Result;

impl<'ctx,'a> BinaryOpBuilder<'ctx,'a> for LEArrayValue<'ctx,'a> {
    fn build_add(&self, llvm_builder: &Builder<'ctx>, llvm_context: &Context, rhs: &Self) -> Result<Self> {
        todo!()
    }

    fn build_sub(&self, llvm_builder: &Builder<'ctx>, llvm_context: &Context, rhs: &Self) -> Result<Self> {
        todo!()
    }

    fn build_mul(&self, llvm_builder: &Builder<'ctx>, llvm_context: &Context, rhs: &Self) -> Result<Self> {
        todo!()
    }

    fn build_div(&self, llvm_builder: &Builder<'ctx>, llvm_context: &Context, rhs: &Self) -> Result<Self> {
        todo!()
    }

    fn build_cmp(&self, llvm_builder: &Builder<'ctx>, llvm_context: &Context, rhs: &Self, op: CompareOperator) -> Result<LEIntegerValue> {
        todo!()
    }
}