use std::fmt::{Display, Formatter};

use inkwell::types::{ArrayType, BasicTypeEnum, FloatType, FunctionType, PointerType};
use inkwell::values::{AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::code_generator::builder::le_type::{LEArrayType, LEBasicType, LEBasicTypeEnum, LEBasicValue, LEFloatType, LEFunctionType, LEIntegerType, LEPointerType, LEStructType, LEVectorType};
use crate::error::CompileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEIntegerValue<'ctx> {
    pub ty: LEIntegerType<'ctx>,
    pub llvm_value: IntValue<'ctx>,
}

impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEIntegerValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::IntegerValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEIntegerType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEIntegerValue<'ctx> {
    type LEType = LEIntegerType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::IntegerValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::IntValue(self.llvm_value)
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::IntValue(i) = value {
            if let LEBasicTypeEnum::IntegerType(t) = ty {
                return Ok(LEIntegerValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
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

impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEFloatValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::FloatValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEFloatType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEFloatValue<'ctx> {
    type LEType = LEFloatType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::FloatValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::FloatValue(self.llvm_value)
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::FloatValue(i) = value {
            if let LEBasicTypeEnum::FloatType(t) = ty {
                return Ok(LEFloatValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
    }
}

impl<'ctx> Display for LEFloatValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEArrayValue<'ctx> {
    pub ty: LEArrayType<'ctx>,
    pub llvm_value: ArrayValue<'ctx>,
}

impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEArrayValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::ArrayValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEArrayType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEArrayValue<'ctx> {
    type LEType = LEArrayType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::ArrayValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::ArrayValue(self.llvm_value)
    }

    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::ArrayValue(i) = value {
            if let LEBasicTypeEnum::ArrayType(t) = ty {
                return Ok(LEArrayValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
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


impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEPointerValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::PointerValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEPointerType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEPointerValue<'ctx> {
    type LEType = LEPointerType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::PointerValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::PointerValue(self.llvm_value)
    }
    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::PointerValue(i) = value {
            if let LEBasicTypeEnum::PointerType(t) = ty {
                return Ok(LEPointerValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
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

impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEStructValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::StructValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEStructType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEStructValue<'ctx> {
    type LEType = LEStructType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::StructValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::StructValue(self.llvm_value)
    }
    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::StructValue(i) = value {
            if let LEBasicTypeEnum::StructType(t) = ty {
                return Ok(LEStructValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
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

impl<'ctx> TryFrom<LEBasicValueEnum<'ctx>> for LEVectorValue<'ctx> {
    type Error = CompileError;

    fn try_from(value: LEBasicValueEnum<'ctx>) -> Result<Self, Self::Error> {
        if let LEBasicValueEnum::VectorValue(v) = value {
            Ok(v)
        } else {
            Err(CompileError::type_mismatched("LEVectorType".into(), value.get_le_type().to_string()))
        }
    }
}

impl<'ctx> LEBasicValue<'ctx> for LEVectorValue<'ctx> {
    type LEType = LEVectorType<'ctx>;

    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        LEBasicValueEnum::VectorValue(self.clone())
    }

    fn get_le_type(&self) -> Self::LEType {
        self.ty.clone()
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::VectorValue(self.llvm_value)
    }
    fn from_type_and_llvm_value(ty: LEBasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> crate::code_generator::builder::Result<Self> {
        if let BasicValueEnum::VectorValue(i) = value {
            if let LEBasicTypeEnum::VectorType(t) = ty {
                return Ok(LEVectorValue { ty: t, llvm_value: i });
            }
        }
        Err(CompileError::type_mismatched("LEIntegerType".into(), ty.to_string()))
    }
}

impl<'ctx> Display for LEVectorValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LEBasicValueEnum<'ctx> {
    IntegerValue(LEIntegerValue<'ctx>),
    FloatValue(LEFloatValue<'ctx>),
    PointerValue(LEPointerValue<'ctx>),
    ArrayValue(LEArrayValue<'ctx>),
    StructValue(LEStructValue<'ctx>),
    VectorValue(LEVectorValue<'ctx>),
}


impl<'ctx> LEBasicValueEnum<'ctx> {
    fn as_le_basic_value_enum(&self) -> LEBasicValueEnum<'ctx> {
        self.clone()
    }

    pub(crate) fn get_le_type(&self) -> LEBasicTypeEnum<'ctx> {
        match self {
            LEBasicValueEnum::IntegerValue(v) => { v.get_le_type().as_le_basic_type_enum() }
            LEBasicValueEnum::FloatValue(v) => { v.get_le_type().as_le_basic_type_enum() }
            LEBasicValueEnum::PointerValue(v) => { v.get_le_type().as_le_basic_type_enum() }
            LEBasicValueEnum::ArrayValue(v) => { v.get_le_type().as_le_basic_type_enum() }
            LEBasicValueEnum::StructValue(v) => { v.get_le_type().as_le_basic_type_enum() }
            LEBasicValueEnum::VectorValue(v) => { v.get_le_type().as_le_basic_type_enum() }
        }
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        match self {
            LEBasicValueEnum::IntegerValue(v) => { v.llvm_value.into() }
            LEBasicValueEnum::FloatValue(v) => { v.llvm_value.into() }
            LEBasicValueEnum::PointerValue(v) => { v.llvm_value.into() }
            LEBasicValueEnum::ArrayValue(v) => { v.llvm_value.into() }
            LEBasicValueEnum::StructValue(v) => { v.llvm_value.into() }
            LEBasicValueEnum::VectorValue(v) => { v.llvm_value.into() }
        }
    }

    pub fn from_llvm_basic_value_enum_and_type(v: BasicValueEnum<'ctx>, ty: LEBasicTypeEnum<'ctx>) -> Self {
        match (v, ty) {
            (BasicValueEnum::IntValue(v), LEBasicTypeEnum::IntegerType(t)) => { LEBasicValueEnum::IntegerValue(LEIntegerValue { ty: t, llvm_value: v }) }
            (BasicValueEnum::FloatValue(v), LEBasicTypeEnum::FloatType(t)) => { LEBasicValueEnum::FloatValue(LEFloatValue { ty: t, llvm_value: v }) }
            (BasicValueEnum::ArrayValue(v), LEBasicTypeEnum::ArrayType(t)) => { LEBasicValueEnum::ArrayValue(LEArrayValue { ty: t, llvm_value: v }) }
            (BasicValueEnum::StructValue(v), LEBasicTypeEnum::StructType(t)) => { LEBasicValueEnum::StructValue(LEStructValue { ty: t, llvm_value: v }) }
            (BasicValueEnum::VectorValue(v), LEBasicTypeEnum::VectorType(t)) => { LEBasicValueEnum::VectorValue(LEVectorValue { ty: t, llvm_value: v }) }
            (BasicValueEnum::PointerValue(v), LEBasicTypeEnum::PointerType(t)) => { LEBasicValueEnum::PointerValue(LEPointerValue { ty: t, llvm_value: v }) }
            _ => { unreachable!() }
        }
    }
    pub fn to_llvm_basic_value_enum(&self) -> BasicValueEnum<'ctx> {
        match self {
            LEBasicValueEnum::IntegerValue(i) => { BasicValueEnum::IntValue(i.llvm_value) }
            LEBasicValueEnum::FloatValue(i) => { BasicValueEnum::FloatValue(i.llvm_value) }
            LEBasicValueEnum::PointerValue(i) => { BasicValueEnum::PointerValue(i.llvm_value) }
            LEBasicValueEnum::ArrayValue(i) => { BasicValueEnum::ArrayValue(i.llvm_value) }
            LEBasicValueEnum::StructValue(i) => { BasicValueEnum::StructValue(i.llvm_value) }
            LEBasicValueEnum::VectorValue(i) => { BasicValueEnum::VectorValue(i.llvm_value) }
        }
    }
}
