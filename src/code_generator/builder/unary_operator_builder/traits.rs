use inkwell::builder::Builder;

use crate::code_generator::builder::{LEBasicValue, LEBoolValue, LEPointerValue};
use crate::code_generator::context::LEContext;
use crate::code_generator::Result;

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
    And,

    Or,

    Xor,
}

pub trait BasicMathOperateValue<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_add_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self;
    fn build_sub_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self;
    fn build_mul_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self;
    fn build_div_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self;
    fn build_cmp_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, op: CompareBinaryOperator, rhs: Self) -> LEBoolValue<'ctx>;
}

pub trait ModOperateValue<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_mod_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self;
}

pub trait LogicBinaryOperateValue<'ctx>: LEBasicValue<'ctx> + Sized {
    fn build_binary_logic_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, op: CompareBinaryOperator, rhs: Self) -> LEBoolValue<'ctx>;
}

pub trait MemberAccessOperateValue<'ctx> {
    fn build_dot_unchecked(&self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>>;
    fn build_index_unchecked(&self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, index: usize) -> Result<LEPointerValue<'ctx>>;
}
