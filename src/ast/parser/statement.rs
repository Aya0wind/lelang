use anyhow::Result;
use crate::ast::{BExpr, ForLoop, parse_primary_expression, parse_little_brace_expression, parse_variable_declaration, Statement, VariableNode, parse_expression, parse_identifier_expression};
use crate::ast::parser::condition::parse_if_condition;
use crate::ast::parser::for_loop::parse_for_loop;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};


pub fn parse_statement(lexer: &mut LELexer) -> Result<Statement> {
    let next_token = lexer.current_result()?;
    let res: Result<Statement, anyhow::Error> = match next_token {
        LEToken::KeyWord(keyword) => {
            match keyword {
                KeyWord::Return => {
                    lexer.consume_keyword()?;
                    Ok(Statement::Return(parse_expression(lexer)?))
                }
                KeyWord::If => Ok(Statement::If(parse_if_condition(lexer)?)),
                KeyWord::For => Ok(Statement::ForLoop(parse_for_loop(lexer)?)),
                _ => { Err(SyntaxError::unexpect_token(TokenType::Identifier, lexer.current_result()?.clone(), lexer.line()).into()) }
            }
        }
        _ => { Ok(Statement::Expressions(parse_expression(lexer)?)) }
    };
    let statement = res?;
    match statement {
        Statement::If(_) | Statement::ForLoop(_) => {}
        _ => { lexer.consume_semicolon()?; }
    }
    Ok(statement)
}


