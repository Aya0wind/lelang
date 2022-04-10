use std::fmt::{Display, Formatter};

use crate::ast::parser::{parse_function, parse_function_prototype, parse_variable_declaration};
use crate::ast::ParseResult;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};
use crate::lexer::{BinaryOperator, Number};

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
    pub op: BinaryOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: String,
    pub params: Vec<Expr>,
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
    pub condition: Option<Box<Expr>>,
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
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub pos: Position,
}


#[derive(Debug)]
pub struct IfStatement {
    pub cond: Box<Expr>,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub number: Number,
    pub pos: Position,
}

#[derive(Debug)]
pub struct UnaryOpExpression {
    pub op: BinaryOperator,
    pub expr: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Variable {
    pub prototype: VariablePrototype,
    pub value: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct VariablePrototype {
    pub type_name: String,
    pub name: String,
    pub pos: Position,
}


#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub pos: Position,
}


#[derive(Debug)]
pub enum Expr {
    BinaryOperator(BinaryOpExpression),
    UnaryOperator(UnaryOpExpression),
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
    CallExpression(FunctionCall),
}

#[derive(Debug)]
pub enum Statement {
    Expressions(Box<Expr>),
    VariableDefinition(Variable),
    Return(Box<Expr>),
    If(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Void,
}

#[derive(Debug)]
pub struct Ast {
    pub globals: Vec<Variable>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub extern_functions: Vec<ExternalFunction>,
}


impl Ast {
    pub fn from_lexer(tokens: LELexer) -> ParseResult<Self> {
        let mut ast = Self { globals: vec![], function_definitions: vec![], extern_functions: vec![] };
        ast.parse(tokens)?;
        Ok(ast)
    }

    fn parse(&mut self, mut tokens: LELexer) -> ParseResult<()> {
        loop {
            let next_token = tokens.current();
            match next_token {
                None => { break; }
                Some(token) => {
                    if let LEToken::KeyWord(keyword) = token {
                        match keyword {
                            KeyWord::Declare => {
                                tokens.consume_keyword()?;
                                let function_prototype = parse_function_prototype(&mut tokens)?;
                                tokens.consume_semicolon()?;
                                self.extern_functions.push(function_prototype);
                            }
                            KeyWord::FunctionDefine => {
                                let function = parse_function(&mut tokens)?;
                                self.function_definitions.push(function);
                            }
                            KeyWord::VariableDeclare => {
                                let variable = parse_variable_declaration(&mut tokens)?;
                                self.globals.push(variable);
                            }
                            _ => {
                                return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare, LEToken::KeyWord(keyword.clone())).into());
                            }
                        }
                    } else {
                        return Err(SyntaxError::unexpect_token(TokenType::FunctionDeclare,token.clone()).into());
                    }
                }
            }
        }
        Ok(())
    }
}
