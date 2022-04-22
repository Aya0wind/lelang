use crate::ast::nodes::IfStatement;
use crate::ast::parser::common::{parse_code_block, parse_little_par_expression};
use crate::error::Result;
use crate::error::SyntaxError;
use crate::lexer::{KeyWord, LELexer, LEToken};

pub fn parse_if_statement(lexer: &mut LELexer) -> Result<IfStatement> {
    let start_pos = lexer.pos();
    lexer.consume();
    let cond_value = parse_little_par_expression(lexer)?;
    let then_block = parse_code_block(lexer)?;
    let current = lexer.current();
    if let Some(LEToken::KeyWord(KeyWord::Else)) = current {
        lexer.consume();
        let else_block = parse_code_block(lexer)?;
        Ok(IfStatement {
            cond: cond_value,
            then_block,
            else_block: Some(else_block),
            pos: start_pos.sum(&lexer.pos()),
        })
    } else {
        Ok(IfStatement {
            cond: cond_value,
            then_block,
            else_block: None,
            pos: start_pos.sum(&lexer.pos()),
        })
    }
}

