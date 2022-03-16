
use anyhow::Result;

use crate::ast::{BExpr, BinaryOperatorNode, CodeBlock, Expr, FunctionCallNode, get_operator_precedence, IdentifierNode, NumberLiteralNode, Param, parse_statement, parse_variable_annotation, parse_variable_declaration, VParseResult};
use crate::ast::Expr::{BinaryOperator, Identifier, NumberLiteral};
use crate::error::{SyntaxError, TokenParserError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken, Operator};

pub fn parse_call_expression(lexer: &mut LELexer, function_name: String) -> Result<BExpr> {
    lexer.next_result()?;
    let mut params = vec![];
    loop {
        let current_token = lexer.current_result()?;
        match current_token {
            LEToken::RightLittleBrace => {
                lexer.next_result()?;
                return Ok(Box::new(Expr::CallExpression(FunctionCallNode {
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
            lhs = Box::new(BinaryOperator(BinaryOperatorNode {
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
        LEToken::LeftLittleBrace => {
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

pub fn parse_little_brace_expression(lexer: &mut LELexer) -> VParseResult {
    lexer.consume_left_little_brace()?;
    let expression = parse_expression(lexer)?;
    lexer.consume_right_little_brace()?;
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
        LEToken::LeftLittleBrace => { parse_little_brace_expression(lexer) }
        _ => { Err(SyntaxError::missing_expression(lexer.line()).into()) }
    }
}

pub fn parse_expression(lexer: &mut LELexer) -> VParseResult {
    let primary = parse_primary_expression(lexer)?;
    parse_binary_ops(lexer, primary, 0)
}

pub fn parse_code_block(lexer: &mut LELexer) -> Result<CodeBlock> {
    lexer.consume_left_big_brace()?;
    let mut block = CodeBlock { expression: vec![],variables:vec![] };
    while let Ok(current)=lexer.current_result(){
        if current==&LEToken::RightBigBrace{
            break;
        }else if current==&LEToken::KeyWord(KeyWord::VariableDeclare){
            block.variables.push(parse_variable_declaration(lexer)?);
        }else{
            block.expression.push(parse_statement(lexer)?);
        }
    }
    lexer.consume_right_big_brace()?;
    Ok(block)
}
