use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::values::FunctionValue;

use crate::ast::Position;
use crate::code_generator::builder::LEVariable;
use crate::code_generator::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct CompilerContext<'s> {
    pub symbols: SymbolTable<'s>,
    pub current_function: Option<FunctionValue<'s>>,
    pub return_variable: Option<LEVariable<'s>>,
    pub return_block: Option<BasicBlock<'s>>,
    pub current_pos: Position,
}


impl<'s> CompilerContext<'s> {
    pub fn new(llvm_context: &'s Context) -> Self {
        Self {
            symbols: SymbolTable::new(llvm_context),
            current_function: None,
            return_variable: None,
            return_block: None,
            current_pos: Position { line: 0 }
        }
    }

    pub fn push_block_table(&mut self) {
        self.symbols.push_block_table();
    }

    pub fn pop_block_table(&mut self) {
        self.symbols.pop_block_table();
    }

    pub fn set_current_context(&mut self, current_function: FunctionValue<'s>, return_variable: LEVariable<'s>, return_block: BasicBlock<'s>) {
        self.current_function = Some(current_function);
        self.return_variable = Some(return_variable);
        self.return_block = Some(return_block);
    }
}
