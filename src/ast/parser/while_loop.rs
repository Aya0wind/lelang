use anyhow::Result;

use crate::ast::nodes::WhileLoop;
use crate::ast::parser::common::{parse_code_block, parse_expression};
use crate::ast::ParseResult;
use crate::lexer::LELexer;

pub fn parse_while_loop(lexer: &mut LELexer) -> ParseResult<WhileLoop> {
    lexer.consume_keyword()?;
    lexer.consume_left_par()?;
    let cond = if let Ok(expr) = parse_expression(lexer) {
        Some(expr)
    } else {
        None
    };
    lexer.consume_right_par()?;
    let code_block = parse_code_block(lexer)?;
    Ok(WhileLoop {
        condition: cond,
        code_block,
        pos: lexer.pos(),
    })
}