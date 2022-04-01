use std::collections::HashMap;

use anyhow::Result;
use inkwell::context::Context;
use inkwell::values::FunctionValue;

use crate::ast::Position;
use crate::code_generator::builder::{IntegerType, LETypeEnum, LEVariable, NumericTypeEnum};
use crate::error::CompileError;

#[derive(Clone, Debug, Copy)]
pub enum Symbol<'s> {
    Type(LETypeEnum<'s>),
    Variable(LEVariable<'s>),
    Function(FunctionValue<'s>),
}

#[derive(Default, Debug)]
pub struct SymbolTable<'s> {
    table: Vec<HashMap<String, Symbol<'s>>>,
}


impl<'s> SymbolTable<'s> {
    pub fn new(llvm_context: &'s Context) -> Self {
        Self {
            table: vec![HashMap::<String, Symbol>::from([
                ("i8".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: true, value: llvm_context.i8_type() })))),
                ("i16".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: true, value: llvm_context.i16_type() })))),
                ("i32".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: true, value: llvm_context.i32_type() })))),
                ("i64".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: true, value: llvm_context.i64_type() })))),
                ("u8".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: false, value: llvm_context.i8_type() })))),
                ("u16".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: false, value: llvm_context.i16_type() })))),
                ("u32".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: false, value: llvm_context.i32_type() })))),
                ("u64".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: false, value: llvm_context.i64_type() })))),
                ("f32".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::FloatType(llvm_context.f32_type())))),
                ("f64".into(), Symbol::Type(LETypeEnum::NumericType(NumericTypeEnum::FloatType(llvm_context.f32_type())))),
            ])]
        }
    }
    pub fn get_type(&self, type_name: &str, pos: Position) -> Result<LETypeEnum<'s>> {
        let symbol = self.get_symbol(type_name, pos).ok_or(CompileError::unknown_identifier(type_name.into(), pos))?;
        if let Symbol::Type(t) = symbol {
            Ok(t)
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into(), pos).into())
        }
    }
    pub fn get_variable(&self, variable: &str, pos: Position) -> Result<LEVariable<'s>> {
        let symbol = self.get_symbol(variable, pos).ok_or(CompileError::unknown_identifier(variable.into(), pos))?;
        if let Symbol::Variable(v) = symbol {
            Ok(v)
        } else {
            Err(CompileError::identifier_is_not_type(variable.into(), pos).into())
        }
    }
    pub fn get_function(&self, function: &str, pos: Position) -> Result<FunctionValue<'s>> {
        let symbol = self.get_symbol(function, pos).ok_or(CompileError::unknown_identifier(function.into(), pos))?;
        if let Symbol::Function(f) = symbol {
            Ok(f)
        } else {
            Err(CompileError::identifier_is_not_type(function.into(), pos).into())
        }
    }

    pub fn get_symbol(&self, identifier: &str, _: Position) -> Option<Symbol<'s>> {
        for block_symbols in self.table.iter().rev() {
            if let Some(symbol) = block_symbols.get(identifier) {
                return Some(*symbol);
            }
        }
        None
    }

    pub fn insert_global_symbol(&mut self, name: String, symbol: Symbol<'s>, pos: Position) -> Result<()> {
        let global_table = self.table.first_mut().unwrap();
        if global_table.contains_key(&name) {
            return Err(CompileError::identifier_already_defined(name, "variable".into(), pos).into());
        } else {
            global_table.entry(name).or_insert(symbol);
        }
        Ok(())
    }

    pub fn insert_global_variable(&mut self, name: String, value: LEVariable<'s>, pos: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Variable(value), pos)
    }

    pub fn insert_global_type(&mut self, name: String, value: LETypeEnum<'s>, pos: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Type(value), pos)
    }
    pub fn insert_global_function(&mut self, name: String, value: FunctionValue<'s>, pos: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Function(value), pos)
    }

    pub fn insert_local_function(&mut self, name: String, value: FunctionValue<'s>, pos: Position) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into(), pos).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Function(value));
            Ok(())
        }
    }

    pub fn insert_local_type(&mut self, name: String, value: LETypeEnum<'s>, pos: Position) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "type".into(), pos).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Type(value));
            Ok(())
        }
    }
    pub fn push_block_table(&mut self) {
        self.table.push(HashMap::default());
    }
    pub fn pop_block_table(&mut self) {
        self.table.pop();
    }
    pub fn insert_local_variable(&mut self, name: String, value: LEVariable<'s>, pos: Position) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into(), pos).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Variable(value));
            Ok(())
        }
    }
}



