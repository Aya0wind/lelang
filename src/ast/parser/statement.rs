use std::fmt::Debug;

use anyhow::Result;

use crate::ast::{BExpr, parse_expression, parse_little_brace_expression, parse_variable_declaration, VariableNode};
use crate::ast::parser::condition::{IfCondition, parse_if_condition};
use crate::ast::parser::for_loop::ForLoop;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};

#[derive(Debug)]
pub enum Statement {
    Expressions(BExpr),
    VariableDeclare(VariableNode),
    Return(BExpr),
    If(IfCondition),
    ForLoop(ForLoop),
}


pub fn parse_statement(lexer: &mut LELexer) -> Result<Statement> {
    let next_token = lexer.current_result()?;
    let res: Result<Statement, anyhow::Error> = match next_token {
        LEToken::KeyWord(keyword) => {
            match keyword {
                KeyWord::VariableDeclare => Ok(Statement::VariableDeclare(parse_variable_declaration(lexer)?)),
                KeyWord::Return => {
                    lexer.consume_keyword()?;
                    Ok(Statement::Return(parse_expression(lexer)?))
                }
                KeyWord::If => Ok(Statement::If(parse_if_condition(lexer)?)),
                _ => { Err(SyntaxError::unexpect_token(TokenType::Identifier, lexer.current_result()?.clone(), lexer.line()).into()) }
            }
        }
        LEToken::Identifier(_) => {
            Ok(Statement::Expressions(parse_expression(lexer)?))
        }
        LEToken::LeftLittleBrace => { Ok(Statement::Expressions(parse_little_brace_expression(lexer)?)) }
        _ => { Err(SyntaxError::unexpect_token(TokenType::Identifier, lexer.current_result()?.clone(), lexer.line()).into()) }
    };
    let statement = res?;
    match statement {
        Statement::If(_) | Statement::ForLoop(_) => {}
        _ => { lexer.consume_semicolon()?; }
    }
    Ok(statement)
}