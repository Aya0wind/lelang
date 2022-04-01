use anyhow::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValue, BasicMetadataValueEnum, FunctionValue};

use crate::ast::Position;
use crate::code_generator::builder::{CompareOperator, LEVariable, NumericTypeEnum};
use crate::code_generator::builder::llvm_wrapper::{IntegerValue, LETypeEnum, LEValueEnum, NumericValueEnum};
use crate::code_generator::builder::numeric_operator_builder::NumericOperatorBuilder;
use crate::error::CompileError;

pub struct LEBuilder<'s> {
    pub llvm_context: &'s Context,
    pub llvm_builder: Builder<'s>,
}

impl<'s> LEBuilder<'s> {
    pub fn new(llvm_context: &'s Context, llvm_builder: Builder<'s>) -> Self {
        Self { llvm_context, llvm_builder }
    }


    pub fn build_load_variable(&self, variable: LEVariable<'s>) -> Result<LEValueEnum<'s>> {
        match variable.ty {
            LETypeEnum::NumericType(_) => {
                Ok(self.llvm_builder.build_load(variable.pointer, "").into())
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_add(&self, lhs: LEValueEnum<'s>, rhs: LEValueEnum<'s>) -> Result<LEValueEnum<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LEValueEnum::NumericValue(right_number)) => {
                return Ok(LEValueEnum::NumericValue(NumericOperatorBuilder::build_numeric_add(
                    &self.llvm_builder,
                    self.llvm_context,
                    left_number, right_number)?));
            }
            _ => { unimplemented!() }
        }
    }


    pub fn build_sub(&self, lhs: LEValueEnum<'s>, rhs: LEValueEnum<'s>) -> Result<LEValueEnum<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LEValueEnum::NumericValue(right_number)) => {
                return Ok(
                    LEValueEnum::NumericValue(NumericOperatorBuilder::build_numeric_sub(
                        &self.llvm_builder,
                        self.llvm_context,
                        left_number,
                        right_number)?)
                );
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_mul(&self, lhs: LEValueEnum<'s>, rhs: LEValueEnum<'s>) -> Result<LEValueEnum<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LEValueEnum::NumericValue(right_number)) => {
                return Ok(LEValueEnum::NumericValue(NumericOperatorBuilder::build_numeric_mul(
                    &self.llvm_builder,
                    self.llvm_context,
                    left_number,
                    right_number)?));
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_div(&self, lhs: LEValueEnum<'s>, rhs: LEValueEnum<'s>) -> Result<LEValueEnum<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LEValueEnum::NumericValue(right_number)) => {
                Ok(LEValueEnum::NumericValue(NumericOperatorBuilder::build_numeric_div(
                    &self.llvm_builder,
                    self.llvm_context,
                    left_number,
                    right_number)?))
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_cast(&self, lhs: LEValueEnum<'s>, rhs: LETypeEnum<'s>) -> Result<LEValueEnum<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LETypeEnum::NumericType(target_type)) => {
                Ok(LEValueEnum::NumericValue(NumericOperatorBuilder::build_numeric_cast(
                    &self.llvm_builder,
                    self.llvm_context,
                    left_number,
                    target_type)?))
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_compare(&self, lhs: LEValueEnum<'s>, rhs: LEValueEnum<'s>, op: CompareOperator) -> Result<IntegerValue<'s>> {
        match (lhs, rhs) {
            (LEValueEnum::NumericValue(left_number), LEValueEnum::NumericValue(right_number)) => {
                NumericOperatorBuilder::build_numeric_compare(
                    &self.llvm_builder,
                    self.llvm_context,
                    left_number,
                    right_number,
                    op)
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_store(&self, variable: LEVariable<'s>, rhs: LEValueEnum<'s>, pos: Position) -> Result<LEValueEnum<'s>> {
        if let LEValueEnum::NumericValue(rhs) = rhs {
            if let LETypeEnum::NumericType(numeric_ty) = variable.ty {
                return NumericOperatorBuilder::build_numeric_assign(
                    &self.llvm_builder,
                    self.llvm_context,
                    variable.pointer,
                    numeric_ty,
                    rhs).map(LEValueEnum::NumericValue)
            }
        }
        Err(CompileError::type_mismatched(variable.ty.to_string(), rhs.get_type().to_string(), pos).into())
    }

    pub fn build_alloca(&self, value: LETypeEnum<'s>) -> Result<LEVariable<'s>> {
        match value {
            LETypeEnum::NumericType(number) => {
                match number {
                    NumericTypeEnum::FloatType(f) => {
                        let alloc = self.llvm_builder.build_alloca(f, "");
                        Ok(LEVariable { ty: value, pointer: alloc })
                    }
                    NumericTypeEnum::IntegerType(i) => {
                        let alloc = self.llvm_builder.build_alloca(i.value, "");
                        Ok(LEVariable { ty: i.value.into(), pointer: alloc })
                    }
                }
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_load(&self, variable: LEVariable<'s>) -> Result<LEValueEnum<'s>> {
        match variable.ty {
            LETypeEnum::NumericType(_) => {
                let pointer = variable.pointer;
                Ok(self.llvm_builder.build_load(pointer, "").into())
            }
            _ => { unimplemented!() }
        }
    }


    pub fn build_call(&self, function: FunctionValue<'s>, params: &[LEValueEnum<'s>]) -> Result<LEValueEnum<'s>> {
        let mut args = vec![];
        for (param, argument) in function.get_param_iter().zip(params.iter()) {
            let param_type: LETypeEnum = param.get_type().into();
            if param_type != argument.get_type() {
                let argument = self.build_cast(*argument, param_type)?;
                match argument {
                    LEValueEnum::NumericValue(number) => {
                        let arg = match number {
                            NumericValueEnum::Float(f) => {
                                BasicMetadataValueEnum::FloatValue(f)
                            }
                            NumericValueEnum::Integer(i) => {
                                BasicMetadataValueEnum::IntValue(i.value)
                            }
                        };
                        args.push(arg);
                    }
                    _ => { unimplemented!() }
                }
            } else {
                match argument {
                    LEValueEnum::NumericValue(number) => {
                        let arg = match number {
                            NumericValueEnum::Float(f) => {
                                BasicMetadataValueEnum::FloatValue(*f)
                            }
                            NumericValueEnum::Integer(i) => {
                                BasicMetadataValueEnum::IntValue(i.value)
                            }
                        };
                        args.push(arg);
                    }
                    _ => { unimplemented!() }
                }
            }
        }
        let site_value = self.llvm_builder.build_call(function, &args, "");
        Ok(site_value.as_any_value_enum().into())
    }
}