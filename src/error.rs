#![allow(unused)]
use thiserror::Error;
use crate::lexer::{LEToken, LELexer};
use std::fmt::{Display, Formatter};
#[derive(Debug,Error)]
#[allow(unused)]
pub enum TokenParserError{
    #[error("[line {line}]Error:Got unrecognized token:{token}")]
    UnrecognizedToken{
        token:LEToken,
        line:usize,
    }
}


#[derive(Debug,PartialEq)]
pub enum TokenType {
    If,
    Else,
    FunctionDeclare,
    VariableDeclare,
    Return,
    Colon,
    Semicolon,
    LeftLittleBrace,
    RightLittleBrace,
    LeftMiddleBrace,
    RightMiddleBrace,
    RightBigBrace,
    LeftBigBrace,
    Comma,
    Operator,
    ReturnTypeAllow,
    Identifier,
    NumberLiteral,
    StringLiteral
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

#[allow(unused)]
#[derive(Debug,Error)]
pub enum SyntaxError{
    #[error("[line {line}]Unexpected token: '{found}', expect: '{expect}'")]
    UnexpectToken{
        expect:TokenType,
        found:LEToken,
        line:usize,
    },
    #[error("[line {line}] Missing token:\'{expect}\'")]
    MissingToken{
        expect:TokenType,
        line:usize,
    },
    #[error("[line {line}] Missing expression.")]
    MissingExpression{
        line:usize,
    },
    #[error("End of file")]
    EOF
}
#[allow(unused)]
#[derive(Debug,Error)]
pub enum CompileError{
    #[error("unknown_identifier")]
    UnknownIdentifier(String),
    #[error("unknown_type")]
    UnknownType(String)
}

impl TokenParserError{
    pub fn unrecognized_token(token:LEToken, line:usize) ->Self{
        Self::UnrecognizedToken{token,line}
    }
}

impl CompileError {
    pub fn unknown_type(name:String) ->Self{
        Self::UnknownType(name)
    }
    pub fn unknown_identifier(name:String) ->Self{
        Self::UnknownIdentifier(name)
    }
}

impl SyntaxError{
    pub fn unexpect_token(expect:TokenType, found:LEToken,line:usize) ->Self{
        Self::UnexpectToken{expect,found,line}
    }
    pub fn missing_expression(line:usize) ->Self{Self::MissingExpression {line}}
    pub fn missing_token(expect:TokenType, line:usize) ->Self{
        Self::MissingToken{expect,line}
    }
    pub fn missing_if(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::If,line}
    }
    pub fn missing_else(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Else,line}
    }
    pub fn missing_function_declare(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::FunctionDeclare,line}
    }
    pub fn missing_variable_declare(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::VariableDeclare,line}
    }
    pub fn missing_return(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Return,line}
    }
    pub fn missing_colon(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Colon,line}
    }
    pub fn missing_left_little_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::LeftLittleBrace,line}
    }
    pub fn missing_right_little_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::RightLittleBrace,line}
    }
    pub fn missing_left_middle_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::LeftMiddleBrace,line}
    }
    pub fn missing_right_middle_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::RightMiddleBrace,line}
    }
    pub fn missing_left_big_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::LeftBigBrace,line}
    }
    pub fn missing_right_big_brace(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::RightBigBrace,line}
    }
    pub fn missing_comma(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Comma,line}
    }
    pub fn missing_operator(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Operator,line}
    }
    pub fn missing_return_type_allow(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::ReturnTypeAllow,line}
    }
    pub fn missing_identifier(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::Identifier,line}
    }
    pub fn missing_number_literal(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::NumberLiteral,line}
    }
    pub fn missing_string_literal(line:usize) ->Self{
        Self::MissingToken{expect:TokenType::StringLiteral,line}
    }
}


#[derive(Debug,Error)]
#[allow(unused)]
enum JITCompileError{

}

