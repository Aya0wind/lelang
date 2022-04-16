use std::clone;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};

use crate::ast::parser::{parse_function, parse_function_prototype, parse_structure, parse_variable_declaration};
use crate::ast::ParseResult;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};
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

#[derive(Debug, Clone)]
pub struct BinaryOpExpression {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_name: String,
    pub params: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init_statement: Box<Statement>,
    pub condition: Box<Statement>,
    pub iterate: Box<Statement>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Option<Box<Expr>>,
    pub code_block: CodeBlock,
    pub pos: Position,
}


#[derive(Debug, Clone)]
pub struct FunctionPrototype {
    pub name: String,
    pub param_types: Vec<TypeDeclarator>,
    pub return_type: Option<TypeDeclarator>,
    pub pos: Position,
}


#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub prototype: FunctionPrototype,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub pos: Position,
}


#[derive(Debug, Clone)]
pub struct IfStatement {
    pub cond: Box<Expr>,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub number: Number,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct UnaryOpExpression {
    pub op: Operator,
    pub expr: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub prototype: VariablePrototype,
    pub value: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct VariablePrototype {
    pub type_declarator: Option<TypeDeclarator>,
    pub name: String,
    pub pos: Position,
}


#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ArrayInitializer {
    pub elements: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ArrayDeclarator {
    pub element_type: TypeDeclarator,
    pub len: u32,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub name: String,
    pub members: Vec<(String, TypeDeclarator)>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct StructureInitializer {
    pub structure_name: String,
    pub member_initial_values: Vec<(String, Box<Expr>)>,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub content: String,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub enum TypeDeclarator {
    TypeIdentifier(String),
    Array(Box<ArrayDeclarator>),
    Reference(Box<TypeDeclarator>),
}


#[derive(Debug, Clone)]
pub enum Expr {
    BinaryOperator(BinaryOpExpression),
    UnaryOperator(UnaryOpExpression),
    NumberLiteral(NumberLiteral),
    ArrayInitializer(ArrayInitializer),
    StructureInitializer(StructureInitializer),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    CallExpression(FunctionCall),
}


#[derive(Debug, Clone)]
pub enum Statement {
    Expressions(Box<Expr>),
    VariableDefinition(Variable),
    Return(Box<Expr>),
    If(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Void,
}


#[derive(Debug, Clone)]
pub struct Ast {
    pub globals_variables: Vec<Variable>,
    pub globals_structures: Vec<Structure>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub extern_functions: Vec<FunctionPrototype>,
}


impl Ast {
    pub fn from_lexer(tokens: LELexer) -> ParseResult<Self> {
        let mut ast = Self { globals_variables: vec![], globals_structures: vec![], function_definitions: vec![], extern_functions: vec![] };
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
                                self.globals_variables.push(variable);
                            }
                            KeyWord::StructureDeclare => {
                                let structure = parse_structure(&mut tokens)?;
                                self.globals_structures.push(structure);
                            }
                            _ => {
                                return Err(SyntaxError::UnexpectToken { expect: TokenType::FunctionDeclare, found: LEToken::KeyWord(keyword.clone()) });
                            }
                        }
                    } else {
                        return Err(SyntaxError::UnexpectToken { expect: TokenType::FunctionDeclare, found: token.clone() });
                    }
                }
            }
        }
        Ok(())
    }
}
