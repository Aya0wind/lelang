use crate::ast::nodes::Statement;
use crate::ast::parser::common::parse_expression;
use crate::ast::parser::for_loop::parse_for_loop;
use crate::ast::parser::if_statement::parse_if_statement;
use crate::ast::parser::variable_parser::parse_variable_declaration;
use crate::ast::parser::while_loop::parse_while_loop;
use crate::error::{LEError, Result};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken};

pub fn parse_statement(lexer: &mut LELexer) -> Result<Statement> {
    let next_token = lexer.current()
        .ok_or_else(|| LEError::new_syntax_error(
            SyntaxError::missing_token(vec![TokenType::Return, TokenType::VariableDeclare, TokenType::If, TokenType::For, TokenType::While]),
            lexer.pos()))?;
    match next_token {
        LEToken::KeyWord(ref keyword) => {
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
                KeyWord::If => Ok(Statement::If(parse_if_statement(lexer)?)),
                KeyWord::For => Ok(Statement::ForLoop(parse_for_loop(lexer)?)),
                KeyWord::While => Ok(Statement::WhileLoop(parse_while_loop(lexer)?)),
                _ => {
                    Err(LEError::new_syntax_error(
                        SyntaxError::unexpect_token(
                            vec![TokenType::Return, TokenType::VariableDeclare, TokenType::If, TokenType::For, TokenType::While], next_token.clone())
                        , lexer.pos()))
                }
            }
        }
        LEToken::Semicolon => {
            lexer.consume_semicolon()?;
            Ok(Statement::Void(lexer.pos()))
        }
        _ => {
            let expr = parse_expression(lexer)?;
            lexer.consume_semicolon()?;
            Ok(Statement::Expressions(expr))
        }
    }
}


