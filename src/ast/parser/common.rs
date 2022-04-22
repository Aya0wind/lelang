use crate::ast::nodes::{ASTNode, BinaryOpExpression, CodeBlock, Expr, FunctionCall, FunctionDefinition, Identifier, NumberLiteral, Structure, StructureInitializer, TypeDeclarator, UnaryOpExpression};
use crate::ast::parser::array::parse_array_initializer;
use crate::ast::parser::parse_structure_initializer;
use crate::ast::parser::statement::parse_statement;
use crate::ast::parser::type_declarator::parse_type_declarator;
use crate::error::{LEError, SyntaxError, TokenType};
use crate::error::Result;
use crate::lexer::{LELexer, LEToken, Operator, Position};

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


pub fn parse_annotation(lexer: &mut LELexer) -> Result<(String, TypeDeclarator)> {
    let identifier = lexer.consume_identifier()?;
    lexer.consume_colon()?;
    let type_declarator = parse_type_declarator(lexer)?;
    Ok((identifier, type_declarator))
}

pub fn parse_call_expression(lexer: &mut LELexer, function_name: Identifier) -> Result<Box<Expr>> {
    let start_pos = lexer.pos();
    lexer.consume();
    let mut params = vec![];
    loop {
        let current_token = lexer.current()
            .ok_or_else(|| LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Comma]), lexer.pos()))?;
        match current_token {
            LEToken::RightPar => {
                lexer.consume();
                return Ok(Box::new(Expr::CallExpression(FunctionCall {
                    function_name,
                    params,
                    pos: start_pos.sum(&lexer.pos()),
                })));
            }
            LEToken::Comma => {
                lexer.consume();
            }
            _ => {
                params.push(*parse_expression(lexer)?);
            }
        }
    }
}


pub fn parse_binary_ops(lexer: &mut LELexer, mut lhs: Box<Expr>, expression_precedence: usize) -> Result<Box<Expr>> {
    let lhs_pos = lhs.pos();
    loop {
        let current_token = lexer.current().ok_or(
            LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar, TokenType::Comma]), lexer.pos())
        )?;
        if let LEToken::Operator(op) = current_token {
            let precedence = get_operator_precedence(&op);
            if precedence < expression_precedence {
                return Ok(lhs);
            }
            lexer.consume();
            let mut rhs = parse_primary_expression(lexer)?;
            rhs = parse_binary_ops(lexer, rhs, precedence + 1)?;
            let rhs_pos = rhs.pos();
            lhs = Box::new(Expr::BinaryOperator(BinaryOpExpression {
                op: op.clone(),
                left: lhs,
                right: rhs,
                pos: lhs_pos.sum(&rhs_pos),
            }))
        } else {
            return Ok(lhs);
        }
    }
}

pub fn parse_identifier_expression(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let start_pos = lexer.pos();
    let identifier = Identifier { name: lexer.consume_identifier()?, pos: start_pos.clone() };
    let current_token = lexer.current().ok_or(
        LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::LeftPar, TokenType::LeftBrace]), lexer.pos())
    )?;
    match current_token {
        LEToken::LeftPar => {
            Ok(parse_call_expression(lexer, identifier)?)
        }
        LEToken::LeftBrace => {
            let (initializer, pos) = parse_structure_initializer(lexer)?;
            Ok(Box::new(Expr::StructureInitializer(StructureInitializer {
                structure_name: identifier,
                member_initial_values: initializer,
                pos: start_pos.sum(&pos),
            })))
        }
        _ => {
            Ok(Box::new(Expr::Identifier(identifier)))
        }
    }
}

pub fn parse_number_expression(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let start_pos = lexer.pos();
    let number = lexer.consume_number_literal()?;
    Ok(Box::new(Expr::NumberLiteral(NumberLiteral { number, pos: start_pos })))
}

pub fn parse_little_par_expression(lexer: &mut LELexer) -> Result<Box<Expr>> {
    lexer.consume_left_par()?;
    let expression = parse_expression(lexer)?;
    lexer.consume_right_par()?;
    Ok(expression)
}

pub fn parse_unary_ops(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let start_pos = lexer.pos();
    let op = lexer.consume_operator()?;
    Ok(Box::new(Expr::UnaryOperator(UnaryOpExpression {
        op: Operator::Sub,
        expr: parse_primary_expression(lexer)?,
        pos: start_pos.sum(&lexer.pos()),
    })))
}

pub fn parse_primary_expression(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let current_token = lexer.current().ok_or(
        LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::LeftPar, TokenType::LeftBrace]), lexer.pos())
    )?;
    match current_token {
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
        _ => {
            Err(LEError::new_syntax_error(SyntaxError::unexpect_token(
                vec![TokenType::Operator, TokenType::NumberLiteral, TokenType::Identifier, TokenType::LeftBracket, TokenType::LeftPar],
                current_token.clone()), lexer.pos()))
        }
    }
}

pub fn parse_expression(lexer: &mut LELexer) -> Result<Box<Expr>> {
    let primary = parse_primary_expression(lexer)?;
    parse_binary_ops(lexer, primary, 0)
}

pub fn parse_code_block(lexer: &mut LELexer) -> Result<CodeBlock> {
    let start_pos = lexer.pos();
    lexer.consume_left_brace()?;
    let mut statements = vec![];
    while let Some(current) = lexer.current() {
        if current == LEToken::RightBrace {
            break;
        }
        statements.push(parse_statement(lexer)?);
    }
    lexer.consume_right_brace()?;
    Ok(CodeBlock {
        statements,
        pos: start_pos.sum(&lexer.pos()),
    }
    )
}



