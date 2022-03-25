use anyhow::Result;

use crate::ast::{ForLoop, parse_code_block, parse_expression, parse_statement, parse_variable_declaration, Statement};
use crate::lexer::LELexer;

pub fn parse_for_loop(lexer: &mut LELexer) -> Result<ForLoop> {
    lexer.consume_keyword()?;
    lexer.consume_left_par()?;
    let initial = parse_statement(lexer)?;
    let cond = parse_statement(lexer)?;
    let step = parse_statement(lexer)?;
    lexer.consume_right_par()?;
    let code_block = parse_code_block(lexer)?;
    Ok(ForLoop {
        init_statement: Box::from(initial),
        condition: Box::from(cond),
        iterate: Box::new(step),
        code_block,
    })
}