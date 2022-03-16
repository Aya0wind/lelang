use crate::lexer::{Number, Operator};



#[derive(Debug)]
pub struct BinaryOperatorNode {
    pub op: Operator,
    pub left: BExpr,
    pub right: BExpr,
}

#[derive(Debug)]
pub struct FunctionCallNode {
    pub function_name:String,
    pub params:Vec<BExpr>
}





#[derive(Debug)]
pub struct ForLoop{
    pub init_statement:VariableNode,
    pub condition:BExpr,
    pub iterate:Box<Statement>,
    pub code_block:CodeBlock,
}

#[derive(Debug)]
pub struct FunctionNode {
    pub name: String,
    pub params: Vec<Annotation>,
    pub return_type: String,
    pub code_block: CodeBlock,
}

#[derive(Debug)]
pub struct Annotation {
    pub identifier: String,
    pub type_name: String,
}

#[derive(Debug)]
pub enum Param {
    Identifier(String),
    Number(Number),
}

#[derive(Debug)]
pub struct CodeBlock {
    pub expression: Vec<Statement>,
    pub variables:Vec<VariableNode>
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
    CallExpression(FunctionCallNode),
}


pub type BExpr = Box<Expr>;

#[derive(Debug)]
pub enum Statement {
    Expressions(BExpr),
    Return(BExpr),
    If(IfStatement),
    ForLoop(ForLoop),
}
