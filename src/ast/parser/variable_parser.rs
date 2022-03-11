use anyhow::Result;

use crate::ast::{parse_expression, parse_primary_expression, parse_variable_annotation, VariableNode};
use crate::error::SyntaxError;
use crate::error::TokenType;
use crate::lexer::{LELexer, LEToken, Operator};

pub fn parse_variable_declaration(lexer: &mut LELexer) -> Result<VariableNode> {
    lexer.consume_keyword()?;
    let anno = parse_variable_annotation(lexer)?;
    let equal_op = lexer.consume_operator()?;
    if Operator::Assign == equal_op {
        let initial_value = parse_expression(lexer)?;
        Ok(VariableNode { type_name: anno.type_name, name: anno.identifier, value: initial_value })
    } else {
        Err(SyntaxError::unexpect_token(TokenType::Operator, LEToken::Operator(equal_op), lexer.line()).into())
    }
}