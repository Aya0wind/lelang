use anyhow::Result;

use crate::ast::{BExpr, BinaryOpExpression, CodeBlock, Expr, FunctionCall, FunctionDefinition, IdentifierNode, NumberLiteralNode, Param, parse_statement, parse_variable_annotation, parse_variable_declaration};
use crate::ast::Expr::{BinaryOperator, Identifier, NumberLiteral};
use crate::error::{SyntaxError, TokenParserError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken, Operator};

pub fn get_operator_precedence(op: &Operator) -> usize {
    match op {
        Operator::Plus => { 20 }
        Operator::Sub => { 20 }
        Operator::Mul => { 40 }
        Operator::Div => { 40 }
        Operator::Assign => { 10 }
        Operator::Equal => { 10 }
        Operator::GreaterThan => { 10 }
        Operator::LessThan => { 10 }
        Operator::GreaterOrEqualThan => { 10 }
        Operator::LessOrEqualThan => { 10 }
    }
}


pub type VParseResult = Result<BExpr>;
pub type FParseResult = Result<FunctionDefinition>;


pub fn parse_call_expression(lexer: &mut LELexer, function_name: String) -> Result<BExpr> {
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
                })));
            }
            LEToken::Comma => {
                lexer.next_result()?;
            }
            _ => {
                params.push(parse_expression(lexer)?);
            }
        }
    }
}


pub fn parse_binary_ops(lexer: &mut LELexer, mut lhs: BExpr, expression_precedence: usize) -> VParseResult {
    loop {
        if let LEToken::Operator(op) = lexer.own_current()? {
            let precedence = get_operator_precedence(&op);
            if precedence < expression_precedence {
                return Ok(lhs);
            }
            lexer.next_result()?;
            let mut rhs = parse_primary_expression(lexer)?;
            rhs = parse_binary_ops(lexer, rhs, precedence + 1)?;
            lhs = Box::new(BinaryOperator(BinaryOpExpression {
                op,
                left: lhs,
                right: rhs,
            }))
        } else {
            return Ok(lhs);
        }
    }
}

pub fn parse_identifier_expression(lexer: &mut LELexer) -> VParseResult {
    let identifier = lexer.consume_identifier()?;
    match lexer.current_result()? {
        LEToken::LeftPar => {
            Ok(parse_call_expression(lexer, identifier)?)
        }
        _ => {
            Ok(Box::new(Identifier(IdentifierNode { name: identifier })))
        }
    }
}

pub fn parse_number_expression(lexer: &mut LELexer) -> VParseResult {
    let number = lexer.consume_number_literal()?;
    Ok(Box::new(NumberLiteral(NumberLiteralNode { number })))
}

pub fn parse_little_par_expression(lexer: &mut LELexer) -> VParseResult {
    lexer.consume_left_par()?;
    let expression = parse_expression(lexer)?;
    lexer.consume_right_par()?;
    Ok(expression)
}

pub fn parse_primary_expression(lexer: &mut LELexer) -> VParseResult {
    let next = lexer.own_current()?;
    match next {
        LEToken::NumberLiteral(_) => {
            parse_number_expression(lexer)
        }
        LEToken::Identifier(_) => {
            parse_identifier_expression(lexer)
        }
        LEToken::LeftPar => { parse_little_par_expression(lexer) }
        _ => { Err(SyntaxError::missing_expression(lexer.line()).into()) }
    }
}

pub fn parse_expression(lexer: &mut LELexer) -> VParseResult {
    let primary = parse_primary_expression(lexer)?;
    parse_binary_ops(lexer, primary, 0)
}

pub fn parse_code_block(lexer: &mut LELexer) -> Result<CodeBlock> {
    lexer.consume_left_brace()?;
    let mut block = CodeBlock { statements: vec![] };
    while let Ok(current) = lexer.current_result() {
        if current == &LEToken::RightBrace {
            break;
        }
        block.statements.push(parse_statement(lexer)?);
    }
    lexer.consume_right_brace()?;
    Ok(block)
}



