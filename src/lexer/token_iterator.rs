#![allow(unused)]

use anyhow::Result;
use logos::{Lexer, Logos};
use strum_macros::Display;

use crate::error::{SyntaxError, TokenType};
use crate::lexer::number_parser::parse_number;

fn counter_line(white: &str) -> usize {
    white.bytes().filter(|&x| x == b'\n').count()
}

fn parse_string_literal_token(s: &str) -> Option<String> {
    Some(s.into())
}


#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    #[token("if")]
    If,

    #[token("el")]
    Else,

    #[token("decl")]
    Declare,

    #[token("le")]
    FunctionDeclare,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("var")]
    VariableDeclare,

    #[token("ret")]
    Return,

    #[token("->")]
    ReturnTypeAllow,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LeftPar,

    #[token(")")]
    RightPar,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("}")]
    RightBrace,

    #[token("{")]
    LeftBrace,

    #[token(",")]
    Comma,

    #[token("+")]
    Plus,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("=")]
    Assign,

    #[token("==")]
    Equal,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token(">=")]
    GreaterOrEqualThan,

    #[token("<=")]
    LessOrEqualThan,

    #[regex(r"[\s]+", | lex | counter_line(lex.slice()))]
    WhiteCharacter(usize),

    #[regex(r"##[^\n]*")]
    Comment,

    #[regex("[a-zA-Z_]+[0-9]*", | lex | lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""[0-9a-zA-Z\-\.]*""#, | lex | parse_string_literal_token(lex.slice()))]
    StringLiteral(String),

    #[regex(r#"[0-9]*(\.[0-9]+)?"#, | lex | parse_number(lex).ok())]
    NumberLiteral(Number),

    #[error]
    Error,
}

#[derive(Debug, PartialEq, Display, Clone)]
pub enum KeyWord {
    If,

    Else,

    Declare,

    FunctionDefine,

    VariableDeclare,

    Return,

    For,

    While,
}

#[derive(Debug, PartialEq, Display, Clone)]
pub enum Operator {
    Plus,

    Sub,

    Mul,

    Div,

    Assign,

    Equal,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,
}

#[derive(Debug, PartialEq, Display, Clone)]
#[allow(dead_code)]
pub enum Number {
    Integer(u64),
    Float(f64),
}

#[derive(Debug, PartialEq, Display, Clone)]
pub enum LEToken {
    KeyWord(KeyWord),

    Operator(Operator),

    NumberLiteral(Number),

    StringLiteral(String),

    Identifier(String),

    Colon,

    Comma,

    Semicolon,

    LeftPar,

    RightPar,

    LeftBracket,

    RightBracket,

    RightBrace,

    LeftBrace,

    ReturnTypeAllow,

    EOF,

    Error(String),
}


impl From<LogosToken> for LEToken {
    fn from(logos_token: LogosToken) -> Self {
        match logos_token {
            LogosToken::If => { Self::KeyWord(KeyWord::If) }
            LogosToken::Else => { Self::KeyWord(KeyWord::Else) }
            LogosToken::For => { Self::KeyWord(KeyWord::For) }
            LogosToken::FunctionDeclare => { Self::KeyWord(KeyWord::FunctionDefine) }
            LogosToken::VariableDeclare => { Self::KeyWord(KeyWord::VariableDeclare) }
            LogosToken::Return => { Self::KeyWord(KeyWord::Return) }
            LogosToken::Return => { Self::KeyWord(KeyWord::Return) }
            LogosToken::Colon => { Self::Colon }
            LogosToken::Comma => { Self::Comma }
            LogosToken::Semicolon => { Self::Semicolon }
            LogosToken::LeftPar => { Self::LeftPar }
            LogosToken::RightPar => { Self::RightPar }
            LogosToken::LeftBracket => { Self::LeftBracket }
            LogosToken::RightBracket => { Self::RightBracket }
            LogosToken::RightBrace => { Self::RightBrace }
            LogosToken::LeftBrace => { Self::LeftBrace }
            LogosToken::ReturnTypeAllow => { Self::ReturnTypeAllow }
            LogosToken::Plus => { Self::Operator(Operator::Plus) }
            LogosToken::Sub => { Self::Operator(Operator::Sub) }
            LogosToken::Mul => { Self::Operator(Operator::Mul) }
            LogosToken::Div => { Self::Operator(Operator::Div) }
            LogosToken::Assign => { Self::Operator(Operator::Assign) }
            LogosToken::Equal => { Self::Operator(Operator::Equal) }
            LogosToken::StringLiteral(literal) => { Self::StringLiteral(literal) }
            LogosToken::NumberLiteral(num) => { Self::NumberLiteral(num) }
            LogosToken::Identifier(identifier) => { Self::Identifier(identifier) }
            LogosToken::Error => { Self::Error("unknown character".into()) }
            LogosToken::GreaterThan => { Self::Operator(Operator::GreaterThan) }
            LogosToken::LessThan => { Self::Operator(Operator::LessThan) }
            LogosToken::GreaterOrEqualThan => { Self::Operator(Operator::GreaterOrEqualThan) }
            LogosToken::LessOrEqualThan => { Self::Operator(Operator::LessOrEqualThan) }
            LogosToken::Declare => { Self::KeyWord(KeyWord::Declare) }
            LogosToken::While => { Self::KeyWord(KeyWord::While) }
            _ => { unreachable!("unknown character handling not implement yet") }
        }
    }
}

/// 词法分析器
/// 拆分代码为Token stream，使用迭代器形式返回token
/// 为支持LL(1)分析，可前向看一个token
/// # Example
/// ```
/// use std::fs::File;
/// use std::fs::File;
/// use std::io::Read;
/// use std::io::Read;
/// use lelang::lexer::LELexer;
/// let mut f = File::open("benches/test_case/lexer_test.le").unwrap();
/// let mut buffer = String::new();
/// f.read_to_string(&mut buffer).unwrap();
/// let lexer = LELexer::new(&buffer).unwrap();
/// for token in lexer{
///     eprintln!("{:?}",token);
/// }
/// ```
pub struct LELexer<'s> {
    inner: Lexer<'s, LogosToken>,
    current_line: usize,
    current: Option<LEToken>,
}


impl<'s> Iterator for LELexer<'s> {
    type Item = LEToken;

    fn next(&mut self) -> Option<Self::Item> {
        let inner_iter = self.inner.by_ref().find(|next| {
            match next {
                LogosToken::Comment => {
                    self.current_line += 1;
                    false
                }
                LogosToken::WhiteCharacter(lines) => {
                    self.current_line += lines;
                    false
                }
                _ => { true }
            }
        });
        match inner_iter {
            None => { self.current.take() }
            Some(x) => { self.current.replace(x.into()) }
        }
    }
}

impl<'s> LELexer<'s> {
    pub fn new(s: &'s str) -> Option<Self> {
        let mut s = Self { inner: LogosToken::lexer(s), current_line: 0, current: None };
        s.next();
        Some(s)
    }


    /// 获取迭代器当前指向的token，如果不存在则返回EOF错误信息
    pub fn current_result(&self) -> Result<&LEToken> {
        self.current.as_ref().ok_or_else(|| SyntaxError::EOF.into())
    }

    /// 获取迭代器当前指向的token，如果不存在则返回None
    pub fn current(&self) -> Option<&LEToken> {
        self.current.as_ref()
    }


    /// 获取迭代器当前指向的token的副本
    pub fn own_current(&self) -> Result<LEToken> {
        Ok(self.current_result()?.clone())
    }


    ///获取迭代器当前指向的token的行号
    pub fn line(&self) -> usize {
        self.current_line
    }

    ///获取迭代器当前指向token的所有权，并将迭代器指向下一个token
    pub fn next_result(&mut self) -> Result<LEToken> {
        self.next().ok_or_else(|| SyntaxError::EOF.into())
    }

    pub fn consume_keyword(&mut self) -> Result<KeyWord> {
        let consume = self.next_result()?;
        if let LEToken::KeyWord(keyword) = consume {
            Ok(keyword)
        } else {
            Err(Box::new(SyntaxError::unexpect_token(TokenType::Identifier, consume.clone(), self.current_line.into())).into())
        }
    }

    pub fn consume_operator(&mut self) -> Result<Operator> {
        let consume = self.next_result()?;
        if let LEToken::Operator(operator) = consume {
            Ok(operator)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Operator, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_number_literal(&mut self) -> Result<Number> {
        let consume = self.next_result()?;
        if let LEToken::NumberLiteral(number) = consume {
            Ok(number)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::NumberLiteral, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_string_literal(&mut self) -> Result<String> {
        let consume = self.next_result()?;
        if let LEToken::StringLiteral(string_literal) = consume {
            Ok(string_literal)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::StringLiteral, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_identifier(&mut self) -> Result<String> {
        let consume = self.next_result()?;
        if let LEToken::Identifier(identifier) = consume {
            Ok(identifier)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Identifier, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_colon(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::Colon = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Colon, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_comma(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::Comma = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Comma, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_semicolon(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::Semicolon = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Semicolon, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_left_par(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::LeftPar = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::LeftPar, consume.clone(), self.current_line.into()).into())
        }
    }
    pub fn consume_right_par(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::RightPar = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightPar, consume.clone(), self.current_line.into()).into())
        }
    }
    pub fn consume_right_middle_brace(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::RightBracket = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightBracket, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_left_brace(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::LeftBrace = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::LeftBrace, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_right_brace(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::RightBrace = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightBrace, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn consume_return_type_allow(&mut self) -> Result<()> {
        let consume = self.next_result()?;
        if let LEToken::ReturnTypeAllow = consume {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::ReturnTypeAllow, consume.clone(), self.current_line.into()).into())
        }
    }

    pub fn check_current_keyword(&mut self) -> Result<&KeyWord> {
        let check_current = self.current_result()?;
        if let LEToken::KeyWord(keyword) = check_current {
            Ok(keyword)
        } else {
            Err(Box::new(SyntaxError::unexpect_token(TokenType::Identifier, check_current.clone(), self.current_line.into())).into())
        }
    }

    pub fn check_current_operator(&mut self) -> Result<&Operator> {
        let check_current = self.current_result()?;
        if let LEToken::Operator(operator) = check_current {
            Ok(operator)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Operator, check_current.clone(), self.current_line.into()).into())
        }
    }

    pub fn check_current_number_literal(&mut self) -> Result<&Number> {
        let check_current = self.current_result()?;
        if let LEToken::NumberLiteral(number) = check_current {
            Ok(number)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::NumberLiteral, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_string_literal(&mut self) -> Result<&String> {
        let check_current = self.current_result()?;
        if let LEToken::StringLiteral(string_literal) = check_current {
            Ok(string_literal)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::StringLiteral, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_identifier(&mut self) -> Result<&String> {
        let check_current = self.current_result()?;
        if let LEToken::Identifier(identifier) = check_current {
            Ok(identifier)
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Identifier, check_current.clone(), self.current_line.into()).into())
        }
    }

    pub fn check_current_colon(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::Colon = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Colon, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_comma(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::Comma = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Comma, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_semicolon(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::Semicolon = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::Semicolon, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_left_little_brace(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::LeftPar = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::LeftPar, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_right_little_brace(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::RightPar = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightPar, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_right_middle_brace(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::RightBracket = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightBracket, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_left_big_brace(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::LeftBrace = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::LeftBrace, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_right_big_brace(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::RightBrace = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::RightBrace, check_current.clone(), self.current_line.into()).into())
        }
    }
    pub fn check_current_return_type_allow(&mut self) -> Result<()> {
        let check_current = self.current_result()?;
        if let LEToken::ReturnTypeAllow = check_current {
            Ok(())
        } else {
            Err(SyntaxError::unexpect_token(TokenType::ReturnTypeAllow, check_current.clone(), self.current_line.into()).into())
        }
    }
}