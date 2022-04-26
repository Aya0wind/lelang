use crate::ast::nodes::{ArrayDeclarator, ArrayInitializer, Expr, FunctionCall};
use crate::ast::parser::{parse_call_expression, parse_expression};
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::error::{LEError, Result};
use crate::error::{SyntaxError, TokenType};
use crate::error::TokenType::Identifier;
use crate::lexer::{LELexer, LEToken, Number};

pub fn parse_array_initializer(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let start_pos = lexer.pos();
    lexer.consume_left_bracket()?;
    let mut elements = vec![];
    loop {
        let current_token = lexer.current().ok_or_else(|| LEError::new_syntax_error(
            SyntaxError::missing_token(vec![TokenType::RightBracket]),
            lexer.pos(),
        ))?;
        match current_token {
            LEToken::RightBracket => {
                lexer.consume();
                break;
            }
            LEToken::Comma => {
                lexer.consume();
            }
            // LEToken::Semicolon=>{
            //     lexer.consume();
            //     let number = lexer.consume_number_literal()?;
            //     lexer.consume_right_bracket()?;
            //     let elements =  match number {
            //         Number::Integer(i) => {vec![elements[0]; i as usize]}
            //         Number::Float(f) => {
            //            return Err(LEError::new_syntax_error(
            //                SyntaxError::ArraySizeMustBeInteger,
            //                lexer.pos(),
            //            ));
            //         }
            //     };
            //     return Ok(Box::new(Expr::ArrayInitializer(ArrayInitializer { elements, pos: start_pos.sum(&lexer.pos()) })))
            // }
            _ => {
                elements.push(*parse_expression(lexer)?);
            }
        }
    }
    Ok(Box::new(Expr::ArrayInitializer(ArrayInitializer { elements, pos: start_pos.sum(&lexer.pos()) })))
}


pub fn parse_array_declarator(lexer: &mut LELexer) -> Result<ArrayDeclarator> {
    let start_pos = lexer.pos();
    lexer.consume_left_bracket()?;
    let element_type = parse_type_declarator(lexer)?;
    lexer.consume_semicolon()?;
    let len = lexer.consume_number_literal()?;
    if let Number::Integer(len) = len {
        lexer.consume_right_bracket()?;
        Ok(ArrayDeclarator {
            element_type,
            len: len as u32,
            pos: start_pos.sum(&lexer.pos()),
        })
    } else {
        Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::NumberLiteral, Identifier], LEToken::NumberLiteral(len)), lexer.pos()))
    }
}