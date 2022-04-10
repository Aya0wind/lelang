use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::values::FunctionValue;
use nom::combinator::{into, map};

use crate::ast::nodes::Position;
use crate::code_generator::builder::llvm_type_wrapper::{LEArrayType, LEBasicType, LEBasicTypeGenericRef, LEFloatType, LEFunctionValue, LEIntegerType, LEPointerType, LEPointerValue, LEStructType, LEVectorType};
use crate::error::CompileError;

use super::builder::Result;

#[derive(Clone, Debug)]
pub enum Symbol<'ctx, 'a> {
    Type(LEBasicTypeGenericRef<'ctx>),
    Variable(LEPointerValue<'ctx>),
    Function(LEFunctionValue<'ctx, 'a>),
}


#[derive(Default, Debug)]
pub struct SymbolTable<'ctx, 'a> {
    table: Vec<HashMap<String, Symbol<'ctx, 'a>>>,
    types: Vec<GenericTypeDropGuard<'ctx>>,
}

pub struct GenericTypeDropGuard<'ctx> {
    pointer: LEBasicTypeGenericRef<'ctx>,
}

impl<'ctx> Drop for GenericTypeDropGuard<'ctx> {
    fn drop(&mut self) {
        match self.pointer {
            LEBasicTypeGenericRef::IntegerType(i) => { unsafe { Box::from_raw(i as *mut LEIntegerType); } }
            LEBasicTypeGenericRef::FloatType(i) => { unsafe { Box::from_raw(i as *mut LEFloatType); } }
            LEBasicTypeGenericRef::PointerType(i) => { unsafe { Box::from_raw(i as *mut LEPointerType); } }
            LEBasicTypeGenericRef::ArrayType(i) => { unsafe { Box::from_raw(i as *mut LEArrayType); } }
            LEBasicTypeGenericRef::StructType(i) => { unsafe { Box::from_raw(i as *mut LEStructType); } }
            LEBasicTypeGenericRef::VectorType(i) => { unsafe { Box::from_raw(i as *mut LEVectorType); } }
            LEBasicTypeGenericRef::UnitType => {}
        }
    }
}

impl<'ctx, 'a> SymbolTable<'ctx, 'a> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        let intrinsic_types = [
            ("i8".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: true, llvm_type: llvm_context.i8_type() })))),
            ("i16".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: true, llvm_type: llvm_context.i16_type() })))),
            ("i32".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: true, llvm_type: llvm_context.i32_type() })))),
            ("i64".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: true, llvm_type: llvm_context.i64_type() })))),
            ("u8".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: false, llvm_type: llvm_context.i8_type() })))),
            ("u16".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: false, llvm_type: llvm_context.i16_type() })))),
            ("u32".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: false, llvm_type: llvm_context.i32_type() })))),
            ("u64".into(), LEBasicTypeGenericRef::IntegerType(Box::into_raw(Box::new(LEIntegerType { signed: false, llvm_type: llvm_context.i64_type() })))),
            ("f32".into(), LEBasicTypeGenericRef::FloatType(Box::into_raw(Box::new(LEFloatType { llvm_type: llvm_context.f32_type() })))),
            ("f64".into(), LEBasicTypeGenericRef::FloatType(Box::into_raw(Box::new(LEFloatType { llvm_type: llvm_context.f64_type() })))),
        ];
        Self {
            table: vec![
                HashMap::<String, Symbol>::from(
                    intrinsic_types.map(|(n, type_pointer)|(n, Symbol::Type(type_pointer)))
                )
            ],
            types: intrinsic_types.map(|(_, type_pointer)| { GenericTypeDropGuard { pointer: type_pointer } }).collect_vec(),
        }
    }

    pub fn get_type<T:LEBasicType<'ctx>>(&self, type_name: &str) -> Result<T> {
        let symbol = self.get_symbol(type_name).ok_or_else(|| CompileError::unknown_identifier(type_name.into()))?;
        if let Symbol::Type(t) = symbol {
            t.try_into()
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into()).into())
        }
    }

    pub fn get_generic_type(&self, type_name: &str) -> Result<LEBasicTypeGenericRef<'ctx>> {
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

    pub fn get_function(&self, function: &str) -> Result<LEFunctionValue<'ctx, 'a>> {
        let symbol = self.get_symbol(function).ok_or_else(|| CompileError::unknown_identifier(function.into()))?;
        if let Symbol::Function(f) = symbol {
            Ok(f)
        } else {
            Err(CompileError::identifier_is_not_type(function.into()).into())
        }
    }

    pub fn get_symbol(&self, identifier: &str) -> Option<Symbol<'ctx, 'a>> {
        for block_symbols in self.table.iter().rev() {
            if let Some(symbol) = block_symbols.get(identifier) {
                return Some(*symbol);
            }
        }
        None
    }

    pub fn insert_global_symbol(&mut self, name: String, symbol: Symbol<'ctx, 'a>) -> Result<()> {
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

    pub fn insert_global_type(&mut self, name: String, value: LEBasicTypeGenericRef<'ctx>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Type(value))
    }
    pub fn insert_global_function(&mut self, name: String, value: LEFunctionValue<'ctx,'a>) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Function(value))
    }

    pub fn insert_local_function(&mut self, name: String, value: LEFunctionValue<'ctx,'a>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "function".into()).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Function(value));
            Ok(())
        }
    }

    pub fn insert_local_type(&mut self, name: String, value: LEBasicTypeGenericRef<'ctx>) -> Result<()> {
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

    pub fn insert_local_variable(&mut self, name: String, value: LEPointerValue<'ctx>) -> Result<()> {
        let local_symbols = self.table.last_mut().unwrap();
        if local_symbols.contains_key(&name) {
            Err(CompileError::identifier_already_defined(name, "variable".into()).into())
        } else {
            local_symbols.entry(name).or_insert(Symbol::Variable(value));
            Ok(())
        }
    }
}



