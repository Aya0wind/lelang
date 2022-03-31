use std::collections::HashMap;

use anyhow::Result;
use inkwell::context::Context;
use inkwell::values::FunctionValue;

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
    pub fn get_type(&self, type_name: &str) -> Result<LETypeEnum<'s>> {
        let symbol = self.get_symbol(type_name).ok_or(CompileError::unknown_identifier(type_name.into()))?;
        if let Symbol::Type(t) = symbol {
            Ok(t)
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into()).into())
        }
    }
    pub fn get_variable(&self, variable: &str) -> Result<LEVariable<'s>> {
        let symbol = self.get_symbol(variable).ok_or(CompileError::unknown_identifier(variable.into()))?;
        if let Symbol::Variable(v) = symbol {
            Ok(v)
        } else {
            Err(CompileError::identifier_is_not_type(variable.into()).into())
        }
    }
    pub fn get_function(&self, function: &str) -> Result<FunctionValue<'s>> {
        let symbol = self.get_symbol(function).ok_or(CompileError::unknown_identifier(function.into()))?;
        if let Symbol::Function(f) = symbol {
            Ok(f)
        } else {
            Err(CompileError::identifier_is_not_type(function.into()).into())
        }
    }

    pub fn get_symbol(&self, identifier: &str) -> Option<Symbol<'s>> {
        for block_symbols in self.table.iter().rev() {
            if let Some(symbol) = block_symbols.get(identifier) {
                return Some(*symbol);
            }
        }
        None
    }

    pub fn insert_global_symbol(&mut self, name: String, symbol: Symbol<'s>) -> Result<()> {
        let global_table = self.table.first_mut().unwrap();
        if global_table.contains_key(&name) {
            return Err(CompileError::identifier_already_defined(name, "variable".into()).into());
        } else {
            global_table.entry(name).or_insert(symbol);
        }
        Ok(())
    }

    pub fn insert_global_variable(&mut self, name: String, value: LEVariable<'s>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Variable(value))
    }

    pub fn insert_global_type(&mut self, name: String, value: LETypeEnum<'s>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Type(value))
    }
    pub fn insert_global_function(&mut self, name: String, value: FunctionValue<'s>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Function(value))
    }

    pub fn insert_local_function(&mut self, name: String, value: FunctionValue<'s>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into()).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Function(value));
            Ok(())
        }
    }

    pub fn insert_local_type(&mut self, name: String, value: LETypeEnum<'s>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "type".into()).into())
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
    pub fn insert_local_variable(&mut self, name: String, value: LEVariable<'s>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into()).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Variable(value));
            Ok(())
        }
    }
}


