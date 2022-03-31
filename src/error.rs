#![allow(unused)]

use std::fmt::{Display, Formatter};

use thiserror::Error;

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
    #[error("[line {line}]Unexpected token: `{found}`, expect: `{expect}`")]
    UnexpectToken {
        expect: TokenType,
        found: LEToken,
        line: usize,
    },
    #[error("[line {line}] Missing token:`{expect}`")]
    MissingToken {
        expect: TokenType,
        line: usize,
    },
    #[error("[line {line}] Missing expression.")]
    MissingExpression {
        line: usize,
    },
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
    #[error("expect a function name, but identifier `{identifier}` is not a variable")]
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

impl TokenParserError {
    pub fn unrecognized_token() -> Self {
        Self::UnrecognizedToken
    }
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
    pub fn unexpect_token(expect: TokenType, found: LEToken, line: usize) -> Self {
        Self::UnexpectToken { expect, found, line }
    }
    pub fn missing_expression(line: usize) -> Self { Self::MissingExpression { line } }
    pub fn missing_token(expect: TokenType, line: usize) -> Self {
        Self::MissingToken { expect, line }
    }
    pub fn missing_if(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::If, line }
    }
    pub fn missing_else(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Else, line }
    }
    pub fn missing_function_declare(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::FunctionDeclare, line }
    }
    pub fn missing_variable_declare(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::VariableDeclare, line }
    }
    pub fn missing_return(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Return, line }
    }
    pub fn missing_colon(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Colon, line }
    }
    pub fn missing_left_little_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::LeftPar, line }
    }
    pub fn missing_right_little_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::RightPar, line }
    }
    pub fn missing_left_middle_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::LeftBracket, line }
    }
    pub fn missing_right_middle_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::RightBracket, line }
    }
    pub fn missing_left_big_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::LeftBrace, line }
    }
    pub fn missing_right_big_brace(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::RightBrace, line }
    }
    pub fn missing_comma(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Comma, line }
    }
    pub fn missing_operator(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Operator, line }
    }
    pub fn missing_return_type_allow(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::ReturnTypeAllow, line }
    }
    pub fn missing_identifier(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::Identifier, line }
    }
    pub fn missing_number_literal(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::NumberLiteral, line }
    }
    pub fn missing_string_literal(line: usize) -> Self {
        Self::MissingToken { expect: TokenType::StringLiteral, line }
    }
}


#[derive(Debug, Error)]
#[allow(unused)]
enum JITCompileError {}

