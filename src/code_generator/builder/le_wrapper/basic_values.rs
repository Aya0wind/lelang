use std::fmt::{Display, Formatter};

use enum_dispatch::enum_dispatch;
use inkwell::types::{ArrayType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType};
use inkwell::values::{AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::code_generator::builder::{LEArrayType, LEBoolType, LEStructType, LEValue, LEVectorType};
use crate::code_generator::builder::le_wrapper::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEFloatType, LEIntegerType, LEPointerType};
use crate::error::CompileError;

use super::super::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEIntegerValue<'ctx> {
    pub ty: LEIntegerType<'ctx>,
    pub llvm_value: IntValue<'ctx>,
}


impl<'ctx> LEValue<'ctx> for LEIntegerValue<'ctx> {
    type LLVM_Value_Type = IntValue<'ctx>;
    type LEType = LEIntegerType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::IntValue(i) = value {
            if let LEBasicTypeEnum::IntegerType(t) = ty {
                return Ok(LEIntegerValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}


impl<'ctx> LEBasicValue<'ctx> for LEIntegerValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Integer(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}

impl<'ctx> Display for LEIntegerValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFloatValue<'ctx> {
    pub ty: LEFloatType<'ctx>,
    pub llvm_value: FloatValue<'ctx>,
}

impl<'ctx> LEValue<'ctx> for LEFloatValue<'ctx> {
    type LLVM_Value_Type = FloatValue<'ctx>;
    type LEType = LEFloatType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::FloatValue(i) = value {
            if let LEBasicTypeEnum::FloatType(t) = ty {
                return Ok(LEFloatValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEFloatValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Float(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}


impl<'ctx> Display for LEFloatValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEBoolValue<'ctx> {
    pub ty: LEBoolType<'ctx>,
    pub llvm_value: IntValue<'ctx>,
}

impl<'ctx> LEValue<'ctx> for LEBoolValue<'ctx> {
    type LLVM_Value_Type = IntValue<'ctx>;
    type LEType = LEBoolType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::IntValue(i) = value {
            if let LEBasicTypeEnum::BoolType(t) = ty {
                return Ok(LEBoolValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEBoolValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Bool(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}


impl<'ctx> Display for LEBoolValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEArrayValue<'ctx> {
    pub ty: LEArrayType<'ctx>,
    pub llvm_value: ArrayValue<'ctx>,
}


impl<'ctx> LEValue<'ctx> for LEArrayValue<'ctx> {
    type LLVM_Value_Type = ArrayValue<'ctx>;
    type LEType = LEArrayType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::ArrayValue(i) = value {
            if let LEBasicTypeEnum::ArrayType(t) = ty {
                return Ok(LEArrayValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEArrayValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Array(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}

impl<'ctx> Display for LEArrayValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEPointerValue<'ctx> {
    pub ty: LEPointerType<'ctx>,
    pub llvm_value: PointerValue<'ctx>,
}


impl<'ctx> LEValue<'ctx> for LEPointerValue<'ctx> {
    type LLVM_Value_Type = PointerValue<'ctx>;
    type LEType = LEPointerType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::PointerValue(i) = value {
            return Ok(LEPointerValue { ty: ty.get_pointer_type(), llvm_value: i });
        }
        Err(CompileError::TypeMismatched { expect: "LEPointerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEPointerValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Pointer(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}

impl<'ctx> Display for LEPointerValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEStructValue<'ctx> {
    pub ty: LEStructType<'ctx>,
    pub llvm_value: StructValue<'ctx>,
}


impl<'ctx> LEValue<'ctx> for LEStructValue<'ctx> {
    type LLVM_Value_Type = StructValue<'ctx>;
    type LEType = LEStructType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::StructValue(i) = value {
            if let LEBasicTypeEnum::StructType(t) = ty {
                return Ok(LEStructValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEStructValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Struct(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}

impl<'ctx> Display for LEStructValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEVectorValue<'ctx> {
    pub ty: LEVectorType<'ctx>,
    pub llvm_value: VectorValue<'ctx>,
}


impl<'ctx> LEValue<'ctx> for LEVectorValue<'ctx> {
    type LLVM_Value_Type = VectorValue<'ctx>;
    type LEType = LEVectorType<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        self.llvm_value
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> Result<Self> {
        if let BasicValueEnum::VectorValue(i) = value {
            if let LEBasicTypeEnum::VectorType(t) = ty {
                return Ok(LEVectorValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::TypeMismatched { expect: "LEIntegerType".into(), found: ty.to_string() })
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEVectorValue<'ctx> {
    fn to_le_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::Vector(self.clone())
    }

    fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        self.ty.clone().to_le_type_enum()
    }
}

impl<'ctx> Display for LEVectorValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[enum_dispatch]
pub enum LEBasicValueEnum<'ctx> {
    Integer(LEIntegerValue<'ctx>),
    Float(LEFloatValue<'ctx>),
    Bool(LEBoolValue<'ctx>),
    Pointer(LEPointerValue<'ctx>),
    Array(LEArrayValue<'ctx>),
    Struct(LEStructValue<'ctx>),
    Vector(LEVectorValue<'ctx>),
}


impl<'ctx> LEValue<'ctx> for LEBasicValueEnum<'ctx> {
    type LLVM_Value_Type = BasicValueEnum<'ctx>;
    type LEType = LEBasicTypeEnum<'ctx>;

    fn get_llvm_value(&self) -> Self::LLVM_Value_Type {
        match self {
            LEBasicValueEnum::Integer(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Float(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Bool(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Pointer(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Array(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Struct(i) => { i.get_llvm_basic_value() }
            LEBasicValueEnum::Vector(i) => { i.get_llvm_basic_value() }
        }
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, v: BasicValueEnum<'ctx>) -> Result<Self> {
        match (v, ty) {
            (BasicValueEnum::IntValue(v), LEBasicTypeEnum::IntegerType(t)) => { Ok(LEBasicValueEnum::Integer(LEIntegerValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::IntValue(v), LEBasicTypeEnum::BoolType(t)) => { Ok(LEBasicValueEnum::Bool(LEBoolValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::FloatValue(v), LEBasicTypeEnum::FloatType(t)) => { Ok(LEBasicValueEnum::Float(LEFloatValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::ArrayValue(v), LEBasicTypeEnum::ArrayType(t)) => { Ok(LEBasicValueEnum::Array(LEArrayValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::StructValue(v), LEBasicTypeEnum::StructType(t)) => { Ok(LEBasicValueEnum::Struct(LEStructValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::VectorValue(v), LEBasicTypeEnum::VectorType(t)) => { Ok(LEBasicValueEnum::Vector(LEVectorValue { ty: t, llvm_value: v })) }
            (BasicValueEnum::PointerValue(v), LEBasicTypeEnum::PointerType(t)) => { Ok(LEBasicValueEnum::Pointer(LEPointerValue { ty: t, llvm_value: v })) }
            _ => { unreachable!() }
        }
    }
}


impl<'ctx> LEBasicValueEnum<'ctx> {
    // fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
    //     self.clone()
    // }
    //
    // pub(crate) fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
    //     match self {
    //         LEBasicValueEnum::Integer(v) => { v.get_le_type().as_le_basic_type_enum() }
    //         LEBasicValueEnum::Float(v) => { v.get_le_type().as_le_basic_type_enum() }
    //         LEBasicValueEnum::Pointer(v) => { v.get_le_type().as_le_basic_type_enum() }
    //         LEBasicValueEnum::Array(v) => { v.get_le_type().as_le_basic_type_enum() }
    //         LEBasicValueEnum::Struct(v) => { v.get_le_type().as_le_basic_type_enum() }
    //         LEBasicValueEnum::Vector(v) => { v.get_le_type().as_le_basic_type_enum() }
    //     }
    // }
    //
    // fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
    //     match self {
    //         LEBasicValueEnum::Integer(v) => { v.llvm_value.into() }
    //         LEBasicValueEnum::Float(v) => { v.llvm_value.into() }
    //         LEBasicValueEnum::Pointer(v) => { v.llvm_value.into() }
    //         LEBasicValueEnum::Array(v) => { v.llvm_value.into() }
    //         LEBasicValueEnum::Struct(v) => { v.llvm_value.into() }
    //         LEBasicValueEnum::Vector(v) => { v.llvm_value.into() }
    //     }
    // }

    // pub fn from_llvm_basic_value_enum_and_type(v: BasicValueEnum<'ctx>, ty: LEBasicTypeEnum<'ctx>) -> Self {
    //     match (v, ty) {
    //         (BasicValueEnum::IntValue(v), LEBasicTypeEnum::IntegerType(t)) => { LEBasicValueEnum::Integer(LEIntegerValue { ty: t, llvm_value: v }) }
    //         (BasicValueEnum::FloatValue(v), LEBasicTypeEnum::FloatType(t)) => { LEBasicValueEnum::Float(LEFloatValue { ty: t, llvm_value: v }) }
    //         (BasicValueEnum::ArrayValue(v), LEBasicTypeEnum::ArrayType(t)) => { LEBasicValueEnum::Array(LEArrayValue { ty: t, llvm_value: v }) }
    //         (BasicValueEnum::StructValue(v), LEBasicTypeEnum::StructType(t)) => { LEBasicValueEnum::Struct(LEStructValue { ty: t, llvm_value: v }) }
    //         (BasicValueEnum::VectorValue(v), LEBasicTypeEnum::VectorType(t)) => { LEBasicValueEnum::Vector(LEVectorValue { ty: t, llvm_value: v }) }
    //         (BasicValueEnum::PointerValue(v), LEBasicTypeEnum::PointerType(t)) => { LEBasicValueEnum::Pointer(LEPointerValue { ty: t, llvm_value: v }) }
    //         _ => { unreachable!() }
    //     }
    // }

    pub fn to_llvm_basic_value_enum(&self) -> BasicValueEnum<'ctx> {
        match self {
            LEBasicValueEnum::Integer(i) => { BasicValueEnum::IntValue(i.llvm_value) }
            LEBasicValueEnum::Float(i) => { BasicValueEnum::FloatValue(i.llvm_value) }
            LEBasicValueEnum::Bool(i) => { BasicValueEnum::IntValue(i.llvm_value) }
            LEBasicValueEnum::Pointer(i) => { BasicValueEnum::PointerValue(i.llvm_value) }
            LEBasicValueEnum::Array(i) => { BasicValueEnum::ArrayValue(i.llvm_value) }
            LEBasicValueEnum::Struct(i) => { BasicValueEnum::StructValue(i.llvm_value) }
            LEBasicValueEnum::Vector(i) => { BasicValueEnum::VectorValue(i.llvm_value) }
        }
    }
}
