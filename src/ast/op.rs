// ast/op
// operators

#![allow(unused)] // during early dev

type Identifier = String;
type Integer = i32;
type NonZeroDecimalDigit = u8;
type DecimalDigit = u8;
type BinaryDigit = u8;

#[derive(Debug, PartialEq)]
enum UnaryOperator {
    Add,     //     +
    Sub,     //     | -
    Not,     //     | !
    Inv,     //     | ~
    And,     //     | &
    InvAnd,  //     | ~&
    Or,      //     | |
    InvOr,   //     | ~|
    Exor,    //     | ^
    InvExor, //     | ~^
    ExorInv, //     | ^~
}

#[derive(Debug, PartialEq)]
enum BinaryOperator {
    Add,             // +
    Sub,             // | -
    Mul,             // | *
    Div,             // | /
    Mod,             // | %
    Eq,              // | ==
    NotEq,           // | !=
    Equal,           // | ===
    NotEqual,        // | !==
    EqOption,        // | ==?
    NotEqOption,     // | !=?
    And,             // | &&
    Or,              // | ||
    Exp,             // | **
    Less,            // | <
    LessEq,          // | <=
    Greater,         // | >
    GreaterEq,       // | >=
    BinaryAnd,       // | &
    BinaryOr,        // | |
    Exor,            // | ^
    ExorInv,         // | ^~
    InvExor,         // | ~^
    RightShift,      // | >>
    LeftShift,       // | <<
    RightShiftArith, // | >>>
    LeftShiftArith,  // | <<<
    To,              // | ->
    FromTo,          // | <->
}

#[derive(Debug, PartialEq)]
enum IncOrDecOperator {
    Inc, // ++
    Dec, // --
}

#[derive(Debug, PartialEq)]
enum UnaryModulePathOperator {
    Not,     // !
    Inv,     // ~
    And,     // &
    InvAnd,  // ~&
    Or,      // !
    InvOr,   // ~|
    Exor,    // ^
    InvExor, // ~^
    ExorInv, // ^~
}

#[derive(Debug, PartialEq)]
enum BinaryModulePathOperator {
    Equal,     // ==
    NotEqual,  // !=
    And,       // &&
    Or,        // ||
    BinaryAnd, // &
    BinaryOr,  // |
    Exor,      // ^
    ExorInv,   // ^~
    InvExor,   // ~^
}
