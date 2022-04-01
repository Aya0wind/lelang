#![allow(unused)]

use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::ast::Position;
use crate::lexer::{LELexer, LEToken};

#[derive(Debug, Error)]
#[allow(unused)]
pub enum TokenParserError {
    #[error("Error:Got unrecognized token")]
    UnrecognizedToken
}


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
    #[error("[ERROR]line({pos}): Unexpected token: `{found}`, expect: `{expect}`")]
    UnexpectToken {
        expect: TokenType,
        found: LEToken,
        pos: Position,
    },
    #[error("[ERROR]line({pos}): Missing token:`{expect}`")]
    MissingToken {
        expect: TokenType,
        pos: Position,
    },
    #[error("[ERROR]line({pos}): Missing expression.")]
    MissingExpression {
        pos: Position,
    },
    #[error("End of file")]
    EOF,
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("[ERROR]line({pos}):unknown identifier:{identifier}")]
    UnknownIdentifier {
        identifier: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):expect a type name, but identifier `{identifier}` is not a type")]
    IdentifierIsNotType {
        identifier: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):expect a variable, but identifier `{identifier}` is not a variable")]
    IdentifierIsNotVariable {
        identifier: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):expect a function name, but identifier `{identifier}` is not a function")]
    IdentifierIsNotFunction {
        identifier: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):identifier `{identifier}` is already defined, which is a `{symbol_name}`")]
    IdentifierAlreadyDefined {
        identifier: String,
        symbol_name: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):expect a variable, but identifier `{identifier}` is not a variable")]
    CanOnlyAssignVariable {
        identifier: String,
        pos: Position,
    },
    #[error("[ERROR]line({pos}):expect a type `{expect}`, but got `{found}`")]
    TypeMismatched {
        expect: String,
        found: String,
        pos: Position,
    },

}

impl TokenParserError {
    pub fn unrecognized_token() -> Self {
        Self::UnrecognizedToken
    }
}

impl CompileError {
    pub fn identifier_is_not_type(identifier: String, pos: Position) -> Self {
        Self::IdentifierIsNotType { identifier, pos }
    }
    pub fn can_only_assign_variable(identifier: String, pos: Position) -> Self {
        Self::CanOnlyAssignVariable { identifier, pos }
    }
    pub fn unknown_identifier(identifier: String, pos: Position) -> Self {
        Self::UnknownIdentifier { identifier, pos }
    }
    pub fn identifier_is_not_variable(identifier: String, pos: Position) -> Self {
        Self::IdentifierIsNotVariable { identifier, pos }
    }
    pub fn identifier_is_not_function(identifier: String, pos: Position) -> Self {
        Self::IdentifierIsNotFunction { identifier, pos }
    }

    pub fn identifier_already_defined(identifier: String, symbol_name: String, pos: Position) -> Self {
        Self::IdentifierIsNotFunction { identifier, pos }
    }

    pub fn type_mismatched(expect: String, found: String, pos: Position) -> Self {
        Self::TypeMismatched { expect, found, pos }
    }
}

impl SyntaxError {
    pub fn unexpect_token(expect: TokenType, found: LEToken, pos: Position) -> Self {
        Self::UnexpectToken { expect, found, pos }
    }
    pub fn missing_expression(pos: Position) -> Self { Self::MissingExpression { pos } }
    pub fn missing_token(expect: TokenType, pos: Position) -> Self {
        Self::MissingToken { expect, pos }
    }
    pub fn missing_if(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::If, pos }
    }
    pub fn missing_else(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Else, pos }
    }
    pub fn missing_function_declare(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::FunctionDeclare, pos }
    }
    pub fn missing_variable_declare(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::VariableDeclare, pos }
    }
    pub fn missing_return(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Return, pos }
    }
    pub fn missing_colon(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Colon, pos }
    }
    pub fn missing_left_little_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::LeftPar, pos }
    }
    pub fn missing_right_little_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::RightPar, pos }
    }
    pub fn missing_left_middle_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::LeftBracket, pos }
    }
    pub fn missing_right_middle_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::RightBracket, pos }
    }
    pub fn missing_left_big_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::LeftBrace, pos }
    }
    pub fn missing_right_big_brace(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::RightBrace, pos }
    }
    pub fn missing_comma(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Comma, pos }
    }
    pub fn missing_operator(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Operator, pos }
    }
    pub fn missing_return_type_allow(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::ReturnTypeAllow, pos }
    }
    pub fn missing_identifier(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::Identifier, pos }
    }
    pub fn missing_number_literal(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::NumberLiteral, pos }
    }
    pub fn missing_string_literal(pos: Position) -> Self {
        Self::MissingToken { expect: TokenType::StringLiteral, pos }
    }
}


#[derive(Debug, Error)]
#[allow(unused)]
enum JITCompileError {}

