use std::rc::Rc;

use inkwell::types::FunctionType;

use crate::code_generator::builder::le_wrapper::LEBasicTypeEnum;

#[derive(Debug, Clone, PartialEq, Eq)]
struct LEFunctionTypeInner<'ctx> {
    pub llvm_type: FunctionType<'ctx>,
    pub return_type: Option<LEBasicTypeEnum<'ctx>>,
    pub param_types: Vec<LEBasicTypeEnum<'ctx>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LEFunctionType<'ctx> {
    inner: Rc<LEFunctionTypeInner<'ctx>>,
}

impl<'ctx> LEFunctionType<'ctx> {
    pub fn return_type(&self) -> Option<LEBasicTypeEnum<'ctx>> {
        self.inner.return_type.clone()
    }
    pub fn param_types(&self) -> &[LEBasicTypeEnum<'ctx>] {
        &self.inner.param_types
    }
    pub fn new(llvm_type: FunctionType<'ctx>, return_type: Option<LEBasicTypeEnum<'ctx>>, param_types: Vec<LEBasicTypeEnum<'ctx>>) -> Self {
        Self {
            inner: Rc::new(LEFunctionTypeInner {
                llvm_type,
                return_type,
                param_types,
            })
        }
    }
}

