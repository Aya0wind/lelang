use std::collections::HashMap;

use inkwell::types::FloatType;
use lazy_static::lazy_static;

use crate::code_generator::builder::llvm_wrapper::{IntegerType, NumericTypeEnum};

lazy_static! {
    //类型提升优先级(是否有符号,位长)
    pub static ref INT_TYPE_PROMOTION_PROVIDENCE: HashMap<(bool,u32),u32> = HashMap::from([
                ((true,64),11), //i64
                ((true,32),21), //i32
                ((true,16),31), //i16
                ((true,8),41), //i8
                ((false,64),10), //u64
                ((false,32),20), //u32
                ((false,16),30), //u16
                ((false,8),40), //u8
    ]);

    pub static ref FLOAT_TYPE_PROMOTION_PROVIDENCE: HashMap<u32,u32> = HashMap::from([
        (64,1), //f64
        (32,2) //f32
    ]);
}

pub fn get_float_promotion_providence(ty: &FloatType) -> u32 {
    let width = ty.size_of().get_sign_extended_constant().unwrap();
    if width == 32 {
        2
    } else {
        1
    }
}

pub fn get_integer_promotion_providence(ty: &IntegerType) -> u32 {
    let width = ty.value.get_bit_width();
    let signed = ty.signed;
    *INT_TYPE_PROMOTION_PROVIDENCE.get(&(signed, width)).unwrap()
}

pub fn get_number_providence(ty: &NumericTypeEnum) -> u32 {
    match ty {
        NumericTypeEnum::FloatType(f) => { get_float_promotion_providence(f) }
        NumericTypeEnum::IntegerType(i) => { get_integer_promotion_providence(i) }
    }
}
