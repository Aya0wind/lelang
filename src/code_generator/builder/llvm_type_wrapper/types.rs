use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::DLLStorageClass::Default;
use inkwell::types::{AnyType, ArrayType, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType, VectorType};
use inkwell::values::{AggregateValue, AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};
use nom::combinator::value;

use crate::ast::nodes::Position;
use crate::code_generator::builder::LEContext;
use crate::code_generator::builder::llvm_type_wrapper::{LEArrayValue, LEBasicType, LEBasicValueEnum, LEFloatValue, LEIntegerValue, LEPointerValue, LEStructValue, LEVectorValue};
use crate::code_generator::compile_context::CompilerContext;
use crate::error::CompileError;

use super::super::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEIntegerType<'ctx> {
    pub signed: bool,
    pub llvm_type: IntType<'ctx>,
}


impl<'ctx> TryFrom<LEBasicTypeGenericRef<'ctx>> for &LEIntegerType<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicTypeGenericRef<'ctx>) -> std::result::Result<Self, Self::Error> {
        if let LEBasicTypeGenericRef::IntegerType(int) = value{
            Ok(int as &LEIntegerType)
        }else{
            Err(CompileError::type_mismatched("int".into(),value.to_string()))
        }
    }
}

impl<'ctx> LEBasicType<'ctx> for LEIntegerType<'ctx> {
    type LLVM_Type = IntType<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::IntegerType(self as *const Self)
    }
    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
       BasicTypeEnum::IntType(self.llvm_type)
    }
}




impl<'ctx> LEIntegerType<'ctx> {
    pub fn from_llvm_type(llvm_type: BasicTypeEnum<'ctx>,signed:bool) ->Result<Self> {
        if let BasicTypeEnum::IntType(int) = llvm_type{
            Ok(Self { signed, llvm_type: int })
        }else{
            Err(CompileError::type_mismatched("int".into(), "unit".into()))
        }
    }
}

impl<'ctx> Display for LEIntegerType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFloatType<'ctx> {
    pub llvm_type: FloatType<'ctx>,
}

impl<'ctx> LEBasicType<'ctx> for LEFloatType<'ctx> {
    type LLVM_Type = FloatType<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::FloatType(self as *const Self)
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::FloatType(self.llvm_type)
    }
}

impl<'ctx> Display for LEFloatType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEPointerType<'ctx> {
    pub point_type:LEBasicTypeGenericRef<'ctx>,
    pub llvm_type: PointerType<'ctx>,
}

impl<'ctx> LEBasicType<'ctx> for LEPointerType<'ctx> {
    type LLVM_Type = PointerType<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::PointerType(self as *const Self)
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::PointerType(self.llvm_type)
    }
}

impl<'ctx> LEPointerType<'ctx>{
    pub fn get_point_type(&self)->LEBasicTypeGenericRef<'ctx>{
        self.point_type
    }
}

impl<'ctx> Display for LEPointerType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEArrayType<'ctx> {
    pub llvm_type: ArrayType<'ctx>,
}

impl<'ctx> LEBasicType<'ctx> for LEArrayType<'ctx> {
    type LLVM_Type = ArrayType<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::ArrayType(self as *const Self)
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::ArrayType(self.llvm_type)
    }
}
impl<'ctx> Display for LEArrayType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEStructType<'ctx> {
    pub llvm_type: StructType<'ctx>,
    pub member_offset: HashMap<String,(u32, LEBasicTypeGenericRef<'ctx>)>,
}

impl<'ctx> LEBasicType<'ctx> for LEStructType<'ctx> {
    type LLVM_Type = StructType<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::StructType(self as *const Self)
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::StructType(self.llvm_type)
    }
}

impl<'ctx> Display for LEStructType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}


impl<'ctx> LEStructType<'ctx>{
    pub fn new(context:&LEContext, names:&[&str], member_types:&[LEBasicTypeGenericRef<'ctx>]) ->Self{
        let mut offset = HashMap::default();
        for (index,(name,ty)) in names.iter().zip(member_types.iter()).enumerate() {
            offset.entry(name.into()).or_insert((index as u32,*ty));
        }
        let struct_type = context
            .llvm_context
            .struct_type(
                &member_types
                    .iter()
                    .map(|x|  x.get_basic_llvm_type())
                    .collect::<Vec<_>>()
                , true
            );
        Self{ llvm_type: struct_type, member_offset: offset }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEVectorType<'ctx> {
    pub llvm_type: VectorType<'ctx>,
}
impl<'ctx> LEBasicType<'ctx> for LEVectorType<'ctx> {
    type LLVM_Type = VectorType<'ctx>;

    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        LEBasicTypeGenericRef::VectorType(self as *const Self)
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.llvm_type
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::VectorType(self.llvm_type)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFunctionType<'ctx> {
    pub llvm_type: FunctionType<'ctx>,
    pub return_type:*const LEBasicTypeGenericRef<'ctx>,
    pub param_types:Vec<LEBasicTypeGenericRef<'ctx>>,
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LEBasicTypeGenericRef<'ctx> {
    /// An integer or float type.
    IntegerType(*const LEIntegerType<'ctx>),
    //An float type
    FloatType(*const LEFloatType<'ctx>),
    /// A pointer type.
    PointerType(*const LEPointerType<'ctx>),
    //A array type
    ArrayType(*const LEArrayType<'ctx>),
    /// A contiguous heterogeneous container type.
    StructType(*const LEStructType<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(*const LEVectorType<'ctx>),
    /// A unit type.
    UnitType,
}

impl<'ctx> LEBasicType<'ctx> for LEBasicTypeGenericRef<'ctx> {
    type LLVM_Type = BasicTypeEnum<'ctx>;
    fn as_le_type_generic_ref_enum(&self) -> LEBasicTypeGenericRef<'ctx> {
        match self {
            LEBasicTypeGenericRef::IntegerType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::FloatType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::PointerType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::ArrayType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::StructType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::VectorType(t) => {t.as_le_type_generic_ref_enum()}
            LEBasicTypeGenericRef::UnitType => { LEBasicTypeGenericRef::UnitType}
        }
    }

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        match self {
            LEBasicTypeGenericRef::IntegerType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::FloatType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::PointerType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::ArrayType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::StructType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::VectorType(t) => {t.get_basic_llvm_type()}
            LEBasicTypeGenericRef::UnitType => {unreachable!()}
        }
    }

    fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        self.get_llvm_type()
    }
}


impl<'ctx> Display for LEBasicTypeGenericRef<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LEBasicTypeGenericRef::ArrayType(t) => { write!(f, "{}", t.llvm_type.print_to_string()) }
            LEBasicTypeGenericRef::IntegerType(t) => { write!(f, "{}", t.llvm_type.print_to_string())}
            LEBasicTypeGenericRef::FloatType(t) => { write!(f, "{}", t.llvm_type.print_to_string())}
            LEBasicTypeGenericRef::PointerType(t) => { write!(f, "{}", t.llvm_type.print_to_string()) }
            LEBasicTypeGenericRef::StructType(t) => { write!(f, "{}", t.llvm_type.print_to_string()) }
            LEBasicTypeGenericRef::VectorType(t) => { write!(f, "{}", t.llvm_type.print_to_string()) }
            LEBasicTypeGenericRef::UnitType => { write!(f, "UnitType") }
        }
    }
}

