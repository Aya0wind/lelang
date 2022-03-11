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
    pub init_statement:BExpr,
    pub condition:BExpr,
    pub iterate:BExpr,
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
    pub statements: Vec<Statement>,
}


#[derive(Debug)]
pub struct IfCondition {
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
    VariableDeclare(VariableNode),
    Return(BExpr),
    If(IfCondition),
    ForLoop(ForLoop),
}
