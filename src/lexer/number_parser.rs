extern crate nom;

use anyhow::Result;
use logos::Lexer;
use nom::{bytes::complete::tag, Err, error, IResult, Needed, Parser, sequence::tuple};
use nom::bytes::streaming::take_while;
use nom::character::is_digit;
use nom::combinator::{map, opt};
use nom::error::Error;
use nom::error::ErrorKind;
use nom::number::complete::double;

use crate::error::TokenParserError;
use crate::lexer::LogosToken;

use super::token_iterator::Number;

fn integer(input: &str) -> IResult<&str, u64> {
    let mut counter = 0;
    for byte in input.as_bytes() {
        if is_digit(*byte) {
            counter += 1;
        } else if *byte == b'.' {
            return Err(Err::Incomplete(Needed::new(1)))
        }
    }
    Ok((&input[counter..], input[..counter].parse::<u64>().unwrap()))
}

fn parse(input: &str) -> Result<(Number, usize)> {
    if let Ok((remain, number)) = integer(input) {
        Ok((Number::Integer(number), remain.len()))
    } else if let Ok((remain, number)) = double::<_, nom::error::Error<&str>>(input) {
        Ok((Number::Float(number), remain.len()))
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
