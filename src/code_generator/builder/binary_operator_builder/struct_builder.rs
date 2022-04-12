use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::StructValue;

use crate::code_generator::builder::binary_operator_builder::traits::{BinaryOpBuilder, CompareOperator};
use crate::code_generator::builder::le_type::{LEIntegerValue, LEStructValue};
use crate::code_generator::builder::LEContext;

use super::super::Result;

impl<'ctx> BinaryOpBuilder<'ctx> for LEStructValue<'ctx> {
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

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareOperator) -> Result<LEIntegerValue<'ctx>> {
        todo!()
    }
}