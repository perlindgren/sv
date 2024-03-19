// ast/expr
// expressions
use either::*;
use nom::{IResult, Parser};
use nom_derive::*;

// inc_or_dec_expression
//     inc_or_dec_operator { attribute_instance } variable_lvalue
//     | variable_lvalue { attribute_instance } inc_or_dec_operator
// conditional_expression
//     cond_predicate ? { attribute_instance } expression : expression
// constant_expression
//     constant_primary
//     | unary_operator { attribute_instance } constant_primary
//     | constant_expression binary_operator { attribute_instance } constant_expression
//     | constant_expression ? { attribute_instance } constant_expression : constant_expression
// constant_mintypmax_expression
//     constant_expression
//     | constant_expression : constant_expression : constant_expression
// constant_param_expression
//     constant_mintypmax_expression
//     | data_type
//     | $
// param_expression
//     mintypmax_expression
//     | data_type
//     | $
// constant_range_expression
//     constant_expression
//     | constant_part_select_range
// constant_part_select_range
//     constant_range
//     | constant_indexed_range
// constant_range
//     constant_expression : constant_expression
// constant_indexed_range
//     constant_expression +: constant_expression
//     | constant_expression -: constant_expression
// expression
//     primary
//     | unary_operator { attribute_instance } primary
//     | inc_or_dec_expression
//     | ( operator_assignment )
//     | expression binary_operator { attribute_instance } expression
//     | conditional_expression
//     | inside_expression
//     | tagged_union_expression
// tagged_union_expression
//     tagged member_identifier [ expression ]
// inside_expression
//     expression inside { open_range_list }
// value_range
//     expression
//     | [ expression : expression ]
// mintypmax_expression
//     expression
//     | expression : expression : expression
// module_path_conditional_expression
//     module_path_expression ? { attribute_instance } module_path_expression : module_path_expression
// module_path_expression
//     module_path_primary
//     | unary_module_path_operator { attribute_instance } module_path_primary
//     | module_path_expression binary_module_path_operator { attribute_instance } module_path_expression
//     | module_path_conditional_expression
// module_path_mintypmax_expression
//     module_path_expression
//     | module_path_expression : module_path_expression : module_path_expression
// part_select_range
//     constant_range
//     | indexed_range
// indexed_range
//     expression +: constant_expression
//     | expression -: constant_expression
// genvar_expression
//     constant_expression

// constant_primary
//     primary_literal
//     | ps_parameter_identifier constant_select
//     | specparam_identifier [ [ constant_range_expression ] ]
//     | genvar_identifier 38
//     | formal_port_identifier constant_select
//     | [ package_scope | class_scope ] enum_identifier
//     | constant_concatenation [ [ constant_range_expression ] ]
//     | constant_multiple_concatenation [ [ constant_range_expression ] ]
//     | constant_function_call
//     | constant_let_expression
//     | ( constant_mintypmax_expression )
//     | constant_cast
//     | constant_assignment_pattern_expression
//     | type_reference 39
// module_path_primary
//     number
//     | identifier
//     | module_path_concatenation
//     | module_path_multiple_concatenation
//     | function_subroutine_call
//     | ( module_path_mintypmax_expression )
// primary
//     primary_literal
//     | [ class_qualifier | package_scope ] hierarchical_identifier select
//     | empty_queue
//     | concatenation [ [ range_expression ] ]
//     | multiple_concatenation [ [ range_expression ] ]
//     | function_subroutine_call
//     | let_expression
//     | ( mintypmax_expression )
//     | cast
//     | assignment_pattern_expression
//     | streaming_concatenation
//     | sequence_method_call
//     | this 40
//     | $ 41
//     | null
// class_qualifier
//     [ local:: 42 ] [ implicit_class_handle . | class_scope ]
// range_expression
//     expression
//     | part_select_range
// primary_literal
//     number
//     | time_literal
//     | unbased_unsized_literal
//     | string_literal
// time_literal43
//     unsigned_number time_unit
//     | fixed_point_number time_unit
// time_unit
//     s
//     | ms
//     | us
//     | ns
//     | ps
//     | fs
// implicit_class_handle40
//     this
//     | super
//     | this . super
// bit_select
//     { [ expression ] }
// select
//     [ { . member_identifier bit_select } . member_identifier ] bit_select [ [ part_select_range ] ]
// nonrange_select
//     [ { . member_identifier bit_select } . member_identifier ] bit_select
// constant_bit_select
//     { [ constant_expression ] }
// constant_select
//     [ { . member_identifier constant_bit_select } . member_identifier ] constant_bit_select [ [ constant_part_select_range ] ]
// constant_cast
//     casting_type ' ( constant_expression )
// constant_let_expression
//     let_expression 44
// cast
//     casting_type ' ( expression )

// Expression left-side values

// net_lvalue
//     ps_or_hierarchical_net_identifier constant_select
//     | { net_lvalue { , net_lvalue } }
//     | [ assignment_pattern_expression_type ] assignment_pattern_net_lvalue
// variable_lvalue
//     [ implicit_class_handle . | package_scope ] hierarchical_variable_identifier select 45
//     | { variable_lvalue { , variable_lvalue } }
//     | [ assignment_pattern_expression_type ] assignment_pattern_variable_lvalue
//     | streaming_concatenation 46
// nonrange_variable_lvalue
//     [ implicit_class_handle . | package_scope ] hierarchical_variable_identifier nonrange_select

#[derive(Debug, PartialEq)]
enum ModulePathPrimary {
    Number,
}

#[derive(Debug, PartialEq)]
enum Numbers {
    IntegralNumber(IntegralNumber),
    RealNumber,
}

#[derive(Debug, PartialEq)]
enum IntegralNumber {
    DecimalNumber(DecimalNumber),
    // OctalNumber,
    // BinaryNumber(Option(Size), BinaryBase, BinaryValue),
    // HexNumber,
}

#[derive(Debug, PartialEq)]
enum DecimalNumber {
    UnsignedNumber(UnsignedNumber),
    // | [ size ] decimal_base unsigned_number
    // | [ size ] decimal_base x_digit { _ }
    // | [ size ] decimal_base z_digit { _ }
}

#[derive(Debug, PartialEq)]
struct UnsignedNumber {
    h: DecimalDigit,
    t: Vec<SvEither<Us, DecimalDigit>>,
}

#[derive(Debug, PartialEq)]
struct Us(char);

#[derive(Debug, PartialEq)]
struct SvEither<L, R>(Either<L, R>);

#[derive(Debug, PartialEq, Nom)]
struct DecimalDigits {
    c: Vec<u8>,
}
#[derive(Debug, PartialEq, Nom)]
struct DecimalDigit {
    c: u8,
}

#[test]
fn test() {
    let c = DecimalDigit::parse("0".as_bytes());
    println!("c {:?}", c);
}
// use nom::combinator::map;
// use nom::number::complete::le_u16;
// impl<'a> Parse<&'a [u8]> for S2 {
//     fn parse(i: &'a [u8]) -> IResult<&'a [u8], S2> {
//         map(
//             u8,           // little-endian
//             |c| S2 { c }, // return a struct S2
//         )(i)
//     }
// }

// struct NonZeroUnsignedNumber {
//     h: NonZeroDecimalDigit,
//     t: Vec<__DecimalDigit>,
// }

// enum NonZeroDecimalDigit { _ | decimal_digit }

//     non_zero_unsigned_number

// enum DecimalNumber {
//     UnsignedNumber,

// }
//     | [ size ] decimal_base unsigned_number
//     | [ size ] decimal_base x_digit { _ }
//     | [ size ] decimal_base z_digit { _ }
// binary_number
//     [ size ] binary_base binary_value
// octal_number
//     [ size ] octal_base octal_value
// hex_number
//     [ size ] hex_base hex_value
// sign
//     +
//     | -
// size
//     non_zero_unsigned_number
// non_zero_unsigned_number32
//     non_zero_decimal_digit { _ | decimal_digit }
// real_number32
//     fixed_point_number
//     | unsigned_number [ . unsigned_number ] exp [ sign ] unsigned_number
// fixed_point_number32
//     unsigned_number . unsigned_number
// exp
//     e
//     | E
// unsigned_number32
//     decimal_digit { _ | decimal_digit }
// binary_value32
//     binary_digit { _ | binary_digit }
// octal_value32
//     octal_digit { _ | octal_digit }
// hex_value32
//     hex_digit { _ | hex_digit }
// decimal_base32
//     ' [ s | S ] d
//     | ' [ s | S ] D
// binary_base32
//     ' [ s | S ] b
//     | ' [ s | S ] B
// octal_base32
//     ' [ s | S ] o
//     | ' [ s | S ] O
// hex_base32
//     ' [ s | S ] h
//     | ' [ s | S ] H
