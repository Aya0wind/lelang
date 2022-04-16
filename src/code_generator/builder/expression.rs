use crate::code_generator::builder::{LEBasicValueEnum, LEPointerValue};
use crate::code_generator::builder::Result;
use crate::error::CompileError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpressionValue<'ctx> {
    Left(LEPointerValue<'ctx>),
    Right(LEBasicValueEnum<'ctx>),
    Unit,
}

impl<'ctx> ExpressionValue<'ctx> {
    pub fn is_left_value(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    pub fn is_right_value(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn to_left_value(&self) -> Result<LEPointerValue<'ctx>> {
        if let Self::Left(v) = self {
            Ok(v.clone())
        } else {
            Err(CompileError::ExpressionIsNotLeftValueExpression)
        }
    }
    pub fn to_right_value(&self) -> Result<LEBasicValueEnum<'ctx>> {
        if let Self::Right(v) = self {
            Ok(v.clone())
        } else {
            Err(CompileError::ExpressionIsNotRightValueExpression)
        }
    }
}

