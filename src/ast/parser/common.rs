use anyhow::Result;

use crate::ast::nodes::{BinaryOpExpression, CodeBlock, Expr, FunctionCall, FunctionDefinition, Identifier, NumberLiteral, Position, Structure, StructureInitializer, TypeDeclarator, UnaryOpExpression};
use crate::ast::parser::array::parse_array_initializer;
use crate::ast::parser::parse_structure_initializer;
use crate::ast::parser::statement::parse_statement;
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::ast::ParseResult;
use crate::error::SyntaxError;
use crate::lexer::{LELexer, LEToken, Operator};

fn get_operator_precedence(op: &Operator) -> usize {
    match op {
        Operator::Plus => { 20 }
        Operator::Sub => { 20 }
        Operator::Mul => { 40 }
        Operator::Mod => { 40 }
        Operator::Div => { 40 }
        Operator::Assign => { 10 }
        Operator::Equal => { 10 }
        Operator::NotEqual => { 10 }
        Operator::GreaterThan => { 10 }
        Operator::LessThan => { 10 }
        Operator::GreaterOrEqualThan => { 10 }
        Operator::LessOrEqualThan => { 10 }
        Operator::Dot => { 60 }
        Operator::And => { 5 }
        Operator::Or => { 5 }
        Operator::Xor => { 5 }
        Operator::Not => { 5 }
        Operator::Rev => { 5 }
    }
}


pub fn parse_annotation(lexer: &mut LELexer) -> ParseResult<(String, TypeDeclarator)> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let type_declarator = parse_type_declarator(lexer)?;
    Ok((identifier, type_declarator))
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
        LEToken::LeftBrace => {
            let initializer = parse_structure_initializer(lexer)?;
            Ok(Box::new(Expr::StructureInitializer(StructureInitializer { structure_name: identifier, member_initial_values: initializer })))
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
        op: Operator::Sub,
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
        LEToken::LeftBracket => {
            parse_array_initializer(lexer)
        }
        LEToken::LeftPar => { parse_little_par_expression(lexer) }
        _ => { Err(SyntaxError::MissingExpression {}) }
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



