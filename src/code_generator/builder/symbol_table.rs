use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::values::FunctionValue;
use nom::combinator::{into, map};

use crate::ast::nodes::Position;
use crate::code_generator::builder::le_type::{LEArrayType, LEBasicType, LEBasicTypeEnum, LEFloatType, LEFunctionValue, LEIntegerType, LEPointerType, LEPointerValue, LEStructType, LEVectorType};
use crate::error::CompileError;

use super::Result;

#[derive(Clone, Debug)]
pub enum Symbol<'ctx> {
    Type(LEBasicTypeEnum<'ctx>),
    Variable(LEPointerValue<'ctx>),
    Function(LEFunctionValue<'ctx>),
}


#[derive(Default, Debug)]
pub struct SymbolTable<'ctx> {
    table: Vec<HashMap<String, Symbol<'ctx>>>,
}


impl<'ctx> SymbolTable<'ctx> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        let intrinsic_types = [
            ("i8".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i8_type(), true)))),
            ("i16".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i16_type(), true)))),
            ("i32".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i32_type(), true)))),
            ("i64".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i64_type(), true)))),
            ("u8".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i8_type(), false)))),
            ("u16".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i16_type(), false)))),
            ("u32".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i32_type(), false)))),
            ("u64".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(LEIntegerType::from_llvm_type(llvm_context.i64_type(), false)))),
            ("f32".into(), Symbol::Type(LEBasicTypeEnum::FloatType(LEFloatType::from_llvm_type(llvm_context.f32_type(), false)))),
            ("f64".into(), Symbol::Type(LEBasicTypeEnum::FloatType(LEFloatType::from_llvm_type(llvm_context.f64_type(), true)))),
        ];
        Self { table: vec![HashMap::from(intrinsic_types)] }
    }

    pub fn get_type<T: LEBasicType<'ctx>>(&self, type_name: &str) -> Result<T> {
        let symbol = self.get_symbol(type_name).ok_or_else(|| CompileError::unknown_identifier(type_name.into()))?;
        if let Symbol::Type(t) = symbol {
            t.try_into()
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into()).into())
        }
    }

    pub fn get_generic_type(&self, type_name: &str) -> Result<LEBasicTypeEnum<'ctx>> {
        let symbol = self.get_symbol(type_name).ok_or_else(|| CompileError::unknown_identifier(type_name.into()))?;
        if let Symbol::Type(t) = symbol {
            Ok(t)
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into()).into())
        }
    }

    pub fn get_variable(&self, variable: &str) -> Result<LEPointerValue<'ctx>> {
        let symbol = self.get_symbol(variable).ok_or_else(|| CompileError::unknown_identifier(variable.into()))?;
        if let Symbol::Variable(v) = symbol {
            Ok(v)
        } else {
            Err(CompileError::identifier_is_not_type(variable.into()).into())
        }
    }

    pub fn get_function(&self, function: &str) -> Result<LEFunctionValue<'ctx>> {
        let symbol = self.get_symbol(function).ok_or_else(|| CompileError::unknown_identifier(function.into()))?;
        if let Symbol::Function(f) = symbol {
            Ok(f)
        } else {
            Err(CompileError::identifier_is_not_type(function.into()).into())
        }
    }

    pub fn get_symbol(&self, identifier: &str) -> Option<Symbol<'ctx>> {
        for block_symbols in self.table.iter().rev() {
            if let Some(symbol) = block_symbols.get(identifier) {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn insert_global_symbol(&mut self, name: String, symbol: Symbol<'ctx>) -> Result<()> {
        let global_table = self.table.first_mut().unwrap();
        if global_table.contains_key(&name) {
            return Err(CompileError::identifier_already_defined(name, "variable".into()).into());
        } else {
            global_table.entry(name).or_insert(symbol);
        }
        Ok(())
    }

    pub fn insert_global_variable(&mut self, name: String, value: LEPointerValue<'ctx>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Variable(value))
    }

    pub fn insert_global_type(&mut self, name: String, value: LEBasicTypeEnum<'ctx>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Type(value))
    }
    pub fn insert_global_function(&mut self, name: String, value: LEFunctionValue<'ctx>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Function(value))
    }

    pub fn insert_local_function(&mut self, name: String, value: LEFunctionValue<'ctx>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into()))
        } else {
            local_symbols.entry(name).or_insert(Symbol::Function(value));
            Ok(())
        }
    }

    pub fn insert_local_type(&mut self, name: String, value: LEBasicTypeEnum<'ctx>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "type".into()))
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

    pub fn insert_local_variable(&mut self, name: String, value: LEPointerValue<'ctx>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into()))
        } else {
            local_symbols.entry(name).or_insert(Symbol::Variable(value));
            Ok(())
        }
    }
}



