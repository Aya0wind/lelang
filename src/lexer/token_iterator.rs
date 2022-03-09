use logos::{Lexer, Logos};
use strum_macros::Display;
use crate::error::{SyntaxError, TokenType};
use anyhow::Result;
use crate::lexer::number_parser::parse_number;

fn counter_line(white:&str)->usize{
    white.bytes().filter(|&x|x==b'\n').count()
}

fn parse_num_literal_token<'s>(lexer: &mut logos::Lexer<'s, LogosToken>)->Option<Number>{
    Some(parse_number(lexer))
}

fn parse_string_literal_token<'s>(lexer: &mut logos::Lexer<'s, LogosToken>)->Option<String>{
    Some(String::new())
}


#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken{

    #[token("if")]
    If,

    #[token("el")]
    Else,

    #[token("le")]
    FunctionDeclare,

    #[token("var")]
    VariableDeclare,

    #[token("ret")]
    Return,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LeftLittleBrace,

    #[token(")")]
    RightLittleBrace,

    #[token("[")]
    LeftMiddleBrace,

    #[token("]")]
    RightMiddleBrace,

    #[token("}")]
    RightBigBrace,

    #[token("{")]
    LeftBigBrace,

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

    #[token("->")]
    ReturnTypeAllow,

    #[regex(r"[\s]+", |lex|counter_line(lex.slice()))]
    WhiteCharacter(usize),

    #[regex(r"#[\x20-\x7F]+\n+")]
    Comment,

    #[regex("[a-zA-Z_]+[0-9]*", |lex|lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""[0-9a-zA-Z\-\.]*""#, |lex|parse_string_literal_token(lex))]
    StringLiteral(String),

    #[regex(r#"-?[0-9]"#, |lex|parse_num_literal_token(lex))]
    NumberLiteral(Number),

    #[error]
    Error,
}
#[derive(Debug, PartialEq,Display,Clone)]
pub enum KeyWord {
    If,

    Else,

    FunctionDeclare,

    VariableDeclare,

    Return,
}

#[derive(Debug, PartialEq,Display,Clone)]
pub enum Operator {
    Plus,

    Sub,

    Mul,

    Div,

    Assign,

    Equal,
}

#[derive(Debug, PartialEq,Display,Clone)]
#[allow(dead_code)]
pub enum Number {
    I8(i8),

    I16(i16),

    I32(i32),

    I64(i64),

    U8( u8),

    U16(u16),

    U32(u32),

    U64(u64),

    F32(f32),

    F64(f64),

}

#[derive(Debug, PartialEq,Display,Clone)]
pub enum LEToken{
    KeyWord(KeyWord),

    Operator(Operator),

    NumberLiteral(Number),

    StringLiteral(String),

    Identifier(String),

    Colon,

    Comma,

    Semicolon,

    LeftLittleBrace,

    RightLittleBrace,

    LeftMiddleBrace,

    RightMiddleBrace,

    RightBigBrace,

    LeftBigBrace,

    ReturnTypeAllow,

    EOF,

    Error(String),
}


impl From<LogosToken> for LEToken {
    fn from(logos_token: LogosToken) -> Self {
        match logos_token {
            LogosToken::If => { Self::KeyWord(KeyWord::If) }
            LogosToken::Else => { Self::KeyWord(KeyWord::Else) }
            LogosToken::FunctionDeclare => { Self::KeyWord(KeyWord::FunctionDeclare) }
            LogosToken::VariableDeclare => { Self::KeyWord(KeyWord::VariableDeclare) }
            LogosToken::Return => { Self::KeyWord(KeyWord::Return) }
            LogosToken::Colon => { Self::Colon }
            LogosToken::Comma => { Self::Comma }
            LogosToken::Semicolon => { Self::Semicolon }
            LogosToken::LeftLittleBrace => { Self::LeftLittleBrace }
            LogosToken::RightLittleBrace => { Self::RightLittleBrace }
            LogosToken::LeftMiddleBrace => { Self::LeftMiddleBrace }
            LogosToken::RightMiddleBrace => { Self::RightMiddleBrace }
            LogosToken::RightBigBrace => { Self::RightBigBrace }
            LogosToken::LeftBigBrace => { Self::LeftBigBrace }
            LogosToken::ReturnTypeAllow => {Self::ReturnTypeAllow}
            LogosToken::Plus => { Self::Operator(Operator::Plus) }
            LogosToken::Sub => { Self::Operator(Operator::Sub) }
            LogosToken::Mul => { Self::Operator(Operator::Mul) }
            LogosToken::Div => { Self::Operator(Operator::Div) }
            LogosToken::Assign => { Self::Operator(Operator::Assign) }
            LogosToken::Equal => { Self::Operator(Operator::Equal) }
            LogosToken::StringLiteral(literal) => { Self::StringLiteral(literal) }
            LogosToken::NumberLiteral(num) => { Self::NumberLiteral(num) }
            LogosToken::Identifier(identifier) => { Self::Identifier(identifier) }
            LogosToken::Error=>{Self::Error("unknown character".into())}
            _ => { unreachable!("unknown character handling not implement yet") }
        }
    }
}

pub struct LELexer<'s> {
    inner: Lexer<'s, LogosToken>,
    current_line:usize,
    current:LEToken,
}


impl<'s> Iterator for LELexer<'s> {
    type Item = LEToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().skip_while(|next|{
            match next{
                LogosToken::Comment=>{self.current_line+=1;true}
                LogosToken::WhiteCharacter(lines)=>{self.current_line+=lines;true}
                _=>{false}
            }
        }).map(|x|{
            let le_token:LEToken  = x.into();
            self.current=le_token.clone();
            le_token
        }).next()
    }
}

impl<'s> LELexer<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { inner: LogosToken::lexer(s),current_line:0,current:LEToken::EOF, }
    }

    pub fn current(&self)->&LEToken{
        &self.current
    }
    pub fn line(&self)->usize{
        self.current_line
    }


    pub fn next_result(&mut self)->Result<LEToken>{
        self.next().ok_or(SyntaxError::EOF.into())
    }

    pub fn expect_keyword(&mut self)->Result<KeyWord>{
        let expect = self.next().ok_or(SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::KeyWord(keyword)=expect{
            Ok(keyword)
        }else{
            Err(Box::new(SyntaxError::unexpect_token(TokenType::Identifier, expect, self.current_line)).into())
        }
    }

    pub fn expect_operator(&mut self) ->Result<Operator>{
        let expect = self.next().ok_or(SyntaxError::missing_operator(self.current_line))?;
        if let LEToken::Operator(operator)=expect{
            Ok(operator)
        }else{
            Err(SyntaxError::unexpect_token(TokenType::Operator, expect, self.current_line).into())
        }
    }

    pub fn expect_number_literal(&mut self) ->Result<Number>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_number_literal(self.current_line))?;
        if let LEToken::NumberLiteral(number)=expect{
            Ok(number)
        }else{
            Err(SyntaxError::unexpect_token(TokenType::NumberLiteral, expect, self.current_line).into())
        }
    }
    pub fn expect_string_literal(&mut self) ->Result<String>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::StringLiteral(string_literal)=expect{
            Ok(string_literal)
        }else{
            Err(SyntaxError::unexpect_token(TokenType::StringLiteral, expect, self.current_line).into())
        }
    }
    pub fn expect_identifier(&mut self) ->Result<String>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::Identifier(identifier)=expect{
            Ok(identifier)
        }else{
            Err(SyntaxError::unexpect_token(TokenType::Identifier, expect, self.current_line).into())
        }
    }

    pub fn expect_colon(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::Colon=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::Colon, expect, self.current_line).into())
        }
    }
    pub fn expect_comma(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::Comma=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::Comma, expect, self.current_line).into())
        }
    }
    pub fn expect_semicolon(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::Semicolon=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::Semicolon, expect, self.current_line).into())
        }
    }
    pub fn expect_left_little_brace(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::LeftLittleBrace=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::LeftLittleBrace, expect, self.current_line).into())
        }
    }
    pub fn expect_right_little_brace(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::RightLittleBrace=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::RightLittleBrace, expect, self.current_line).into())
        }
    }
    pub fn expect_right_middle_brace(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::RightMiddleBrace=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::RightMiddleBrace, expect, self.current_line).into())
        }
    }
    pub fn expect_left_big_brace(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::LeftBigBrace=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::LeftBigBrace, expect, self.current_line).into())
        }
    }
    pub fn expect_right_big_brace(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::RightBigBrace=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::RightBigBrace, expect, self.current_line).into())
        }
    }
    pub fn expect_return_type_allow(&mut self) ->Result<()>{
        let expect = self.next().ok_or_else(|| SyntaxError::missing_identifier(self.current_line))?;
        if let LEToken::ReturnTypeAllow=expect{
            Ok(())
        }else{
            Err(SyntaxError::unexpect_token(TokenType::ReturnTypeAllow, expect, self.current_line).into())
        }
    }
}