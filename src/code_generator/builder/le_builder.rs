use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, InstructionValue};

use crate::ast::nodes::TypeDeclarator;
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEBoolType, LEBoolValue, LEFloatType, LEFloatValue, LEFunctionValue, LEIntegerType, LEIntegerValue, LEPointerType, LEPointerValue, LEType, LEValue};
use crate::code_generator::builder::binary_operator_builder::{CompareBinaryOperator, LogicBinaryOperator, MathOperateBuilder, MemberAccessOperateValue, ModOperateValue};
use crate::code_generator::builder::expression::Expression;
use crate::code_generator::context::LEContext;
use crate::error::CompileError;
use crate::lexer::{Operator, Position};

use super::super::Result;

pub struct LEBuilder<'ctx> {
    pub llvm_builder: Builder<'ctx>,
}

impl<'ctx> LEBuilder<'ctx> {
    pub fn new(llvm_builder: Builder<'ctx>) -> Self {
        Self {
            llvm_builder,
        }
    }

    pub fn read_expression(&self, le_context: &LEContext<'ctx>, expr: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match expr {
            Expression::Left(left_value) => {
                Ok(self.build_load(le_context, left_value))
            }
            Expression::Right(right_value) => {
                Ok(right_value)
            }
            Expression::Unit => {
                Err(CompileError::ExpressionIsNotRightValueExpression)
            }
        }
    }


    pub fn build_add(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_add(le_context, self.read_expression(le_context, lhs)?, self.read_expression(le_context, rhs)?)
    }


    pub fn build_sub(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_sub(le_context, self.read_expression(le_context, lhs)?, self.read_expression(le_context, rhs)?)
    }

    pub fn build_mul(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_mul(le_context, self.read_expression(le_context, lhs)?, self.read_expression(le_context, rhs)?)
    }

    pub fn build_div(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_div(le_context, self.read_expression(le_context, lhs)?, self.read_expression(le_context, rhs)?)
    }

    pub fn build_cast(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: LEBasicTypeEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_cast(le_context, self.read_expression(le_context, lhs)?, rhs)
    }

    pub fn build_compare(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        builder.build_compare(le_context, self.read_expression(le_context, lhs)?, self.read_expression(le_context, rhs)?, op)
    }


    pub fn build_binary_logic(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>, op: LogicBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        let left_value = builder.build_cast(
            le_context,
            self.read_expression(le_context, lhs)?,
            le_context.bool_type().to_le_type_enum(),
        )?;
        let right_value = builder.build_cast(
            le_context,
            self.read_expression(le_context, rhs)?,
            le_context.bool_type().to_le_type_enum(),
        )?;
        Ok(builder.build_logic(le_context, left_value.into_bool_value().unwrap(), right_value.into_bool_value().unwrap(), op))
    }

    pub fn build_unary_logic(&self, le_context: &LEContext<'ctx>, target: Expression<'ctx>, op: LogicBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        unimplemented!()
    }

    pub fn build_mod(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, rhs: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left = self.read_expression(le_context, lhs)?;
        let right = self.read_expression(le_context, rhs)?;
        let left_type = LEBasicValue::get_le_type(&left);
        let right_type = LEBasicValue::get_le_type(&right);
        match (left, right) {
            (LEBasicValueEnum::Integer(left_int), LEBasicValueEnum::Integer(right_int)) => {
                Ok(left_int.build_mod_unchecked(le_context, &self.llvm_builder, right_int).to_le_value_enum())
            }
            _ => {
                Err(CompileError::NoSuitableBinaryOperator {
                    op: Operator::Mod,
                    left_type: left_type.to_string(),
                    right_type: right_type.to_string(),
                })
            }
        }
    }

    pub fn build_dot(&self, le_context: &LEContext<'ctx>, lhs: Expression<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>> {
        if let Expression::Left(left_expr) = lhs {
            left_expr.build_dot_unchecked(le_context, &self.llvm_builder, member_name)
        } else {
            Err(CompileError::ExpressionIsNotRightValueExpression)
        }
    }


    pub fn build_call(&self, le_context: &LEContext<'ctx>, function: LEFunctionValue<'ctx>, params: &[Expression<'ctx>]) -> Result<Expression<'ctx>> {
        let mut args = vec![];
        let builder = MathOperateBuilder::new(&self.llvm_builder);
        for (param_type, argument) in function.ty.param_types().iter().zip(params.iter()) {
            let argument_value = self.read_expression(le_context, argument.clone())?;
            let argument_type = LEBasicValue::get_le_type(&argument_value);
            if param_type != &argument_type {
                return Err(CompileError::TypeMismatched { expect: param_type.to_string(), found: argument_type.to_string() });
            }
            args.push(BasicMetadataValueEnum::from(argument_value.to_llvm_basic_value_enum()));
        }
        let site_value = self.llvm_builder.build_call(function.llvm_value, &args, "");
        if let Some(v) = site_value.try_as_basic_value().left() {
            Ok(Expression::Right(LEBasicValueEnum::from_type_and_llvm_value(function.ty.return_type().unwrap(), v)?))
        } else {
            Ok(Expression::Unit)
        }
    }

    // pub fn build_alloca(&self, le_context: &LEContext<'ctx>, initial_value: ExpressionValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
    //     let value_type = self.read_expression_value(le_context, initial_value.clone())?.get_le_type();
    //     let llvm_pointer_value = self.llvm_builder.build_alloca(value_type.get_llvm_type(), "");
    //     let pointer_type = LEPointerType::new(le_context, value_type.clone());
    //     let ptr = LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value };
    //     self.build_store(le_context, ptr.clone(), initial_value)?;
    //     Ok(ptr)
    // }

    pub fn build_alloca(&self, le_context: &LEContext<'ctx>, ty: LEBasicTypeEnum<'ctx>) -> LEPointerValue<'ctx> {
        let llvm_pointer_value = self.llvm_builder.build_alloca(ty.get_llvm_type(), "");
        let pointer_type = LEPointerType::new(le_context, ty);
        LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value }
    }

    pub fn build_alloca_with_initial_value(&self, le_context: &LEContext<'ctx>, initial_value: LEBasicValueEnum<'ctx>) -> LEPointerValue<'ctx> {
        let target_type = LEBasicValue::get_le_type(&initial_value);
        let llvm_pointer_value = self.build_alloca(le_context, target_type.clone());
        self.llvm_builder.build_store(llvm_pointer_value.llvm_value, initial_value.get_llvm_value());
        let pointer_type = LEPointerType::new(le_context, target_type);
        llvm_pointer_value
    }

    pub fn build_global_alloca(ty: LEBasicTypeEnum<'ctx>, module: &Module<'ctx>, address_space: Option<AddressSpace>) -> LEPointerValue<'ctx> {
        let global_ptr = match ty {
            LEBasicTypeEnum::Integer(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Bool(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Float(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Pointer(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Array(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Struct(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Vector(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
        }.as_pointer_value();
        LEPointerValue::from_type_and_llvm_value(ty, BasicValueEnum::PointerValue(global_ptr)).unwrap()
    }

    pub fn build_global_alloca_with_initial_value(&self, value: LEBasicValueEnum<'ctx>, module: &Module<'ctx>, address_space: Option<AddressSpace>) -> LEPointerValue<'ctx> {
        let target_type = LEBasicValue::get_le_type(&value);
        let global_ptr = match target_type {
            LEBasicTypeEnum::Integer(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Bool(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Float(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Pointer(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Array(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Struct(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
            LEBasicTypeEnum::Vector(ref t) => { module.add_global(t.get_llvm_type(), address_space, "") }
        }.as_pointer_value();
        self.llvm_builder.build_store(global_ptr, value.get_llvm_value());
        LEPointerValue::from_type_and_llvm_value(target_type, BasicValueEnum::PointerValue(global_ptr)).unwrap()
    }


    pub fn build_load(&self, le_context: &LEContext<'ctx>, ptr: LEPointerValue<'ctx>) -> LEBasicValueEnum<'ctx> {
        let value_enum = self.llvm_builder.build_load(ptr.llvm_value, "");
        LEBasicValueEnum::from_type_and_llvm_value(ptr.ty.get_point_type(), value_enum).unwrap()
    }

    pub(crate) fn build_store(&self, le_context: &LEContext<'ctx>, ptr: LEPointerValue<'ctx>, value: LEBasicValueEnum<'ctx>) -> Result<()> {
        self.llvm_builder.build_store(ptr.llvm_value, value.to_llvm_basic_value_enum());
        Ok(())
    }

    pub fn build_assign(&self, le_context: &LEContext<'ctx>, target: Expression<'ctx>, value: Expression<'ctx>) -> Result<LEPointerValue<'ctx>> {
        if let Expression::Left(left_value) = target {
            let casted_value = self.build_cast(le_context, value, left_value.ty.get_point_type())?;
            self.llvm_builder.build_store(left_value.llvm_value, casted_value.to_llvm_basic_value_enum());
            Ok(left_value)
        } else {
            Err(CompileError::ExpressionIsNotLeftValueExpression)
        }
    }

    pub fn build_neg(&self, le_context: &LEContext<'ctx>, value: Expression<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let target_value = self.read_expression(le_context, value)?;
        match target_value {
            LEBasicValueEnum::Integer(i) => {
                let result = self.llvm_builder.build_int_neg(i.llvm_value, "");
                Ok(LEIntegerValue { ty: i.ty, llvm_value: result }.to_le_value_enum())
            }
            LEBasicValueEnum::Float(f) => {
                let result = self.llvm_builder.build_float_neg(f.llvm_value, "");
                Ok(LEFloatValue { ty: f.ty, llvm_value: result }.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
}
