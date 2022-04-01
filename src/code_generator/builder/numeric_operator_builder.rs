use std::cmp::Ordering;

use anyhow::Result;
use inkwell::{FloatPredicate, IntPredicate};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{FloatValue, IntValue, PointerValue};
use nom::combinator::value;

use crate::code_generator::builder::llvm_wrapper::{IntegerValue, NumericTypeEnum, NumericValueEnum};
use crate::code_generator::builder::type_checker::get_number_providence;

pub struct NumericOperatorBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperator {
    Equal,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,
}


impl NumericOperatorBuilder {
    pub fn build_numeric_add<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: NumericValueEnum<'s>, rhs: NumericValueEnum<'s>) -> Result<NumericValueEnum<'s>> {
        let (left_providence, right_providence) = (get_number_providence(&lhs.get_type()), get_number_providence(&rhs.get_type()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int.value, left_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_add(left_int.value, cast_value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_float_to_signed_int(right_float, left_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(right_float, left_int.value.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_add(left_int.value, cast_value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_signed_int_to_float(right_int.value, left_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(right_int.value, left_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_add(left_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(right_float, left_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_add(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(left_int.value, right_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_add(cast_value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_signed_int_to_float(left_int.value, right_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(left_int.value, right_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_add(right_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_float_to_signed_int(left_float, right_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(left_float, right_int.value.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_add(cast_value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(left_float, right_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_add(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Equal => {
                //类型相同，无需提升
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_add(left_int.value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_add(left_float, right_float, "")
                            )
                        )
                    }
                    _ => { unreachable!() }
                }
            }
        }
    }

    pub fn build_numeric_sub<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: NumericValueEnum<'s>, rhs: NumericValueEnum<'s>) -> Result<NumericValueEnum<'s>> {
        let (left_providence, right_providence) = (get_number_providence(&lhs.get_type()), get_number_providence(&rhs.get_type()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int.value, left_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_sub(cast_value,right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_float_to_signed_int(right_float, left_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(right_float, left_int.value.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_sub(left_int.value, cast_value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value: FloatValue = if right_int.signed {
                            llvm_builder.build_signed_int_to_float(right_int.value, left_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(right_int.value, left_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_sub(left_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(right_float, left_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_sub(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(left_int.value, right_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_sub(cast_value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_signed_int_to_float(left_int.value, right_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(left_int.value, right_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_sub(right_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_float_to_signed_int(left_float, right_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(left_float, right_int.value.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_sub(cast_value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(left_float, right_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_sub(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Equal => {
                //类型相同，无需提升
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_sub(left_int.value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_sub(left_float, right_float, "")
                            )
                        )
                    }
                    _ => { unreachable!() }
                }
            }
        }
    }

    pub fn build_numeric_mul<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: NumericValueEnum<'s>, rhs: NumericValueEnum<'s>) -> Result<NumericValueEnum<'s>> {
        let (left_providence, right_providence) = (get_number_providence(&lhs.get_type()), get_number_providence(&rhs.get_type()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int_)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int_.value, left_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_mul(left_int.value, cast_value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_float_to_signed_int(right_float, left_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(right_float, left_int.value.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_mul(left_int.value, cast_value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value: FloatValue = if right_int.signed {
                            llvm_builder.build_signed_int_to_float(right_int.value, left_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(right_int.value, left_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_mul(left_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(right_float, left_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_mul(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(left_int.value, right_int.value.get_type(), "");
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_mul(cast_value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_signed_int_to_float(left_int.value, right_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(left_int.value, right_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_mul(right_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_float_to_signed_int(left_float, right_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(left_float, right_int.value.get_type(), "")
                        };
                        let value = right_int.value;
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_mul(cast_value, value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(left_float, right_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_mul(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Equal => {
                //类型相同，无需提升
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        Ok(
                            NumericValueEnum::Integer(
                                IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_mul(left_int.value, right_int.value, "") }
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_mul(left_float, right_float, "")
                            )
                        )
                    }
                    _ => { unreachable!() }
                }
            }
        }
    }

    pub fn build_numeric_div<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: NumericValueEnum<'s>, rhs: NumericValueEnum<'s>) -> Result<NumericValueEnum<'s>> {
        let (left_providence, right_providence) = (get_number_providence(&lhs.get_type()), get_number_providence(&rhs.get_type()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int_)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int_.value, left_int.value.get_type(), "");
                        if left_int.signed {
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_signed_div(left_int.value, cast_value, "") }
                                )
                            )
                        } else {
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_unsigned_div(left_int.value, cast_value, "") }
                                )
                            )
                        }
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        if left_int.signed {
                            let cast_value = llvm_builder.build_float_to_signed_int(right_float, left_int.value.get_type(), "");
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_signed_div(left_int.value, cast_value, "") }
                                )
                            )
                        } else {
                            let cast_value = llvm_builder.build_float_to_unsigned_int(right_float, left_int.value.get_type(), "");
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_unsigned_div(left_int.value, cast_value, "") }
                                )
                            )
                        }
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_signed_int_to_float(right_int.value, left_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(right_int.value, left_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_div(left_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(right_float, left_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_div(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        if right_int.signed {
                            let cast_value = llvm_builder.build_int_cast(left_int.value, right_int.value.get_type(), "");
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_signed_div(left_int.value, cast_value, "") }
                                )
                            )
                        } else {
                            let cast_value = llvm_builder.build_int_cast(left_int.value, right_int.value.get_type(), "");
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_unsigned_div(left_int.value, cast_value, "") }
                                )
                            )
                        }
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_signed_int_to_float(left_int.value, right_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(left_int.value, right_float.get_type(), "")
                        };
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_div(right_float, cast_value, "")
                            )
                        )
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        if right_int.signed {
                            let cast_value = llvm_builder.build_float_to_signed_int(left_float, right_int.value.get_type(), "");
                            Ok(NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_signed_div(cast_value, right_int.value, "") }
                            ))
                        } else {
                            let cast_value = llvm_builder.build_float_to_unsigned_int(left_float, right_int.value.get_type(), "");
                            Ok(NumericValueEnum::Integer(
                                IntegerValue { signed: right_int.signed, value: llvm_builder.build_int_unsigned_div(cast_value, right_int.value, "") }
                            ))
                        }
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(left_float, right_float.get_type(), "");
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_div(left_float, cast_value, "")
                            )
                        )
                    }
                }
            }
            Ordering::Equal => {
                //类型相同，无需提升
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        if left_int.signed {
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_signed_div(left_int.value, right_int.value, "") }
                                )
                            )
                        } else {
                            Ok(
                                NumericValueEnum::Integer(
                                    IntegerValue { signed: left_int.signed, value: llvm_builder.build_int_unsigned_div(left_int.value, right_int.value, "") }
                                )
                            )
                        }
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        Ok(
                            NumericValueEnum::Float(
                                llvm_builder.build_float_div(left_float, right_float, "")
                            )
                        )
                    }
                    _ => { unreachable!() }
                }
            }
        }
    }

    pub fn build_numeric_compare<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: NumericValueEnum<'s>, rhs: NumericValueEnum<'s>, op: CompareOperator) -> Result<IntegerValue<'s>> {
        let (left_providence, right_providence) = (get_number_providence(&lhs.get_type()), get_number_providence(&rhs.get_type()));
        match left_providence.cmp(&right_providence) {
            Ordering::Less => {
                //右边的类型要提升至左边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int_)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int_.value, left_int.value.get_type(), "");
                        Self::build_integer_compare(llvm_builder, llvm_context, left_int.value, cast_value, left_int.signed, op)
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_float_to_signed_int(right_float, left_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(right_float, left_int.value.get_type(), "")
                        };
                        Self::build_integer_compare(llvm_builder, llvm_context, left_int.value, cast_value, left_int.signed, op)
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value: FloatValue = if right_int.signed {
                            llvm_builder.build_signed_int_to_float(right_int.value, left_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(right_int.value, left_float.get_type(), "")
                        };
                        Self::build_float_compare(llvm_builder, llvm_context, cast_value, left_float, op)
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(right_float, left_float.get_type(), "");
                        Self::build_float_compare(llvm_builder, llvm_context, left_float, cast_value, op)
                    }
                }
            }
            Ordering::Greater => {
                //左边的类型要提升至右边的类型
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = llvm_builder.build_int_cast(right_int.value, left_int.value.get_type(), "");
                        Self::build_integer_compare(llvm_builder, llvm_context, left_int.value, cast_value, right_int.signed, op)
                    }
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Float(right_float)) => {
                        let cast_value = if left_int.signed {
                            llvm_builder.build_signed_int_to_float(left_int.value, right_float.get_type(), "")
                        } else {
                            llvm_builder.build_unsigned_int_to_float(left_int.value, right_float.get_type(), "")
                        };
                        Self::build_float_compare(llvm_builder, llvm_context, cast_value, right_float, op)
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Integer(right_int)) => {
                        let cast_value = if right_int.signed {
                            llvm_builder.build_float_to_signed_int(left_float, right_int.value.get_type(), "")
                        } else {
                            llvm_builder.build_float_to_unsigned_int(left_float, right_int.value.get_type(), "")
                        };
                        Self::build_integer_compare(llvm_builder, llvm_context, cast_value, right_int.value.clone(), right_int.signed, op)
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        let cast_value = llvm_builder.build_float_cast(left_float, right_float.get_type(), "");
                        Self::build_float_compare(llvm_builder, llvm_context, left_float, cast_value, op)
                    }
                }
            }
            Ordering::Equal => {
                //类型相同，无需提升
                match (lhs, rhs) {
                    (NumericValueEnum::Integer(left_int), NumericValueEnum::Integer(right_int)) => {
                        Self::build_integer_compare(llvm_builder, llvm_context, left_int.value, right_int.value, left_int.signed, op)
                    }
                    (NumericValueEnum::Float(left_float), NumericValueEnum::Float(right_float)) => {
                        Self::build_float_compare(llvm_builder, llvm_context, left_float, right_float, op)
                    }
                    _ => { unreachable!() }
                }
            }
        }
    }

    fn build_integer_compare<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: IntValue<'s>, rhs: IntValue<'s>, signed: bool, op: CompareOperator) -> Result<IntegerValue<'s>> {
        if signed {
            Ok(
                IntegerValue {
                    signed,
                    value: match op {
                        CompareOperator::Equal => { llvm_builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "") }
                        CompareOperator::GreaterThan => { llvm_builder.build_int_compare(IntPredicate::SGT, lhs, rhs, "") }
                        CompareOperator::LessThan => { llvm_builder.build_int_compare(IntPredicate::SLT, lhs, rhs, "") }
                        CompareOperator::GreaterOrEqualThan => { llvm_builder.build_int_compare(IntPredicate::SGE, lhs, rhs, "") }
                        CompareOperator::LessOrEqualThan => { llvm_builder.build_int_compare(IntPredicate::SLE, lhs, rhs, "") }
                    },
                }
            )
        } else {
            Ok(IntegerValue {
                signed,
                value: match op {
                    CompareOperator::Equal => { llvm_builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "") }
                    CompareOperator::GreaterThan => { llvm_builder.build_int_compare(IntPredicate::UGT, lhs, rhs, "") }
                    CompareOperator::LessThan => { llvm_builder.build_int_compare(IntPredicate::ULT, lhs, rhs, "") }
                    CompareOperator::GreaterOrEqualThan => { llvm_builder.build_int_compare(IntPredicate::UGT, lhs, rhs, "") }
                    CompareOperator::LessOrEqualThan => { llvm_builder.build_int_compare(IntPredicate::ULT, lhs, rhs, "") }
                },
            })
        }
    }

    fn build_float_compare<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, lhs: FloatValue<'s>, rhs: FloatValue<'s>, op: CompareOperator) -> Result<IntegerValue<'s>> {
        match op {
            CompareOperator::Equal => { Ok(IntegerValue { signed: true, value: llvm_builder.build_float_compare(FloatPredicate::OEQ, lhs, rhs, "") }) }
            CompareOperator::GreaterThan => { Ok(IntegerValue { signed: true, value: llvm_builder.build_float_compare(FloatPredicate::OGT, lhs, rhs, "") }) }
            CompareOperator::LessThan => { Ok(IntegerValue { signed: true, value: llvm_builder.build_float_compare(FloatPredicate::OLT, lhs, rhs, "") }) }
            CompareOperator::GreaterOrEqualThan => { Ok(IntegerValue { signed: true, value: llvm_builder.build_float_compare(FloatPredicate::OGE, lhs, rhs, "") }) }
            CompareOperator::LessOrEqualThan => { Ok(IntegerValue { signed: true, value: llvm_builder.build_float_compare(FloatPredicate::OLE, lhs, rhs, "") }) }
        }
    }

    pub fn build_numeric_assign<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, variable_pointer: PointerValue<'s>, variable_ty: NumericTypeEnum<'s>, rhs: NumericValueEnum<'s>) -> Result<NumericValueEnum<'s>> {
        let cast_value = Self::build_numeric_cast(llvm_builder, llvm_context, rhs, variable_ty)?;
        match cast_value {
            NumericValueEnum::Float(f) => { llvm_builder.build_store(variable_pointer, f); }
            NumericValueEnum::Integer(i) => { llvm_builder.build_store(variable_pointer, i.value); }
        }
        Ok(cast_value)
    }

    pub fn build_numeric_cast<'s>(llvm_builder: &Builder<'s>, llvm_context: &Context, from: NumericValueEnum<'s>, to: NumericTypeEnum<'s>) -> Result<NumericValueEnum<'s>> {
        if from.get_type()==to {
            return Ok(from)
        }
        match (from, to) {
            (NumericValueEnum::Integer(left_int), NumericTypeEnum::IntegerType(target_type)) => {
                Ok(
                    NumericValueEnum::Integer(
                        IntegerValue { signed: target_type.signed, value: llvm_builder.build_int_cast(left_int.value, target_type.value, "") }
                    )
                )
            }
            (NumericValueEnum::Integer(left_int), NumericTypeEnum::FloatType(target_type)) => {
                if left_int.signed {
                    Ok(
                        NumericValueEnum::Float(
                            llvm_builder.build_signed_int_to_float(left_int.value, target_type, "")
                        )
                    )
                } else {
                    Ok(
                        NumericValueEnum::Float(
                            llvm_builder.build_unsigned_int_to_float(left_int.value, target_type, "")
                        )
                    )
                }
            }
            (NumericValueEnum::Float(left_float), NumericTypeEnum::IntegerType(target_type)) => {
                if target_type.signed {
                    Ok(
                        NumericValueEnum::Integer(
                            IntegerValue { signed: target_type.signed, value: llvm_builder.build_float_to_signed_int(left_float, target_type.value, "") }
                        )
                    )
                } else {
                    Ok(
                        NumericValueEnum::Integer(
                            IntegerValue { signed: target_type.signed, value: llvm_builder.build_float_to_unsigned_int(left_float, target_type.value, "") }
                        )
                    )
                }
            }
            (NumericValueEnum::Float(left_float), NumericTypeEnum::FloatType(target_type)) => {
                Ok(
                    NumericValueEnum::Float(
                        llvm_builder.build_float_cast(left_float, target_type, "")
                    )
                )
            }
        }
    }
}
