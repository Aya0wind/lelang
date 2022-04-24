use inkwell::context::Context;

use crate::ast::nodes::{Identifier, TypeDeclarator};
use crate::code_generator::builder::{LEBasicTypeEnum, LEBoolType, LEFloatType, LEFunctionValue, LEIntegerType, LEPointerValue};
use crate::code_generator::context::compile_context::CompilerContext;
use crate::code_generator::Result;
use crate::lexer::Position;

pub struct LEContext<'ctx> {
    pub llvm_context: &'ctx Context,
    pub compiler_context: CompilerContext<'ctx>,
}

impl<'ctx> LEContext<'ctx> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        Self {
            llvm_context,
            compiler_context: CompilerContext::new(llvm_context),
        }
    }
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

    pub fn get_variable(&self, name: &str) -> Result<LEPointerValue<'ctx>> {
        self.compiler_context.get_variable(name)
    }

    pub fn get_generic_type(&self, type_declarator: &TypeDeclarator) -> Result<LEBasicTypeEnum<'ctx>> {
        self.compiler_context.get_type(type_declarator)
    }


    pub fn get_generic_variable(&self, name: &str) -> Result<LEPointerValue<'ctx>> {
        self.compiler_context.get_variable(name)
    }

    pub fn insert_local_function(&mut self, name: String, function: LEFunctionValue<'ctx>, position: Position) -> Result<LEFunctionValue<'ctx>> {
        self.compiler_context.insert_local_function(name, function.clone(), position)?;
        Ok(function)
    }

    pub fn insert_global_function(&mut self, name: String, function: LEFunctionValue<'ctx>, position: Position) -> Result<LEFunctionValue<'ctx>> {
        self.compiler_context.insert_global_function(name, function.clone(), position)?;
        Ok(function)
    }

    pub fn insert_local_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>, position: Position) -> Result<LEBasicTypeEnum<'ctx>> {
        self.compiler_context.insert_local_type(name, ty.clone(), position)?;
        Ok(ty)
    }

    pub fn insert_global_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>, position: Position) -> Result<LEBasicTypeEnum<'ctx>> {
        self.compiler_context.insert_global_type(name, ty.clone(), position)?;
        Ok(ty)
    }

    pub fn insert_local_variable(&mut self, name: String, pointer: LEPointerValue<'ctx>, position: Position) -> Result<LEPointerValue<'ctx>> {
        self.compiler_context.insert_local_variable(name, pointer.clone(), position)?;
        Ok(pointer)
    }

    pub fn insert_global_variable(&mut self, name: String, pointer: LEPointerValue<'ctx>, position: Position) -> Result<LEPointerValue<'ctx>> {
        self.compiler_context.insert_global_variable(name, pointer.clone(), position)?;
        Ok(pointer)
    }

    pub fn get_similar_variable(&self, name: &str) -> Identifier {
        unimplemented!()
    }
}
