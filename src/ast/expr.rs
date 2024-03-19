// ast/expr
// expressions
use either::*;

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
pub(crate) struct NonZeroUnsignedNumber {
    pub(crate) h: NonZeroDecimalDigit,
    pub(crate) t: Vec<Either<Us, DecimalDigit>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UnsignedNumber {
    pub(crate) h: DecimalDigit,
    pub(crate) t: Vec<Either<Us, DecimalDigit>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BinaryValue {
    pub(crate) h: BinaryDigit,
    pub(crate) t: Vec<Either<Us, BinaryDigit>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Us(pub(crate) char);

#[derive(Debug, PartialEq)]
pub(crate) struct NonZeroDecimalDigit {
    pub(crate) c: char,
}

#[derive(Debug, PartialEq)]
pub(crate) struct DecimalDigit {
    pub(crate) c: char,
}

#[derive(Debug, PartialEq)]
pub(crate) enum BinaryDigit {
    X(char),
    Z(char),
    Zero(char),
    One(char),
}

use std::fmt;

impl fmt::Display for NonZeroUnsignedNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.h);
        for d in &self.t {
            match d {
                Either::Left(l) => write!(f, "{}", l),
                Either::Right(r) => write!(f, "{}", r),
            };
        }
        Ok(())
    }
}

impl fmt::Display for UnsignedNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.h);
        for d in &self.t {
            match d {
                Either::Left(l) => write!(f, "{}", l),
                Either::Right(r) => write!(f, "{}", r),
            };
        }
        Ok(())
    }
}

impl fmt::Display for Us {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for DecimalDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}

impl fmt::Display for BinaryDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryDigit::X(x) => write!(f, "{}", x),
            BinaryDigit::Z(z) => write!(f, "{}", z),
            BinaryDigit::Zero(c) => write!(f, "{}", c),
            BinaryDigit::One(c) => write!(f, "{}", c),
        }
    }
}

impl fmt::Display for NonZeroDecimalDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}

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
