use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, FunctionValue};

use crate::ast::nodes::Position;
use crate::code_generator::builder::binary_operator_builder::traits::{BinaryOpBuilder, CompareOperator};
use crate::code_generator::builder::llvm_type_wrapper::{LEArrayValue, LEBasicTypeGenericRef, LEBasicValue, LEBasicValueEnum, LEFloatType, LEFloatValue, LEIntegerType, LEIntegerValue, LEPointerValue, LEStructValue};
use crate::code_generator::builder::llvm_type_wrapper::LEBasicValueEnum::IntegerValue;
use crate::error::CompileError;

use super::super::Result;

pub struct GenericBuilder;

impl GenericBuilder {
    pub fn build_float_to_integer<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, float_value: LEFloatValue<'ctx, 'a>, rhs: &'a LEIntegerType<'ctx>) -> Result<LEIntegerValue<'ctx, 'a>> {
        if rhs.signed {
            Ok(LEIntegerValue { ty: rhs, llvm_value: llvm_builder.build_float_to_signed_int(float_value.llvm_value, rhs.llvm_type, "") })
        } else {
            Ok(LEIntegerValue { ty: rhs, llvm_value: llvm_builder.build_float_to_unsigned_int(float_value.llvm_value, rhs.llvm_type, "") })
        }
    }

    pub fn build_integer_to_float<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, int_value: LEIntegerValue<'ctx, 'a>, rhs: &'a LEFloatType<'ctx>) -> Result<LEFloatValue<'ctx, 'a>> {
        if int_value.ty.signed {
            Ok(LEFloatValue { ty: rhs, llvm_value: llvm_builder.build_signed_int_to_float(int_value.llvm_value, rhs.llvm_type, "") })
        } else {
            Ok(LEFloatValue { ty: rhs, llvm_value: llvm_builder.build_unsigned_int_to_float(int_value.llvm_value, rhs.llvm_type, "") })
        }
    }

    pub fn build_add<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                Ok(left.build_add(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::FloatValue(right)) => {
                Ok(left.build_add(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::FloatValue(right)) => {
                let casted_value = Self::build_float_to_integer(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_add(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                let casted_value = Self::build_integer_to_float(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_add(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::StructValue(left), LEBasicValueEnum::StructValue(right)) => {
                Ok(left.build_add(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_sub<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                Ok(left.build_sub(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::FloatValue(right)) => {
                Ok(left.build_sub(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::FloatValue(right)) => {
                let casted_value = Self::build_float_to_integer(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_sub(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                let casted_value = Self::build_integer_to_float(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_sub(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::StructValue(left), LEBasicValueEnum::StructValue(right)) => {
                Ok(left.build_sub(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_mul<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                Ok(left.build_mul(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::FloatValue(right)) => {
                Ok(left.build_mul(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::FloatValue(right)) => {
                let casted_value = Self::build_float_to_integer(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_mul(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                let casted_value = Self::build_integer_to_float(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_mul(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::StructValue(left), LEBasicValueEnum::StructValue(right)) => {
                Ok(left.build_mul(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_div<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                Ok(left.build_div(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::FloatValue(right)) => {
                Ok(left.build_div(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::FloatValue(right)) => {
                let casted_value = Self::build_float_to_integer(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_div(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                let casted_value = Self::build_integer_to_float(llvm_builder, llvm_context, right, left.ty)?;
                Ok(left.build_div(llvm_builder, llvm_context, &casted_value)?.as_le_value_enum())
            }
            (LEBasicValueEnum::StructValue(left), LEBasicValueEnum::StructValue(right)) => {
                Ok(left.build_div(llvm_builder, llvm_context, &right)?.as_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    pub fn build_compare<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>, op: CompareOperator) -> Result<LEIntegerValue<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                left.build_cmp(llvm_builder, llvm_context, &right, op)
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::FloatValue(right)) => {
                left.build_cmp(llvm_builder, llvm_context, &right, op)
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicValueEnum::FloatValue(right)) => {
                let casted_value = Self::build_float_to_integer(llvm_builder, llvm_context, right, left.ty)?;
                left.build_cmp(llvm_builder, llvm_context, &casted_value, op)
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicValueEnum::IntegerValue(right)) => {
                let casted_value = Self::build_integer_to_float(llvm_builder, llvm_context, right, left.ty)?;
                left.build_cmp(llvm_builder, llvm_context, &casted_value, op)
            }
            (LEBasicValueEnum::StructValue(left), LEBasicValueEnum::StructValue(right)) => {
                left.build_cmp(llvm_builder, llvm_context, &right, op)
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_cast<'ctx, 'a>(llvm_builder: &Builder<'ctx>, llvm_context: &Context, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: &'a LEBasicTypeGenericRef<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match (lhs, rhs) {
            (LEBasicValueEnum::IntegerValue(left), LEBasicTypeGenericRef::IntegerType(right)) => {
                Ok(LEIntegerValue { ty: right, llvm_value: llvm_builder.build_int_cast(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicTypeGenericRef::FloatType(right)) => {
                Ok(LEFloatValue { ty: right, llvm_value: llvm_builder.build_float_cast(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
            }
            (LEBasicValueEnum::IntegerValue(left), LEBasicTypeGenericRef::FloatType(right)) => {
                if left.ty.signed {
                    Ok(LEFloatValue { ty: right, llvm_value: llvm_builder.build_signed_int_to_float(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
                } else {
                    Ok(LEFloatValue { ty: right, llvm_value: llvm_builder.build_unsigned_int_to_float(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
                }
            }
            (LEBasicValueEnum::FloatValue(left), LEBasicTypeGenericRef::IntegerType(right)) => {
                if right.signed {
                    Ok(LEIntegerValue { ty: right, llvm_value: llvm_builder.build_float_to_signed_int(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
                } else {
                    Ok(LEIntegerValue { ty: right, llvm_value: llvm_builder.build_float_to_unsigned_int(left.llvm_value, right.llvm_type, "") }.as_le_value_enum())
                }
            }
            _ => { unimplemented!() }
        }
    }
}