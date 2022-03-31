use std::fmt::{Display, Formatter};

use inkwell::types::{AnyType, AnyTypeEnum, ArrayType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType, VectorType};
use inkwell::values::{AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::code_generator::builder::traits::{LEType, LEValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerValue<'s> {
    pub signed: bool,
    pub value: IntValue<'s>,
}

impl<'s> LEValue<'s> for IntegerValue<'s> {
    fn as_le_value_enum(&self) -> LEValueEnum<'s> {
        LEValueEnum::NumericValue(NumericValueEnum::Integer(*self))
    }

    fn as_any_value_enum(&self) -> AnyValueEnum<'s> {
        todo!()
    }
}

impl<'s> IntegerValue<'s> {
    pub fn get_type(&self) -> IntegerType {
        IntegerType { signed: self.signed, value: self.value.get_type() }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerType<'s> {
    pub signed: bool,
    pub value: IntType<'s>,
}

impl<'s> LEType<'s> for IntegerType<'s> {
    fn as_le_type_enum(&self) -> LETypeEnum<'s> {
        LETypeEnum::NumericType(NumericTypeEnum::IntegerType(*self))
    }

    fn as_any_type_enum(&self) -> AnyValueEnum<'s> {
        todo!()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericTypeEnum<'s> {
    FloatType(FloatType<'s>),
    IntegerType(IntegerType<'s>),
}

impl<'s> NumericTypeEnum<'s> {
    pub fn to_basic_type_enum(&self) -> BasicTypeEnum {
        match self {
            NumericTypeEnum::FloatType(f) => { BasicTypeEnum::FloatType(*f) }
            NumericTypeEnum::IntegerType(i) => { BasicTypeEnum::IntType(i.value) }
        }
    }
}

impl<'s> From<AnyTypeEnum<'s>> for NumericTypeEnum<'s> {
    fn from(_: AnyTypeEnum<'s>) -> Self {
        todo!()
    }
}


impl<'s> LEType<'s> for NumericTypeEnum<'s> {
    fn as_le_type_enum(&self) -> LETypeEnum<'s> {
        LETypeEnum::NumericType(*self)
    }

    fn as_any_type_enum(&self) -> AnyValueEnum<'s> {
        todo!()
    }
}


impl<'s> Display for NumericTypeEnum<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericTypeEnum::FloatType(float) => { write!(f, "{}", float.print_to_string().to_string()) }
            NumericTypeEnum::IntegerType(i) => { write!(f, "{}", i.value.print_to_string().to_string()) }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericValueEnum<'s> {
    Float(FloatValue<'s>),
    Integer(IntegerValue<'s>),
}


impl<'s> LEValue<'s> for NumericValueEnum<'s> {
    fn as_le_value_enum(&self) -> LEValueEnum<'s> {
        todo!()
    }

    fn as_any_value_enum(&self) -> AnyValueEnum<'s> {
        todo!()
    }
}

impl<'s> NumericValueEnum<'s> {
    pub fn to_basic_value_enum(&self) -> BasicValueEnum {
        match self {
            NumericValueEnum::Float(f) => { BasicValueEnum::FloatValue(*f) }
            NumericValueEnum::Integer(i) => { BasicValueEnum::IntValue(i.value) }
        }
    }

    pub fn get_type(&self) -> NumericTypeEnum {
        match self {
            NumericValueEnum::Float(f) => { NumericTypeEnum::FloatType(f.get_type()) }
            NumericValueEnum::Integer(i) => { NumericTypeEnum::IntegerType(i.get_type()) }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LEValueEnum<'s> {
    NumericValue(NumericValueEnum<'s>),
    ArrayValue(ArrayValue<'s>),
    FunctionValue(FunctionValue<'s>),
    PointerValue(PointerValue<'s>),
    StructValue(StructValue<'s>),
    VectorValue(VectorValue<'s>),
    UnitValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LETypeEnum<'s> {
    ArrayType(ArrayType<'s>),
    /// A function return and parameter definition.
    FunctionType(FunctionType<'s>),
    /// An integer or float type.
    NumericType(NumericTypeEnum<'s>),
    /// A pointer type.
    PointerType(PointerType<'s>),
    /// A contiguous heterogeneous container type.
    StructType(StructType<'s>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(VectorType<'s>),
    /// A unit type.
    UnitType,
}


impl<'s, A: AnyType<'s>> From<A> for LETypeEnum<'s> {
    fn from(a: A) -> Self {
        match a.as_any_type_enum() {
            AnyTypeEnum::ArrayType(a) => { LETypeEnum::ArrayType(a) }
            AnyTypeEnum::FloatType(a) => { LETypeEnum::NumericType(NumericTypeEnum::FloatType(a)) }
            AnyTypeEnum::FunctionType(a) => { LETypeEnum::FunctionType(a) }
            AnyTypeEnum::IntType(a) => { LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: true, value: a })) }
            AnyTypeEnum::PointerType(a) => { LETypeEnum::PointerType(a) }
            AnyTypeEnum::StructType(a) => { LETypeEnum::StructType(a) }
            AnyTypeEnum::VectorType(a) => { LETypeEnum::VectorType(a) }
            AnyTypeEnum::VoidType(_) => { LETypeEnum::UnitType }
        }
    }
}


impl<'s, A: AnyValue<'s>> From<A> for LEValueEnum<'s> {
    fn from(a: A) -> Self {
        match a.as_any_value_enum() {
            AnyValueEnum::ArrayValue(o) => { LEValueEnum::ArrayValue(o) }
            AnyValueEnum::IntValue(o) => { LEValueEnum::NumericValue(NumericValueEnum::Integer(IntegerValue { signed: true, value: o })) }
            AnyValueEnum::FloatValue(o) => { LEValueEnum::NumericValue(NumericValueEnum::Float(o)) }
            AnyValueEnum::FunctionValue(o) => { LEValueEnum::FunctionValue(o) }
            AnyValueEnum::PointerValue(o) => { LEValueEnum::PointerValue(o) }
            AnyValueEnum::StructValue(o) => { LEValueEnum::StructValue(o) }
            AnyValueEnum::VectorValue(o) => { LEValueEnum::VectorValue(o) }
            AnyValueEnum::PhiValue(_) => { LEValueEnum::UnitValue }
            AnyValueEnum::InstructionValue(_) => { LEValueEnum::UnitValue }
        }
    }
}

impl<'s> LEValueEnum<'s> {
    pub fn get_type(&self) -> LETypeEnum {
        match self {
            LEValueEnum::NumericValue(o) => {
                match o {
                    NumericValueEnum::Float(f) => { f.get_type().into() }
                    NumericValueEnum::Integer(i) => { LETypeEnum::NumericType(NumericTypeEnum::IntegerType(IntegerType { signed: i.signed, value: i.get_type().value })) }
                }
            }
            LEValueEnum::ArrayValue(o) => { o.get_type().into() }
            LEValueEnum::FunctionValue(o) => { o.get_type().into() }
            LEValueEnum::PointerValue(o) => { o.get_type().into() }
            LEValueEnum::StructValue(o) => { o.get_type().into() }
            LEValueEnum::VectorValue(o) => { o.get_type().into() }
            LEValueEnum::UnitValue => { LETypeEnum::UnitType }
        }
    }
}


impl<'s> Display for LETypeEnum<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LETypeEnum::ArrayType(t) => { write!(f, "{}", t.print_to_string()) }
            LETypeEnum::FunctionType(t) => { write!(f, "{}", t.print_to_string()) }
            LETypeEnum::NumericType(t) => { write!(f, "{}", t) }
            LETypeEnum::PointerType(t) => { write!(f, "{}", t.print_to_string()) }
            LETypeEnum::StructType(t) => { write!(f, "{}", t.print_to_string()) }
            LETypeEnum::VectorType(t) => { write!(f, "{}", t.print_to_string()) }
            LETypeEnum::UnitType => { write!(f, "UnitType") }
        }
    }
}
