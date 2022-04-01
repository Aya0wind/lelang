use std::fmt::{Display, Formatter, write};

use crate::lexer::{Number, Operator};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub(crate) line: usize,
}

impl From<usize> for Position {
    fn from(line: usize) -> Self {
        Self { line }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.line)
    }
}

#[derive(Debug)]
pub struct BinaryOpExpression {
    pub op: Operator,
    pub left: BExpr,
    pub right: BExpr,
    pub pos: Position,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub params: Vec<BExpr>,
    pub pos: Position,
}


#[derive(Debug)]
pub struct ForLoop {
    pub init_statement: Box<Statement>,
    pub condition: Box<Statement>,
    pub iterate: Box<Statement>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug)]
pub struct WhileLoop {
    pub condition: Option<BExpr>,
    pub code_block: CodeBlock,
    pub pos: Position,
}


#[derive(Debug)]
pub struct ExternalFunction {
    pub name: String,
    pub param_types: Vec<String>,
    pub return_type: Option<String>,
    pub pos: Position,
}


#[derive(Debug)]
pub struct FunctionDefinition {
    pub prototype: ExternalFunction,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
    pub pos: Position,
}


#[derive(Debug)]
pub enum Param {
    Identifier(String),
    Number(Number),
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub pos: Position,
}


#[derive(Debug)]
pub struct IfStatement {
    pub cond: BExpr,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct NumberLiteralNode {
    pub number: Number,
    pub pos: Position,
}

impl NumberLiteralNode {
    pub fn new(number: Number, pos: Position) -> Self {
        Self { number, pos }
    }
}

#[derive(Debug)]
pub struct UnaryOperatorNode {
    pub op: Operator,
    pub param: BExpr,
    pub pos: Position,
}

#[derive(Debug)]
pub struct VariableNode {
    pub prototype: VariablePrototype,
    pub value: BExpr,
    pub pos: Position,
}

#[derive(Debug)]
pub struct VariablePrototype {
    pub type_name: String,
    pub name: String,
    pub pos: Position,
}


#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
    pub pos: Position,
}


#[derive(Debug)]
pub enum Expr {
    BinaryOperator(BinaryOpExpression),
    NumberLiteral(NumberLiteralNode),
    Identifier(IdentifierNode),
    CallExpression(FunctionCall),
}

pub enum Declaration {
    Function(ExternalFunction),
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
