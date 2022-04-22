use std::clone;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};

use crate::ast::parser::{parse_extern_function_prototype, parse_function, parse_structure, parse_variable_declaration};
use crate::error::{LEError, Result, SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken, Position};
use crate::lexer::{Number, Operator};

pub trait ASTNode {
    fn pos(&self) -> Position;
}

#[derive(Debug, Clone)]
pub struct BinaryOpExpression {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub pos: Position,
}

impl ASTNode for BinaryOpExpression {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}


#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_name: Identifier,
    pub params: Vec<Expr>,
    pub pos: Position,
}

impl ASTNode for FunctionCall {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init_statement: Box<Statement>,
    pub condition: Box<Statement>,
    pub iterate: Box<Statement>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

impl ASTNode for ForLoop {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Option<Box<Expr>>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

impl ASTNode for WhileLoop {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct FunctionPrototype {
    pub identifier: Identifier,
    pub param_types: Vec<TypeDeclarator>,
    pub return_type: Option<TypeDeclarator>,
    pub pos: Position,
}

impl ASTNode for FunctionPrototype {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub prototype: FunctionPrototype,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

impl ASTNode for FunctionDefinition {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub pos: Position,
}

impl ASTNode for CodeBlock {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub cond: Box<Expr>,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub pos: Position,
}

impl ASTNode for IfStatement {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub number: Number,
    pub pos: Position,
}

impl ASTNode for NumberLiteral {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct UnaryOpExpression {
    pub op: Operator,
    pub expr: Box<Expr>,
    pub pos: Position,
}

impl ASTNode for UnaryOpExpression {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub prototype: VariablePrototype,
    pub value: Box<Expr>,
    pub pos: Position,
}

impl ASTNode for Variable {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct VariablePrototype {
    pub type_declarator: Option<TypeDeclarator>,
    pub identifier: Identifier,
    pub pos: Position,
}

impl ASTNode for VariablePrototype {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub pos: Position,
}

impl ASTNode for Identifier {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct ArrayInitializer {
    pub elements: Vec<Expr>,
    pub pos: Position,
}

impl ASTNode for ArrayInitializer {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct ArrayDeclarator {
    pub element_type: TypeDeclarator,
    pub len: u32,
    pub pos: Position,
}

impl ASTNode for ArrayDeclarator {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub struct Structure {
    pub name: String,
    pub members: Vec<(String, TypeDeclarator)>,
    pub pos: Position,
}

impl ASTNode for Structure {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StructureInitializer {
    pub structure_name: Identifier,
    pub member_initial_values: Vec<(String, Box<Expr>)>,
    pub pos: Position,
}

impl ASTNode for StructureInitializer {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub content: String,
    pub pos: Position,
}

impl ASTNode for StringLiteral {
    fn pos(&self) -> Position {
        self.pos.clone()
    }
}
#[derive(Debug, Clone)]
pub enum TypeDeclarator {
    TypeIdentifier(Identifier),
    Array(Box<ArrayDeclarator>),
    Reference(Box<TypeDeclarator>),
}

impl ASTNode for TypeDeclarator {
    fn pos(&self) -> Position {
        match self {
            TypeDeclarator::TypeIdentifier(e) => { e.pos() }
            TypeDeclarator::Array(e) => { e.pos() }
            TypeDeclarator::Reference(e) => { e.pos() }
        }
    }
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

impl ASTNode for Expr {
    fn pos(&self) -> Position {
        match self {
            Expr::BinaryOperator(e) => { e.pos() }
            Expr::UnaryOperator(e) => { e.pos() }
            Expr::NumberLiteral(e) => { e.pos() }
            Expr::ArrayInitializer(e) => { e.pos() }
            Expr::StructureInitializer(e) => { e.pos() }
            Expr::StringLiteral(e) => { e.pos() }
            Expr::Identifier(e) => { e.pos() }
            Expr::CallExpression(e) => { e.pos() }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expressions(Box<Expr>),
    VariableDefinition(Variable),
    Return(Box<Expr>),
    If(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Void(Position),
}

impl ASTNode for Statement {
    fn pos(&self) -> Position {
        match self {
            Statement::Expressions(e) => { e.pos() }
            Statement::VariableDefinition(e) => { e.pos() }
            Statement::Return(e) => { e.pos() }
            Statement::If(e) => { e.pos() }
            Statement::ForLoop(e) => { e.pos() }
            Statement::WhileLoop(e) => { e.pos() }
            Statement::Void(p) => { p.clone() }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Ast {
    pub globals_variables: Vec<Variable>,
    pub globals_structures: Vec<Structure>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub extern_functions: Vec<FunctionPrototype>,
}


impl Ast {
    pub fn from_lexer(tokens: LELexer) -> Result<Self> {
        let mut ast = Self { globals_variables: vec![], globals_structures: vec![], function_definitions: vec![], extern_functions: vec![] };
        ast.parse(tokens)?;
        Ok(ast)
    }

    fn parse(&mut self, mut lexer: LELexer) -> Result<()> {
        loop {
            let next_token = lexer.current();
            match next_token {
                None => { break; }
                Some(token) => {
                    if let LEToken::KeyWord(keyword) = token {
                        match keyword {
                            KeyWord::Declare => {
                                lexer.consume_keyword()?;
                                let function_prototype = parse_extern_function_prototype(&mut lexer)?;
                                lexer.consume_semicolon()?;
                                self.extern_functions.push(function_prototype);
                            }
                            KeyWord::FunctionDefine => {
                                let function = parse_function(&mut lexer)?;
                                self.function_definitions.push(function);
                            }
                            KeyWord::VariableDeclare => {
                                let variable = parse_variable_declaration(&mut lexer)?;
                                self.globals_variables.push(variable);
                            }
                            KeyWord::StructureDeclare => {
                                let structure = parse_structure(&mut lexer)?;
                                self.globals_structures.push(structure);
                            }
                            _ => {
                                return Err(LEError::new_syntax_error(
                                    SyntaxError::unexpect_token(vec![TokenType::FunctionDeclare], LEToken::KeyWord(keyword.clone())),
                                    lexer.pos()));
                            }
                        }
                    } else {
                        return Err(LEError::new_syntax_error(
                            SyntaxError::unexpect_token(vec![TokenType::FunctionDeclare], token.clone()),
                            lexer.pos()));
                    }
                }
            }
        }
        Ok(())
    }
}
