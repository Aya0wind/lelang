use std::env::VarError::NotPresent;

use crate::ast::nodes::{Identifier, Variable, VariablePrototype};
use crate::ast::parser::{parse_annotation, parse_type_declarator};
use crate::ast::parser::common::parse_expression;
use crate::error::{LEError, Result};
use crate::error::SyntaxError;
use crate::error::TokenType;
use crate::lexer::{LELexer, LEToken, Operator};

pub fn parse_variable_declaration(lexer: &mut LELexer) -> Result<Variable> {
    let start_pos = lexer.pos();
    lexer.consume_keyword()?;
    let prototype_start_pos = lexer.pos();
    let identifier = Identifier { name: lexer.consume_identifier()?, pos: prototype_start_pos.clone() };
    let expect_colon = lexer.current().ok_or(LEError::new_syntax_error(
        SyntaxError::missing_token(vec![TokenType::Colon]),
        lexer.pos()))?;
    let type_declarator = if let LEToken::Colon = expect_colon {
        lexer.consume_colon()?;
        Some(parse_type_declarator(lexer)?)
    } else {
        None
    };
    let prototype_end_pos = lexer.pos();
    let equal_op = lexer.consume_operator()?;
    if Operator::Assign == equal_op {
        let initial_value = parse_expression(lexer)?;
        Ok(Variable {
            prototype: VariablePrototype {
                type_declarator,
                identifier,
                pos: prototype_start_pos.sum(&prototype_end_pos),
            },
            value: initial_value,
            pos: start_pos.sum(&lexer.pos()),
        })
    } else {
        Err(LEError::new_syntax_error(
            SyntaxError::unexpect_token(vec![TokenType::Operator], LEToken::Operator(equal_op)),
            lexer.pos()))
    }
}