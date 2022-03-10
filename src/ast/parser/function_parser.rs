use crate::ast::{FParseResult, FunctionNode, parse_code_block, Statement};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken, Number};

#[derive(Debug)]
pub struct Annotation {
    pub identifier: String,
    pub type_name: String,
}

#[derive(Debug)]
pub enum Param {
    Identifier(String),
    Number(Number),
}

#[derive(Debug)]
pub struct CodeBlock {
    pub(crate) statements: Vec<Statement>,
}

pub fn parse_variable_annotation(lexer: &mut LELexer) -> anyhow::Result<Annotation> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let type_name = lexer.consume_identifier()?;
    Ok(Annotation { identifier, type_name })
}

pub fn parse_function_params(lexer: &mut LELexer) -> anyhow::Result<Vec<Annotation>> {
    lexer.consume_left_little_brace()?;
    let mut params = vec![];
    loop {
        params.push(parse_variable_annotation(lexer)?);
        let expect_comma = lexer.own_current()?;
        match lexer.consume_comma() {
            Ok(_) => {}
            Err(_) => {
                return if expect_comma == LEToken::RightLittleBrace {
                    Ok(params)
                } else {
                    Err(SyntaxError::unexpect_token(TokenType::RightLittleBrace, expect_comma, lexer.line()).into())
                };
            }
        }
    }
}


pub fn parse_function(lexer: &mut LELexer) -> FParseResult {
    lexer.consume_keyword()?;
    let name = lexer.consume_identifier()?;
    let params = parse_function_params(lexer)?;
    lexer.consume_return_type_allow()?;
    eprintln!("{:?}", lexer.current_result()?);
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
