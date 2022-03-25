use anyhow::Result;

use crate::ast::{BExpr, CodeBlock, IfStatement, parse_code_block, parse_little_par_expression};
use crate::lexer::{KeyWord, LELexer, LEToken};

pub fn parse_if_condition(lexer: &mut LELexer) -> Result<IfStatement> {
    lexer.next_result()?;
    let cond_value = parse_little_par_expression(lexer)?;
    let then_block = parse_code_block(lexer)?;
    let current = lexer.current_result()?;
    if let &LEToken::KeyWord(KeyWord::Else) = current {
        lexer.next_result()?;
        let else_block = parse_code_block(lexer)?;
        Ok(IfStatement {
            cond: cond_value,
            then_block,
            else_block: Some(else_block),
        })
    } else {
        Ok(IfStatement {
            cond: cond_value,
            then_block,
            else_block: None,
        })
    }
}

