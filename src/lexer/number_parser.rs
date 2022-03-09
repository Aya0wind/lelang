use std::str::{from_utf8, FromStr};
use logos::Lexer;
use super::token_iterator::Number;
extern crate nom;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    multi::many0,
    branch::alt,
    sequence::tuple
};
use nom::character::is_digit;
use crate::lexer::{LEToken, LogosToken};

type ParseResult<'a> = Option<(&'a [u8], &'a [u8])>;

pub trait Parser<'a> {
    fn parse(&mut self, input: &'a [u8]) -> ParseResult<'a>;
}

impl<'a, F> Parser<'a> for F
    where
        F: FnMut(&'a [u8]) -> ParseResult,
{
    fn parse(&mut self, input: &'a [u8]) -> ParseResult<'a> {
        self(input)
    }
}

fn option<'a, P: Parser<'a>>(mut parser: P) -> impl FnMut(&'a [u8]) -> ParseResult {
    move |input: &'a [u8]| match parser.parse(input) {
        Some(out) => Some(out),
        None => Some((input, &input[0..0])),
    }
}

fn alt2<'a, P1: Parser<'a>, P2: Parser<'a>>(
    (mut parser1, mut parser2): (P1, P2),
) -> impl FnMut(&'a [u8]) -> ParseResult {
    move |input: &[u8]| match parser1.parse(input) {
        Some(out) => Some(out),
        None => parser2.parse(input),
    }
}

fn alt3<'a, P1: Parser<'a>, P2: Parser<'a>, P3: Parser<'a>>(
    (mut parser1, mut parser2, mut parser3): (P1, P2, P3),
) -> impl FnMut(&'a [u8]) -> ParseResult {
    move |input: &[u8]| match parser1.parse(input) {
        Some(out) => Some(out),
        None => match parser2.parse(input) {
            Some(out) => Some(out),
            None => parser3.parse(input),
        },
    }
}

fn operator(input: &[u8]) -> ParseResult {
    let first = input.first()?;
    if *first == b'-' || *first == b'+' {
        Some((&input[1..], &input[0..1]))
    } else {
        None
    }
}

fn dot(input: &[u8]) -> ParseResult {
    let first = input.first()?;
    if *first == b'.' {
        Some((&input[1..], &input[0..1]))
    } else {
        None
    }
}

fn num(input: &[u8]) -> ParseResult {
    let mut counter = 0;
    for byte in input {
        if byte.is_ascii_digit() {
            counter += 1;
        } else {
            break;
        }
    }
    if counter > 0 {
        Some((&input[counter..], &input[0..counter]))
    } else {
        None
    }
}

fn interger(input: &[u8]) -> ParseResult {
    num(option(operator)(input)?.0)
}

fn decimal<'a>(input: &'a [u8]) -> ParseResult<'a> {
    let first = move |i: &'a [u8]| num(dot(num(i)?.0)?.0);
    let second = move |i: &'a [u8]| dot(num(i)?.0);
    let third = move |i: &'a [u8]| num(dot(i)?.0);
    alt3((first, second, third))(option(operator)(input)?.0)
}



pub fn parse_number(input: &mut Lexer<LogosToken>) -> Number {
    let (int,exclude) = interger(input.slice().as_bytes()).unwrap();
    input.bump(int.len());
    Number::I32( 1)
}

// mod test{
//     use crate::tokenlizer::number_parser::parse_number;
//
//     #[test]
//     fn test_parse_number(){
//         for num in ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]{
//             eprintln!("{:?}",parse_number(num.as_bytes()));
//         }
//     }
// }
