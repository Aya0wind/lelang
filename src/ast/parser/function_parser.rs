
use crate::ast::{FParseResult, FunctionNode, parse_code_block, Statement};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LEToken, Number, LELexer};

#[derive(Debug)]
pub struct Annotation{
    pub identifier: String,
    pub type_name: String,
}

#[derive(Debug)]
pub enum Param{
    Identifier(String),
    Number(Number),
}

#[derive(Debug)]
pub struct CodeBlock{
    pub(crate) statements:Vec<Statement>
}

pub fn parse_variable_annotation(lexer: &mut LELexer) -> anyhow::Result<Annotation> {
    let identifier = lexer.expect_identifier()?;
    lexer.expect_colon()?;
    let type_name = lexer.expect_identifier()?;
    Ok(Annotation { identifier, type_name })
}

pub fn parse_function_params(lexer: &mut LELexer) -> anyhow::Result<Vec<Annotation>> {
    lexer.expect_left_little_brace()?;
    let mut params = vec![];
    loop {
        params.push(parse_variable_annotation(lexer)?);
        let expect_comma = lexer.next_result()?;
        if expect_comma != LEToken::Comma {
            return if expect_comma == LEToken::RightLittleBrace {
                Ok(params)
            } else {
                Err(SyntaxError::unexpect_token(TokenType::RightLittleBrace, expect_comma, lexer.line()).into())
            };
        }
    }
}

pub fn parse_function(lexer: &mut LELexer) -> FParseResult {
    let name = lexer.expect_identifier()?;
    let params = parse_function_params(lexer)?;
    let return_type = {
        let expect_return_type = lexer.next_result()?;
        if let LEToken::ReturnTypeAllow = expect_return_type {
            lexer.expect_identifier()?
        }else{
            lexer.expect_left_big_brace()?;
            "void".into()
        }
    };
    let code_block = parse_code_block(lexer)?;
    let function  = FunctionNode {
        name,
        params,
        return_type,
        code_block,
    };
    Ok(function)
}
