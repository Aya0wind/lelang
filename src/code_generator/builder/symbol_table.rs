use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::values::FunctionValue;

use crate::ast::nodes::{Position, TypeDeclarator};
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValueEnum, LEBoolType, LEBoolValue, LEFloatType, LEFunctionValue, LEIntegerType, LEPointerValue};
use crate::error::CompileError;

use super::Result;

#[derive(Clone, Debug)]
pub enum Symbol<'ctx> {
    Type(LEBasicTypeEnum<'ctx>),
    Variable(LEPointerValue<'ctx>),
    Function(LEFunctionValue<'ctx>),
}


#[derive(Debug)]
pub struct SymbolTable<'ctx> {
    llvm_context: &'ctx Context,
    table: Vec<HashMap<String, Symbol<'ctx>>>,
    bool_type: LEBoolType<'ctx>,
    i8_type: LEIntegerType<'ctx>,
    i16_type: LEIntegerType<'ctx>,
    i32_type: LEIntegerType<'ctx>,
    i64_type: LEIntegerType<'ctx>,
    u8_type: LEIntegerType<'ctx>,
    u16_type: LEIntegerType<'ctx>,
    u32_type: LEIntegerType<'ctx>,
    u64_type: LEIntegerType<'ctx>,
    f32_type: LEFloatType<'ctx>,
    f64_type: LEFloatType<'ctx>,
}


impl<'ctx> SymbolTable<'ctx> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        let bool_type = LEBoolType::from_llvm_type(llvm_context.bool_type());

        let i8_type = LEIntegerType::from_llvm_type(llvm_context.i8_type(), true);
        let i16_type = LEIntegerType::from_llvm_type(llvm_context.i16_type(), true);
        let i32_type = LEIntegerType::from_llvm_type(llvm_context.i32_type(), true);
        let i64_type = LEIntegerType::from_llvm_type(llvm_context.i64_type(), true);

        let u8_type = LEIntegerType::from_llvm_type(llvm_context.i8_type(), false);
        let u16_type = LEIntegerType::from_llvm_type(llvm_context.i16_type(), false);
        let u32_type = LEIntegerType::from_llvm_type(llvm_context.i32_type(), false);
        let u64_type = LEIntegerType::from_llvm_type(llvm_context.i64_type(), false);


        let f32_type = LEFloatType::from_llvm_type(llvm_context.f32_type(), false);
        let f64_type = LEFloatType::from_llvm_type(llvm_context.f64_type(), true);

        let intrinsic_types = [
            ("bool".into(), Symbol::Type(LEBasicTypeEnum::BoolType(bool_type.clone()))),
            ("i8".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(i8_type.clone()))),
            ("i16".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(i16_type.clone()))),
            ("i32".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(i32_type.clone()))),
            ("i64".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(i64_type.clone()))),
            ("u8".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(u8_type.clone()))),
            ("u16".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(u16_type.clone()))),
            ("u32".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(u32_type.clone()))),
            ("u64".into(), Symbol::Type(LEBasicTypeEnum::IntegerType(u64_type.clone()))),
            ("f32".into(), Symbol::Type(LEBasicTypeEnum::FloatType(f32_type.clone()))),
            ("f64".into(), Symbol::Type(LEBasicTypeEnum::FloatType(f64_type.clone()))),
        ];
        Self {
            table: vec![HashMap::from(intrinsic_types)],
            bool_type,
            i8_type,
            i16_type,
            i32_type,
            i64_type,
            u8_type,
            u16_type,
            u32_type,
            u64_type,
            f32_type,
            f64_type,
            llvm_context,
        }
    }


    pub fn get_type(&self, type_declarator: &TypeDeclarator) -> Result<LEBasicTypeEnum<'ctx>> {
        match type_declarator {
            TypeDeclarator::TypeIdentifier(identifier) => {
                let symbol = self.get_symbol(&identifier).ok_or_else(|| CompileError::UnknownIdentifier { name: identifier.clone() })?;
                if let Symbol::Type(t) = symbol {
                    Ok(t)
                } else {
                    Err(CompileError::IdentifierIsNotType { name: identifier.into() })
                }
            }
            TypeDeclarator::Array(array) => {
                let element_type = self.get_type(&array.element_type)?;
                let array_type = LEBasicType::get_array_type(&element_type, array.len);
                Ok(array_type.to_le_type_enum())
            }
            TypeDeclarator::Reference(reference) => {
                let point_type = self.get_type(reference)?;
                let pointer_type = LEBasicType::get_pointer_type(&point_type);
                Ok(pointer_type.to_le_type_enum())
            }
        }
    }

    pub fn get_variable(&self, variable: &str) -> Result<LEPointerValue<'ctx>> {
        let symbol = self.get_symbol(variable).ok_or_else(|| CompileError::UnknownIdentifier { name: variable.into() })?;
        if let Symbol::Variable(v) = symbol {
            Ok(v)
        } else {
            Err(CompileError::IdentifierIsNotType { name: variable.into() })
        }
    }

    pub fn get_function(&self, function: &str) -> Result<LEFunctionValue<'ctx>> {
        let symbol = self.get_symbol(function).ok_or_else(|| CompileError::UnknownIdentifier { name: function.into() })?;
        if let Symbol::Function(f) = symbol {
            Ok(f)
        } else {
            Err(CompileError::IdentifierIsNotType { name: function.into() })
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
            return Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position: Position { line: 0 } });
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
            Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position: Position { line: 0 } })
        } else {
            local_symbols.entry(name).or_insert(Symbol::Function(value));
            Ok(())
        }
    }

    pub fn insert_local_type(&mut self, name: String, value: LEBasicTypeEnum<'ctx>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position: Position { line: 0 } })
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
            Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position: Position { line: 0 } })
        } else {
            local_symbols.entry(name).or_insert(Symbol::Variable(value));
            Ok(())
        }
    }
    pub fn bool_type(&self) -> LEBoolType<'ctx> {
        self.bool_type.clone()
    }
    pub fn i8_type(&self) -> LEIntegerType<'ctx> {
        self.i8_type.clone()
    }
    pub fn i16_type(&self) -> LEIntegerType<'ctx> {
        self.i16_type.clone()
    }
    pub fn i32_type(&self) -> LEIntegerType<'ctx> {
        self.i32_type.clone()
    }
    pub fn i64_type(&self) -> LEIntegerType<'ctx> {
        self.i64_type.clone()
    }
    pub fn u8_type(&self) -> LEIntegerType<'ctx> {
        self.u8_type.clone()
    }
    pub fn u16_type(&self) -> LEIntegerType<'ctx> {
        self.u16_type.clone()
    }
    pub fn u32_type(&self) -> LEIntegerType<'ctx> {
        self.u32_type.clone()
    }
    pub fn u64_type(&self) -> LEIntegerType<'ctx> {
        self.u64_type.clone()
    }
    pub fn float_type(&self) -> LEFloatType<'ctx> {
        self.f32_type.clone()
    }
    pub fn double_type(&self) -> LEFloatType<'ctx> {
        self.f64_type.clone()
    }
}



