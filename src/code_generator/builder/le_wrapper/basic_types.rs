use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer, write};
use std::rc::Rc;

use enum_dispatch::enum_dispatch;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{AnyType, ArrayType, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType, VectorType};
use inkwell::values::{AggregateValue, AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

use crate::ast::nodes::Position;
use crate::code_generator::builder::{LEArrayValue, LEBoolValue, LEContext, LEFloatValue, LEIntegerValue, LEPointerValue, LEStructValue, LEType, LEVectorValue};
use crate::code_generator::builder::le_wrapper::LEBasicType;
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

impl<'ctx> LEType<'ctx> for LEIntegerType<'ctx> {
    type LLVM_Type = IntType<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Integer"
    }
}


impl<'ctx> LEBasicType<'ctx> for LEIntegerType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::IntegerType(self.clone())
    }

    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
    }
}


impl<'ctx> LEIntegerType<'ctx> {
    pub fn from_llvm_type(llvm_type: IntType<'ctx>, signed: bool) -> Self {
        Self { inner: Rc::new(LEIntegerTypeInner { llvm_type, signed }) }
    }
    pub fn signed(&self) -> bool {
        self.inner.signed
    }

    pub fn const_array(&self, values: &[LEIntegerValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
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

impl<'ctx> LEType<'ctx> for LEFloatType<'ctx> {
    type LLVM_Type = FloatType<'ctx>;
    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Float"
    }
}

impl<'ctx> LEBasicType<'ctx> for LEFloatType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::FloatType(self.clone())
    }

    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
    }
}

impl<'ctx> LEFloatType<'ctx> {
    pub fn from_llvm_type(llvm_type: FloatType<'ctx>, is_double: bool) -> Self {
        Self { inner: Rc::new(LEFloatTypeInner { llvm_type, is_double }) }
    }

    pub fn is_double(&self) -> bool {
        self.inner.is_double
    }

    pub fn const_array(&self, values: &[LEFloatValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
    }
}

impl<'ctx> Display for LEFloatType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct LEBoolTypeInner<'ctx> {
    pub llvm_type: IntType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEBoolType<'ctx> {
    inner: Rc<LEBoolTypeInner<'ctx>>,
}

impl<'ctx> LEType<'ctx> for LEBoolType<'ctx> {
    type LLVM_Type = IntType<'ctx>;
    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Bool"
    }
}

impl<'ctx> LEBasicType<'ctx> for LEBoolType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::BoolType(self.clone())
    }

    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
    }
}

impl<'ctx> LEBoolType<'ctx> {
    pub fn from_llvm_type(llvm_type: IntType<'ctx>) -> Self {
        Self { inner: Rc::new(LEBoolTypeInner { llvm_type }) }
    }

    pub fn const_array(&self, values: &[LEBoolValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
    }

    pub fn const_true_value(&self) -> LEBoolValue<'ctx> {
        LEBoolValue { ty: self.clone(), llvm_value: self.get_llvm_type().const_all_ones() }
    }

    pub fn const_false_value(&self) -> LEBoolValue<'ctx> {
        LEBoolValue { ty: self.clone(), llvm_value: self.get_llvm_type().const_zero() }
    }
}

impl<'ctx> Display for LEBoolType<'ctx> {
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

impl<'ctx> LEType<'ctx> for LEPointerType<'ctx> {
    type LLVM_Type = PointerType<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Pointer"
    }
}


impl<'ctx> LEBasicType<'ctx> for LEPointerType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::PointerType(self.clone())
    }


    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
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

    pub fn const_array(&self, values: &[LEPointerValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
    }
}

impl<'ctx> Display for LEPointerType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct LEArrayTypeInner<'ctx> {
    pub element_type: LEBasicTypeEnum<'ctx>,
    pub llvm_type: ArrayType<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEArrayType<'ctx> {
    inner: Rc<LEArrayTypeInner<'ctx>>,
}

impl<'ctx> LEType<'ctx> for LEArrayType<'ctx> {
    type LLVM_Type = ArrayType<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Array"
    }
}


impl<'ctx> LEBasicType<'ctx> for LEArrayType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::ArrayType(self.clone())
    }


    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
    }
}

impl<'ctx> LEArrayType<'ctx> {
    pub fn from_llvm_type(llvm_type: ArrayType<'ctx>, element_type: LEBasicTypeEnum<'ctx>) -> Self {
        Self {
            inner: Rc::new(LEArrayTypeInner {
                element_type,
                llvm_type,
            }
            )
        }
    }

    pub fn const_array(&self, values: &[LEArrayValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
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

impl<'ctx> LEType<'ctx> for LEStructType<'ctx> {
    type LLVM_Type = StructType<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Struct"
    }
}


impl<'ctx> LEBasicType<'ctx> for LEStructType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::StructType(self.clone())
    }


    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
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
        let struct_type = context.llvm_context.opaque_struct_type("struct");
        struct_type.set_body(&member_types
            .iter()
            .map(|x| x.get_llvm_basic_type())
            .collect::<Vec<_>>(), true,
        );
        Self { inner: Rc::new(LEStructTypeInner { llvm_type: struct_type, member_offset: offset }) }
    }
    pub fn get_member_offset(&self, name: &str) -> Option<u32> {
        let offset = self.inner.member_offset.get(name)?;
        Some(offset.0)
    }

    pub fn get_member_offset_and_type(&self, name: &str) -> Option<(u32, LEBasicTypeEnum<'ctx>)> {
        let offset = self.inner.member_offset.get(name)?.clone();
        Some(offset)
    }

    pub fn get_member_type(&self, name: &str) -> Option<LEBasicTypeEnum> {
        let offset = self.inner.member_offset.get(name)?;
        Some(offset.1.clone())
    }

    pub fn const_array(&self, values: &[LEStructValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
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

impl<'ctx> LEVectorType<'ctx> {
    pub fn const_array(&self, values: &[LEVectorValue<'ctx>]) -> LEArrayValue<'ctx> {
        let llvm_values = values.iter().map(|v| v.llvm_value).collect::<Vec<_>>();
        let array_value = self.get_llvm_type().const_array(&llvm_values);
        LEArrayValue { ty: self.get_array_type(values.len() as u32), llvm_value: array_value }
    }
}

impl<'ctx> LEType<'ctx> for LEVectorType<'ctx> {
    type LLVM_Type = VectorType<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        self.inner.llvm_type
    }

    fn name(&self) -> &'static str {
        "Vector"
    }
}


impl<'ctx> LEBasicType<'ctx> for LEVectorType<'ctx> {
    fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
        LEBasicTypeEnum::VectorType(self.clone())
    }

    fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let array_type = llvm_type.array_type(len);
        LEArrayType {
            inner: Rc::new(LEArrayTypeInner {
                element_type: self.clone().to_le_type_enum(),
                llvm_type: array_type,
            })
        }
    }

    fn get_pointer_type(&self) -> LEPointerType<'ctx> {
        let llvm_type = self.get_llvm_type();
        let pointer_type = llvm_type.ptr_type(AddressSpace::Generic);
        LEPointerType {
            inner: Rc::new(LEPointerTypeInner {
                point_type: self.to_le_type_enum(),
                llvm_type: pointer_type,
            })
        }
    }
}

impl<'ctx> Display for LEVectorType<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[enum_dispatch]
pub enum LEBasicTypeEnum<'ctx> {
    /// An integer or float type.
    IntegerType(LEIntegerType<'ctx>),
    //An float type
    FloatType(LEFloatType<'ctx>),
    //An bool type
    BoolType(LEBoolType<'ctx>),
    /// A pointer type.
    PointerType(LEPointerType<'ctx>),
    //A array type
    ArrayType(LEArrayType<'ctx>),
    /// A contiguous heterogeneous container type.
    StructType(LEStructType<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    VectorType(LEVectorType<'ctx>),
}

// impl<'ctx> LEBasicType<'ctx> for LEBasicTypeEnum<'ctx> {
//     fn to_le_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
//         self.clone()
//     }
//
//     fn get_array_type(&self, len: u32) -> LEArrayType<'ctx> {
//         match self {
//             LEBasicTypeEnum::IntegerType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::FloatType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::BoolType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::PointerType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::ArrayType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::StructType(t) => {t.get_array_type(len)}
//             LEBasicTypeEnum::VectorType(t) => {t.get_array_type(len)}
//         }
//     }
//
//     fn get_pointer_type(&self) -> LEPointerType<'ctx> {
//         match self {
//             LEBasicTypeEnum::IntegerType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::FloatType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::BoolType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::PointerType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::ArrayType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::StructType(t) => {t.get_pointer_type()}
//             LEBasicTypeEnum::VectorType(t) => {t.get_pointer_type()}
//         }
//     }
// }

impl<'ctx> LEType<'ctx> for LEBasicTypeEnum<'ctx> {
    type LLVM_Type = BasicTypeEnum<'ctx>;

    fn get_llvm_type(&self) -> Self::LLVM_Type {
        match self {
            LEBasicTypeEnum::IntegerType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::FloatType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::BoolType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::PointerType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::ArrayType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::StructType(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::VectorType(t) => { t.get_llvm_basic_type() }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            LEBasicTypeEnum::IntegerType(t) => { t.name() }
            LEBasicTypeEnum::FloatType(t) => { t.name() }
            LEBasicTypeEnum::BoolType(t) => { t.name() }
            LEBasicTypeEnum::PointerType(t) => { t.name() }
            LEBasicTypeEnum::ArrayType(t) => { t.name() }
            LEBasicTypeEnum::StructType(t) => { t.name() }
            LEBasicTypeEnum::VectorType(t) => { t.name() }
        }
    }

    fn get_llvm_basic_type(&self) -> BasicTypeEnum<'ctx> {
        self.get_llvm_type()
    }
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

    pub fn into_bool_type(self) -> Option<LEBoolType<'ctx>> {
        if let LEBasicTypeEnum::BoolType(i) = self {
            Some(i)
        } else {
            None
        }
    }
    // pub fn as_le_basic_type_enum(&self) -> LEBasicTypeEnum<'ctx> {
    //     unsafe {
    //         match self {
    //             LEBasicTypeEnum::IntegerType(t) => { t.as_le_basic_type_enum() }
    //             LEBasicTypeEnum::FloatType(t) => { t.as_le_basic_type_enum() }
    //             LEBasicTypeEnum::PointerType(t) => { t.as_le_basic_type_enum() }
    //             LEBasicTypeEnum::ArrayType(t) => { t.as_le_basic_type_enum() }
    //             LEBasicTypeEnum::StructType(t) => { t.as_le_basic_type_enum() }
    //             LEBasicTypeEnum::VectorType(t) => { t.as_le_basic_type_enum() }
    //         }
    //     }
    // }
    //
    // pub fn get_llvm_type(&self) -> BasicTypeEnum<'ctx> {
    //     match self {
    //         LEBasicTypeEnum::IntegerType(t) => { t.get_basic_llvm_type() }
    //         LEBasicTypeEnum::FloatType(t) => { t.get_basic_llvm_type() }
    //         LEBasicTypeEnum::PointerType(t) => { t.get_basic_llvm_type() }
    //         LEBasicTypeEnum::ArrayType(t) => { t.get_basic_llvm_type() }
    //         LEBasicTypeEnum::StructType(t) => { t.get_basic_llvm_type() }
    //         LEBasicTypeEnum::VectorType(t) => { t.get_basic_llvm_type() }
    //     }
    // }
    //
    // pub fn get_basic_llvm_type(&self) -> BasicTypeEnum<'ctx> {
    //     self.get_llvm_type().into()
    // }
    //
    //
    //
    // pub fn get_array_type(&self, llvm_context: &Context, len: u32) -> LEArrayType<'ctx> {
    //     match self {
    //         LEBasicTypeEnum::IntegerType(t) => {t.get_array_type(llvm_context, len)}
    //         LEBasicTypeEnum::FloatType(t) => {t.get_array_type(llvm_context, len)}
    //         LEBasicTypeEnum::PointerType(t) => {t.get_array_type(llvm_context, len)}
    //         LEBasicTypeEnum::ArrayType(t) => {t.get_array_type(llvm_context, len)}
    //         LEBasicTypeEnum::StructType(t) => {t.get_array_type(llvm_context, len)}
    //         LEBasicTypeEnum::VectorType(t) => {t.get_array_type(llvm_context, len)}
    //     }
    // }
    //
    // pub fn get_pointer_type(&self, llvm_context: &Context) -> LEPointerType<'ctx> {
    //     match self {
    //         LEBasicTypeEnum::IntegerType(t) => {t.get_pointer_type(llvm_context)}
    //         LEBasicTypeEnum::FloatType(t) => {t.get_pointer_type(llvm_context)}
    //         LEBasicTypeEnum::PointerType(t) => {t.get_pointer_type(llvm_context)}
    //         LEBasicTypeEnum::ArrayType(t) => {t.get_pointer_type(llvm_context)}
    //         LEBasicTypeEnum::StructType(t) => {t.get_pointer_type(llvm_context)}
    //         LEBasicTypeEnum::VectorType(t) => {t.get_pointer_type(llvm_context)}
    //     }
    // }
}


impl<'ctx> Display for LEBasicTypeEnum<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                LEBasicTypeEnum::ArrayType(t) => { t.fmt(f) }
                LEBasicTypeEnum::IntegerType(t) => { t.fmt(f) }
                LEBasicTypeEnum::BoolType(t) => { t.fmt(f) }
                LEBasicTypeEnum::FloatType(t) => { t.fmt(f) }
                LEBasicTypeEnum::PointerType(t) => { t.fmt(f) }
                LEBasicTypeEnum::StructType(t) => { t.fmt(f) }
                LEBasicTypeEnum::VectorType(t) => { t.fmt(f) }
            }
        }
    }
}

