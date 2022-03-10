use anyhow::Result;

use crate::ast::{BExpr, CodeBlock, parse_code_block, parse_little_brace_expression};
use crate::lexer::{KeyWord, LELexer, LEToken};

#[derive(Debug)]
pub struct IfCondition {
    cond: BExpr,
    then_block: CodeBlock,
    else_block: Option<CodeBlock>,
}

pub fn parse_if_condition(lexer: &mut LELexer) -> Result<IfCondition> {
    lexer.consume_keyword()?;
    let cond_value = parse_little_brace_expression(lexer)?;
    let then_block = parse_code_block(lexer)?;
    let current = lexer.current_result()?;
    if let &LEToken::KeyWord(KeyWord::Else) = current {
        lexer.next_result()?;
        let else_block = parse_code_block(lexer)?;
        Ok(IfCondition {
            cond: cond_value,
            then_block,
            else_block: Some(else_block),
        })
    } else {
        Ok(IfCondition {
            cond: cond_value,
            then_block,
            else_block: None,
        })
    }
}

