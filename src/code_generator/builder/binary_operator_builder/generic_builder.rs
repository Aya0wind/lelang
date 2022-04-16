use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, FunctionValue};

use crate::ast::nodes::Position;
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEBoolValue, LEContext, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEType};
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
            (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::IntegerType(right)) => {
                Ok(LEIntegerValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_int_cast(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
            }
            (LEBasicValueEnum::Float(left), LEBasicTypeEnum::FloatType(right)) => {
                Ok(LEFloatValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_float_cast(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
            }
            (LEBasicValueEnum::Integer(left), LEBasicTypeEnum::FloatType(right)) => {
                if left.ty.signed() {
                    Ok(LEFloatValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_signed_int_to_float(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
                } else {
                    Ok(LEFloatValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_unsigned_int_to_float(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
                }
            }
            (LEBasicValueEnum::Float(left), LEBasicTypeEnum::IntegerType(right)) => {
                if right.signed() {
                    Ok(LEIntegerValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_float_to_signed_int(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
                } else {
                    Ok(LEIntegerValue { ty: right.clone(), llvm_value: le_context.llvm_builder.build_float_to_unsigned_int(left.llvm_value, right.get_llvm_type(), "") }.to_le_value_enum())
                }
            }
            _ => { Ok(lhs) }
        }
    }
}