use anyhow::Result;

use crate::ast::nodes::{Variable, VariablePrototype};
use crate::ast::parser::common::parse_expression;
use crate::ast::parser::function_parser::parse_variable_annotation;
use crate::ast::ParseResult;
use crate::error::SyntaxError;
use crate::error::TokenType;
use crate::lexer::{BinaryOperator, LELexer, LEToken};

pub fn parse_variable_declaration(lexer: &mut LELexer) -> ParseResult<Variable> {
    lexer.consume_keyword()?;
    let (name, type_name) = parse_variable_annotation(lexer)?;
    let equal_op = lexer.consume_operator()?;
    if BinaryOperator::Assign == equal_op {
        let initial_value = parse_expression(lexer)?;
        Ok(Variable {
            prototype: VariablePrototype {
                type_name,
                name,
                pos: lexer.pos(),
            },
            value: initial_value,
            pos: lexer.pos(),
        })
    } else {
        Err(SyntaxError::unexpect_token(TokenType::Operator, LEToken::Operator(equal_op)))
    }
}