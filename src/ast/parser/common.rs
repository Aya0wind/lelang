use anyhow::Result;

use crate::ast::nodes::{BinaryOpExpression, CodeBlock, Expr, FunctionCall, FunctionDefinition, Identifier, NumberLiteral, Position, UnaryOpExpression};
use crate::ast::parser::statement::parse_statement;
use crate::ast::ParseResult;
use crate::error::SyntaxError;
use crate::lexer::{BinaryOperator, LELexer, LEToken, UnaryOperator};

fn get_operator_precedence(op: &BinaryOperator) -> usize {
    match op {
        BinaryOperator::Plus => { 20 }
        BinaryOperator::Sub => { 20 }
        BinaryOperator::Mul => { 40 }
        BinaryOperator::Div => { 40 }
        BinaryOperator::Assign => { 10 }
        BinaryOperator::Equal => { 10 }
        BinaryOperator::GreaterThan => { 10 }
        BinaryOperator::LessThan => { 10 }
        BinaryOperator::GreaterOrEqualThan => { 10 }
        BinaryOperator::LessOrEqualThan => { 10 }
    }
}


pub fn parse_call_expression(lexer: &mut LELexer, function_name: String) -> ParseResult<Box<Expr>> {
    lexer.next_result()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.current_result()?;
        match current_token {
            LEToken::RightPar => {
                lexer.next_result()?;
                return Ok(Box::new(Expr::CallExpression(FunctionCall {
                    function_name,
                    params,
                    pos: lexer.pos(),
                })));
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                params.push(*parse_expression(lexer)?);
            }
        }
    }
}


pub fn parse_binary_ops(lexer: &mut LELexer, mut lhs: Box<Expr>, expression_precedence: usize) -> ParseResult<Box<Expr>> {
    loop {
        if let LEToken::Operator(op) = lexer.own_current()? {
            let precedence = get_operator_precedence(&op);
            if precedence < expression_precedence {
                return Ok(lhs);
            }
            lexer.next_result()?;
            let mut rhs = parse_primary_expression(lexer)?;
            rhs = parse_binary_ops(lexer, rhs, precedence + 1)?;
            lhs = Box::new(Expr::BinaryOperator(BinaryOpExpression {
                op,
                left: lhs,
                right: rhs,
                pos: lexer.pos(),
            }))
        } else {
            return Ok(lhs);
        }
    }
}

pub fn parse_identifier_expression(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    let identifier = lexer.consume_identifier()?;
    match lexer.current_result()? {
        LEToken::LeftPar => {
            Ok(parse_call_expression(lexer, identifier)?)
        }
        _ => {
            Ok(Box::new(Expr::Identifier(Identifier { name: identifier, pos: lexer.pos() })))
        }
    }
}

pub fn parse_number_expression(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    let number = lexer.consume_number_literal()?;
    Ok(Box::new(Expr::NumberLiteral(NumberLiteral { number, pos: lexer.pos() })))
}

pub fn parse_little_par_expression(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    lexer.consume_left_par()?;
    let expression = parse_expression(lexer)?;
    lexer.consume_right_par()?;
    Ok(expression)
}

pub fn parse_unary_ops(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    let op = lexer.consume_binary_operator()?;
    Ok(Box::new(Expr::UnaryOperator(UnaryOpExpression {
        op: UnaryOperator::Sub,
        expr: parse_primary_expression(lexer)?,
        pos: lexer.pos(),
    })))
}

pub fn parse_primary_expression(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    let next = lexer.own_current()?;
    match next {
        LEToken::Operator(op) => {
            parse_unary_ops(lexer)
        }
        LEToken::NumberLiteral(_) => {
            parse_number_expression(lexer)
        }
        LEToken::Identifier(_) => {
            parse_identifier_expression(lexer)
        }
        LEToken::LeftPar => { parse_little_par_expression(lexer) }
        _ => { Err(SyntaxError::missing_expression()) }
    }
}

pub fn parse_expression(lexer: &mut LELexer) -> ParseResult<Box<Expr>> {
    let primary = parse_primary_expression(lexer)?;
    parse_binary_ops(lexer, primary, 0)
}

pub fn parse_code_block(lexer: &mut LELexer) -> ParseResult<CodeBlock> {
    lexer.consume_left_brace()?;
    let mut block = CodeBlock { statements: vec![], pos: lexer.pos() };
    while let Ok(current) = lexer.current_result() {
        if current == &LEToken::RightBrace {
            break;
        }
        block.statements.push(parse_statement(lexer)?);
    }
    lexer.consume_right_brace()?;
    Ok(block)
}



