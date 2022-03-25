use std::any::Any;
use std::collections::HashMap;

use anyhow::Result;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{FunctionValue, PointerValue};

use crate::error::CompileError;

pub type VariableTable<'s> = HashMap<String, PointerValue<'s>>;
pub type TypeTable<'s> = HashMap<String, BasicTypeEnum<'s>>;
pub type FunctionTable<'s> = HashMap<String, FunctionValue<'s>>;

#[derive(Debug, Default, PartialEq)]
pub struct SymbolTable<'s> {
    pub variables: VariableTable<'s>,
    pub types: TypeTable<'s>,
    pub functions: FunctionTable<'s>,
}


#[derive(Debug)]
pub struct CompilerContext<'s> {
    pub global_symbols: SymbolTable<'s>,
    pub local_symbols: Vec<SymbolTable<'s>>,
}

impl<'s> Default for CompilerContext<'s> {
    fn default() -> Self {
        Self {
            global_symbols: Default::default(),
            local_symbols: vec![SymbolTable::default()],
        }
    }
}


impl<'s> CompilerContext<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_local_symbols(&mut self) {
        self.local_symbols.clear();
        self.local_symbols.push(SymbolTable::default())
    }

    pub fn push_block_table(&mut self) {
        self.local_symbols.push(SymbolTable::default());
    }
    pub fn pop_block_table(&mut self) {
        self.local_symbols.pop();
    }


    pub fn get_type(&self, type_name: &str) -> Result<BasicTypeEnum<'s>> {
        for block_symbol in self.local_symbols.iter().rev() {
            if let Some(s) = block_symbol
                .types
                .get(type_name) { return Ok(*s); }
        }
        match self.global_symbols.types.get(type_name) {
            None => { Err(CompileError::identifier_is_not_type(type_name.into()).into()) }
            Some(s) => { Ok(*s) }
        }
    }

    pub fn get_variable(&self, variable_name: &str) -> Result<PointerValue<'s>> {
        for block_symbol in self.local_symbols.iter().rev() {
            if let Some(s) = block_symbol
                .variables
                .get(variable_name) { return Ok(*s); }
        }
        match self.global_symbols.variables.get(variable_name) {
            None => { Err(CompileError::identifier_is_not_variable(variable_name.into()).into()) }
            Some(s) => { Ok(*s) }
        }
    }


    pub fn get_function(&self, function_name: &str) -> Result<FunctionValue<'s>> {
        for block_symbol in self.local_symbols.iter().rev() {
            if let Some(s) = block_symbol
                .functions
                .get(function_name) { return Ok(*s); }
        }
        match self.global_symbols.functions.get(function_name) {
            None => { Err(CompileError::identifier_is_not_function(function_name.into()).into()) }
            Some(s) => { Ok(*s) }
        }
    }

    pub fn insert_global_function(&mut self, name: String, value: FunctionValue<'s>) -> Result<()> {
        let function = &mut self.global_symbols.functions;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }

    pub fn insert_global_variable(&mut self, name: String, value: PointerValue<'s>) -> Result<()> {
        let function = &mut self.global_symbols.variables;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }
    pub fn insert_global_type(&mut self, name: String, value: BasicTypeEnum<'s>) -> Result<()> {
        let function = &mut self.global_symbols.types;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "type".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }
    pub fn insert_local_function(&mut self, name: String, value: FunctionValue<'s>) -> Result<()> {
        let function = &mut self.local_symbols.last_mut().unwrap().functions;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }
    pub fn insert_local_type(&mut self, name: String, value: BasicTypeEnum<'s>) -> Result<()> {
        let function = &mut self.local_symbols.last_mut().unwrap().types;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }
    pub fn insert_local_variable(&mut self, name: String, value: PointerValue<'s>) -> Result<()> {
        let function = &mut self.local_symbols.last_mut().unwrap().variables;
        if function.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "type".into()).into())
        } else {
            function.entry(name).or_insert(value);
            Ok(())
        }
    }
}



