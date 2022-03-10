extern crate nom;

use anyhow::Result;
use logos::Lexer;
use nom::{
    bytes::complete::tag,
    IResult,
    sequence::tuple,
};
use nom::bytes::streaming::take_while;
use nom::character::complete::char;
use nom::combinator::opt;

use crate::error::TokenParserError;
use crate::lexer::LogosToken;

use super::token_iterator::Number;

fn unsigned_integer(input: &str) -> IResult<&str, u64> {
    let i = input.parse::<u64>().unwrap();
    Ok(("", i))
}

fn unsigned_float(input: &str) -> IResult<&str, f64> {
    let x: (&str, (&str, char, &str)) = tuple((take_while(|x: char| x.is_digit(10)), char('.'), take_while(|x: char| x.is_digit(10))))(input)?;
    let result = format!("{}.{}", x.1.0, x.1.2).parse::<f64>().unwrap();
    Ok((x.0, result))
}


fn integer(input: &str) -> IResult<&str, (u64, bool)> {
    let result = opt(tag("-"))(input)?;
    let mut negative = false;
    if let Some(_) = result.1 {
        negative = true;
    }
    let integer = unsigned_integer(result.0)?;
    Ok((integer.0, (integer.1, negative)))
}

fn float(input: &str) -> IResult<&str, (f64, bool)> {
    let result = opt(tag("-"))(input)?;
    let mut negative = false;
    if result.1.is_some() {
        negative = true;
    }
    let float = unsigned_float(result.0)?;
    Ok((float.0, (float.1, negative)))
}

fn parse(input: &str) -> Result<(Number, usize)> {
    if let Ok((remain, (number, signed))) = float(input) {
        Ok((Number::Float(number, signed), remain.len()))
    } else if let Ok((remain, (number, signed))) = integer(input) {
        Ok((Number::Integer(number, signed), remain.len()))
    } else {
        Err(TokenParserError::unrecognized_token().into())
    }
}

pub fn parse_number(input: &mut Lexer<LogosToken>) -> Result<Number> {
    let result = parse(input.slice())?;
    Ok(result.0)
}

#[allow(unused)]
mod test {
    use crate::lexer::number_parser::parse;

    #[test]
    fn test_parse_number() {
        for num in ["-2.33 abc", "100000 sfsdas", "-", "3.14", "4.1", "-0.9", "210", "0", "3e+7", "-1.2759877"] {
            eprintln!("{:?}", parse(num))
        }
    }
}
