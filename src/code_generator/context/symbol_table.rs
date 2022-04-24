use std::collections::HashMap;

use inkwell::context::Context;

use crate::ast::nodes::TypeDeclarator;
use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBoolType, LEBoolValue, LEFloatType, LEFunctionValue, LEIntegerType, LEPointerValue};
use crate::code_generator::Result;
use crate::error::CompileError;
use crate::lexer::Position;

#[derive(Clone, Debug)]
pub struct MetaData {
    defined_pos: Position,
    is_built_in: bool,
}

#[derive(Clone, Debug)]
pub struct Variable<'ctx> {
    pointer: LEPointerValue<'ctx>,
    meta: MetaData,
}

#[derive(Clone, Debug)]
pub struct Type<'ctx> {
    inner: LEBasicTypeEnum<'ctx>,
    meta: MetaData,
}

#[derive(Clone, Debug)]
pub struct Function<'ctx> {
    inner: LEFunctionValue<'ctx>,
    meta: MetaData,
}

#[derive(Clone, Debug)]
pub enum Symbol<'ctx> {
    Type(Type<'ctx>),
    Variable(Variable<'ctx>),
    Function(Function<'ctx>),
}

impl<'ctx> Symbol<'ctx> {
    pub fn is_builtin(&self) -> bool {
        match self {
            Symbol::Type(v) => { v.meta.is_built_in }
            Symbol::Variable(v) => { v.meta.is_built_in }
            Symbol::Function(v) => { v.meta.is_built_in }
        }
    }
}

#[derive(Clone, Debug)]
struct BuiltinTypes<'ctx> {
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


#[derive(Debug, Clone)]
pub struct SymbolTable<'ctx> {
    llvm_context: &'ctx Context,
    table: Vec<HashMap<String, Symbol<'ctx>>>,
    builtin_types: BuiltinTypes<'ctx>,
}

impl<'ctx> BuiltinTypes<'ctx> {
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
        Self {
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
        }
    }
}


impl<'ctx> SymbolTable<'ctx> {
    pub fn new(llvm_context: &'ctx Context) -> Self {
        let builtin_types = BuiltinTypes::new(llvm_context);

        let intrinsic_types = [
            ("bool".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Bool(builtin_types.bool_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("i8".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.i8_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("i16".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.i16_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("i32".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.i32_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("i64".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.i64_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("u8".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.u8_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("u16".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.u16_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("u32".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.u32_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("u64".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Integer(builtin_types.u64_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("f32".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Float(builtin_types.f32_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
            ("f64".into(), Symbol::Type(
                Type {
                    inner: LEBasicTypeEnum::Float(builtin_types.f64_type.clone()),
                    meta: MetaData { defined_pos: Position { range: 0..0 }, is_built_in: true },
                }
            )),
        ];
        Self {
            table: vec![HashMap::from(intrinsic_types)],
            llvm_context,
            builtin_types,
        }
    }


    pub fn get_type(&self, type_declarator: &TypeDeclarator) -> Result<LEBasicTypeEnum<'ctx>> {
        match type_declarator {
            TypeDeclarator::TypeIdentifier(identifier) => {
                let symbol = self.get_symbol(&identifier.name).ok_or_else(|| CompileError::UnknownIdentifier { identifier: identifier.name.clone() })?;
                if let Symbol::Type(t) = symbol {
                    Ok(t.inner)
                } else {
                    Err(CompileError::IdentifierIsNotType { identifier: identifier.name.clone() })
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
        let symbol = self.get_symbol(variable).ok_or_else(|| CompileError::UnknownIdentifier { identifier: variable.into() })?;
        if let Symbol::Variable(v) = symbol {
            Ok(v.pointer)
        } else {
            Err(CompileError::IdentifierIsNotType { identifier: variable.into() })
        }
    }

    pub fn get_function(&self, function: &str) -> Result<LEFunctionValue<'ctx>> {
        let symbol = self.get_symbol(function).ok_or_else(|| CompileError::UnknownIdentifier { identifier: function.into() })?;
        if let Symbol::Function(f) = symbol {
            Ok(f.inner)
        } else {
            Err(CompileError::IdentifierIsNotType { identifier: function.into() })
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
        if let Some(symbol) = global_table.get(&name) {
            return if !symbol.is_builtin() {
                let defined_position = match symbol {
                    Symbol::Type(t) => { t.meta.defined_pos.clone() }
                    Symbol::Variable(v) => { v.meta.defined_pos.clone() }
                    Symbol::Function(f) => { f.meta.defined_pos.clone() }
                };
                Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position })
            } else {
                Err(CompileError::CanNotRedefineBuiltinTypes { identifier: name })
            };
        } else {
            global_table.entry(name).or_insert(symbol);
        }
        Ok(())
    }

    pub fn insert_local_symbol(&mut self, name: String, symbol: Symbol<'ctx>) -> Result<()> {
        if let Some(symbol) = self.get_symbol(&name) {
            return if !symbol.is_builtin() {
                let defined_position = match symbol {
                    Symbol::Type(t) => { t.meta.defined_pos.clone() }
                    Symbol::Variable(v) => { v.meta.defined_pos.clone() }
                    Symbol::Function(f) => { f.meta.defined_pos.clone() }
                };
                Err(CompileError::IdentifierAlreadyDefined { identifier: name, defined_position })
            } else {
                Err(CompileError::CanNotRedefineBuiltinTypes { identifier: name })
            };
        } else {
            let local_table = self.table.last_mut().unwrap();
            local_table.entry(name).or_insert(symbol);
        }
        Ok(())
    }

    pub fn insert_global_variable(&mut self, name: String, value: LEPointerValue<'ctx>, position: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Variable(Variable { pointer: value, meta: MetaData { defined_pos: position, is_built_in: false } }))
    }

    pub fn insert_global_type(&mut self, name: String, value: LEBasicTypeEnum<'ctx>, defined_position: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Type(Type { inner: value, meta: MetaData { defined_pos: defined_position, is_built_in: false } }))
    }
    pub fn insert_global_function(&mut self, name: String, value: LEFunctionValue<'ctx>, defined_position: Position) -> Result<()> {
        self.insert_global_symbol(name, Symbol::Function(Function { inner: value, meta: MetaData { defined_pos: defined_position, is_built_in: false } }))
    }

    pub fn insert_local_function(&mut self, name: String, value: LEFunctionValue<'ctx>, defined_position: Position) -> Result<()> {
        self.insert_local_symbol(name, Symbol::Function(Function { inner: value, meta: MetaData { defined_pos: defined_position, is_built_in: false } }))
    }

    pub fn insert_local_type(&mut self, name: String, value: LEBasicTypeEnum<'ctx>, defined_position: Position) -> Result<()> {
        self.insert_local_symbol(name, Symbol::Type(Type { inner: value, meta: MetaData { defined_pos: defined_position, is_built_in: false } }))
    }

    pub fn insert_local_variable(&mut self, name: String, value: LEPointerValue<'ctx>, position: Position) -> Result<()> {
        self.insert_local_symbol(name, Symbol::Variable(Variable { pointer: value, meta: MetaData { defined_pos: position, is_built_in: false } }))
    }

    pub fn push_block_table(&mut self) {
        self.table.push(HashMap::default());
    }
    pub fn pop_block_table(&mut self) {
        self.table.pop();
    }

    pub fn bool_type(&self) -> LEBoolType<'ctx> {
        self.builtin_types.bool_type.clone()
    }
    pub fn i8_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.i8_type.clone()
    }
    pub fn i16_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.i16_type.clone()
    }
    pub fn i32_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.i32_type.clone()
    }
    pub fn i64_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.i64_type.clone()
    }
    pub fn u8_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.u8_type.clone()
    }
    pub fn u16_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.u16_type.clone()
    }
    pub fn u32_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.u32_type.clone()
    }
    pub fn u64_type(&self) -> LEIntegerType<'ctx> {
        self.builtin_types.u64_type.clone()
    }
    pub fn float_type(&self) -> LEFloatType<'ctx> {
        self.builtin_types.f32_type.clone()
    }
    pub fn double_type(&self) -> LEFloatType<'ctx> {
        self.builtin_types.f64_type.clone()
    }
}



