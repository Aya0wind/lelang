use std::fmt::{Display, Formatter};

use inkwell::types::{ArrayType, BasicTypeEnum, FloatType, FunctionType, PointerType};
use inkwell::values::{AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::code_generator::builder::llvm_type_wrapper::{LEArrayType, LEBasicType, LEBasicTypeGenericRef, LEBasicValue, LEFloatType, LEFunctionType, LEIntegerType, LEPointerType, LEStructType, LEVectorType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LEIntegerValue<'ctx,'a> {
    pub ty: &'a LEIntegerType<'ctx>,
    pub llvm_value: IntValue<'ctx>,
}

impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEIntegerValue<'ctx, 'a> {
    type LEType = LEIntegerType<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::IntegerValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::IntValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEIntegerValue<'ctx,'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LEFloatValue<'ctx,'a> {
    pub ty:&'a LEFloatType<'ctx>,
    pub llvm_value: FloatValue<'ctx>,
}
impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEFloatValue<'ctx, 'a> {
    type LEType = LEFloatType<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::FloatValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::FloatValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEFloatValue<'ctx,'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}





#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEFunctionValue<'ctx,'a> {
    pub ty:&'a LEFunctionType<'ctx>,
    pub llvm_value: FunctionValue<'ctx>,
}



#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEArrayValue<'ctx,'a> {
    pub ty:&'a LEArrayType<'ctx>,
    pub llvm_value: ArrayValue<'ctx>,
}
impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEArrayValue<'ctx, 'a> {
    type LEType = LEArrayType<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::ArrayValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::ArrayValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEArrayValue<'ctx,'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}





#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEPointerValue<'ctx> {
    pub ty: LEBasicTypeGenericRef<'ctx>,
    pub llvm_value: PointerValue<'ctx>,
}
impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEPointerValue<'ctx> {
    type LEType = LEBasicTypeGenericRef<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::PointerValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        &self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::PointerValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEPointerValue<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}





#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEStructValue<'ctx, 'a> {
    pub ty: &'a LEStructType<'ctx>,
    pub llvm_value: StructValue<'ctx>,
}
impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEStructValue<'ctx, 'a> {
    type LEType = LEStructType<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::StructValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::StructValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEStructValue<'ctx,'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct LEVectorValue<'ctx, 'a> {
    pub ty: &'a LEVectorType<'ctx>,
    pub llvm_value: VectorValue<'ctx>,
}
impl<'ctx, 'a> LEBasicValue<'ctx,'a> for LEVectorValue<'ctx, 'a> {
    type LEType = LEVectorType<'ctx>;

    fn as_le_value_enum(&self) -> LEBasicValueEnum<'ctx, 'a> {
        LEBasicValueEnum::VectorValue(*self)
    }

    fn get_le_type(&self) -> &'a Self::LEType {
        self.ty
    }

    fn get_basic_llvm_value(&self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::VectorValue(self.llvm_value)
    }
}

impl<'ctx,'a> Display for LEVectorValue<'ctx,'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}







#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LEBasicValueEnum<'ctx,'a> {
    IntegerValue(LEIntegerValue<'ctx,'a>),
    FloatValue(LEFloatValue<'ctx,'a>),
    PointerValue(LEPointerValue<'ctx>),
    ArrayValue(LEArrayValue<'ctx,'a>),
    StructValue(LEStructValue<'ctx,'a>),
    VectorValue(LEVectorValue<'ctx,'a>),
    UnitValue,
}

impl<'ctx, 'a> LEBasicValueEnum<'ctx,'a> {
    pub fn from_llvm_basic_value_enum_and_type(v:BasicValueEnum<'ctx>, ty:&'a LEBasicTypeGenericRef<'ctx>) ->Self{
        match (v,ty) {
            (BasicValueEnum::IntValue(v), LEBasicTypeGenericRef::IntegerType(t)) => { LEBasicValueEnum::IntegerValue(LEIntegerValue{ ty: t, llvm_value: v }) }
            (BasicValueEnum::FloatValue(v), LEBasicTypeGenericRef::FloatType(t)) => { LEBasicValueEnum::FloatValue(LEFloatValue{ ty: t, llvm_value: v }) }
            (BasicValueEnum::ArrayValue(v), LEBasicTypeGenericRef::ArrayType(t)) => { LEBasicValueEnum::ArrayValue(LEArrayValue{ ty: t, llvm_value: v }) }
            (BasicValueEnum::StructValue(v), LEBasicTypeGenericRef::StructType(t)) => { LEBasicValueEnum::StructValue(LEStructValue{ ty: t, llvm_value: v }) }
            (BasicValueEnum::VectorValue(v), LEBasicTypeGenericRef::VectorType(t)) => { LEBasicValueEnum::VectorValue(LEVectorValue{ ty: t, llvm_value: v }) }
            (BasicValueEnum::PointerValue(v), LEBasicTypeGenericRef::PointerType(t)) => { LEBasicValueEnum::PointerValue(LEPointerValue{ ty: t.as_le_type_generic_ref_enum(), llvm_value: v }) }
            _=>{unimplemented!()}
        }
    }
}
