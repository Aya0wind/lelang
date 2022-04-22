use std::rc::Rc;

use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, InstructionValue};
use nom::error::context;

use crate::ast::nodes::TypeDeclarator;
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEBoolType, LEBoolValue, LEFloatType, LEFloatValue, LEFunctionValue, LEIntegerType, LEIntegerValue, LEPointerType, LEPointerValue, LEType, LEValue};
use crate::code_generator::builder::binary_operator_builder::{CompareBinaryOperator, GenericBuilder, LogicBinaryOperator, MathOperatorBuilder, MemberAccessOperatorBuilder};
use crate::code_generator::builder::compile_context::CompilerContext;
use crate::code_generator::builder::expression::ExpressionValue;
use crate::error::CompileError;
use crate::lexer::{Operator, Position};
use crate::lexer::LEToken::Semicolon;

pub type Result<T> = std::result::Result<T, CompileError>;

pub struct LEContext<'ctx> {
    pub llvm_context: &'ctx inkwell::context::Context,
    pub llvm_builder: Builder<'ctx>,
    pub compiler_context: CompilerContext<'ctx>,
}

impl<'ctx> LEContext<'ctx> {
    pub fn bool_type(&self) -> LEBoolType<'ctx> {
        self.compiler_context.symbols.bool_type()
    }
    pub fn i8_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.i8_type()
    }
    pub fn i16_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.i16_type()
    }
    pub fn i32_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.i32_type()
    }
    pub fn i64_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.i64_type()
    }
    pub fn u8_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.u8_type()
    }
    pub fn u16_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.u16_type()
    }
    pub fn u32_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.u32_type()
    }
    pub fn u64_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.symbols.u64_type()
    }
    pub fn float_type(&self) -> LEFloatType<'ctx> {
        self.compiler_context.symbols.float_type()
    }
    pub fn double_type(&self) -> LEFloatType<'ctx> {
        self.compiler_context.symbols.double_type()
    }
}

pub struct LEGenerator<'ctx> {
    pub context: LEContext<'ctx>,
}

impl<'ctx> LEGenerator<'ctx> {
    pub fn new(llvm_context: &'ctx inkwell::context::Context, llvm_builder: Builder<'ctx>) -> Self {
        Self {
            context: LEContext {
                llvm_context,
                llvm_builder,
                compiler_context: CompilerContext::new(llvm_context),
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> Result<LEPointerValue<'ctx>> {
        self.context.compiler_context.get_variable(name)
    }

    pub fn get_generic_type(&self, type_declarator: &TypeDeclarator) -> Result<LEBasicTypeEnum<'ctx>> {
        self.context.compiler_context.get_type(type_declarator)
    }


    pub fn get_generic_variable(&self, name: &str) -> Result<LEPointerValue<'ctx>> {
        self.context.compiler_context.get_variable(name)
    }

    pub fn read_expression_value(&self, expr: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        match expr {
            ExpressionValue::Left(left_value) => {
                Ok(self.build_load(left_value))
            }
            ExpressionValue::Right(right_value) => {
                Ok(right_value)
            }
            ExpressionValue::Unit => {
                Err(CompileError::ExpressionIsNotRightValueExpression)
            }
        }
    }


    pub fn build_add(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_add(&self.context, self.read_expression_value(lhs)?, self.read_expression_value(rhs)?)
    }

    pub fn build_dot(&self, lhs: ExpressionValue<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>> {
        if let ExpressionValue::Left(left_expr) = lhs {
            left_expr.build_dot(&self.context, member_name)
        } else {
            Err(CompileError::ExpressionIsNotRightValueExpression)
        }
    }


    pub fn build_sub(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_sub(&self.context, self.read_expression_value(lhs)?, self.read_expression_value(rhs)?)
    }

    pub fn build_mul(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_mul(&self.context, self.read_expression_value(lhs)?, self.read_expression_value(rhs)?)
    }

    pub fn build_div(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_div(&self.context, self.read_expression_value(lhs)?, self.read_expression_value(rhs)?)
    }

    pub fn build_cast(&self, lhs: ExpressionValue<'ctx>, rhs: LEBasicTypeEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_cast(&self.context, self.read_expression_value(lhs)?, rhs)
    }

    pub fn build_compare(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>, op: CompareBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        GenericBuilder::build_compare(&self.context, self.read_expression_value(lhs)?, self.read_expression_value(rhs)?, op)
    }

    pub fn build_call(&self, function: LEFunctionValue<'ctx>, params: &[ExpressionValue<'ctx>]) -> Result<ExpressionValue<'ctx>> {
        let mut args = vec![];
        for (param_type, argument) in function.ty.param_types().iter().zip(params.iter()) {
            let argument_value = self.read_expression_value(argument.clone())?;
            let value = if param_type != &argument_value.get_le_type() {
                GenericBuilder::build_cast(&self.context, argument_value, param_type.clone())?
            } else {
                argument_value
            };
            args.push(BasicMetadataValueEnum::from(value.to_llvm_basic_value_enum()));
        }
        let site_value = self.context.llvm_builder.build_call(function.llvm_value, &args, "");
        if let Some(v) = site_value.try_as_basic_value().left() {
            Ok(ExpressionValue::Right(LEBasicValueEnum::from_type_and_llvm_value(function.ty.return_type().unwrap(), v)?))
        } else {
            Ok(ExpressionValue::Unit)
        }
    }

    pub(crate) fn build_alloca(&self, initial_value: ExpressionValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let value_type = self.read_expression_value(initial_value.clone())?.get_le_type();
        let llvm_pointer_value = self.context.llvm_builder.build_alloca(value_type.get_llvm_type(), "");
        let pointer_type = LEPointerType::new(&self.context, value_type.clone());
        let ptr = LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value };
        self.build_store(ptr.clone(), initial_value)?;
        Ok(ptr)
    }

    pub fn build_alloca_without_initialize(&self, ty: LEBasicTypeEnum<'ctx>) -> LEPointerValue<'ctx> {
        let llvm_pointer_value = self.context.llvm_builder.build_alloca(ty.get_llvm_type(), "");
        let pointer_type = LEPointerType::new(&self.context, ty);
        LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value }
    }

    pub fn build_load(&self, ptr: LEPointerValue<'ctx>) -> LEBasicValueEnum<'ctx> {
        let value_enum = self.context.llvm_builder.build_load(ptr.llvm_value, "");
        LEBasicValueEnum::from_type_and_llvm_value(ptr.ty.get_point_type(), value_enum).unwrap()
    }

    pub fn build_load_variable(&self, name: &str) -> Result<LEBasicValueEnum<'ctx>> {
        let pointer_value = self.get_variable(name)?;
        Ok(self.build_load(pointer_value))
    }

    pub fn build_assign(&self, target: ExpressionValue<'ctx>, value: ExpressionValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        if let ExpressionValue::Left(left_value) = target {
            let casted_value = self.build_cast(value, left_value.ty.get_point_type())?;
            self.context.llvm_builder.build_store(left_value.llvm_value, casted_value.to_llvm_basic_value_enum());
            Ok(left_value)
        } else {
            Err(CompileError::ExpressionIsNotLeftValueExpression)
        }
    }

    pub fn build_neg(&self, value: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let target_value = self.read_expression_value(value)?;
        match target_value {
            LEBasicValueEnum::Integer(i) => {
                let result = self.context.llvm_builder.build_int_neg(i.llvm_value, "");
                Ok(LEIntegerValue { ty: i.ty, llvm_value: result }.to_le_value_enum())
            }
            LEBasicValueEnum::Float(f) => {
                let result = self.context.llvm_builder.build_float_neg(f.llvm_value, "");
                Ok(LEFloatValue { ty: f.ty, llvm_value: result }.to_le_value_enum())
            }
            _ => { unimplemented!() }
        }
    }

    pub fn build_binary_logic(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>, op: LogicBinaryOperator) -> Result<LEBoolValue<'ctx>> {
        let left_value = GenericBuilder::build_cast(
            &self.context,
            self.read_expression_value(lhs)?,
            self.context.bool_type().to_le_type_enum(),
        )?;
        let right_value = GenericBuilder::build_cast(
            &self.context,
            self.read_expression_value(rhs)?,
            self.context.bool_type().to_le_type_enum(),
        )?;
        GenericBuilder::build_logic(&self.context, left_value.into_bool_value().unwrap(), right_value.into_bool_value().unwrap(), op)
    }

    pub fn build_mod(&self, lhs: ExpressionValue<'ctx>, rhs: ExpressionValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let left = self.read_expression_value(lhs)?;
        let right = self.read_expression_value(rhs)?;
        let left_type = LEBasicValue::get_le_type(&left);
        let right_type = LEBasicValue::get_le_type(&right);
        match (left, right) {
            (LEBasicValueEnum::Integer(left_int), LEBasicValueEnum::Integer(right_int)) => {
                Ok(left_int.build_mod(&self.context, right_int)?.to_le_value_enum())
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


    pub fn build_store(&self, ptr: LEPointerValue<'ctx>, value: ExpressionValue<'ctx>) -> Result<()> {
        self.context.llvm_builder.build_store(ptr.llvm_value, self.read_expression_value(value)?.to_llvm_basic_value_enum());
        Ok(())
    }

    fn build_store_with_value(&self, ptr: LEPointerValue<'ctx>, value: LEBasicValueEnum<'ctx>) -> Result<()> {
        self.context.llvm_builder.build_store(ptr.llvm_value, value.to_llvm_basic_value_enum());
        Ok(())
    }

    pub fn create_local_variable(&mut self, name: String, initial_value: ExpressionValue<'ctx>, position: Position) -> Result<LEPointerValue<'ctx>> {
        let variable_pointer = self.build_alloca(initial_value)?;
        self.context.compiler_context.insert_local_variable(name, variable_pointer.clone(), position)?;
        Ok(variable_pointer)
    }

    pub fn create_local_variable_with_exact_type(&mut self, name: String, initial_value: ExpressionValue<'ctx>, ty: &TypeDeclarator, position: Position) -> Result<LEPointerValue<'ctx>> {
        let variable_type = self.get_generic_type(ty)?;
        let variable_pointer = self.build_alloca_without_initialize(variable_type);
        self.build_store(variable_pointer.clone(), initial_value)?;
        self.context.compiler_context.insert_local_variable(name, variable_pointer.clone(), position)?;
        Ok(variable_pointer)
    }


    pub fn create_global_variable(&mut self, name: String, initial_value: ExpressionValue<'ctx>, module: &Module<'ctx>, position: Position) -> Result<()> {
        let value = self.read_expression_value(initial_value)?;
        let variable_type = LEBasicValue::get_le_type(&value);
        let variable = match variable_type.clone() {
            LEBasicTypeEnum::Integer(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Bool(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Float(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Pointer(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Array(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Struct(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Vector(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
        };
        let pointer_value = LEPointerValue::from_type_and_llvm_value(variable_type, BasicValueEnum::PointerValue(variable.as_pointer_value()))?;
        self.context.compiler_context.insert_global_variable(name, pointer_value, position)?;
        Ok(())
    }

    fn build_alloca_global(ty: LEBasicTypeEnum<'ctx>, module: &Module<'ctx>) -> LEPointerValue<'ctx> {
        let global_ptr = match ty {
            LEBasicTypeEnum::Integer(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Bool(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Float(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Pointer(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Array(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Struct(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::Vector(ref t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
        }.as_pointer_value();
        LEPointerValue::from_type_and_llvm_value(ty, BasicValueEnum::PointerValue(global_ptr)).unwrap()
    }

    pub fn create_global_variable_with_exact_type(&mut self,
                                                  name: String,
                                                  initial_value: ExpressionValue<'ctx>,
                                                  ty: LEBasicTypeEnum<'ctx>,
                                                  module: &Module<'ctx>,
                                                  position: Position,
    ) -> Result<()> {
        let value = self.build_cast(initial_value, ty)?;
        let variable_type = LEBasicValue::get_le_type(&value);
        let variable = Self::build_alloca_global(variable_type, module);
        self.build_store_with_value(variable.clone(), value)?;
        self.context.compiler_context.insert_global_variable(name, variable, position)?;
        Ok(())
    }


    pub fn insert_local_function(&mut self, name: String, function: LEFunctionValue<'ctx>, position: Position) -> Result<LEFunctionValue<'ctx>> {
        self.context.compiler_context.insert_local_function(name, function.clone(), position)?;
        Ok(function)
    }

    pub fn insert_global_function(&mut self, name: String, function: LEFunctionValue<'ctx>, position: Position) -> Result<LEFunctionValue<'ctx>> {
        self.context.compiler_context.insert_global_function(name, function.clone(), position)?;
        Ok(function)
    }

    pub fn insert_local_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>, position: Position) -> Result<LEBasicTypeEnum<'ctx>> {
        self.context.compiler_context.insert_local_type(name, ty.clone(), position)?;
        Ok(ty)
    }

    pub fn insert_global_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>, position: Position) -> Result<LEBasicTypeEnum<'ctx>> {
        self.context.compiler_context.insert_global_type(name, ty.clone(), position)?;
        Ok(ty)
    }
}
