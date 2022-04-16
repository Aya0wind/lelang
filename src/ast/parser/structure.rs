use crate::ast::nodes::{Expr, Position, Structure, StructureInitializer};
use crate::ast::parser::{parse_annotation, parse_expression, parse_function_params, parse_type_declarator};
use crate::ast::ParseResult;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken};

pub fn parse_structure(lexer: &mut LELexer) -> ParseResult<Structure> {
    lexer.consume_keyword()?;
    let structure_name = lexer.consume_identifier()?;
    lexer.consume_left_brace()?;
    let mut members = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightBrace => {
                lexer.next_result()?;
                break;
            }
            LEToken::Identifier(_) => {
                members.push(parse_annotation(lexer)?);
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                return Err(SyntaxError::UnexpectToken { expect: TokenType::RightBrace, found: current_token });
            }
        }
    }
    Ok(Structure { name: structure_name, members, pos: lexer.pos() })
}


fn parse_member_initializer(lexer: &mut LELexer) -> ParseResult<(String, Box<Expr>)> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let initial_value = parse_expression(lexer)?;
    Ok((identifier, initial_value))
}


pub fn parse_structure_initializer(lexer: &mut LELexer) -> ParseResult<Vec<(String, Box<Expr>)>> {
    lexer.consume_left_brace()?;
    let mut members = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightBrace => {
                lexer.next_result()?;
                break;
            }
            LEToken::Identifier(_) => {
                members.push(parse_member_initializer(lexer)?);
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                return Err(SyntaxError::UnexpectToken { expect: TokenType::RightBrace, found: current_token });
            }
        }
    }
    Ok(members)
}