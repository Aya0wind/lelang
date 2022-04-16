use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::code_generator::builder::{LEBasicValue, LEBoolValue, LEContext, LEIntegerValue, LEPointerValue};

use super::super::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareBinaryOperator {
    Equal,

    NotEqual,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicBinaryOperator {
    LogicAnd,

    LogicOr,

    LogicXor,
}

pub trait ArithmeticOperatorBuilder<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_add(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_sub(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_mul(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_div(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>>;
}

pub trait MathOperatorBuilder<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_mod(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self>;
}

pub trait LogicBinaryOperatorBuilder<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_binary_logic(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>>;
}

pub trait MemberAccessOperatorBuilder<'ctx> {
    fn build_dot(&self, le_context: &LEContext<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>>;
    fn build_index(&self, le_context: &LEContext<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>>;
}
