use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;

use inkwell::{FloatPredicate, IntPredicate};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::FloatType;
use inkwell::values::{FloatValue, IntValue, PointerValue};
use lazy_static::lazy_static;

use crate::code_generator::builder::le_type::{LEBasicType, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue};
use crate::code_generator::builder::LEContext;
use crate::error::CompileError;

use super::super::Result;
use super::traits::{BinaryOpBuilder, CompareOperator};

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


impl<'ctx> BinaryOpBuilder<'ctx> for LEIntegerValue<'ctx> {
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

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareOperator) -> Result<LEIntegerValue<'ctx>> {
        let (left_providence, right_providence) = (get_integer_promotion_providence(&self.ty), get_integer_promotion_providence(&rhs.ty));
        let (casted_left, casted_right) = match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                (LEIntegerValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs.clone())
            }
            Ordering::Greater => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_int_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                (LEIntegerValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs.clone())
            }
            Ordering::Equal => {
                (self.clone(), rhs.clone())
            }
        };
        if casted_left.ty.signed() {
            match op {
                CompareOperator::Equal => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::EQ, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::GreaterThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SGT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::LessThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SLT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::GreaterOrEqualThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SGE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::LessOrEqualThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::SLE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
            }
        } else {
            match op {
                CompareOperator::Equal => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::EQ, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::GreaterThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::UGT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::LessThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::ULT, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::GreaterOrEqualThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::UGE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
                CompareOperator::LessOrEqualThan => {
                    Ok(LEIntegerValue { ty: casted_left.ty, llvm_value: le_context.llvm_builder.build_int_compare(IntPredicate::ULE, casted_left.llvm_value, casted_right.llvm_value, "") })
                }
            }
        }
    }
}


impl<'ctx> BinaryOpBuilder<'ctx> for LEFloatValue<'ctx> {
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

    fn build_cmp(self, le_context: &LEContext<'ctx>, rhs: Self, op: CompareOperator) -> Result<LEIntegerValue<'ctx>> {
        let (left_providence, right_providence) = (get_float_promotion_providence(self.ty.borrow()), get_float_promotion_providence(rhs.ty.borrow()));
        let (casted_left, casted_right) = match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(rhs.llvm_value, self.ty.get_llvm_type(), "");
                (LEFloatValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs)
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                let cast_value = le_context.llvm_builder.build_float_cast(self.llvm_value, rhs.ty.get_llvm_type(), "");
                (LEFloatValue { ty: self.ty.clone(), llvm_value: cast_value }, rhs)
            }
            Ordering::Equal => {
                (self.clone(), rhs)
            }
        };
        match op {
            CompareOperator::Equal => {
                Ok(LEIntegerValue { ty: le_context.i8_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OEQ, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareOperator::GreaterThan => {
                Ok(LEIntegerValue { ty: le_context.i8_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OGT, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareOperator::LessThan => {
                Ok(LEIntegerValue { ty: le_context.i8_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OLT, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareOperator::GreaterOrEqualThan => {
                Ok(LEIntegerValue { ty: le_context.i8_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OGE, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
            CompareOperator::LessOrEqualThan => {
                Ok(LEIntegerValue { ty: le_context.i8_type(), llvm_value: le_context.llvm_builder.build_float_compare(FloatPredicate::OLE, casted_left.llvm_value, casted_right.llvm_value, "") })
            }
        }
    }
}

