use inkwell::builder::Builder;

use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEBoolValue, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::{LogicBinaryOperator, ModOperateValue};
use crate::code_generator::builder::binary_operator_builder::traits::{BasicMathOperateValue, CompareBinaryOperator};
use crate::code_generator::context::LEContext;
use crate::code_generator::Result;
use crate::error::CompileError;
use crate::lexer::Operator;

pub struct MathOperateBuilder<'a, 'ctx> {
    llvm_builder: &'a Builder<'ctx>,
}


impl<'a, 'ctx> MathOperateBuilder<'a, 'ctx> {
    pub fn new(llvm_builder: &'a Builder<'ctx>) -> Self {
        Self { llvm_builder }
    }


    pub fn build_float_to_integer(&self, le_context: &LEContext<'ctx>, float_value: LEFloatValue<'ctx>, rhs: LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        if rhs.signed() {
            Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_float_to_signed_int(float_value.llvm_value, rhs.get_llvm_type(), "") })
        } else {
            Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_float_to_unsigned_int(float_value.llvm_value, rhs.get_llvm_type(), "") })
        }
    }

    pub fn build_integer_to_float(&self, le_context: &LEContext<'ctx>, int_value: LEIntegerValue<'ctx>, rhs: LEFloatType<'ctx>) -> Result<LEFloatValue<'ctx>> {
        if int_value.ty.signed() {
            Ok(LEFloatValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_signed_int_to_float(int_value.llvm_value, rhs.get_llvm_type(), "") })
        } else {
            Ok(LEFloatValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_unsigned_int_to_float(int_value.llvm_value, rhs.get_llvm_type(), "") })
        }
    }

    pub fn build_integer_to_integer(&self, le_context: &LEContext<'ctx>, lhs: LEIntegerValue<'ctx>, rhs: LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_int_cast(lhs.llvm_value, rhs.get_llvm_type(), "") })
    }
    pub fn build_bool_to_integer(&self, le_context: &LEContext<'ctx>, lhs: LEBoolValue<'ctx>, rhs: LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_int_cast(lhs.llvm_value, rhs.get_llvm_type(), "") })
    }

    pub fn build_float_to_float(&self, le_context: &LEContext<'ctx>, lhs: LEFloatValue<'ctx>, rhs: LEFloatType<'ctx>) -> Result<LEFloatValue<'ctx>> {
        Ok(LEFloatValue { ty: rhs.clone(), llvm_value: self.llvm_builder.build_float_cast(lhs.llvm_value, rhs.get_llvm_type(), "") })
    }

    pub fn build_add(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        let right_type = LEBasicValue::get_le_type(&rhs);
        if left_type == right_type {
            match (lhs, rhs) {
                (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                    Ok(left.build_add_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                    Ok(left.build_add_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                _ => {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Plus,
                        left_type: left_type.to_string(),
                        right_type: right_type.to_string(),
                    })
                }
            }
        } else {
            Err(CompileError::NoSuitableBinaryOperator {
                op: Operator::Plus,
                left_type: left_type.to_string(),
                right_type: right_type.to_string(),
            })
        }
    }

    pub fn build_sub(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        let right_type = LEBasicValue::get_le_type(&rhs);
        if left_type == right_type {
            match (lhs, rhs) {
                (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                    Ok(left.build_sub_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                    Ok(left.build_sub_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                _ => {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Plus,
                        left_type: left_type.to_string(),
                        right_type: right_type.to_string(),
                    })
                }
            }
        } else {
            Err(CompileError::NoSuitableBinaryOperator {
                op: Operator::Plus,
                left_type: left_type.to_string(),
                right_type: right_type.to_string(),
            })
        }
    }
    pub fn build_mul(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        let right_type = LEBasicValue::get_le_type(&rhs);
        if left_type == right_type {
            match (lhs, rhs) {
                (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                    Ok(left.build_mul_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                    Ok(left.build_mul_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                _ => {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Plus,
                        left_type: left_type.to_string(),
                        right_type: right_type.to_string(),
                    })
                }
            }
        } else {
            Err(CompileError::NoSuitableBinaryOperator {
                op: Operator::Plus,
                left_type: left_type.to_string(),
                right_type: right_type.to_string(),
            })
        }
    }
    pub fn build_div(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        let right_type = LEBasicValue::get_le_type(&rhs);
        if left_type == right_type {
            match (lhs, rhs) {
                (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                    Ok(left.build_div_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                    Ok(left.build_div_unchecked(le_context, self.llvm_builder, right).to_le_value_enum())
                }
                _ => {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Plus,
                        left_type: left_type.to_string(),
                        right_type: right_type.to_string(),
                    })
                }
            }
        } else {
            Err(CompileError::NoSuitableBinaryOperator {
                op: Operator::Plus,
                left_type: left_type.to_string(),
                right_type: right_type.to_string(),
            })
        }
    }
    pub fn build_compare(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        let right_type = LEBasicValue::get_le_type(&rhs);
        if left_type == right_type {
            match (lhs, rhs) {
                (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                    Ok(left.build_cmp_unchecked(le_context, self.llvm_builder, op, right))
                }
                (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                    Ok(left.build_cmp_unchecked(le_context, self.llvm_builder, op, right))
                }
                _ => {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Plus,
                        left_type: left_type.to_string(),
                        right_type: right_type.to_string(),
                    })
                }
            }
        } else {
            Err(CompileError::NoSuitableBinaryOperator {
                op: Operator::Plus,
                left_type: left_type.to_string(),
                right_type: right_type.to_string(),
            })
        }
    }

    pub fn build_cast(&self, le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicTypeEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left_type = LEBasicValue::get_le_type(&lhs);
        if left_type == rhs {
            Ok(lhs)
        } else {
            match (lhs.clone(), rhs.clone()) {
                (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::Integer(right)) => {
                    Ok(self.build_integer_to_integer(le_context, left, right)?.to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicTypeEnum::Float(right)) => {
                    Ok(self.build_float_to_float(le_context, left, right)?.to_le_value_enum())
                }
                (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::Float(right)) => {
                    Ok(self.build_integer_to_float(le_context, left, right)?.to_le_value_enum())
                }
                (LEBasicValueEnum::Float(left), LEBasicTypeEnum::Integer(right)) => {
                    Ok(self.build_float_to_integer(le_context, left, right)?.to_le_value_enum())
                }
                (LEBasicValueEnum::Bool(left), LEBasicTypeEnum::Integer(right)) => {
                    Ok(self.build_bool_to_integer(le_context, left, right)?.to_le_value_enum())
                }
                _ => { Err(CompileError::TypeMismatched { expect: rhs.to_string(), found: rhs.to_string() }) }
            }
        }
    }

    pub fn build_mod(&self, le_context: &LEContext<'ctx>, lhs: LEIntegerValue<'ctx>, rhs: LEIntegerValue<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        if lhs.ty == rhs.ty {
            Ok(lhs.build_mod_unchecked(le_context, self.llvm_builder, rhs))
        } else {
            Err(CompileError::TypeMismatched { expect: lhs.ty.to_string(), found: rhs.ty.to_string() })
        }
    }


    pub fn build_logic(&self, le_context: &LEContext<'ctx>, lhs: LEBoolValue<'ctx>, rhs: LEBoolValue<'ctx>, op: LogicBinaryOperator) -> LEBoolValue<'ctx> {
        let result = match op {
            LogicBinaryOperator::And => { self.llvm_builder.build_and(lhs.llvm_value, rhs.llvm_value, "") }
            LogicBinaryOperator::Or => { self.llvm_builder.build_or(lhs.llvm_value, rhs.llvm_value, "") }
            LogicBinaryOperator::Xor => { self.llvm_builder.build_xor(lhs.llvm_value, rhs.llvm_value, "") }
        };
        LEBoolValue { ty: lhs.ty.clone(), llvm_value: result }
    }
}