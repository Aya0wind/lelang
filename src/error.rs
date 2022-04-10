#![allow(unused)]

use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::ast::nodes::Position;
use crate::lexer::{LELexer, LEToken};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    If,
    Else,
    FunctionDeclare,
    VariableDeclare,
    Return,
    Colon,
    Semicolon,
    LeftPar,
    RightPar,
    LeftBracket,
    RightBracket,
    RightBrace,
    LeftBrace,
    Comma,
    Operator,
    ReturnTypeAllow,
    Identifier,
    NumberLiteral,
    StringLiteral,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unexpected token: `{found}`, expect: `{expect}`")]
    UnexpectToken {
        expect: TokenType,
        found: LEToken,
    },
    #[error("Missing token:`{expect}`")]
    MissingToken {
        expect: TokenType,
    },
    #[error("Missing expression.")]
    MissingExpression {},
    #[error("End of file")]
    EOF,
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("unknown identifier:{identifier}")]
    UnknownIdentifier {
        identifier: String,
    },
    #[error("expect a type name, but identifier `{identifier}` is not a type")]
    IdentifierIsNotType {
        identifier: String,
    },
    #[error("expect a variable, but identifier `{identifier}` is not a variable")]
    IdentifierIsNotVariable {
        identifier: String,
    },
    #[error("expect a function name, but identifier `{identifier}` is not a function")]
    IdentifierIsNotFunction {
        identifier: String,
    },
    #[error("identifier `{identifier}` is already defined, which is a `{symbol_name}`")]
    IdentifierAlreadyDefined {
        identifier: String,
        symbol_name: String,
    },
    #[error("expect a variable, but identifier `{identifier}` is not a variable")]
    CanOnlyAssignVariable {
        identifier: String,
    },
    #[error("expect a type `{expect}`, but got `{found}`")]
    TypeMismatched {
        expect: String,
        found: String,
    },

}


impl CompileError {
    pub fn identifier_is_not_type(identifier: String) -> Self {
        Self::IdentifierIsNotType { identifier }
    }
    pub fn can_only_assign_variable(identifier: String) -> Self {
        Self::CanOnlyAssignVariable { identifier }
    }
    pub fn unknown_identifier(identifier: String) -> Self {
        Self::UnknownIdentifier { identifier }
    }
    pub fn identifier_is_not_variable(identifier: String) -> Self {
        Self::IdentifierIsNotVariable { identifier }
    }
    pub fn identifier_is_not_function(identifier: String) -> Self {
        Self::IdentifierIsNotFunction { identifier }
    }

    pub fn identifier_already_defined(identifier: String, symbol_name: String) -> Self {
        Self::IdentifierIsNotFunction { identifier }
    }

    pub fn type_mismatched(expect: String, found: String) -> Self {
        Self::TypeMismatched { expect, found }
    }
}

impl SyntaxError {
    pub fn unexpect_token(expect: TokenType, found: LEToken) -> Self {
        Self::UnexpectToken { expect, found }
    }
    pub fn missing_expression() -> Self {
        Self::MissingExpression {  }
    }
    pub fn missing_token(expect: TokenType) -> Self {
        Self::MissingToken { expect }
    }
    pub fn missing_if() -> Self {
        Self::MissingToken { expect: TokenType::If }
    }
    pub fn missing_else() -> Self {
        Self::MissingToken { expect: TokenType::Else }
    }
    pub fn missing_function_declare() -> Self {
        Self::MissingToken { expect: TokenType::FunctionDeclare }
    }
    pub fn missing_variable_declare() -> Self {
        Self::MissingToken { expect: TokenType::VariableDeclare }
    }
    pub fn missing_return() -> Self {
        Self::MissingToken { expect: TokenType::Return }
    }
    pub fn missing_colon() -> Self {
        Self::MissingToken { expect: TokenType::Colon }
    }
    pub fn missing_left_little_brace() -> Self {
        Self::MissingToken { expect: TokenType::LeftPar }
    }
    pub fn missing_right_little_brace() -> Self {
        Self::MissingToken { expect: TokenType::RightPar }
    }
    pub fn missing_left_middle_brace() -> Self {
        Self::MissingToken { expect: TokenType::LeftBracket }
    }
    pub fn missing_right_middle_brace() -> Self {
        Self::MissingToken { expect: TokenType::RightBracket }
    }
    pub fn missing_left_big_brace() -> Self {
        Self::MissingToken { expect: TokenType::LeftBrace }
    }
    pub fn missing_right_big_brace() -> Self {
        Self::MissingToken { expect: TokenType::RightBrace }
    }
    pub fn missing_comma() -> Self {
        Self::MissingToken { expect: TokenType::Comma }
    }
    pub fn missing_operator() -> Self {
        Self::MissingToken { expect: TokenType::Operator }
    }
    pub fn missing_return_type_allow() -> Self {
        Self::MissingToken { expect: TokenType::ReturnTypeAllow }
    }
    pub fn missing_identifier() -> Self {
        Self::MissingToken { expect: TokenType::Identifier }
    }
    pub fn missing_number_literal() -> Self {
        Self::MissingToken { expect: TokenType::NumberLiteral }
    }
    pub fn missing_string_literal() -> Self {
        Self::MissingToken { expect: TokenType::StringLiteral }
    }
}

#[allow(unused)]
#[derive(Debug,Error)]
pub enum LEError {
    #[error("[line:{pos}]SyntaxError:{syntax_error}")]
    SyntaxError{
        syntax_error:SyntaxError,
        pos:Position
    },
    #[error("[line:{pos}]CompileError:{compile_error}")]
    CompileError{
        compile_error:CompileError,
        pos:Position
    }
}

impl LEError {
    pub fn new_syntax_error(error:SyntaxError,pos:Position)->Self{
        Self::SyntaxError {syntax_error:error, pos }
    }
    pub fn new_compile_error(error:CompileError,pos:Position)->Self{
        Self::CompileError {compile_error:error, pos }
    }
}
