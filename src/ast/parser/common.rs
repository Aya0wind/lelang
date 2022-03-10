use anyhow::Result;

use crate::ast::{BExpr, BinaryOperatorNode, CodeBlock, IdentifierNode, NumberLiteralNode, Param, parse_statement, VParseResult};
use crate::ast::Expr::{BinaryOperator, Identifier, NumberLiteral};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LELexer, LEToken};

pub fn parse_call_param_list(lexer: &mut LELexer) -> Result<Vec<Param>> {
    lexer.consume_left_little_brace()?;
    let mut params = vec![];
    loop {
        let value = lexer.next_result()?;
        if let LEToken::Identifier(identifier) = value {
            params.push(Param::Identifier(identifier));
        } else if let LEToken::NumberLiteral(number) = value {
            params.push(Param::Number(number));
        } else {
            return Err(SyntaxError::unexpect_token(TokenType::Identifier, value, lexer.line()).into());
        }
        let expect_comma = lexer.next_result()?;
        if expect_comma != LEToken::Comma {
            return if expect_comma == LEToken::RightLittleBrace {
                Ok(params)
            } else {
                Err(SyntaxError::unexpect_token(TokenType::RightLittleBrace, expect_comma, lexer.line()).into())
            };
        }
    }
}


pub fn parse_unary(lexer: &mut LELexer) -> VParseResult {
    unimplemented!()
}

pub fn parse_binary(lexer: &mut LELexer, lhs: BExpr) -> VParseResult {
    unimplemented!()
}


pub fn parse_call_expression(lexer: &mut LELexer, function_name: &str) -> VParseResult {
    unimplemented!()
}

pub fn parse_identifier_expression(lexer: &mut LELexer) -> VParseResult {
    let identifier = lexer.consume_identifier()?;
    if let LEToken::Operator(op) = lexer.current_result()? {
        let op = op.clone();
        lexer.next_result()?;
        Ok(Box::new(BinaryOperator(BinaryOperatorNode {
            op,
            left: Box::new(Identifier(IdentifierNode { name: identifier })),
            right: parse_expression(lexer)?,
        })))
    } else {
        Ok(Box::new(Identifier(IdentifierNode { name: identifier })))
    }
}

pub fn parse_number_expression(lexer: &mut LELexer) -> VParseResult {
    let number = lexer.consume_number_literal()?;
    if let LEToken::Operator(op) = lexer.current_result()? {
        let op = op.clone();
        lexer.next_result()?;
        Ok(Box::new(BinaryOperator(BinaryOperatorNode {
            op,
            left: Box::new(NumberLiteral(NumberLiteralNode::new(number))),
            right: parse_expression(lexer)?,
        })))
    } else {
        Ok(Box::new(NumberLiteral(NumberLiteralNode::new(number))))
    }
}


pub fn parse_expression(lexer: &mut LELexer) -> VParseResult {
    let next = lexer.own_current()?;
    match next {
        LEToken::NumberLiteral(_) => {
            parse_number_expression(lexer)
        }
        LEToken::Identifier(_) => {
            parse_identifier_expression(lexer)
        }
        LEToken::LeftLittleBrace => { parse_little_brace_expression(lexer) }
        _ => { Err(SyntaxError::missing_expression(lexer.line()).into()) }
    }
}

pub fn parse_little_brace_expression(lexer: &mut LELexer) -> VParseResult {
    lexer.consume_left_little_brace()?;
    let primary_expression = parse_expression(lexer)?;
    lexer.consume_right_little_brace()?;
    Ok(primary_expression)
}


pub fn parse_code_block(lexer: &mut LELexer) -> Result<CodeBlock> {
    lexer.consume_left_big_brace()?;
    let mut block = CodeBlock { statements: vec![] };
    while let Ok(statement) = parse_statement(lexer) {
        block.statements.push(statement);
    }
    lexer.consume_right_big_brace()?;
    Ok(block)
}
