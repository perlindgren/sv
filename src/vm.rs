// simple nom example
//! This module contains a parser that parses a hex color string into a [Color] struct.
//! The hex color string can be in the following format `#RRGGBB`.
//! For example, `#FF0000` is red.

#[allow(unused_imports)]
use crate::ast::*;

use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};
use std::num::ParseIntError;

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
