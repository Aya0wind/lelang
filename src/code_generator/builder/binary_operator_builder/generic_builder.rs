use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, FunctionValue};

use crate::ast::nodes::Position;
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEBoolValue, LEContext, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::{LogicBinaryOperator, MathOperatorBuilder};
use crate::code_generator::builder::binary_operator_builder::traits::{ArithmeticOperatorBuilder, CompareBinaryOperator};
use crate::error::CompileError;

use super::super::Result;

pub struct GenericBuilder;

impl GenericBuilder {
    pub fn build_float_to_integer<'ctx>(le_context: &LEContext<'ctx>, float_value: LEFloatValue<'ctx>, rhs: LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        if rhs.signed() {
            Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_float_to_signed_int(float_value.llvm_value, rhs.get_llvm_type(), "") })
        } else {
            Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_float_to_unsigned_int(float_value.llvm_value, rhs.get_llvm_type(), "") })
        }
    }

    pub fn build_integer_to_float<'ctx>(le_context: &LEContext<'ctx>, int_value: LEIntegerValue<'ctx>, rhs: LEFloatType<'ctx>) -> Result<LEFloatValue<'ctx>> {
        if int_value.ty.signed() {
            Ok(LEFloatValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_signed_int_to_float(int_value.llvm_value, rhs.get_llvm_type(), "") })
        } else {
            Ok(LEFloatValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_unsigned_int_to_float(int_value.llvm_value, rhs.get_llvm_type(), "") })
        }
    }

    pub fn build_integer_to_integer<'ctx>(le_context: &LEContext<'ctx>, lhs: LEIntegerValue<'ctx>, rhs: LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        Ok(LEIntegerValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_int_cast(lhs.llvm_value, rhs.get_llvm_type(), "") })
    }

    pub fn build_float_to_float<'ctx>(le_context: &LEContext<'ctx>, lhs: LEFloatValue<'ctx>, rhs: LEFloatType<'ctx>) -> Result<LEFloatValue<'ctx>> {
        Ok(LEFloatValue { ty: rhs.clone(), llvm_value: le_context.llvm_builder.build_float_cast(lhs.llvm_value, rhs.get_llvm_type(), "") })
    }

    pub fn build_add<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                Ok(left.build_add(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                Ok(left.build_add(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Float(right)) => {
                let casted_value = Self::build_float_to_integer(le_context, right, left.ty.clone())?;
                Ok(left.build_add(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Integer(right)) => {
                let casted_value = Self::build_integer_to_float(le_context, right, left.ty.clone())?;
                Ok(left.build_add(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Struct(left), LEBasicValueEnum::Struct(right)) => {
                Ok(left.build_add(le_context, right)?.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_sub<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                Ok(left.build_sub(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                Ok(left.build_sub(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Float(right)) => {
                let casted_value = Self::build_float_to_integer(le_context, right, left.ty.clone())?;
                Ok(left.build_sub(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Integer(right)) => {
                let casted_value = Self::build_integer_to_float(le_context, right, left.ty.clone())?;
                Ok(left.build_sub(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Struct(left), LEBasicValueEnum::Struct(right)) => {
                Ok(left.build_sub(le_context, right)?.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_mul<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                Ok(left.build_mul(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                Ok(left.build_mul(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Float(right)) => {
                let casted_value = Self::build_float_to_integer(le_context, right, left.ty.clone())?;
                Ok(left.build_mul(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Integer(right)) => {
                let casted_value = Self::build_integer_to_float(le_context, right, left.ty.clone())?;
                Ok(left.build_mul(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Struct(left), LEBasicValueEnum::Struct(right)) => {
                Ok(left.build_mul(le_context, right)?.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_div<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                Ok(left.build_div(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                Ok(left.build_div(le_context, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Float(right)) => {
                let casted_value = Self::build_float_to_integer(le_context, right, left.ty.clone())?;
                Ok(left.build_div(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Integer(right)) => {
                let casted_value = Self::build_integer_to_float(le_context, right, left.ty.clone())?;
                Ok(left.build_div(le_context, casted_value)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Struct(left), LEBasicValueEnum::Struct(right)) => {
                Ok(left.build_div(le_context, right)?.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_compare<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Integer(right)) => {
                left.build_cmp(le_context, right, op)
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Float(right)) => {
                left.build_cmp(le_context, right, op)
            }
            (LEBasicValueEnum::Integer(left), LEBasicValueEnum::Float(right)) => {
                let casted_value = Self::build_float_to_integer(le_context, right, left.ty.clone())?;
                left.build_cmp(le_context, casted_value, op)
            }
            (LEBasicValueEnum::Float(left), LEBasicValueEnum::Integer(right)) => {
                let casted_value = Self::build_integer_to_float(le_context, right, left.ty.clone())?;
                left.build_cmp(le_context, casted_value, op)
            }
            (LEBasicValueEnum::Struct(left), LEBasicValueEnum::Struct(right)) => {
                left.build_cmp(le_context, right, op)
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_cast<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicTypeEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match (lhs.clone(), rhs) {
            (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::Integer(right)) => {
                Ok(Self::build_integer_to_integer(le_context, left, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicTypeEnum::Float(right)) => {
                Ok(Self::build_float_to_float(le_context, left, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::Float(right)) => {
                Ok(Self::build_integer_to_float(le_context, left, right)?.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicTypeEnum::Integer(right)) => {
                Ok(Self::build_float_to_integer(le_context, left, right)?.to_le_value_enum())
            }
            _ => { Ok(lhs) }
        }
    }

    pub fn build_mod<'ctx>(le_context: &LEContext<'ctx>, lhs: LEIntegerValue<'ctx>, rhs: LEIntegerValue<'ctx>) -> Result<LEIntegerValue<'ctx>> {
        lhs.build_mod(le_context, rhs)
    }


    pub fn build_logic<'ctx>(le_context: &LEContext<'ctx>, lhs: LEBoolValue<'ctx>, rhs: LEBoolValue<'ctx>, op: LogicBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let result = match op {
            LogicBinaryOperator::LogicAnd => { le_context.llvm_builder.build_and(lhs.llvm_value, rhs.llvm_value, "") }
            LogicBinaryOperator::LogicOr => { le_context.llvm_builder.build_or(lhs.llvm_value, rhs.llvm_value, "") }
            LogicBinaryOperator::LogicXor => { le_context.llvm_builder.build_xor(lhs.llvm_value, rhs.llvm_value, "") }
        };
        Ok(LEBoolValue { ty: lhs.ty.clone(), llvm_value: result })
    }
}