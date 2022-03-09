use std::fmt::{Debug};
use crate::ast::{BExpr, parse_expression, parse_little_brace_expression, parse_variable_declaration, VariableNode};
use crate::lexer::{KeyWord, LELexer, LEToken};
use anyhow::Result;
use crate::error::{SyntaxError, TokenType};

#[derive(Debug)]
pub enum  Statement{
    Expressions(BExpr),
    VariableDeclare(VariableNode),
    Return(BExpr),
    If(BExpr),
}


pub fn parse_if_statement(lexer: &mut LELexer) -> Result<Statement>{
    unimplemented!()
}


pub fn parse_statement(lexer: &mut LELexer) -> Result<Statement>{
    let next_token = lexer.next_result()?;
    let res:Result<Statement,anyhow::Error> = match next_token {
        LEToken::KeyWord(keyword) => {
            match keyword{
                KeyWord::VariableDeclare => Ok(Statement::VariableDeclare(parse_variable_declaration(lexer)?)),
                KeyWord::Return => Ok(Statement::Return(parse_expression(lexer)?)),
                _=>{Err(SyntaxError::unexpect_token(TokenType::Identifier,lexer.current().clone(),lexer.line()).into())}
            }
        }
        LEToken::Identifier(identifier) => {
            return Ok(Statement::Expressions(parse_expression(lexer)?))
        }
        LEToken::LeftLittleBrace => {return Ok(Statement::Expressions(parse_little_brace_expression(lexer)?))}
        _=>{Err(SyntaxError::unexpect_token(TokenType::Identifier,lexer.current().clone(),lexer.line()).into())}
    };
    eprintln!("{:#?}",res);
    if let LEToken::Semicolon=lexer.current(){
        res
    }else{
        Err(SyntaxError::unexpect_token(TokenType::Semicolon,lexer.current().clone(),lexer.line()).into())
    }
}