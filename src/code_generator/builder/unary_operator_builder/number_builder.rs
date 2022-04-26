use inkwell::{FloatPredicate, IntPredicate};
use inkwell::builder::Builder;
use inkwell::values::PointerValue;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::code_generator::builder::{LEBoolValue, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::ModOperateValue;
use crate::code_generator::context::LEContext;
use crate::code_generator::Result;

use super::traits::{BasicMathOperateValue, CompareBinaryOperator};

impl<'ctx> ModOperateValue<'ctx> for LEIntegerValue<'ctx> {
    fn build_mod_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        if self.ty.signed() {
            LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_signed_rem(self.llvm_value, rhs.llvm_value, "") }
        } else {
            LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_unsigned_rem(self.llvm_value, rhs.llvm_value, "") }
        }
    }
}

impl<'ctx> BasicMathOperateValue<'ctx> for LEIntegerValue<'ctx> {
    fn build_add_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEIntegerValue {
            ty: self.ty.clone().clone(),
            llvm_value: llvm_builder.build_int_add(self.llvm_value, rhs.llvm_value, ""),
        }
    }

    fn build_sub_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_sub(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_mul_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_mul(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_div_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        if self.ty.signed() {
            LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_signed_div(self.llvm_value, rhs.llvm_value, "") }
        } else {
            LEIntegerValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_int_unsigned_div(self.llvm_value, rhs.llvm_value, "") }
        }
    }

    fn build_cmp_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, op: CompareBinaryOperator, rhs: Self) -> LEBoolValue<'ctx> {
        if self.ty.signed() {
            match op {
                CompareBinaryOperator::Equal => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::EQ, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::GreaterThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::SGT, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::LessThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::SLT, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::GreaterOrEqualThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::SGE, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::LessOrEqualThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::SLE, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::NotEqual => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::NE, self.llvm_value, rhs.llvm_value, "") }
                }
            }
        } else {
            match op {
                CompareBinaryOperator::Equal => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::EQ, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::GreaterThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::UGT, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::LessThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::ULT, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::GreaterOrEqualThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::UGE, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::LessOrEqualThan => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::ULE, self.llvm_value, rhs.llvm_value, "") }
                }
                CompareBinaryOperator::NotEqual => {
                    LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_int_compare(IntPredicate::NE, self.llvm_value, rhs.llvm_value, "") }
                }
            }
        }
    }
}


impl<'ctx> BasicMathOperateValue<'ctx> for LEFloatValue<'ctx> {
    fn build_add_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEFloatValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_float_add(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_sub_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEFloatValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_float_sub(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_mul_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEFloatValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_float_mul(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_div_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, rhs: Self) -> Self {
        LEFloatValue { ty: self.ty.clone(), llvm_value: llvm_builder.build_float_div(self.llvm_value, rhs.llvm_value, "") }
    }

    fn build_cmp_unchecked(self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, op: CompareBinaryOperator, rhs: Self) -> LEBoolValue<'ctx> {
        match op {
            CompareBinaryOperator::Equal => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::OEQ, self.llvm_value, rhs.llvm_value, "") }
            }
            CompareBinaryOperator::GreaterThan => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::OGT, self.llvm_value, rhs.llvm_value, "") }
            }
            CompareBinaryOperator::LessThan => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::OLT, self.llvm_value, rhs.llvm_value, "") }
            }
            CompareBinaryOperator::GreaterOrEqualThan => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::OGE, self.llvm_value, rhs.llvm_value, "") }
            }
            CompareBinaryOperator::LessOrEqualThan => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::OLE, self.llvm_value, rhs.llvm_value, "") }
            }
            CompareBinaryOperator::NotEqual => {
                LEBoolValue { ty: le_context.bool_type(), llvm_value: llvm_builder.build_float_compare(FloatPredicate::ONE, self.llvm_value, rhs.llvm_value, "") }
            }
        }
    }
}

