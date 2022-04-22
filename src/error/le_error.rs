#![allow(unused)]

use std::fmt::{Display, format, Formatter, Write};
use std::ops::Range;

use ariadne::{CharSet, Color, Config, Fmt, Label, Report, ReportBuilder, ReportKind, Span};
use nom::combinator::map;
use thiserror::Error;

use crate::error::TokenType::Colon;
use crate::lexer::{LELexer, LEToken, Operator, Position};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    If,
    For,
    While,
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
    SingleArrow,
    Identifier,
    NumberLiteral,
    StringLiteral,
}

impl TokenType {
    pub fn to_token_str(&self) -> &'static str {
        match self {
            TokenType::If => { "if" }
            TokenType::For => { "for" }
            TokenType::While => { "while" }
            TokenType::Else => { "el" }
            TokenType::FunctionDeclare => { "decl" }
            TokenType::VariableDeclare => { "var" }
            TokenType::Return => { "ret" }
            TokenType::Colon => { ":" }
            TokenType::Semicolon => { ";" }
            TokenType::LeftPar => { "(" }
            TokenType::RightPar => { ")" }
            TokenType::LeftBracket => { "[" }
            TokenType::RightBracket => { "]" }
            TokenType::RightBrace => { "}" }
            TokenType::LeftBrace => { "{" }
            TokenType::Comma => { "," }
            TokenType::Operator => { "Operator" }
            TokenType::SingleArrow => { "->" }
            TokenType::Identifier => { "Identifier" }
            TokenType::NumberLiteral => { "Number" }
            TokenType::StringLiteral => { "String" }
        }
    }
}


impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct TokenTypes {
    collection: Vec<TokenType>,
}

impl Display for TokenTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut types = self.collection.iter();
        if let Some(ty) = types.next() {
            f.write_fmt(format_args!("`{}`", ty.to_token_str()))?;
            for ty in types {
                f.write_str(" or ")?;
                f.write_fmt(format_args!("`{}`", ty.to_token_str()))?;
            }
        }
        Ok(())
    }
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("unexpected token: `{found}`, expect: `{expect}`.")]
    UnexpectToken {
        expect: TokenTypes,
        found: LEToken,
    },
    #[error("missing token : `{expect}`.")]
    MissingToken {
        expect: TokenTypes,
    },
    #[error("array size must be an integer.")]
    ArraySizeMustBeInteger,
}

impl SyntaxError {
    pub fn to_leerror(self, pos: Position) -> LEError {
        LEError::SyntaxError { syntax_error: self, position: pos }
    }
}

impl SyntaxError {
    pub fn missing_token(expect: Vec<TokenType>) -> Self {
        SyntaxError::MissingToken {
            expect: TokenTypes { collection: expect },
        }
    }

    pub fn unexpect_token(expect: Vec<TokenType>, found: LEToken) -> Self {
        SyntaxError::UnexpectToken {
            expect: TokenTypes { collection: expect },
            found,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("can not find symbol `{identifier}`")]
    UnknownIdentifier {
        identifier: String,
    },

    #[error("expect a type name, but identifier `{identifier}` is not a type")]
    IdentifierIsNotType {
        identifier: String,
    },

    #[error("expect a call able expression, but expression `{identifier}` is not a function")]
    IdentifierIsNotCallable {
        identifier: String,
    },

    #[error("expect a left value expression, but expression is not")]
    ExpressionIsNotLeftValueExpression,

    #[error("expect a right value expression, but expression is not")]
    ExpressionIsNotRightValueExpression,

    #[error("identifier `{identifier}` is already defined.")]
    IdentifierAlreadyDefined {
        identifier: String,
        defined_position: Position,
    },

    #[error("identifier `{identifier}` is builtin keyword or function, but redefined here")]
    CanNotRedefineBuiltinTypes {
        identifier: String,
    },

    #[error("no suitable binary operator `{op}` for type: `{left_type}` and `{right_type}`")]
    NoSuitableBinaryOperator {
        op: Operator,
        left_type: String,
        right_type: String,
    },

    #[error("no suitable unary operator `{op}` for type: `{target_type}`")]
    NoSuitableUnaryOperator {
        op: Operator,
        target_type: String,
    },

    #[error("expect a type `{expect}`, but got `{found}`")]
    TypeMismatched {
        expect: String,
        found: String,
    },

    #[error("type have no member which called :`{member_name}`")]
    NoSuchMember {
        member_name: String,
    },

    #[error("not allowed zero length array")]
    NotAllowZeroLengthArray,
}

impl CompileError {
    pub fn to_leerror(self, pos: Position) -> LEError {
        LEError::CompileError { compile_error: self, position: pos }
    }
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum LEError {
    #[error("[pos:{position}]SyntaxError:{syntax_error}")]
    SyntaxError {
        syntax_error: SyntaxError,
        position: Position,
    },
    #[error("[pos:{position}]CompileError:{compile_error}")]
    CompileError {
        compile_error: CompileError,
        position: Position,
    },
}


impl LEError {
    pub fn new_syntax_error(error: SyntaxError, position: Position) -> Self {
        Self::SyntaxError { syntax_error: error, position }
    }
    pub fn new_compile_error(error: CompileError, position: Position) -> Self {
        Self::CompileError { compile_error: error, position }
    }

    pub fn to_error_report_colored<'s>(&self, src: &'s str) -> Report<(&'s str, Range<usize>)> {
        let code_color = Color::White;
        let help_color = Color::Green;
        match self {
            LEError::SyntaxError { syntax_error, position } => {
                match syntax_error {
                    SyntaxError::UnexpectToken { expect, found } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0)
                            .with_message(syntax_error.to_string().fg(code_color))
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("Got a {} here, but expect {}.", found.to_token_str().fg(Color::Red), expect.to_string().fg(Color::Red)))
                            )
                            .with_help(format!("Considering change it or add a {}", format!("{expect}").fg(help_color)))
                            .finish()
                    }
                    SyntaxError::MissingToken { expect } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(1)
                            .with_message(syntax_error.to_string().fg(code_color))
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("Missing a {} here", expect.to_string().fg(Color::Red)))
                            )
                            .with_help("Considering finish that".to_string())
                            .finish()
                    }
                    SyntaxError::ArraySizeMustBeInteger {} => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message("Can only use signed integer as a array length".to_string())
                            .with_label(Label::new((src, position.range.clone())))
                            .with_help(format!("Considering change it to a {}", "sign integer".to_string().fg(Color::Red)))
                            .finish()
                    }
                }
            }
            LEError::CompileError { compile_error, position } => {
                match compile_error {
                    CompileError::UnknownIdentifier { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("can not find a symbol named: `{}` ", identifier.fg(Color::Red)))
                            )
                            .finish()
                    }
                    CompileError::IdentifierIsNotType { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` is not a type", identifier.fg(Color::Red)))
                            )
                            .finish()
                    }
                    CompileError::IdentifierIsNotCallable { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` is not a function or any callable object", identifier.fg(Color::Red)))
                            )
                            .finish()
                    }
                    CompileError::ExpressionIsNotLeftValueExpression => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("expression is not assignable".to_string())
                            )
                            .finish()
                    }
                    CompileError::ExpressionIsNotRightValueExpression => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("expression have a void type, but access it".to_string())
                            )
                            .finish()
                    }
                    CompileError::IdentifierAlreadyDefined { identifier, defined_position } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("identifier defined here".fg(Color::Blue))
                                    .with_color(Color::Green)
                            )
                            .with_label(
                                Label::new((src, defined_position.range.clone()))
                                    .with_message(format!("but identifier `{}` already defined here", identifier.fg(Color::Red)))
                                    .with_color(Color::Green)
                            )
                            .with_help("considering change the identifier")
                            .finish()
                    }
                    CompileError::NoSuitableBinaryOperator { op, left_type, right_type } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("operator `{op}` not suitable for type `{}` and type `{}` here", left_type.fg(Color::Blue), right_type.fg(Color::Red)))
                            )
                            .with_note(format!("no such operator between type `{}` and type `{}`", left_type.fg(Color::Blue), right_type.fg(Color::Red)))
                            .finish()
                    }
                    CompileError::NoSuitableUnaryOperator { op, target_type } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("operator `{op}` not suitable for type `{}` here", target_type.fg(Color::Red)))
                            )
                            .with_note(format!("no such operator for type `{}`", target_type.fg(Color::Red)))
                            .finish()
                    }
                    CompileError::TypeMismatched { expect, found } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("expect type `{expect}`, but found type `{found}`"))
                            )
                            .with_help(format!("maybe you need a type cast to type `{}` ?`", expect.fg(Color::Green)))
                            .finish()
                    }
                    CompileError::NoSuchMember { member_name } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("member `{}` access at here", member_name.fg(Color::Red)))
                            )
                            .with_help(format!("maybe you want to create a member named `{}` ?`", member_name.fg(Color::Green)))
                            .finish()
                    }
                    CompileError::NotAllowZeroLengthArray => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("length defined here".to_string())
                            )
                            .with_help("change the array length to integer")
                            .finish()
                    }
                    CompileError::CanNotRedefineBuiltinTypes { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(0000)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` defined here", identifier.fg(Color::Green)))
                            )
                            .with_help("maybe you can change the identifier to no keyword or builtin identifier")
                            .finish()
                    }
                }
            }
        }
    }
}
