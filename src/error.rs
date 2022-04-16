#![allow(unused)]

use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::ast::nodes::Position;
use crate::lexer::{LELexer, LEToken, Operator};

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
    SingleAllow,
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
    #[error("unexpected token: `{found}`, expect: `{expect}`")]
    UnexpectToken {
        expect: TokenType,
        found: LEToken,
    },
    #[error("Missing token:`{expect}`")]
    MissingToken {
        expect: TokenType,
    },
    #[error("missing expression.")]
    MissingExpression {},
    #[error("end of file")]
    EOF,
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("unknown identifier:{name}")]
    UnknownIdentifier {
        name: String,
    },

    #[error("expect a type name, but identifier `{name}` is not a type")]
    IdentifierIsNotType {
        name: String,
    },

    #[error("expect a call able expression, but expression `{name}` is not a function")]
    IdentifierIsNotCallable {
        name: String,
    },

    #[error("expect a left value expression, but expression is not")]
    ExpressionIsNotLeftValueExpression,

    #[error("expect a right value expression, but expression is not")]
    ExpressionIsNotRightValueExpression,

    #[error("identifier `{identifier}` is already defined, at: `{defined_position}`")]
    IdentifierAlreadyDefined {
        identifier: String,
        defined_position: Position,
    },

    #[error("no suitable binary operator `{op}` for type: `{}`")]
    NoSuitableBinaryOperator {
        op: Operator,
        left: String,
        right: String,
    },

    #[error("no suitable unary operator `{op}` for type: `{}`")]
    NoSuitableUnaryOperator {
        op: Operator,
        target: String,
    },

    #[error("type mismatched: expect a type `{expect}`, but got `{found}`")]
    TypeMismatched {
        expect: String,
        found: String,
    },

    #[error("type have no member which called :`{member_name}`")]
    NoMember {
        member_name: String,
    },

    #[error("not allowed zero length array")]
    NotAllowZeroLengthArray,

}


#[allow(unused)]
#[derive(Debug, Error)]
pub enum LEError {
    #[error("[line:{pos}]SyntaxError:{syntax_error}")]
    SyntaxError {
        syntax_error: SyntaxError,
        pos: Position,
    },
    #[error("[line:{pos}]CompileError:{compile_error}")]
    CompileError {
        compile_error: CompileError,
        pos: Position,
    },
}

impl LEError {
    pub fn new_syntax_error(error: SyntaxError, pos: Position) -> Self {
        Self::SyntaxError { syntax_error: error, pos }
    }
    pub fn new_compile_error(error: CompileError, pos: Position) -> Self {
        Self::CompileError { compile_error: error, pos }
    }
}
