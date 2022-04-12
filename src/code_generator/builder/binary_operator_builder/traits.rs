use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::code_generator::builder::le_type::{LEBasicValue, LEIntegerValue};
use crate::code_generator::builder::LEContext;

use super::super::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperator {
    Equal,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,
}


pub trait BinaryOpBuilder<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_add(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_sub(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_mul(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_div(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareOperator) -> Result<LEIntegerValue<'ctx>>;
}
