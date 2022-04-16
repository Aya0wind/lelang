use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::StructValue;

use crate::code_generator::builder::{LEBoolValue, LEContext, LEIntegerValue, LEPointerValue, LEStructValue};
use crate::code_generator::builder::binary_operator_builder::{LogicBinaryOperator, MemberAccessOperatorBuilder};
use crate::code_generator::builder::binary_operator_builder::traits::{ArithmeticOperatorBuilder, CompareBinaryOperator};

use super::super::Result;

impl<'ctx> ArithmeticOperatorBuilder<'ctx> for LEStructValue<'ctx> {
    fn build_add(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        todo!()
    }

    fn build_sub(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        todo!()
    }

    fn build_mul(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        todo!()
    }

    fn build_div(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        todo!()
    }

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        todo!()
    }
}
