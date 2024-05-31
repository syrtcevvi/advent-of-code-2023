use std::fs::read_to_string;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::value,
    error::{Error, ErrorKind},
    multi::many_till,
    IResult,
};

const INPUT_FILE: &str = "input.txt";

fn parse_symbolic_digit(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn parse_single_digit(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, char('1')),
        value(2, char('2')),
        value(3, char('3')),
        value(4, char('4')),
        value(5, char('5')),
        value(6, char('6')),
        value(7, char('7')),
        value(8, char('8')),
        value(9, char('9')),
    ))(input)
}

fn parse_digit(input: &str) -> IResult<&str, u8> {
    alt((parse_single_digit, parse_symbolic_digit))(input)
}

fn parse_first_digit(input: &str) -> IResult<&str, u8> {
    let (input, (_, digit)) = many_till(anychar, parse_digit)(input)?;
    Ok((input, digit))
}

fn parse_last_digit(input: &str) -> IResult<&str, u8> {
    let mut tail_length = 1;
    while tail_length != input.len() {
        if let Ok(res) = parse_digit(&input[input.len() - tail_length..]) {
            return Ok(res);
        }
        tail_length += 1;
    }
    Err(nom::Err::Failure(Error::new(input, ErrorKind::Eof)))
}

fn parse_calibration_value(input: &str) -> IResult<&str, u8> {
    let (_, high_digit) = parse_first_digit(input)?;
    let low_digit = parse_last_digit(input).or(Ok((input, high_digit)))?.1;

    Ok(("", high_digit * 10 + low_digit))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calibration_sum = read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .fold(0_u64, |acc, line| {
            acc + parse_calibration_value(line).unwrap().1 as u64
        });

    println!("{calibration_sum}");

    Ok(())
}
