use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

use builder::Result;

use crate::ast::nodes::Position;
use crate::code_generator::builder;
use crate::code_generator::builder::le_type::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEFunctionValue, LEPointerValue};
use crate::code_generator::builder::symbol_table::{Symbol, SymbolTable};
use crate::error::CompileError;

#[derive(Debug)]
pub struct CompilerContext<'ctx> {
    pub symbols: SymbolTable<'ctx>,
    pub current_function: Option<FunctionValue<'ctx>>,
    pub return_variable: Option<LEPointerValue<'ctx>>,
    pub return_block: Option<BasicBlock<'ctx>>,
}


impl<'ctx> CompilerContext<'ctx> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        Self {
            symbols: SymbolTable::new(llvm_context),
            current_function: None,
            return_variable: None,
            return_block: None,
        }
    }

    pub fn push_block_table(&mut self) {
        self.symbols.push_block_table();
    }

    pub fn pop_block_table(&mut self) {
        self.symbols.pop_block_table();
    }

    pub fn set_current_context(&mut self, current_function: FunctionValue<'ctx>, return_variable: Option<LEPointerValue<'ctx>>, return_block: BasicBlock<'ctx>) {
        self.current_function = Some(current_function);
        self.return_variable = return_variable;
        self.return_block = Some(return_block);
    }

    pub fn insert_global_variable(&mut self, name: String, variable: LEPointerValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        self.symbols.insert_global_variable(name.into(), variable.clone())?;
        Ok(variable)
    }

    pub fn insert_local_variable(&mut self, name: String, variable: LEPointerValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        self.symbols.insert_local_variable(name, variable.clone())?;
        Ok(variable)
    }

    pub fn insert_global_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>) -> Result<LEBasicTypeEnum<'ctx>> {
        self.symbols.insert_global_type(name.into(), ty.clone())?;
        Ok(ty)
    }

    pub fn insert_local_type(&mut self, name: String, ty: LEBasicTypeEnum<'ctx>) -> Result<LEBasicTypeEnum<'ctx>> {
        self.symbols.insert_local_type(name.into(), ty.clone())?;
        Ok(ty)
    }

    pub fn insert_global_function(&mut self, name: String, function: LEFunctionValue<'ctx>) -> Result<LEFunctionValue<'ctx>> {
        self.symbols.insert_global_function(name, function.clone())?;
        Ok(function)
    }
    pub fn insert_local_function(&mut self, name: String, function: LEFunctionValue<'ctx>) -> Result<LEFunctionValue<'ctx>> {
        self.symbols.insert_local_function(name, function.clone())?;
        Ok(function)
    }


    pub fn get_variable(&self, identifier: &str) -> Result<LEPointerValue<'ctx>> {
        self.symbols.get_variable(identifier)
    }

    pub fn get_function(&self, identifier: &str) -> Result<LEFunctionValue<'ctx>> {
        self.symbols.get_function(identifier)
    }

    pub fn get_type<T: LEBasicType<'ctx>>(&self, identifier: &str) -> Result<T> {
        self.symbols.get_type::<T>(identifier)
    }

    pub fn get_generic_type(&self, identifier: &str) -> Result<LEBasicTypeEnum<'ctx>> {
        self.symbols.get_generic_type(identifier)
    }

    pub fn get_symbol(&self, identifier: &str) -> Option<Symbol<'ctx>> {
        self.symbols.get_symbol(identifier)
    }
}
