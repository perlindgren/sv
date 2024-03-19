// use either::*;
// use nom::{IResult, Parser};
// use nom_derive::*;

// use sv::ast::expr::DecimalDigit;

// fn main() {
//     let c = DecimalDigit::parse("0".as_bytes());
//     println!("c {:?}", c);
// }
extern crate nom;

fn hello_parser(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::tag("hello")(i)
}

fn main() {
    println!("{:?}", hello_parser("hello"));
    println!("{:?}", hello_parser("hello world"));
    println!("{:?}", hello_parser("goodbye hello again"));
}
