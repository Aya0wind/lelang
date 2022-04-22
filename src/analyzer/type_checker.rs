use crate::ast::nodes::Expr;
use crate::code_generator::builder::LEBasicTypeEnum;
use crate::code_generator::builder::Result;

struct TypeChecker {}

impl TypeChecker {
    pub fn check_call_expression<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}

    pub fn check_number_literal<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}

    pub fn check_array_initializer<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}

    pub fn check_binary_expression<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}


    pub fn check_unay_expression<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}


    pub fn check_struct_initializer<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}


    pub fn check_string_literal<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}


    pub fn check_identifier<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}

    pub fn check_call_expression<'ctx>() -> Result<LEBasicTypeEnum<'ctx>> {}


    pub fn deduce_type(expr: &Expr) -> LEBasicTypeEnum {
        match expr {
            Expr::BinaryOperator(_) => {}
            Expr::UnaryOperator(_) => {}
            Expr::NumberLiteral(_) => {}
            Expr::ArrayInitializer(_) => {}
            Expr::StructureInitializer(_) => {}
            Expr::StringLiteral(_) => {}
            Expr::Identifier(_) => {}
            Expr::CallExpression(_) => {}
        }
    }
}