#![allow(unused)]

use std::fmt::{Debug, Display, Formatter, write};
use std::ops::Range;
use std::rc::Rc;

use logos::{Lexer, Logos};

use crate::error::{LEError, Result, SyntaxError, TokenType};
use crate::lexer::LEToken::Semicolon;
use crate::lexer::number_parser::parse_number;

fn record_span(lexer: &mut Lexer<LogosToken>) {
    let token_start = lexer.span().start;
    let token_range = lexer.slice().len();
    lexer.extras.last_pos = lexer.extras.current_pos.clone();
    lexer.extras.current_pos.range = (token_start..token_start + token_range);
}

fn parse_string_literal_token(s: &str) -> Option<String> {
    Some(s.into())
}

#[derive(Debug, Clone)]
pub struct Position {
    pub range: Range<usize>,
}

impl Position {
    pub fn sum(&self, other: &Self) -> Self {
        use std::cmp::{max, min};
        Self { range: (min(self.range.start, other.range.start)..max(self.range.end, other.range.end)) }
    }
}

#[derive(Debug, Clone)]
pub struct Extra {
    current_pos: Position,
    last_pos: Position,
}


impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Logos, Debug, PartialEq)]
#[logos(extras = Extra)]
pub enum LogosToken {
    #[token("if", | lex | record_span(lex))]
    If,

    #[token("el", | lex | record_span(lex))]
    Else,

    #[token("decl", | lex | record_span(lex))]
    Declare,

    #[token("le", | lex | record_span(lex))]
    FunctionDeclare,

    #[token("struct", | lex | record_span(lex))]
    StructureDeclare,

    #[token("for", | lex | record_span(lex))]
    For,

    #[token("while", | lex | record_span(lex))]
    While,

    #[token("var", | lex | record_span(lex))]
    VariableDeclare,

    #[token("ret", | lex | record_span(lex))]
    Return,

    #[token("true", | lex | record_span(lex))]
    True,

    #[token("false", | lex | record_span(lex))]
    False,

    #[token("->", | lex | record_span(lex))]
    SingleArrow,

    #[token("=>", | lex | record_span(lex))]
    DoubleArrow,

    #[token(":", | lex | record_span(lex))]
    Colon,

    #[token(".", | lex | record_span(lex))]
    Dot,

    #[token(";", | lex | record_span(lex))]
    Semicolon,

    #[token("ref", | lex | record_span(lex))]
    Ref,

    #[token("(", | lex | record_span(lex))]
    LeftPar,

    #[token(")", | lex | record_span(lex))]
    RightPar,

    #[token("[", | lex | record_span(lex))]
    LeftBracket,

    #[token("]", | lex | record_span(lex))]
    RightBracket,

    #[token("}", | lex | record_span(lex))]
    RightBrace,

    #[token("{", | lex | record_span(lex))]
    LeftBrace,

    #[token(",", | lex | record_span(lex))]
    Comma,

    #[token("~", | lex | record_span(lex))]
    Rev,

    #[token("+", | lex | record_span(lex))]
    Plus,

    #[token("-", | lex | record_span(lex))]
    Sub,

    #[token("*", | lex | record_span(lex))]
    Mul,

    #[token("/", | lex | record_span(lex))]
    Div,

    #[token("%", | lex | record_span(lex))]
    Mod,

    #[token("=", | lex | record_span(lex))]
    Assign,

    #[token("as", | lex | record_span(lex))]
    Cast,

    #[token("==", | lex | record_span(lex))]
    Equal,

    #[token("!=", | lex | record_span(lex))]
    NotEqual,

    #[token(">", | lex | record_span(lex))]
    GreaterThan,

    #[token("<", | lex | record_span(lex))]
    LessThan,

    #[token(">=", | lex | record_span(lex))]
    GreaterOrEqualThan,

    #[token("<=", | lex | record_span(lex))]
    LessOrEqualThan,

    #[token("&&", | lex | record_span(lex))]
    And,

    #[token("||", | lex | record_span(lex))]
    Or,

    #[token("!", | lex | record_span(lex))]
    Not,

    #[token("^", | lex | record_span(lex))]
    Xor,

    #[regex(r"[\s]+", logos::skip)]
    WhiteCharacter,

    #[regex(r"##[^\n]*", logos::skip)]
    Comment,

    #[regex("[a-zA-Z_]+[a-zA-Z_0-9]*", | lex | {record_span(lex); lex.slice().to_string()})]
    Identifier(String),

    #[regex(r#""[^\n]*""#, | lex | {record_span(lex); parse_string_literal_token(lex.slice())})]
    StringLiteral(String),

    #[regex(r#"[0-9]*(\.[0-9]+)?"#, | lex | {record_span(lex); parse_number(lex)})]
    NumberLiteral(Number),

    #[error]
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeyWord {
    If,

    Else,

    Declare,

    FunctionDefine,

    VariableDeclare,

    Return,

    For,

    While,

    StructureDeclare,

    Ref,
}

impl Display for KeyWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            KeyWord::If => { "if" }
            KeyWord::Else => { "el" }
            KeyWord::Declare => { "decl" }
            KeyWord::FunctionDefine => { "le" }
            KeyWord::VariableDeclare => { "var" }
            KeyWord::Return => { "ret" }
            KeyWord::For => { "for" }
            KeyWord::While => { "while" }
            KeyWord::StructureDeclare => { "struct" }
            KeyWord::Ref => { "ref" }
        };
        f.write_str(str)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,

    Sub,

    Mul,

    Div,

    Assign,

    Equal,

    NotEqual,

    GreaterThan,

    LessThan,

    GreaterOrEqualThan,

    LessOrEqualThan,

    Dot,

    And,

    Or,

    Xor,

    Not,

    Rev,

    Mod,

    Cast,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Plus => { "+" }
            Operator::Sub => { "-" }
            Operator::Mul => { "*" }
            Operator::Div => { "/" }
            Operator::Assign => { "=" }
            Operator::Equal => { "==" }
            Operator::NotEqual => { "!=" }
            Operator::GreaterThan => { ">" }
            Operator::LessThan => { "<=" }
            Operator::GreaterOrEqualThan => { ">=" }
            Operator::LessOrEqualThan => { "<=" }
            Operator::Dot => { "." }
            Operator::And => { "&&" }
            Operator::Or => { "||" }
            Operator::Xor => { "^" }
            Operator::Not => { "!" }
            Operator::Rev => { "~" }
            Operator::Mod => { "%" }
            Operator::Cast => { "as" }
        };
        f.write_str(s)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Number {
    Integer(u64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(i) => { f.write_fmt(format_args!("{}", i)) }
            Number::Float(float) => { f.write_fmt(format_args!("{}", float)) }
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
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

    SingleArrow,

    DoubleArrow,
}

impl Display for LEToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LEToken::KeyWord(k) => { Display::fmt(k, f) }
            LEToken::Operator(o) => { Display::fmt(o, f) }
            LEToken::NumberLiteral(n) => { Display::fmt(n, f) }
            LEToken::StringLiteral(s) => { Display::fmt(s, f) }
            LEToken::Identifier(i) => { Display::fmt(i, f) }
            LEToken::Colon => { f.write_str(":") }
            LEToken::Comma => { f.write_str(":") }
            LEToken::Semicolon => { f.write_str(";") }
            LEToken::LeftPar => { f.write_str("(") }
            LEToken::RightPar => { f.write_str(")") }
            LEToken::LeftBracket => { f.write_str("[") }
            LEToken::RightBracket => { f.write_str("]") }
            LEToken::RightBrace => { f.write_str("{") }
            LEToken::LeftBrace => { f.write_str("}") }
            LEToken::SingleArrow => { f.write_str("->") }
            LEToken::DoubleArrow => { f.write_str("=>") }
        }
    }
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
            LogosToken::Colon => { Self::Colon }
            LogosToken::Comma => { Self::Comma }
            LogosToken::Semicolon => { Self::Semicolon }
            LogosToken::LeftPar => { Self::LeftPar }
            LogosToken::RightPar => { Self::RightPar }
            LogosToken::LeftBracket => { Self::LeftBracket }
            LogosToken::RightBracket => { Self::RightBracket }
            LogosToken::RightBrace => { Self::RightBrace }
            LogosToken::LeftBrace => { Self::LeftBrace }
            LogosToken::SingleArrow => { Self::SingleArrow }
            LogosToken::Plus => { Self::Operator(Operator::Plus) }
            LogosToken::Sub => { Self::Operator(Operator::Sub) }
            LogosToken::Mul => { Self::Operator(Operator::Mul) }
            LogosToken::Div => { Self::Operator(Operator::Div) }
            LogosToken::Assign => { Self::Operator(Operator::Assign) }
            LogosToken::Equal => { Self::Operator(Operator::Equal) }
            LogosToken::StringLiteral(literal) => { Self::StringLiteral(literal) }
            LogosToken::NumberLiteral(num) => { Self::NumberLiteral(num) }
            LogosToken::Identifier(identifier) => { Self::Identifier(identifier) }
            LogosToken::GreaterThan => { Self::Operator(Operator::GreaterThan) }
            LogosToken::LessThan => { Self::Operator(Operator::LessThan) }
            LogosToken::GreaterOrEqualThan => { Self::Operator(Operator::GreaterOrEqualThan) }
            LogosToken::LessOrEqualThan => { Self::Operator(Operator::LessOrEqualThan) }
            LogosToken::Declare => { Self::KeyWord(KeyWord::Declare) }
            LogosToken::While => { Self::KeyWord(KeyWord::While) }
            LogosToken::StructureDeclare => { Self::KeyWord(KeyWord::StructureDeclare) }
            LogosToken::Dot => { Self::Operator(Operator::Dot) }
            LogosToken::Ref => { Self::KeyWord(KeyWord::Ref) }
            LogosToken::And => { Self::Operator(Operator::And) }
            LogosToken::Or => { Self::Operator(Operator::Or) }
            LogosToken::Xor => { Self::Operator(Operator::Xor) }
            LogosToken::Not => { Self::Operator(Operator::Not) }
            LogosToken::Rev => { Self::Operator(Operator::Rev) }
            LogosToken::True => { Self::Identifier("true".into()) }
            LogosToken::False => { Self::Identifier("false".into()) }
            LogosToken::Mod => { Self::Operator(Operator::Mod) }
            LogosToken::NotEqual => { Self::Operator(Operator::NotEqual) }
            LogosToken::DoubleArrow => { Self::DoubleArrow }
            LogosToken::Cast => { Self::Operator(Operator::Cast) }
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
/// use LELexer;
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
    current: Option<LEToken>,
}


impl<'s> Iterator for LELexer<'s> {
    type Item = LEToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => { self.current.take() }
            Some(x) => { self.current.replace(x.into()) }
        }
    }
}

impl<'s> LELexer<'s> {
    pub fn new(s: &'s str) -> Option<Self> {
        let mut s = Self {
            inner: LogosToken::lexer_with_extras(s, Extra { current_pos: Position { range: (0..0) }, last_pos: Position { range: (0..0) } }),
            current: None,
        };
        s.next();
        Some(s)
    }
    pub fn consume(&mut self) {
        self.next().unwrap();
    }
    /// 获取迭代器当前指向的token，如果不存在则返回None
    pub fn current(&self) -> Option<LEToken> {
        self.current.clone()
    }

    ///获取迭代器当前指向的token的位置
    pub fn pos(&self) -> Position {
        self.inner.extras.current_pos.clone()
    }

    ///获取迭代器当前指向的token的位置
    pub fn last_pos(&self) -> Position {
        self.inner.extras.last_pos.clone()
    }

    pub fn consume_keyword(&mut self) -> Result<KeyWord> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::KeyWord(key) = consume {
                Ok(key)
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Identifier], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Identifier]), current_pos))
        }
    }

    pub fn consume_operator(&mut self) -> Result<Operator> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::Operator(op) = consume {
                Ok(op)
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Operator], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Operator]), current_pos))
        }
    }


    pub fn consume_number_literal(&mut self) -> Result<Number> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::NumberLiteral(number) = consume {
                Ok(number)
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::NumberLiteral], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::NumberLiteral]), current_pos))
        }
    }

    pub fn consume_string_literal(&mut self) -> Result<String> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::StringLiteral(string) = consume {
                Ok(string)
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::StringLiteral], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::StringLiteral]), current_pos))
        }
    }

    pub fn consume_identifier(&mut self) -> Result<String> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::Identifier(ident) = consume {
                Ok(ident)
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Identifier], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Identifier]), current_pos))
        }
    }

    pub fn consume_colon(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::Colon = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Colon], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Colon]), current_pos))
        }
    }

    pub fn consume_comma(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::Comma = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Comma], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Comma]), current_pos))
        }
    }

    pub fn consume_semicolon(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::Semicolon = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::Semicolon], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::Semicolon]), current_pos))
        }
    }

    pub fn consume_left_par(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::LeftPar = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::LeftPar], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::LeftPar]), current_pos))
        }
    }
    pub fn consume_right_par(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::RightPar = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::RightPar], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightPar]), current_pos))
        }
    }

    pub fn consume_left_bracket(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::LeftBracket = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::LeftBracket], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::LeftBracket]), current_pos))
        }
    }
    pub fn consume_right_bracket(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::RightBracket = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::RightBracket], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightBracket]), current_pos))
        }
    }

    pub fn consume_left_brace(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::LeftBrace = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::LeftBrace], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::LeftBrace]), current_pos))
        }
    }

    pub fn consume_right_brace(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::RightBrace = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::RightBrace], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::RightBrace]), current_pos))
        }
    }

    pub fn consume_return_type_allow(&mut self) -> Result<()> {
        let current_pos = self.last_pos();
        let consume = self.next();
        if let Some(consume) = consume {
            if let LEToken::SingleArrow = consume {
                Ok(())
            } else {
                Err(LEError::new_syntax_error(SyntaxError::unexpect_token(vec![TokenType::SingleArrow], consume), current_pos))
            }
        } else {
            Err(LEError::new_syntax_error(SyntaxError::missing_token(vec![TokenType::SingleArrow]), current_pos))
        }
    }
}