use inkwell::builder::Builder;

use crate::code_generator::builder::{LEBoolValue, LEPointerValue, LEStructValue};
use crate::code_generator::builder::binary_operator_builder::MemberAccessOperateValue;
use crate::code_generator::builder::binary_operator_builder::traits::{BasicMathOperateValue, CompareBinaryOperator};
use crate::code_generator::context::LEContext;
use crate::code_generator::Result;

impl<'ctx> BasicMathOperateValue<'ctx> for LEStructValue<'ctx> {
    fn build_add_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        todo!()
    }

    fn build_sub_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        todo!()
    }

    fn build_mul_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        todo!()
    }

    fn build_div_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        todo!()
    }

    fn build_cmp_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, op: CompareBinaryOperator, rhs: Self) -> LEBoolValue<'ctx> {
        todo!()
    }
}
