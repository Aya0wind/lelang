use crate::ast::nodes::WhileLoop;
use crate::ast::parser::common::{parse_code_block, parse_expression};
use crate::error::Result;
use crate::lexer::LELexer;

pub fn parse_while_loop(lexer: &mut LELexer) -> Result<WhileLoop> {
    let start_pos = lexer.pos();
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
        pos: start_pos.sum(&lexer.pos()),
    })
}