use anyhow::Result;

use crate::ast::Statement;
use crate::ast::parser::common::parse_expression;
use crate::ast::parser::condition::parse_if_condition;
use crate::ast::parser::for_loop::parse_for_loop;
use crate::ast::parser::variable_parser::parse_variable_declaration;
use crate::ast::parser::while_loop::parse_while_loop;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};

pub fn parse_statement(lexer: &mut LELexer) -> Result<Statement> {
    let next_token = lexer.current_result()?;
    match next_token {
        LEToken::KeyWord(keyword) => {
            match keyword {
                KeyWord::Return => {
                    lexer.consume_keyword()?;
                    let return_expression = parse_expression(lexer)?;
                    lexer.consume_semicolon()?;
                    Ok(Statement::Return(return_expression))
                }
                KeyWord::VariableDeclare => {
                    let variable_node = parse_variable_declaration(lexer)?;
                    lexer.consume_semicolon()?;
                    Ok(Statement::VariableDefinition(variable_node))
                }
                KeyWord::If => Ok(Statement::If(parse_if_condition(lexer)?)),
                KeyWord::For => Ok(Statement::ForLoop(parse_for_loop(lexer)?)),
                KeyWord::While => Ok(Statement::WhileLoop(parse_while_loop(lexer)?)),
                _ => { Err(SyntaxError::unexpect_token(TokenType::Identifier, lexer.current_result()?.clone(), lexer.line().into()).into()) }
            }
        }
        LEToken::Semicolon => {
            lexer.consume_semicolon()?;
            Ok(Statement::Void)
        }
        _ => {
            let expr = parse_expression(lexer)?;
            lexer.consume_semicolon()?;
            Ok(Statement::Expressions(expr))
        }
    }
}


