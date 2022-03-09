use anyhow::Result;
use crate::ast::{BinaryOperatorNode, CodeBlock, NumberLiteralNode, Param, parse_statement, parse_variable_declaration, Statement, Expr, VParseResult, BExpr, VariableNode, IdentifierNode};
use crate::ast::Expr::{BinaryOperator, Identifier, NumberLiteral};
use crate::error::{SyntaxError, TokenType};
use crate::lexer::{LEToken, Number, LELexer, KeyWord, Operator};



pub fn parse_call_param_list(lexer: &mut LELexer) -> Result<Vec<Param>> {
    lexer.expect_left_little_brace()?;
    let mut params = vec![];
    loop {
        let value = lexer.next_result()?;
        if let LEToken::Identifier(identifier) = value{
            params.push(Param::Identifier(identifier));
        }else if let LEToken::NumberLiteral(number) = value{
            params.push(Param::Number(number));
        }else{
            return Err(SyntaxError::unexpect_token(TokenType::Identifier, value, lexer.line()).into())
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


pub fn parse_unary(lexer: &mut LELexer) -> VParseResult{

    unimplemented!()
}

pub fn parse_binary(lexer: &mut LELexer, lhs: BExpr) -> VParseResult{
    unimplemented!()
}


pub fn parse_call_expression(lexer: &mut LELexer, function_name: &str) -> VParseResult{
    unimplemented!()
}
pub fn parse_identifier_expression(lexer: &mut LELexer,identifier:String)->VParseResult{
    let next = lexer.next_result()?;
    if let LEToken::Operator(op)=next{
        Ok(Box::new(BinaryOperator(BinaryOperatorNode {
            op,
            left: Box::new(Identifier(IdentifierNode{ name: identifier})),
            right: parse_expression(lexer)?
        })))
    }else{
        Ok(Box::new(Identifier(IdentifierNode{ name: identifier })))
    }
}

pub fn parse_number_expression(lexer: &mut LELexer,number:Number)->VParseResult{
    let next = lexer.next_result()?;
    if let LEToken::Operator(op)=next{
        Ok(Box::new(BinaryOperator(BinaryOperatorNode {
            op,
            left: Box::new(NumberLiteral(NumberLiteralNode::new(number))),
            right: parse_expression(lexer)?
        })))
    }else{
        Ok(Box::new(NumberLiteral(NumberLiteralNode::new(number))))
    }
}


pub fn parse_expression(lexer: &mut LELexer) -> VParseResult{
    let next  = lexer.next_result()?;
    match next {
        LEToken::NumberLiteral(number) => {
            parse_number_expression(lexer,number)
        }
        LEToken::Identifier(identifier) => {
            parse_identifier_expression(lexer,identifier)
        }
        LEToken::LeftLittleBrace => { parse_little_brace_expression(lexer) }
        _ => { Err(SyntaxError::missing_expression(lexer.line()).into()) }
    }
}

pub fn parse_little_brace_expression(lexer: &mut LELexer) -> VParseResult{
    let primary_expression = parse_expression(lexer)?;
    lexer.expect_left_little_brace()?;
    Ok(primary_expression)
}




pub fn parse_code_block(lexer:&mut LELexer) ->Result<CodeBlock>{
    lexer.expect_left_big_brace()?;
    let mut block = CodeBlock{statements:vec![]};
    while let Ok(statement) = parse_statement(lexer){
        block.statements.push( statement);
   }
    if lexer.current()==&LEToken::RightBigBrace{
        Ok(block)
    }else{
        Err(SyntaxError::unexpect_token(TokenType::RightBigBrace, lexer.current().clone(), lexer.line()).into())
    }
}
