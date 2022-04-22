use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer, write, Write};
use std::rc::Rc;

use enum_dispatch::enum_dispatch;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{AnyType, ArrayType, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType, VectorType};
use inkwell::values::{AggregateValue, AnyValue, AnyValueEnum, ArrayValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue, VectorValue};

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
        LEBasicTypeEnum::Integer(self.clone())
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
        write!(f, "{}{}", if self.signed() { "i" } else { "u" }, self.inner.llvm_type.get_bit_width())
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
        LEBasicTypeEnum::Float(self.clone())
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
        write!(f, "f{}", if self.is_double() { "64" } else { "32" })
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
        LEBasicTypeEnum::Bool(self.clone())
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
        write!(f, "bool")
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
        LEBasicTypeEnum::Pointer(self.clone())
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
        let mut pointed_type = self.get_point_type();
        let mut point_counter = 1;
        while let LEBasicTypeEnum::Pointer(pointer) = pointed_type {
            pointed_type = pointer.get_point_type();
            point_counter += 1;
        }
        (0..point_counter).for_each(|_| f.write_char('*').unwrap());
        write!(f, "{}", pointed_type)
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
        LEBasicTypeEnum::Array(self.clone())
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
        write!(f, "[{};{}]", self.inner.element_type, self.inner.llvm_type.len())
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
        LEBasicTypeEnum::Struct(self.clone())
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
        write!(f, "{:?}", self.name())
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
        LEBasicTypeEnum::Vector(self.clone())
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
    Integer(LEIntegerType<'ctx>),
    //An float type
    Float(LEFloatType<'ctx>),
    //An bool type
    Bool(LEBoolType<'ctx>),
    /// A pointer type.
    Pointer(LEPointerType<'ctx>),
    //A array type
    Array(LEArrayType<'ctx>),
    /// A contiguous heterogeneous container type.
    Struct(LEStructType<'ctx>),
    /// A contiguous homogeneous "SIMD" container type.
    Vector(LEVectorType<'ctx>),
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
            LEBasicTypeEnum::Integer(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Float(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Bool(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Pointer(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Array(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Struct(t) => { t.get_llvm_basic_type() }
            LEBasicTypeEnum::Vector(t) => { t.get_llvm_basic_type() }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            LEBasicTypeEnum::Integer(t) => { t.name() }
            LEBasicTypeEnum::Float(t) => { t.name() }
            LEBasicTypeEnum::Bool(t) => { t.name() }
            LEBasicTypeEnum::Pointer(t) => { t.name() }
            LEBasicTypeEnum::Array(t) => { t.name() }
            LEBasicTypeEnum::Struct(t) => { t.name() }
            LEBasicTypeEnum::Vector(t) => { t.name() }
        }
    }

    fn get_llvm_basic_type(&self) -> BasicTypeEnum<'ctx> {
        self.get_llvm_type()
    }
}

impl<'ctx> LEBasicTypeEnum<'ctx> {
    pub fn into_int_type(self) -> Option<LEIntegerType<'ctx>> {
        if let LEBasicTypeEnum::Integer(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_float_type(self) -> Option<LEFloatType<'ctx>> {
        if let LEBasicTypeEnum::Float(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_pointer_type(self) -> Option<LEPointerType<'ctx>> {
        if let LEBasicTypeEnum::Pointer(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_array_type(self) -> Option<LEArrayType<'ctx>> {
        if let LEBasicTypeEnum::Array(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_struct_type(self) -> Option<LEStructType<'ctx>> {
        if let LEBasicTypeEnum::Struct(i) = self {
            Some(i)
        } else {
            None
        }
    }
    pub fn into_vector_type(self) -> Option<LEVectorType<'ctx>> {
        if let LEBasicTypeEnum::Vector(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn into_bool_type(self) -> Option<LEBoolType<'ctx>> {
        if let LEBasicTypeEnum::Bool(i) = self {
            Some(i)
        } else {
            None
        }
    }

    pub fn is_integer_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Integer(_))
    }
    pub fn is_float_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Float(_))
    }
    pub fn is_bool_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Bool(_))
    }
    pub fn is_pointer_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Pointer(_))
    }
    pub fn is_struct_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Struct(_))
    }
    pub fn is_array_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Array(_))
    }
    pub fn is_vector_type(&self) -> bool {
        matches!(self,LEBasicTypeEnum::Vector(_))
    }
}


impl<'ctx> Display for LEBasicTypeEnum<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                LEBasicTypeEnum::Array(t) => { t.fmt(f) }
                LEBasicTypeEnum::Integer(t) => { t.fmt(f) }
                LEBasicTypeEnum::Bool(t) => { t.fmt(f) }
                LEBasicTypeEnum::Float(t) => { t.fmt(f) }
                LEBasicTypeEnum::Pointer(t) => { t.fmt(f) }
                LEBasicTypeEnum::Struct(t) => { t.fmt(f) }
                LEBasicTypeEnum::Vector(t) => { t.fmt(f) }
            }
        }
    }
}

