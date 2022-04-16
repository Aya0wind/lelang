extern crate nom;

use anyhow::Result;
use logos::Lexer;
use nom::{Err, IResult, Needed};
use nom::character::is_digit;
use nom::number::complete::double;

use crate::lexer::LogosToken;

use super::token_iterator::Number;

fn integer(input: &str) -> IResult<&str, u64> {
    let mut counter = 0;
    for byte in input.as_bytes() {
        if is_digit(*byte) {
            counter += 1;
        } else if *byte == b'.' {
            return Err(Err::Incomplete(Needed::new(1)));
        }
    }
    Ok((&input[counter..], input[..counter].parse::<u64>().unwrap()))
}

fn parse(input: &str) -> Result<(Number, usize)> {
    if let Ok((remain, number)) = integer(input) {
        Ok((Number::Integer(number), remain.len()))
    } else {
        let (remain, number) = double::<_, nom::error::Error<&str>>(input).unwrap();
        Ok((Number::Float(number), remain.len()))
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
