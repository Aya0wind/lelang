use inkwell::values::PointerValue;

use crate::code_generator::builder::llvm_wrapper::LETypeEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LEVariable<'s> {
    pub ty: LETypeEnum<'s>,
    pub pointer: PointerValue<'s>,
}
