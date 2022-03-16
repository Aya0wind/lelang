use crate::lexer::LELexer;
use anyhow::Result;
use crate::ast::{ForLoop, parse_code_block, parse_expression, parse_statement, parse_variable_declaration};



pub fn parse_for_loop(lexer: &mut LELexer) -> Result<ForLoop> {
    lexer.consume_keyword()?;
    lexer.consume_left_little_brace()?;
    let initial = parse_variable_declaration(lexer)?;
    let cond = parse_expression(lexer)?;
    lexer.consume_semicolon()?;
    let step = parse_statement(lexer)?;
    lexer.consume_right_little_brace()?;
    let code_block = parse_code_block(lexer)?;
    Ok(ForLoop {
        init_statement: initial,
        condition: cond,
        iterate: Box::new(step),
        code_block,
    })
}