use std::env::VarError::NotPresent;

use anyhow::Result;

use crate::ast::nodes::{Variable, VariablePrototype};
use crate::ast::parser::{parse_annotation, parse_type_declarator};
use crate::ast::parser::common::parse_expression;
use crate::ast::ParseResult;
use crate::error::SyntaxError;
use crate::error::TokenType;
use crate::lexer::{LELexer, LEToken, Operator};

pub fn parse_variable_declaration(lexer: &mut LELexer) -> ParseResult<Variable> {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let type_declarator = if let LEToken::Colon = lexer.current_result()? {
        lexer.consume_colon()?;
        Some(parse_type_declarator(lexer)?)
    } else {
        None
    };
    let equal_op = lexer.consume_binary_operator()?;
    if Operator::Assign == equal_op {
        let initial_value = parse_expression(lexer)?;
        Ok(Variable {
            prototype: VariablePrototype {
                type_declarator,
                name,
                pos: lexer.pos(),
            },
            value: initial_value,
            pos: lexer.pos(),
        })
    } else {
        Err(SyntaxError::UnexpectToken { expect: TokenType::Operator, found: LEToken::Operator(equal_op) })
    }
}