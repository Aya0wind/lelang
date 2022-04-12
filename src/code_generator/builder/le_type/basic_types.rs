use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{AnyType, ArrayType, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType, VectorType};
use inkwell::values::{AggregateValue, AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::ast::nodes::Position;
use crate::code_generator::builder::le_type::LEBasicType;
use crate::code_generator::builder::LEContext;
use crate::error::CompileError;

use super::super::Result;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct LEIntegerTypeInner<'ctx> {
    pub signed: bool,
    pub llvm_type: IntType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEIntegerType<'ctx> {
    inner: Rc<LEIntegerTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEIntegerType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::IntegerType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}


impl<'ctx> LEBasicType<'ctx> for LEIntegerType<'ctx> {
    type LLVM_Type = IntType<'ctx>;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::IntegerType(self.clone())
    }
    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::IntType(self.get_llvm_type())
    }
}


impl<'ctx> LEIntegerType<'ctx> {
    pub fn from_llvm_type(llvm_type: IntType<'ctx>, signed: bool) -> Self {
        Self { inner: Rc::new(LEIntegerTypeInner { llvm_type, signed }) }
    }
    pub fn signed(&self) -> bool {
        self.inner.signed
    }
}

impl<'ctx> Display for LEIntegerType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct LEFloatTypeInner<'ctx> {
    pub is_double: bool,
    pub llvm_type: FloatType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFloatType<'ctx> {
    inner: Rc<LEFloatTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEFloatType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::FloatType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}


impl<'ctx> LEBasicType<'ctx> for LEFloatType<'ctx> {
    type LLVM_Type = FloatType<'ctx>;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::FloatType(self.clone())
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::FloatType(self.inner.llvm_type)
    }
}

impl<'ctx> LEFloatType<'ctx> {
    pub fn from_llvm_type(llvm_type: FloatType<'ctx>, is_double: bool) -> Self {
        Self { inner: Rc::new(LEFloatTypeInner { llvm_type, is_double }) }
    }

    pub fn is_double(&self) -> bool {
        self.inner.is_double
    }
}

impl<'ctx> Display for LEFloatType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct LEPointerTypeInner<'ctx> {
    pub point_type: LEBasicTypeEnum<'ctx>,
    pub llvm_type: PointerType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEPointerType<'ctx> {
    inner: Rc<LEPointerTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEPointerType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::PointerType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}

impl<'ctx> LEBasicType<'ctx> for LEPointerType<'ctx> {
    type LLVM_Type = PointerType<'ctx>;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::PointerType(self.clone())
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::PointerType(self.inner.llvm_type)
    }
}

impl<'ctx> LEPointerType<'ctx> {
    pub fn get_point_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.inner.point_type.clone()
    }
    pub fn new(le_context: &LEContext<'ctx>, point_type: LEBasicTypeEnum<'ctx>) -> Self {
        let llvm_type = match point_type.get_llvm_type() {
            BasicTypeEnum::ArrayType(t) => { t.ptr_type(AddressSpace::Generic) }
            BasicTypeEnum::FloatType(t) => { t.ptr_type(AddressSpace::Generic) }
            BasicTypeEnum::IntType(t) => { t.ptr_type(AddressSpace::Generic) }
            BasicTypeEnum::PointerType(t) => { t.ptr_type(AddressSpace::Generic) }
            BasicTypeEnum::StructType(t) => { t.ptr_type(AddressSpace::Generic) }
            BasicTypeEnum::VectorType(t) => { t.ptr_type(AddressSpace::Generic) }
        };
        Self {
            inner: Rc::new(LEPointerTypeInner {
                point_type,
                llvm_type,
            })
        }
    }
    pub fn from_llvm_type(llvm_type: PointerType<'ctx>, point_type: LEBasicTypeEnum<'ctx>) -> Self {
        Self { inner: Rc::new(LEPointerTypeInner { llvm_type, point_type }) }
    }
}

impl<'ctx> Display for LEPointerType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct LEArrayTypeInner<'ctx> {
    pub llvm_type: ArrayType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEArrayType<'ctx> {
    inner: Rc<LEArrayTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEArrayType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::ArrayType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}

impl<'ctx> LEBasicType<'ctx> for LEArrayType<'ctx> {
    type LLVM_Type = ArrayType<'ctx>;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::ArrayType(self.clone())
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::ArrayType(self.inner.llvm_type)
    }
}

impl<'ctx> LEArrayType<'ctx> {
    pub fn from_llvm_type(llvm_type: ArrayType<'ctx>) -> Self {
        Self { inner: Rc::new(LEArrayTypeInner { llvm_type }) }
    }
}

impl<'ctx> Display for LEArrayType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LEStructTypeInner<'ctx> {
    pub llvm_type: StructType<'ctx>,
    pub member_offset: HashMap<String, (u32, LEBasicTypeEnum<'ctx>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEStructType<'ctx> {
    inner: Rc<LEStructTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEStructType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::StructType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}

impl<'ctx> LEBasicType<'ctx> for LEStructType<'ctx> {
    type LLVM_Type = StructType<'ctx>;
    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::StructType(self.clone())
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::StructType(self.inner.llvm_type)
    }
}

impl<'ctx> Display for LEStructType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl<'ctx> LEStructType<'ctx> {
    pub fn from_llvm_type(context: &LEContext<'ctx>, names: &[&str], member_types: &[LEBasicTypeEnum<'ctx>]) -> Self {
        let mut offset = HashMap::default();
        for (index, (name, ty)) in names.iter().zip(member_types.iter()).enumerate() {
            offset.entry(name.to_string()).or_insert((index as u32, ty.clone()));
        }
        let struct_type = context
            .llvm_context
            .struct_type(
                &member_types
                    .iter()
                    .map(|x| x.get_basic_llvm_type())
                    .collect::<Vec<_>>()
                , true,
            );
        Self { inner: Rc::new(LEStructTypeInner { llvm_type: struct_type, member_offset: offset }) }
    }
    fn get_member_offset(&self, name: &str) -> Option<u32> {
        let offset = self.inner.member_offset.get(name)?;
        Some(offset.0)
    }
    fn get_member_type(&self, name: &str) -> Option<LEBasicTypeEnum> {
        let offset = self.inner.member_offset.get(name)?;
        Some(offset.1.clone())
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct LEVectorTypeInner<'ctx> {
    pub llvm_type: VectorType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEVectorType<'ctx> {
    inner: Rc<LEVectorTypeInner<'ctx>>,
}

impl<'ctx> TryFrom<LEBasicTypeEnum<'ctx>> for LEVectorType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeEnum<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeEnum::VectorType(t) = value {
            Ok(t)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".to_string(), value.to_string()))
        }
    }
}

impl<'ctx> LEBasicType<'ctx> for LEVectorType<'ctx> {
    type LLVM_Type = VectorType<'ctx>;

    fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::VectorType(self.clone())
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::VectorType(self.inner.llvm_type)
    }
}

impl<'ctx> Display for LEVectorType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LEBasicTypeEnum<'ctx> {
    /// An integer or float type.
    IntegerType(LEIntegerType<'ctx>),
    //An float type
    FloatType(LEFloatType<'ctx>),
    /// A pointer type.
    PointerType(LEPointerType<'ctx>),
    //A array type
    ArrayType(LEArrayType<'ctx>),
    /// A contiguous heterogeneous container type.
    StructType(LEStructType<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(LEVectorType<'ctx>),
}

impl<'ctx> LEBasicTypeEnum<'ctx> {
    pub fn into_int_type(self) -> Option<LEIntegerType<'ctx>> {
        if let LEBasicTypeEnum::IntegerType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_float_type(self) -> Option<LEFloatType<'ctx>> {
        if let LEBasicTypeEnum::FloatType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_pointer_type(self) -> Option<LEPointerType<'ctx>> {
        if let LEBasicTypeEnum::PointerType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_array_type(self) -> Option<LEArrayType<'ctx>> {
        if let LEBasicTypeEnum::ArrayType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_struct_type(self) -> Option<LEStructType<'ctx>> {
        if let LEBasicTypeEnum::StructType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_vector_type(self) -> Option<LEVectorType<'ctx>> {
        if let LEBasicTypeEnum::VectorType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        unsafe {
            match self {
                LEBasicTypeEnum::IntegerType(t) => { t.as_le_basic_type_enum() }
                LEBasicTypeEnum::FloatType(t) => { t.as_le_basic_type_enum() }
                LEBasicTypeEnum::PointerType(t) => { t.as_le_basic_type_enum() }
                LEBasicTypeEnum::ArrayType(t) => { t.as_le_basic_type_enum() }
                LEBasicTypeEnum::StructType(t) => { t.as_le_basic_type_enum() }
                LEBasicTypeEnum::VectorType(t) => { t.as_le_basic_type_enum() }
            }
        }
    }

    pub fn get_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        match self {
            LEBasicTypeEnum::IntegerType(t) => { t.get_basic_llvm_type() }
            LEBasicTypeEnum::FloatType(t) => { t.get_basic_llvm_type() }
            LEBasicTypeEnum::PointerType(t) => { t.get_basic_llvm_type() }
            LEBasicTypeEnum::ArrayType(t) => { t.get_basic_llvm_type() }
            LEBasicTypeEnum::StructType(t) => { t.get_basic_llvm_type() }
            LEBasicTypeEnum::VectorType(t) => { t.get_basic_llvm_type() }
        }
    }

    pub fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        self.get_llvm_type().into()
    }
}


impl<'ctx> Display for LEBasicTypeEnum<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                LEBasicTypeEnum::ArrayType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
                LEBasicTypeEnum::IntegerType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
                LEBasicTypeEnum::FloatType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
                LEBasicTypeEnum::PointerType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
                LEBasicTypeEnum::StructType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
                LEBasicTypeEnum::VectorType(t) => { write!(f, "{}", t.inner.llvm_type.print_to_string()) }
            }
        }
    }
}

