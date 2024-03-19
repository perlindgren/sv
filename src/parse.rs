// simple nom example

#![allow(unused_imports, dead_code)]

use crate::ast::expr::*;

use nom::character::complete::*;
use nom::combinator::{complete, map_res};
use nom::error::context;

use either::*;
use nom::{
    branch::alt, bytes::complete::*, combinator::*, error::*, multi::fold_many1, sequence::*,
    IResult, Parser,
};

use std::num::ParseIntError;

// pub(crate) fn decimal_number(i: &str) -> IResult<&str, DecimalNumber> {
//     alt((decimal_number, base_unsigned_number))(i)
// }

// pub(crate) fn base_unsigned_number(i: &str)

pub(crate) fn non_zero_unsigned_number(i: &str) -> IResult<&str, NonZeroUnsignedNumber> {
    let (r, h) = non_zero_decimal_digit(i)?;
    let (r, t) = fold_many1(us_decimal_digit, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(r)?;
    Ok((r, NonZeroUnsignedNumber { h, t }))
}

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

pub(crate) fn signed(i: &str) -> IResult<&str, Signed> {
    if let Ok((r, c)) = nom::character::complete::char::<_, ()>('\'')(i) {
        if let Ok((r1, c)) = nom::character::complete::one_of::<_, _, ()>("sS")(r) {
            return Ok((r1, Signed(Some(c))));
        }
        return Ok((r, Signed(None)));
    }
    Ok((i, Signed(None)))
}

pub(crate) fn d(i: &str) -> IResult<&str, D> {
    let (r, c) = nom::character::complete::one_of("dD")(i)?;
    Ok((r, D(c)))
}

pub(crate) fn non_zero_decimal_digit(i: &str) -> IResult<&str, NonZeroDecimalDigit> {
    let (r, c) = nom::character::complete::one_of("123456789")(i)?;
    Ok((r, NonZeroDecimalDigit { c }))
}

pub(crate) fn decimal_digit(i: &str) -> IResult<&str, DecimalDigit> {
    let (r, c) = nom::character::complete::one_of("0123456789")(i)?;
    Ok((r, DecimalDigit { c }))
}

pub(crate) fn binary_digit(i: &str) -> IResult<&str, BinaryDigit> {
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("xX")(i) {
        return Ok((r, BinaryDigit::X(c)));
    }
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("zZ?")(i) {
        return Ok((r, BinaryDigit::Z(c)));
    }
    let (r, c) = nom::character::complete::one_of("01")(i)?;
    Ok((r, BinaryDigit::Digit(c)))
}

pub(crate) fn octal_digit(i: &str) -> IResult<&str, OctalDigit> {
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("xX")(i) {
        return Ok((r, OctalDigit::X(c)));
    }
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("zZ?")(i) {
        return Ok((r, OctalDigit::Z(c)));
    }
    let (r, c) = nom::character::complete::one_of("01234567")(i)?;
    Ok((r, OctalDigit::Digit(c)))
}

pub(crate) fn hex_digit(i: &str) -> IResult<&str, HexDigit> {
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("xX")(i) {
        return Ok((r, HexDigit::X(c)));
    }
    if let Ok((r, c)) = nom::character::complete::one_of::<_, _, ()>("zZ?")(i) {
        return Ok((r, HexDigit::Z(c)));
    }
    let (r, c) = nom::character::complete::one_of("0123456789abcdefABCDEF")(i)?;
    Ok((r, HexDigit::Digit(c)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decimal_number() {
        let s = "1_3_4__5";
        let (_r, c1) = non_zero_unsigned_number(s).unwrap();
        println!("u {:?} ", c1);
        println!("u {}", c1);
        assert_eq!(format!("{}", c1), s);
    }

    #[test]
    fn test_non_zero_unsigned_number() {
        let s = "1_3_4__5";
        let (_r, c1) = non_zero_unsigned_number(s).unwrap();
        println!("u {:?} ", c1);
        println!("u {}", c1);
        assert_eq!(format!("{}", c1), s);
    }

    #[test]
    fn test_non_zero_decimal_digit() {
        let r1 = non_zero_decimal_digit("012");
        let r2 = non_zero_decimal_digit("12");
        println!("r1 {:?} ", r1);
        println!("r2 {:?} ", r2);
        assert!(r1.is_err());
        assert!(r2.is_ok());
    }

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
    fn test_signed() {
        let s = "'s'S'_";
        let (r, s1) = signed(s).unwrap();
        let (r, s2) = signed(r).unwrap();
        let (_r, s3) = signed(r).unwrap();
        // let (_s, d2) = us_decimal_digit(r).unwrap();
        println!("s1 {:?}, {}", s1, s1);
        println!("s2 {:?}, {}", s2, s2);
        println!("s3 {:?}, {}", s3, s3);

        assert_eq!(format!("{}", s1), "'s");
        assert_eq!(format!("{}", s2), "'S");
        assert_eq!(format!("{}", s3), "'");
    }

    #[test]
    fn test_d() {
        let s = "dD_";
        let (r, d1) = d(s).unwrap();
        let (r, d2) = d(r).unwrap();
        let r = d(r);

        println!("d1 {:?}, {}", d1, d1);
        println!("d2 {:?}, {}", d2, d2);
        println!("r {:?}", r);

        assert_eq!(format!("{}", d1), "d");
        assert_eq!(format!("{}", d2), "D");
        assert!(r.is_err());
    }

    #[test]
    fn test_decimal_digit() {
        let (r, d1) = decimal_digit("0234").unwrap();
        let (_s, d2) = decimal_digit(r).unwrap();
        println!("c1 {:?} {}", d1, d1);
        println!("c2 {:?} {}", d2, d2);
    }

    #[test]
    fn test_binary_digit() {
        let s = "010xXzZ?a";
        let (r, b1) = binary_digit(s).unwrap();
        let (r, b2) = binary_digit(r).unwrap();
        let (r, b3) = binary_digit(r).unwrap();
        let (r, b4) = binary_digit(r).unwrap();
        let (r, b5) = binary_digit(r).unwrap();
        let (r, b6) = binary_digit(r).unwrap();
        let (r, b7) = binary_digit(r).unwrap();
        let (r, b8) = binary_digit(r).unwrap();
        let b9 = binary_digit(r);
        println!("b1 {:?} {}", b1, b1);
        println!("b2 {:?} {}", b2, b2);
        println!("b3 {:?} {}", b3, b3);
        println!("b4 {:?} {}", b4, b4);
        println!("b5 {:?} {}", b5, b5);
        println!("b6 {:?} {}", b6, b6);
        println!("b7 {:?} {}", b7, b7);
        println!("b8 {:?} {}", b8, b8);
        println!("b9 {:?} ", b9);
        assert_eq!(format!("{}", b1), "0");
        assert_eq!(format!("{}", b2), "1");
        assert_eq!(format!("{}", b3), "0");
        assert_eq!(format!("{}", b4), "x");
        assert_eq!(format!("{}", b5), "X");
        assert_eq!(format!("{}", b6), "z");
        assert_eq!(format!("{}", b7), "Z");
        assert_eq!(format!("{}", b8), "?");
        assert!(b9.is_err());
    }

    #[test]
    fn test_octal_digit() {
        let s = "013xXzZ?a";
        let (r, b1) = octal_digit(s).unwrap();
        let (r, b2) = octal_digit(r).unwrap();
        let (r, b3) = octal_digit(r).unwrap();
        let (r, b4) = octal_digit(r).unwrap();
        let (r, b5) = octal_digit(r).unwrap();
        let (r, b6) = octal_digit(r).unwrap();
        let (r, b7) = octal_digit(r).unwrap();
        let (r, b8) = octal_digit(r).unwrap();
        let b9 = octal_digit(r);
        println!("b1 {:?} {}", b1, b1);
        println!("b2 {:?} {}", b2, b2);
        println!("b3 {:?} {}", b3, b3);
        println!("b4 {:?} {}", b4, b4);
        println!("b5 {:?} {}", b5, b5);
        println!("b6 {:?} {}", b6, b6);
        println!("b7 {:?} {}", b7, b7);
        println!("b8 {:?} {}", b8, b8);
        println!("b9 {:?} ", b9);
        assert_eq!(format!("{}", b1), "0");
        assert_eq!(format!("{}", b2), "1");
        assert_eq!(format!("{}", b3), "3");
        assert_eq!(format!("{}", b4), "x");
        assert_eq!(format!("{}", b5), "X");
        assert_eq!(format!("{}", b6), "z");
        assert_eq!(format!("{}", b7), "Z");
        assert_eq!(format!("{}", b8), "?");
        assert!(b9.is_err());
    }

    #[test]
    fn test_hex_digit() {
        let s = "afBxXzZ?g";
        let (r, b1) = hex_digit(s).unwrap();
        let (r, b2) = hex_digit(r).unwrap();
        let (r, b3) = hex_digit(r).unwrap();
        let (r, b4) = hex_digit(r).unwrap();
        let (r, b5) = hex_digit(r).unwrap();
        let (r, b6) = hex_digit(r).unwrap();
        let (r, b7) = hex_digit(r).unwrap();
        let (r, b8) = hex_digit(r).unwrap();
        let b9 = hex_digit(r);
        println!("b1 {:?} {}", b1, b1);
        println!("b2 {:?} {}", b2, b2);
        println!("b3 {:?} {}", b3, b3);
        println!("b4 {:?} {}", b4, b4);
        println!("b5 {:?} {}", b5, b5);
        println!("b6 {:?} {}", b6, b6);
        println!("b7 {:?} {}", b7, b7);
        println!("b8 {:?} {}", b8, b8);
        println!("b9 {:?} ", b9);
        assert_eq!(format!("{}", b1), "a");
        assert_eq!(format!("{}", b2), "f");
        assert_eq!(format!("{}", b3), "B");
        assert_eq!(format!("{}", b4), "x");
        assert_eq!(format!("{}", b5), "X");
        assert_eq!(format!("{}", b6), "z");
        assert_eq!(format!("{}", b7), "Z");
        assert_eq!(format!("{}", b8), "?");
        assert!(b9.is_err());
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
