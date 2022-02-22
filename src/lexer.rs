use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("if")]
    If,
    #[token("else")]
    Else,
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
    #[regex(r#""[0-9a-zA-Z\-\.]+""#)]
    StringLiteral,
    #[regex(r"#[\x20-\x7F]+\n+", logos::skip)]
    Comment,
    #[regex("[a-zA-Z]+")]
    Word,
    #[error]
    #[regex(r"[\s]+", logos::skip)]
    Error,
}