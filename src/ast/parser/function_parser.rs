use anyhow::Result;

use crate::ast::{Annotation, FParseResult, FunctionNode, parse_code_block, Statement};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken, Number};


pub fn parse_variable_annotation(lexer: &mut LELexer) -> Result<Annotation> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let type_name = lexer.consume_identifier()?;
    Ok(Annotation { identifier, type_name })
}

// pub fn parse_param_list(lexer: &mut LELexer) -> Result<Vec<Annotation>> {}


pub fn parse_function_params(lexer: &mut LELexer) -> Result<Vec<Annotation>> {
    lexer.consume_left_little_brace()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightLittleBrace => {
                lexer.next_result()?;
                return Ok(params);
            }
            LEToken::Identifier(_) => {
                params.push(parse_variable_annotation(lexer)?);
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                return Err(SyntaxError::unexpect_token(TokenType::RightLittleBrace, current_token, lexer.line()).into());
            }
        }
    }
}


pub fn parse_function(lexer: &mut LELexer) -> FParseResult {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let params = parse_function_params(lexer)?;
    lexer.consume_return_type_allow()?;
    let return_type = lexer.consume_identifier()?;
    let code_block = parse_code_block(lexer)?;
    let function = FunctionNode {
        name,
        params,
        return_type,
        code_block,
    };
    Ok(function)
}
