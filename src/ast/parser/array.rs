use crate::ast::nodes::{ArrayDeclarator, ArrayInitializer, Expr, FunctionCall, Position};
use crate::ast::parser::{parse_call_expression, parse_expression};
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::ast::ParseResult;
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken, Number};

pub fn parse_array_initializer(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    lexer.consume_left_bracket()?;
    let mut elements = vec![];
    loop {
        let current_token = lexer.current_result()?;
        match current_token {
            LEToken::RightBracket => {
                lexer.next_result()?;
                break;
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                elements.push(*parse_expression(lexer)?);
            }
        }
    }
    Ok(Box::new(Expr::ArrayInitializer(ArrayInitializer { elements, pos: lexer.pos() })))
}


pub fn parse_array_declarator(lexer: &mut LELexer) -> ParseResult<ArrayDeclarator> {
    lexer.consume_left_bracket()?;
    let element_type = parse_type_declarator(lexer)?;
    lexer.consume_semicolon()?;
    let len = lexer.consume_number_literal()?;
    if let Number::Integer(len) = len {
        lexer.consume_right_bracket()?;
        Ok(ArrayDeclarator {
            element_type,
            len: len as u32,
            pos: lexer.pos(),
        })
    } else {
        Err(SyntaxError::UnexpectToken { expect: TokenType::NumberLiteral, found: LEToken::NumberLiteral(len) })
    }
}