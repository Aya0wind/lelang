#![allow(unused)]

use std::error::Error;
use std::fmt::{Display, format, Formatter, Write};
use std::ops::Range;

use ariadne::{CharSet, Color, Config, Fmt, Label, Report, ReportBuilder, ReportKind, Span};
use nom::combinator::map;
use thiserror::Error;

use crate::error::TokenType::Colon;
use crate::lexer::{LELexer, LEToken, Operator, Position};

use super::error_list;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    If,
    For,
    While,
    Else,
    FunctionDeclare,
    FunctionDefine,
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
            TokenType::FunctionDefine => { "le" }
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
            f.write_fmt(format_args!("{}", ty.to_token_str()))?;
            for ty in types {
                f.write_str(" or ")?;
                f.write_fmt(format_args!("{}", ty.to_token_str()))?;
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
    #[error("can not find identifier `{identifier}` in this scope")]
    UnknownIdentifier {
        identifier: String,
    },

    #[error("expect a type name, but identifier `{identifier}` is not a type")]
    IdentifierIsNotType {
        identifier: String,
    },

    #[error("expect a type identifier, but found a expression")]
    ExpressionIsNotType {
        pos: Position,
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

    #[error("invalid type cast from `{from}` to `{to}`")]
    InvalidTypeCast {
        from: String,
        to: String,
    },
}

impl CompileError {
    pub fn to_leerror(self, pos: Position) -> LEError {
        LEError::CompileError { compile_error: self, position: pos }
    }
}

#[derive(Debug, Error)]
pub enum LEError {
    #[error("[{position}] SyntaxError:{syntax_error}")]
    SyntaxError {
        syntax_error: SyntaxError,
        position: Position,
    },
    #[error("[{position}] CompileError:{compile_error}")]
    CompileError {
        compile_error: CompileError,
        position: Position,
    },
    #[error("error:{other}")]
    IOError {
        other: Box<dyn Error>
    }
}


impl LEError {
    pub fn new_syntax_error(error: SyntaxError, position: Position) -> Self {
        Self::SyntaxError { syntax_error: error, position }
    }
    pub fn new_compile_error(error: CompileError, position: Position) -> Self {
        Self::CompileError { compile_error: error, position }
    }

    pub fn to_error_report_colored<'s>(&self, src: &'s str) -> ReportBuilder<(&'s str, Range<usize>)> {
        let code_color = Color::White;
        let label_color = Color::Green;
        let help_color = Color::Green;
        let mut loop_rainbow_color = [
            Color::Red,
            Color::Blue,
            Color::Yellow,
            Color::Green,
            Color::Cyan,
            Color::Magenta,
        ].into_iter().cycle();
        match self {
            LEError::SyntaxError { syntax_error, position } => {
                match syntax_error {
                    SyntaxError::UnexpectToken { expect, found } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::UNEXPECT_TOKEN)
                            .with_message(syntax_error.to_string().fg(code_color))
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("Got a token `{}` here, but expect `{}`.", found.fg(loop_rainbow_color.next().unwrap()), expect.to_string().fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help(format!("Considering add `{}`", expect.fg(help_color)))
                    }
                    SyntaxError::MissingToken { expect } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::MISSING_TOKEN)
                            .with_message(syntax_error.to_string().fg(code_color))
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("Missing a `{}` here", expect.to_string().fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help("Considering finish that")
                    }
                    SyntaxError::ArraySizeMustBeInteger {} => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::ARRAY_SIZE_MUST_BE_INTEGER)
                            .with_message("Can only use signed integer as a array length".to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_color(label_color)
                            )
                            .with_help(format!("Considering change it to a `{}`", "sign integer".fg(Color::Green)))
                    }
                }
            }
            LEError::CompileError { compile_error, position } => {
                match compile_error {
                    CompileError::UnknownIdentifier { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::UNKNOWN_IDENTIFIER)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("can not find this identifier `{}`", identifier.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                    }
                    CompileError::IdentifierIsNotType { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::IDENTIFIER_IS_NOT_TYPE)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` is not a type", identifier.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                    }
                    CompileError::IdentifierIsNotCallable { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::IDENTIFIER_IS_NOT_CALLABLE)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` is not a function or any callable object", identifier.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                    }
                    CompileError::ExpressionIsNotLeftValueExpression => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::EXPRESSION_IS_NOT_LEFT_VALUE_EXPRESSION)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("expression is not assignable".to_string())
                                    .with_color(label_color)
                            )
                    }
                    CompileError::ExpressionIsNotRightValueExpression => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::EXPRESSION_IS_NOT_RIGHT_VALUE_EXPRESSION)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("expression have a void type, but access it".to_string())
                                    .with_color(label_color)
                            )
                    }
                    CompileError::IdentifierAlreadyDefined { identifier, defined_position } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::IDENTIFIER_ALREADY_DEFINED)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("identifier defined here".fg(Color::Blue))
                                    .with_color(label_color)
                            )
                            .with_label(
                                Label::new((src, defined_position.range.clone()))
                                    .with_message(format!("but identifier `{}` already defined here", identifier.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help("considering change the identifier")
                    }
                    CompileError::NoSuitableBinaryOperator { op, left_type, right_type } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::NO_SUITABLE_BINARY_OPERATOR)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("operator `{}` not suitable for type `{}` and type `{}` here", op.fg(loop_rainbow_color.next().unwrap()), left_type.fg(loop_rainbow_color.next().unwrap()), right_type.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help(format!("maybe you need a `{}` type cast here？", "as".fg(Color::Green)))
                    }
                    CompileError::NoSuitableUnaryOperator { op, target_type } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::NO_SUITABLE_UNARY_OPERATOR)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("operator `{}` not suitable for type `{}` here", op.fg(loop_rainbow_color.next().unwrap()), target_type.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                    }
                    CompileError::TypeMismatched { expect, found } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::TYPE_MISMATCHED)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("expect type `{}`, but found type `{}`", expect.fg(loop_rainbow_color.next().unwrap()), found.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help(format!("maybe you need a type cast to type `{}` ?`", expect.fg(Color::Green)))
                    }
                    CompileError::NoSuchMember { member_name } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::NO_SUCH_MEMBER)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("member `{}` access at here", member_name.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help(format!("maybe you want to create a member named `{}` ?`", member_name.fg(Color::Green)))
                    }
                    CompileError::NotAllowZeroLengthArray => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::NOT_ALLOW_ZERO_LENGTH_ARRAY)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message("length defined here".to_string())
                                    .with_color(label_color)
                            )
                            .with_help("change the array length to integer")
                    }
                    CompileError::CanNotRedefineBuiltinTypes { identifier } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::CAN_NOT_REDEFINE_BUILTIN_TYPES)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("identifier `{}` defined here", identifier.fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help("maybe you can change the identifier to another, which is not keyword or builtin identifier")
                    }
                    CompileError::ExpressionIsNotType { pos } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::EXPRESSION_IS_NOT_TYPE)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, pos.range.clone()))
                                    .with_message(format!("expression here is not a `{}`", "type".fg(loop_rainbow_color.next().unwrap())))
                                    .with_color(label_color)
                            )
                            .with_help("maybe you can change the identifier to no keyword or builtin identifier")
                    }
                    CompileError::InvalidTypeCast { from, to } => {
                        Report::build(ReportKind::Error, src, position.range.start())
                            .with_code(error_list::INVALID_TYPE_CAST)
                            .with_message(compile_error.to_string())
                            .with_label(
                                Label::new((src, position.range.clone()))
                                    .with_message(format!("type `{}` value cannot cast to type `{}`",
                                                          from.fg(loop_rainbow_color.next().unwrap()),
                                                          to.fg(loop_rainbow_color.next().unwrap()))
                                    )
                                    .with_color(label_color)
                            )
                    }
                }
            }
            LEError::IOError { other } => {
                Report::build(ReportKind::Error, src, 0)
                    .with_message(other.to_string())
            }
        }
    }
}
