use crate::ast::nodes::{FunctionDefinition, FunctionPrototype, Identifier, TypeDeclarator};
use crate::ast::parser::array::parse_array_declarator;
use crate::ast::parser::common::parse_code_block;
use crate::ast::parser::parse_annotation;
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::error::{LEError, Result};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken, Position};

pub fn parse_function_params(lexer: &mut LELexer) -> Result<Vec<(String, TypeDeclarator)>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.current().ok_or(
            LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Comma]), lexer.pos())
        )?;
        match current_token {
            LEToken::RightPar => {
                lexer.consume();
                return Ok(params);
            }
            LEToken::Comma => {
                lexer.consume();
            }
            _ => {
                params.push(parse_annotation(lexer)?);
            }
        }
    }
}

pub fn parse_type_list(lexer: &mut LELexer) -> Result<Vec<TypeDeclarator>> {
    lexer.consume_left_par()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.current().ok_or(
            LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Comma]), lexer.pos())
        )?;
        match current_token {
            LEToken::RightPar => {
                lexer.consume();
                return Ok(params);
            }
            LEToken::Comma => {
                lexer.consume();
            }
            _ => {
                params.push(parse_type_declarator(lexer)?);
            }
        }
    }
}


pub fn parse_extern_function_prototype(lexer: &mut LELexer) -> Result<FunctionPrototype> {
    let start_pos = lexer.pos();
    lexer.consume_keyword()?;
    let identifier_pos = lexer.pos();
    let identifier = Identifier { name: lexer.consume_identifier()?, pos: identifier_pos };
    let param_types = parse_type_list(lexer)?;
    let return_type = parse_function_return_type(lexer)?;
    Ok(FunctionPrototype {
        identifier,
        param_types,
        return_type,
        pos: start_pos.sum(&lexer.pos()),
    })
}


pub fn parse_function_return_type(lexer: &mut LELexer) -> Result<Option<TypeDeclarator>> {
    let current_token = lexer.current().ok_or(
        LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Comma]), lexer.pos())
    )?;
    if let LEToken::SingleArrow = current_token {
        lexer.consume();
        Ok(Some(parse_type_declarator(lexer)?))
    } else {
        Ok(None)
    }
}


pub fn parse_function(lexer: &mut LELexer) -> Result<FunctionDefinition> {
    let start_pos = lexer.pos();
    lexer.consume_keyword()?;
    let identifier_pos = lexer.pos();
    let identifier = Identifier { name: lexer.consume_identifier()?, pos: identifier_pos };
    let params = parse_function_params(lexer)?;
    let return_type = parse_function_return_type(lexer)?;
    let proto_type_pos = start_pos.sum(&lexer.pos());
    let code_block = parse_code_block(lexer)?;
    let function_pos = start_pos.sum(&lexer.pos());
    let mut param_names = Vec::with_capacity(params.len());
    let mut param_types = Vec::with_capacity(params.len());
    params.into_iter().for_each(|anno| {
        param_types.push(anno.1);
        param_names.push(anno.0);
    });
    let function = FunctionDefinition {
        prototype: FunctionPrototype {
            identifier,
            param_types,
            return_type,
            pos: proto_type_pos,
        },
        param_names,
        code_block,
        pos: function_pos,
    };
    Ok(function)
}
