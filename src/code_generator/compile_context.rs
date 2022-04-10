use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

use builder::Result;

use crate::ast::nodes::Position;
use crate::code_generator::builder;
use crate::code_generator::builder::llvm_type_wrapper::{LEBasicTypeGenericRef, LEFunctionValue, LEPointerValue};
use crate::code_generator::symbol_table::{Symbol, SymbolTable};
use crate::error::CompileError;

#[derive(Debug)]
pub struct CompilerContext<'ctx> {
    pub symbols: SymbolTable<'ctx, 'ctx>,
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

    pub fn set_current_context<'a>(&mut self, current_function: FunctionValue<'ctx>, return_variable: Option<LEPointerValue<'ctx>>, return_block: BasicBlock<'ctx>) {
        self.current_function = Some(current_function);
        self.return_variable = return_variable;
        self.return_block = Some(return_block);
    }

    pub fn create_global_variable<'a>(&mut self, name: &str, variable: LEPointerValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        self.symbols.insert_global_variable(name.into(), variable)?;
        Ok(variable)
    }

    pub fn insert_local_variable<'a>(&mut self, name: &str, variable: LEPointerValue<'ctx>) -> Result<LEPointerValue<'ctx>> {
        self.symbols.insert_local_variable(name.into(), variable)?;
        Ok(variable)
    }


    pub fn get_variable<'a>(&self, identifier: &str) -> Result<LEPointerValue<'ctx>> {
        self.symbols.get_variable(identifier)
    }

    pub fn get_function<'a>(&self, identifier: &str) -> Result<LEFunctionValue<'ctx,'a>> {
        self.symbols.get_function(identifier)
    }

    pub fn get_type<'a>(&self, identifier: &str) -> Result<LEBasicTypeGenericRef<'ctx>> {
        self.symbols.get_type(identifier)
    }

    pub fn get_symbol<'a>(&self, identifier: &str) -> Option<Symbol<'ctx, 'a>> {
        self.symbols.get_symbol(identifier)
    }
}
