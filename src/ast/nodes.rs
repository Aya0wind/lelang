use inkwell::values::VectorValue;

use crate::lexer::{Number, Operator};

#[derive(Debug)]
pub struct BinaryOpExpression {
    pub op: Operator,
    pub left: BExpr,
    pub right: BExpr,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub params: Vec<BExpr>,
}


#[derive(Debug)]
pub struct ForLoop {
    pub init_statement: Box<Statement>,
    pub condition: Box<Statement>,
    pub iterate: Box<Statement>,
    pub code_block: CodeBlock,
}

#[derive(Debug)]
pub struct WhileLoop {
    pub condition: Option<BExpr>,
    pub code_block: CodeBlock,
}


#[derive(Debug)]
pub struct ExternFunction {
    pub name: String,
    pub param_types: Vec<String>,
    pub return_type: Option<String>,
}


#[derive(Debug)]
pub struct FunctionDefinition {
    pub prototype: ExternFunction,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
}


#[derive(Debug)]
pub enum Param {
    Identifier(String),
    Number(Number),
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
}


#[derive(Debug)]
pub struct IfStatement {
    pub cond: BExpr,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
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
    pub prototype: VariablePrototype,
    pub value: BExpr,
}

#[derive(Debug)]
pub struct VariablePrototype {
    pub type_name: String,
    pub name: String,
}


#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
}


#[derive(Debug)]
pub enum Expr {
    BinaryOperator(BinaryOpExpression),
    NumberLiteral(NumberLiteralNode),
    Identifier(IdentifierNode),
    CallExpression(FunctionCall),
}

pub enum Declaration {
    Function(ExternFunction),
    Variable(VariableNode),
}


pub type BExpr = Box<Expr>;

#[derive(Debug)]
pub enum Statement {
    Expressions(BExpr),
    VariableDefinition(VariableNode),
    Return(BExpr),
    If(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Void,
}
