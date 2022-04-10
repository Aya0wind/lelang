use inkwell::builder::Builder;
use inkwell::types::BasicType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, FunctionValue};
use nom::combinator::value;

use crate::ast::nodes::Position;
use crate::code_generator::builder::binary_operator_builder::{CompareOperator, GenericBuilder};
use crate::code_generator::builder::llvm_type_wrapper::{LEArrayType, LEArrayValue, LEBasicType, LEBasicTypeGenericRef, LEBasicValue, LEBasicValueEnum, LEIntegerType, LEIntegerValue, LEPointerType, LEPointerValue, LEStructType, LEStructValue};
use crate::code_generator::compile_context::CompilerContext;
use crate::error::CompileError;

pub mod binary_operator_builder;
pub mod unary_operator_builder;
pub mod llvm_type_wrapper;

pub type Result<T> = std::result::Result<T,CompileError>;

pub struct LEContext<'ctx>{
    pub llvm_context: &'ctx inkwell::context::Context,
    pub llvm_builder: Builder<'ctx>,
    pub compiler_context:CompilerContext<'ctx>,
}

pub struct LEBuilder<'ctx> {
    context:LEContext<'ctx>,
}

impl<'ctx> LEBuilder<'ctx> {
    pub fn new(llvm_context: &'ctx inkwell::context::Context, llvm_builder: Builder<'ctx>) -> Self {
        Self{ context: LEContext {
            llvm_context,
            llvm_builder,
            compiler_context: CompilerContext::new(llvm_context)
        } }
    }

    pub fn build_add<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        GenericBuilder::build_add(&self.context.llvm_builder,self.context.llvm_context,lhs,rhs)
    }


    pub fn build_sub<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        GenericBuilder::build_sub(&self.context.llvm_builder,self.context.llvm_context,lhs,rhs)
    }

    pub fn build_mul<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        GenericBuilder::build_mul(&self.context.llvm_builder,self.context.llvm_context,lhs,rhs)
    }

    pub fn build_div<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        GenericBuilder::build_div(&self.context.llvm_builder,self.context.llvm_context,lhs,rhs)
    }

    pub fn build_cast<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: &'a LEBasicTypeGenericRef<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        GenericBuilder::build_cast(&self.context.llvm_builder,self.context.llvm_context, lhs, &rhs)
    }

    pub fn build_compare<'a>(&self, lhs: LEBasicValueEnum<'ctx, 'a>, rhs: LEBasicValueEnum<'ctx, 'a>, op: CompareOperator) -> Result<LEIntegerValue<'ctx,'a>> {
        GenericBuilder::build_compare(&self.context.llvm_builder,self.context.llvm_context,lhs,rhs,op)
    }

    pub fn build_call<'a>(&self, function: FunctionValue<'ctx>, params: &[LEBasicValueEnum<'ctx, 'a>]) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        let mut args = vec![];
        for (param, argument) in function.get_param_iter().zip(params.iter()) {
            let param_type: LEBasicTypeGenericRef = param.get_type().into();
            let value = if param_type != argument.get_type() {
                self.build_cast(*argument, &param_type)?
            }else{
                argument
            };
            args.push(BasicMetadataValueEnum::from(value.to_llvm_basic_value_enum()));
        }
        let site_value = self.context.llvm_builder.build_call(function, &args, "");
        Ok(site_value.as_any_value_enum().into())
    }
}
