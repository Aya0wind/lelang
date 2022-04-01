use anyhow::Result;

use crate::ast::{parse_expression, parse_variable_annotation, VariableNode, VariablePrototype};
use crate::error::SyntaxError;
use crate::error::TokenType;
use crate::lexer::{LELexer, LEToken, Operator};

pub fn parse_variable_declaration(lexer: &mut LELexer) -> Result<VariableNode> {
    lexer.consume_keyword()?;
    let (name, type_name) = parse_variable_annotation(lexer)?;
    let equal_op = lexer.consume_operator()?;
    if Operator::Assign == equal_op {
        let initial_value = parse_expression(lexer)?;
        Ok(VariableNode {
            prototype: VariablePrototype {
                type_name,
                name,
                pos: lexer.line().into()
            },
            value: initial_value,
            pos: lexer.line().into()
        })
    } else {
        Err(SyntaxError::unexpect_token(TokenType::Operator, LEToken::Operator(equal_op), lexer.line().into()).into())
    }
}