use crate::ast::nodes::{Expr, Structure, StructureInitializer};
use crate::ast::parser::{parse_annotation, parse_expression, parse_function_params, parse_type_declarator};
use crate::error::{LEError, Result};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken, Position};

pub fn parse_structure(lexer: &mut LELexer) -> Result<Structure> {
    let start_pos = lexer.pos();
    lexer.consume_keyword()?;
    let structure_name = lexer.consume_identifier()?;
    lexer.consume_left_brace()?;
    let mut members = vec![];
    loop {
        let current_token = lexer.current()
            .ok_or_else(|| LEError::new_syntax_error(
                SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Identifier, TokenType::Comma]),
                lexer.pos()))?;
        match current_token {
            LEToken::RightBrace => {
                lexer.consume();
                break;
            }
            LEToken::Identifier(_) => {
                members.push(parse_annotation(lexer)?);
            }
            LEToken::Comma => {
                lexer.consume();
            }
            _ => {
                return Err(LEError::new_syntax_error(
                    SyntaxError::unexpect_token(vec![TokenType::RightPar, TokenType::Identifier, TokenType::Comma], current_token),
                    lexer.pos()));
            }
        }
    }
    Ok(Structure { name: structure_name, members, pos: start_pos.sum(&lexer.pos()) })
}


fn parse_member_initializer(lexer: &mut LELexer) -> Result<(String, Box<Expr>)> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let initial_value = parse_expression(lexer)?;
    Ok((identifier, initial_value))
}


pub fn parse_structure_initializer(lexer: &mut LELexer) -> Result<(Vec<(String, Box<Expr>)>, Position)> {
    lexer.consume_left_brace()?;
    let mut members = vec![];
    loop {
        let current_token = lexer.current()
            .ok_or_else(|| LEError::new_syntax_error(
                SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Identifier, TokenType::Comma]),
                lexer.pos()))?;
        match current_token {
            LEToken::RightBrace => {
                lexer.consume();
                break;
            }
            LEToken::Identifier(_) => {
                members.push(parse_member_initializer(lexer)?);
            }
            LEToken::Comma => {
                lexer.consume();
            }
            _ => {
                return Err(LEError::new_syntax_error(
                    SyntaxError::unexpect_token(vec![TokenType::RightPar, TokenType::Identifier, TokenType::Comma], current_token.clone()),
                    lexer.pos()));
            }
        }
    }
    Ok((members, lexer.pos()))
}