use inkwell::values::FunctionValue;

use crate::code_generator::builder::le_type::LEFunctionType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFunctionValue<'ctx> {
    pub ty: LEFunctionType<'ctx>,
    pub llvm_value: FunctionValue<'ctx>,
}



