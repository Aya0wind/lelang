use std::any::Any;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;

use inkwell::{FloatPredicate, IntPredicate};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::FloatType;
use inkwell::values::{FloatValue, IntValue, PointerValue};
use lazy_static::lazy_static;

use crate::code_generator::builder::{LEBoolValue, LEContext, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEPointerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::{LogicBinaryOperator, MathOperatorBuilder};
use crate::error::CompileError;

use super::super::Result;
use super::traits::{ArithmeticOperatorBuilder, CompareBinaryOperator};

lazy_static! {
    //类型提升优先级(是否有符号,位长)
    pub static ref INT_TYPE_PROMOTION_PROVIDENCE: HashMap<(bool,u32),u32> = HashMap::from([
                ((true,64),11), //i64
                ((true,32),21), //i32
                ((true,16),31), //i16
                ((true,8),41), //i8
                ((false,64),10), //u64
                ((false,32),20), //u32
                ((false,16),30), //u16
                ((false,8),40), //u8
    ]);

    pub static ref FLOAT_TYPE_PROMOTION_PROVIDENCE: HashMap<u32,u32> = HashMap::from([
        (64,1), //f64
        (32,2) //f32
    ]);
}

pub fn get_float_promotion_providence(ty: &LEFloatType) -> u32 {
    0
}

pub fn get_integer_promotion_providence(ty: &LEIntegerType) -> u32 {
    let width = ty.get_llvm_type().get_bit_width();
    let signed = ty.signed();
    *INT_TYPE_PROMOTION_PROVIDENCE.get(&(signed, width)).unwrap()
}

impl<'ctx> MathOperatorBuilder<'ctx> for LEIntegerValue<'ctx> {
    fn build_mod(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                if self.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone().clone(), llvm_value: le_context.llvm_builder.build_int_signed_rem(self.llvm_value, cast_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone().clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_rem(self.llvm_value, cast_value, "") }
                    )
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                if self.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_signed_rem(cast_value, rhs.llvm_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_rem(cast_value, rhs.llvm_value, "") }
                    )
                }
            }
            Ordering::Equal => {
                if self.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_signed_rem(self.llvm_value, rhs.llvm_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_rem(self.llvm_value, rhs.llvm_value, "") }
                    )
                }
            }
        }
    }
}

impl<'ctx> ArithmeticOperatorBuilder<'ctx> for LEIntegerValue<'ctx> {
    fn build_add(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: self.ty.clone().clone(), llvm_value: le_context.llvm_builder.build_int_add(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_add(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_add(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_sub(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_sub(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: rhs.ty.clone(), llvm_value: le_context.llvm_builder.build_int_sub(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_sub(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_mul(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_mul(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_mul(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_mul(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_div(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                if self.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_signed_div(self.llvm_value, cast_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_div(self.llvm_value, cast_value, "") }
                    )
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                if rhs.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_signed_div(cast_value, rhs.llvm_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_div(cast_value, rhs.llvm_value, "") }
                    )
                }
            }
            Ordering::Equal => {
                if rhs.ty.signed() {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_signed_div(self.llvm_value, rhs.llvm_value, "") }
                    )
                } else {
                    Ok(
                        LEIntegerValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_int_unsigned_div(self.llvm_value, rhs.llvm_value, "") }
                    )
                }
            }
        }
    }

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        let (casted_left, casted_right) = match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                let left_ty = self.ty.clone();
                (self, LEIntegerValue { ty: left_ty, llvm_value: cast_value })
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                (LEIntegerValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs.clone())
            }
            Ordering::Equal => {
                (self, rhs)
            }
        };
        if casted_left.ty.signed() {
            match op {
                CompareBinaryOperator::Equal => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::EQ, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::GreaterThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SGT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::LessThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SLT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::GreaterOrEqualThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SGE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::LessOrEqualThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SLE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::NotEqual => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::NE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
            }
        } else {
            match op {
                CompareBinaryOperator::Equal => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::EQ, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::GreaterThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::UGT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::LessThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::ULT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::GreaterOrEqualThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::UGE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::LessOrEqualThan => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::ULE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareBinaryOperator::NotEqual => {
                    Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::NE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
            }
        }
    }

}


impl<'ctx> ArithmeticOperatorBuilder<'ctx> for LEFloatValue<'ctx> {
    fn build_add(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_float_promotion_providence(&self.ty), get_float_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_add(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_add(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_add(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_sub(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_float_promotion_providence(&self.ty), get_float_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_sub(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_sub(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_sub(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_mul(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_float_promotion_providence(&self.ty), get_float_promotion_providence(&rhs.ty));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_mul(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_mul(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_mul(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_div(self, le_context: &LEContext<'ctx>, rhs: Self) -> Result<Self> {
        let (left_providence, right_providence) = (get_float_promotion_providence(self.ty.borrow()), get_float_promotion_providence(rhs.ty.borrow()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_div(self.llvm_value, cast_value, "") }
                )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_div(cast_value, rhs.llvm_value, "") }
                )
            }
            Ordering::Equal => {
                Ok(
                    LEFloatValue { ty: self.ty.clone(), llvm_value: le_context.llvm_builder.build_float_div(self.llvm_value, rhs.llvm_value, "") }
                )
            }
        }
    }

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let (left_providence, right_providence) = (get_float_promotion_providence(self.ty.borrow()), get_float_promotion_providence(rhs.ty.borrow()));
        let (casted_left, casted_right) = match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                let left_type = self.ty.clone();
                (self, LEFloatValue { ty: left_type, llvm_value: cast_value }, )
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                (LEFloatValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs)
            }
            Ordering::Equal => {
                (self, rhs)
            }
        };
        match op {
            CompareBinaryOperator::Equal => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OEQ, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareBinaryOperator::GreaterThan => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OGT, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareBinaryOperator::LessThan => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OLT, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareBinaryOperator::GreaterOrEqualThan => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OGE, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareBinaryOperator::LessOrEqualThan => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OLE, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareBinaryOperator::NotEqual => {
                Ok(LEBoolValue { ty: le_context.bool_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::ONE, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
        }
    }

}

