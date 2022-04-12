use std::rc::Rc;

use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, InstructionValue};
use nom::combinator::value;

use crate::ast::nodes::Position;
use crate::code_generator::builder::binary_operator_builder::{CompareOperator, GenericBuilder};
use crate::code_generator::builder::compile_context::CompilerContext;
use crate::code_generator::builder::le_type::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEFloatType, LEFunctionValue, LEIntegerType, LEIntegerValue, LEPointerType, LEPointerValue};
use crate::error::CompileError;

pub mod binary_operator_builder;
pub mod unary_operator_builder;
pub mod compile_context;
pub mod symbol_table;
pub mod le_type;

pub type Result<T> = std::result::Result<T, CompileError>;

pub struct LEContext<'ctx> {
    pub llvm_context: &'ctx inkwell::context::Context,
    pub llvm_builder: Builder<'ctx>,
    pub compiler_context: CompilerContext<'ctx>,
}

impl<'ctx> LEContext<'ctx> {
    pub fn i8_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("i8").unwrap()
    }
    pub fn i16_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("i16").unwrap()
    }
    pub fn i32_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("i32").unwrap()
    }
    pub fn i64_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("i64").unwrap()
    }
    pub fn u8_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("u8").unwrap()
    }
    pub fn u16_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("u16").unwrap()
    }
    pub fn u32_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("u32").unwrap()
    }
    pub fn u64_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("u64").unwrap()
    }
    pub fn float_type(&self) -> LEIntegerType<'ctx> {
        self.compiler_context.get_type("f32").unwrap()
    }
    pub fn double_type(&self) -> LEFloatType<'ctx> {
        self.compiler_context.get_type("f64").unwrap()
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

    pub fn get_generic_type(&self, type_name: &str) -> Result<LEBasicTypeEnum<'ctx>> {
        self.context.compiler_context.get_generic_type(type_name)
    }

    pub fn get_type<T: LEBasicType<'ctx>>(&self, type_name: &str) -> Result<T> {
        self.context.compiler_context.get_type(type_name)
    }

    pub fn get_generic_variable(&self, name: &str) -> Result<LEPointerValue<'ctx>> {
        self.context.compiler_context.get_variable(name)
    }


    pub fn build_add(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_add(&self.context, lhs, rhs)
    }


    pub fn build_sub(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_sub(&self.context, lhs, rhs)
    }

    pub fn build_mul(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_mul(&self.context, lhs, rhs)
    }

    pub fn build_div(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_div(&self.context, lhs, rhs)
    }

    pub fn build_cast(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicTypeEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        GenericBuilder::build_cast(&self.context, lhs, rhs)
    }

    // pub fn build_cast_generic<V:LEBasicValue<'ctx>>(&self, lhs: LEBasicValueEnum<'ctx>, rhs: V::LEType) -> Result<V::LEType> {
    //     let basic_value_enum = GenericBuilder::build_cast(&self.context, lhs, rhs)?;
    //
    // }

    pub fn build_compare(&self, lhs: LEBasicValueEnum<'ctx>, rhs: LEBasicValueEnum<'ctx>, op: CompareOperator) -> Result<LEIntegerValue<'ctx>> {
        GenericBuilder::build_compare(&self.context, lhs, rhs, op)
    }

    pub fn build_call(&self, function: LEFunctionValue<'ctx>, params: &[LEBasicValueEnum<'ctx>]) -> Result<Option<LEBasicValueEnum<'ctx>>> {
        let mut args = vec![];
        for (param_type, argument) in function.ty.param_types().iter().zip(params.iter()) {
            let value = if param_type != &argument.get_le_type() {
                self.build_cast(argument.clone(), param_type.clone())?
            } else {
                argument.clone()
            };
            args.push(BasicMetadataValueEnum::from(value.to_llvm_basic_value_enum()));
        }
        let site_value = self.context.llvm_builder.build_call(function.llvm_value, &args, "");
        if let Some(v) = site_value.try_as_basic_value().left() {
            Ok(Some(LEBasicValueEnum::from_llvm_basic_value_enum_and_type(v, function.ty.return_type().unwrap())))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn build_alloca(&self, initial_value: LEBasicValueEnum<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let value_type = initial_value.get_le_type();
        let llvm_pointer_value = self.context.llvm_builder.build_alloca(value_type.get_llvm_type(), "");
        let pointer_type = LEPointerType::new(&self.context, value_type.clone());
        let ptr = LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value };
        self.build_store(ptr.clone(), initial_value)?;
        Ok(ptr)
    }

    pub(crate) fn build_alloca_without_initialize(&self, ty: LEBasicTypeEnum<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let llvm_pointer_value = self.context.llvm_builder.build_alloca(ty.get_llvm_type(), "");
        let pointer_type = LEPointerType::new(&self.context, ty);
        Ok(LEPointerValue { ty: pointer_type, llvm_value: llvm_pointer_value })
    }

    pub(crate) fn build_load(&self, ptr: LEPointerValue<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let value_enum = self.context.llvm_builder.build_load(ptr.llvm_value, "");
        Ok(LEBasicValueEnum::from_llvm_basic_value_enum_and_type(value_enum, ptr.ty.get_point_type()))
    }

    pub fn build_load_variable(&self, name: &str) -> Result<LEBasicValueEnum<'ctx>> {
        let pointer_value = self.get_variable(name)?;
        self.build_load(pointer_value)
    }

    pub fn build_load_variable_generic<T: LEBasicValue<'ctx>>(&self, name: &str) -> Result<T> {
        let pointer_value = self.get_variable(name)?;
        let llvm_pointer_value = self.context.llvm_builder.build_load(pointer_value.llvm_value, "");
        T::from_type_and_llvm_value(pointer_value.ty.get_point_type(), llvm_pointer_value)
    }

    pub fn build_store_variable(&self, name: &str, value: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let pointer_value = self.get_variable(name)?;
        self.context.llvm_builder.build_store(pointer_value.llvm_value, value.to_llvm_basic_value_enum());
        Ok(value)
    }

    pub fn build_store(&self, ptr: LEPointerValue<'ctx>, value: LEBasicValueEnum<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        self.context.llvm_builder.build_store(ptr.llvm_value, value.to_llvm_basic_value_enum());
        Ok(value)
    }


    pub fn build_store_variable_generic<T: LEBasicValue<'ctx>>(&self, name: &str, value: T) -> Result<T> {
        let pointer_value = self.get_variable(name)?;
        self.context.llvm_builder.build_store(pointer_value.llvm_value, value.get_basic_llvm_value());
        Ok(value)
    }


    pub fn create_local_variable(&mut self, name: String, initial_value: LEBasicValueEnum<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let variable_pointer = self.build_alloca(initial_value)?;
        self.context.compiler_context.insert_local_variable(name, variable_pointer.clone())?;
        Ok(variable_pointer)
    }

    // pub fn build_neg(&self,value:LEBasicValueEnum<'ctx>)->Result<LEBasicValueEnum<'ctx>>{
    //     if let LEBasicValueEnum::IntegerValue(i) = value{
    //         self.context.llvm_builder.build_int_neg(i.llvm_value);
    //     }
    // }

    pub fn create_global_variable(&mut self, name: String, initial_value: LEBasicValueEnum<'ctx>, module: &Module<'ctx>) -> Result<LEBasicValueEnum<'ctx>> {
        let variable_type = initial_value.get_le_type();
        let variable = match initial_value.get_le_type() {
            LEBasicTypeEnum::IntegerType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::FloatType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::PointerType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::ArrayType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::StructType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
            LEBasicTypeEnum::VectorType(t) => { module.add_global(t.get_llvm_type(), Some(AddressSpace::Global), "") }
        };
        let pointer_value = LEPointerValue::from_type_and_llvm_value(variable_type, BasicValueEnum::PointerValue(variable.as_pointer_value()))?;
        self.context.compiler_context.insert_global_variable(name.into(), pointer_value)?;
        Ok(initial_value.clone())
    }

    pub fn insert_local_function(&mut self, name: String, function: LEFunctionValue<'ctx>) -> Result<LEFunctionValue<'ctx>> {
        self.context.compiler_context.insert_local_function(name, function.clone())?;
        Ok(function)
    }

    pub fn insert_global_function(&mut self, name: String, function: LEFunctionValue<'ctx>) -> Result<LEFunctionValue<'ctx>> {
        self.context.compiler_context.insert_global_function(name, function.clone())?;
        Ok(function)
    }
}
