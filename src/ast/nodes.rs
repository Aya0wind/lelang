use crate::ast::{Annotation, CodeBlock};
use crate::lexer::{Number, Operator};

pub type BExpr = Box<Expr>;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    pub op: Operator,
    pub left: BExpr,
    pub right: BExpr,
}

#[derive(Debug)]
pub struct CallExpressionNode {}

#[derive(Debug)]
pub struct FunctionNode {
    pub name: String,
    pub params: Vec<Annotation>,
    pub return_type: String,
    pub code_block: CodeBlock,
}

#[derive(Debug)]
pub struct NumberLiteralNode {
    pub number: Number,
}

impl NumberLiteralNode {
    pub fn new(number: Number) -> Self {
        Self { number }
    }
}

#[derive(Debug)]
pub struct UnaryOperatorNode {
    pub op: Operator,
    pub param: BExpr,
}

#[derive(Debug)]
pub struct VariableNode {
    pub type_name: String,
    pub name: String,
    pub value: BExpr,
}

#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
}


#[derive(Debug)]
pub enum Expr {
    BinaryOperator(BinaryOperatorNode),
    NumberLiteral(NumberLiteralNode),
    UnaryOperator(UnaryOperatorNode),
    Identifier(IdentifierNode),
    CallExpression(CallExpressionNode),
}


