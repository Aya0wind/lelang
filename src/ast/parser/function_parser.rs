use crate::ast::nodes::{FunctionDefinition, FunctionPrototype, TypeDeclarator};
use crate::ast::parser::array::parse_array_declarator;
use crate::ast::parser::common::parse_code_block;
use crate::ast::parser::parse_annotation;
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::ast::ParseResult;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken};

pub fn parse_function_params(lexer: &mut LELexer) -> ParseResult<Vec<(String, TypeDeclarator)>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightPar => {
                lexer.next_result()?;
                return Ok(params);
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                params.push(parse_annotation(lexer)?);
            }
        }
    }
}

pub fn parse_type_list(lexer: &mut LELexer) -> ParseResult<Vec<TypeDeclarator>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.own_current()?;
        match current_token {
            LEToken::RightPar => {
                lexer.next_result()?;
                return Ok(params);
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                params.push(parse_type_declarator(lexer)?);
            }
        }
    }
}


pub fn parse_function_prototype(lexer: &mut LELexer) -> ParseResult<FunctionPrototype> {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let param_types = parse_type_list(lexer)?;
    let return_type = parse_function_return_type(lexer)?;
    Ok(FunctionPrototype {
        name,
        param_types,
        return_type,
        pos: lexer.pos(),
    })
}

pub fn parse_function_return_type(lexer: &mut LELexer) -> ParseResult<Option<TypeDeclarator>> {
    if let LEToken::SingleArrow = lexer.current_result()? {
        lexer.next_result()?;
        Ok(Some(parse_type_declarator(lexer)?))
    } else {
        Ok(None)
    }
}


pub fn parse_function(lexer: &mut LELexer) -> ParseResult<FunctionDefinition> {
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
        prototype: FunctionPrototype {
            name,
            param_types,
            return_type,
            pos: lexer.pos(),
        },
        param_names,
        code_block,
        pos: lexer.pos(),
    };
    Ok(function)
}
