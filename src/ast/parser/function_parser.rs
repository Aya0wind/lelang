use anyhow::Result;

use crate::ast::nodes::{ExternalFunction, FunctionDefinition};
use crate::ast::parser::common::{FParseResult, parse_code_block};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken};

pub fn parse_variable_annotation(lexer: &mut LELexer) -> Result<(String, String)> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let type_name = lexer.consume_identifier()?;
    Ok((identifier, type_name))
}


pub fn parse_function_params(lexer: &mut LELexer) -> Result<Vec<(String, String)>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightPar => {
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
                return Err(SyntaxError::unexpect_token(TokenType::RightPar, current_token, lexer.line().into()).into());
            }
        }
    }
}

pub fn parse_type_list(lexer: &mut LELexer) -> Result<Vec<String>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightPar => {
                lexer.next_result()?;
                return Ok(params);
            }
            LEToken::Identifier(ident) => {
                params.push(ident);
                lexer.next_result()?;
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                return Err(SyntaxError::unexpect_token(TokenType::RightPar, current_token, lexer.line().into()).into());
            }
        }
    }
}


pub fn parse_function_prototype(lexer: &mut LELexer) -> Result<ExternalFunction> {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let param_types = parse_type_list(lexer)?;
    let return_type = parse_function_return_type(lexer)?;
    Ok(ExternalFunction {
        name,
        param_types,
        return_type,
        pos: lexer.line().into(),
    })
}

pub fn parse_function_return_type(lexer: &mut LELexer) -> Result<Option<String>> {
    if let LEToken::ReturnTypeAllow = lexer.current_result()? {
        lexer.next_result()?;
        Ok(Some(lexer.consume_identifier()?))
    } else {
        Ok(None)
    }
}


pub fn parse_function(lexer: &mut LELexer) -> FParseResult {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let params = parse_function_params(lexer)?;
    let return_type = parse_function_return_type(lexer)?;
    let code_block = parse_code_block(lexer)?;
    let mut param_names = Vec::with_capacity(params.len());
    let mut param_types = Vec::with_capacity(params.len());
    params.into_iter().for_each(|anno| {
        param_types.push(anno.1);
        param_names.push(anno.0);
    });
    let function = FunctionDefinition {
        prototype: ExternalFunction {
            name,
            param_types,
            return_type,
            pos: lexer.line().into(),
        },
        param_names,
        code_block,
        pos: lexer.line().into(),
    };
    Ok(function)
}
