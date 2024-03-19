// simple nom example

#![allow(unused_imports, dead_code)]

use crate::ast::expr::*;

use nom::character::complete::*;
use nom::combinator::{complete, map_res};
use nom::error::context;

use either::*;
use nom::{
    bytes::complete::*, combinator::*, error::*, multi::fold_many1, sequence::*, IResult, Parser,
};

use std::num::ParseIntError;

pub(crate) fn unsigned_number(i: &str) -> IResult<&str, UnsignedNumber> {
    let (r, h) = decimal_digit(i)?;
    let (r, t) = fold_many1(us_decimal_digit, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(r)?;
    Ok((r, UnsignedNumber { h, t }))
}

pub(crate) fn us_decimal_digit(i: &str) -> IResult<&str, Either<Us, DecimalDigit>> {
    if let Ok((r, c)) = nom::character::complete::char::<_, ()>('_')(i) {
        return Ok((r, Either::Left(Us(c))));
    }
    let (r, d) = decimal_digit(i)?;
    Ok((r, Either::Right(d)))
}

pub(crate) fn decimal_digit(i: &str) -> IResult<&str, DecimalDigit> {
    let (r, c) = nom::character::complete::one_of("0123456789")(i)?;
    Ok((r, DecimalDigit { c }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsigned_number() {
        let s = "0_3_4__5";
        let (_r, c1) = unsigned_number(s).unwrap();
        println!("u {:?} ", c1);
        println!("u {}", c1);
        assert_eq!(format!("{}", c1), s);
    }

    #[test]
    fn test_us_decimal_digit() {
        let (r, d1) = us_decimal_digit("0_34").unwrap();
        let (_s, d2) = us_decimal_digit(r).unwrap();
        println!("d1 {:?}, {}", d1, d1);
        println!("d2 {:?}, {}", d2, d2);
    }

    #[test]
    fn test_decimal_digit() {
        let (r, d1) = decimal_digit("0234").unwrap();
        let (_s, d2) = decimal_digit(r).unwrap();
        println!("c1 {:?} {}", d1, d1);
        println!("c2 {:?} {}", d2, d2);
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

/// Helper functions to match and parse hex digits. These are not [Parser]
/// implementations.
mod helper_fns {
    use super::*;

    /// This function is used by [map_res] and it returns a [Result], not [IResult].
    pub fn parse_str_to_hex_num(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    /// This function is used by [take_while_m_n] and as long as it returns `true`
    /// items will be taken from the input.
    pub fn match_is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    pub fn parse_hex_seg(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, match_is_hex_digit),
            parse_str_to_hex_num,
        )(input)
    }
}

/// These are [Parser] implementations that are used by [hex_color_no_alpha].
mod intermediate_parsers {
    use super::*;

    /// Call this to return function that implements the [Parser] trait.
    pub fn gen_hex_seg_parser_fn<'input, E>() -> impl Parser<&'input str, u8, E>
    where
        E: FromExternalError<&'input str, ParseIntError> + ParseError<&'input str>,
    {
        map_res(
            take_while_m_n(2, 2, helper_fns::match_is_hex_digit),
            helper_fns::parse_str_to_hex_num,
        )
    }
}

/// This is the "main" function that is called by the tests.
fn hex_color_no_alpha(input: &str) -> IResult<&str, Color> {
    // This tuple contains 3 ways to do the same thing.
    let it = (
        helper_fns::parse_hex_seg,
        intermediate_parsers::gen_hex_seg_parser_fn(),
        map_res(
            take_while_m_n(2, 2, helper_fns::match_is_hex_digit),
            helper_fns::parse_str_to_hex_num,
        ),
    );
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple(it)(input)?; // same as `it.parse(input)?`
    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_color() {
        let mut input = String::new();
        input.push_str("#2F14DF");
        input.push('🔅');

        let result = dbg!(hex_color_no_alpha(&input));

        let Ok((remainder, color)) = result else {
            panic!();
        };
        assert_eq!(remainder, "🔅");
        assert_eq!(color, Color::new(47, 20, 223));
    }

    #[test]
    fn parse_invalid_color() {
        let result = dbg!(hex_color_no_alpha("🔅#2F14DF"));
        assert!(result.is_err());
    }
}
