// auto-generated: "lalrpop 0.19.5"
// sha3: a6c9a8ee418c30fb4e403765faee82556968c09b95ff8dc86636b49c77cc
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate alloc;
extern crate core;

mod __parse__Expr {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens
    )]

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input> {
        Variant0(&'input str),
        Variant1(Type),
        Variant2(core::option::Option<Type>),
        Variant3(Vec<Stmt<'input>>),
        Variant4(core::option::Option<Vec<Stmt<'input>>>),
        Variant5(Arg<'input>),
        Variant6(alloc::vec::Vec<Arg<'input>>),
        Variant7(Expr<'input>),
        Variant8(alloc::vec::Vec<Expr<'input>>),
        Variant9(Stmt<'input>),
        Variant10(alloc::vec::Vec<Stmt<'input>>),
        Variant11(BinOp),
        Variant12(core::option::Option<Arg<'input>>),
        Variant13(Vec<Arg<'input>>),
        Variant14(Vec<Expr<'input>>),
        Variant15(core::option::Option<Expr<'input>>),
        Variant16(Function<'input>),
        Variant17(Item<'input>),
        Variant18(alloc::vec::Vec<Item<'input>>),
        Variant19(Literal<'input>),
        Variant20(Program<'input>),
        Variant21(core::option::Option<Stmt<'input>>),
        Variant22(UnaryOp),
    }
    const __ACTION: &[i8] = &[
        // State 0
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 1
        0, -45, -45, 0, -45, 0, 57, -45, 58, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45, 0, -45, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, // State 2
        0, 0, 59, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -85, -85, -85, 0, 0, 0, 0, // State 3
        0, -52, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 64, 65, 0, -52, 66, 67, 0, -52, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, -52, -52, 0, 0, 0, 0, // State 4
        0, 70, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, 0, 0, 0, 71, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, // State 5
        0, -24, -24, 0, -24, 74, -24, -24, -24, 0, 0, 75, 0, -24, -24, -24, 0, -24, -24, -24, 0,
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, // State 6
        0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -57, 76, -57, 0, 0, 0, 0, // State 7
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 8
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 9
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22, 0, 0,
        23, 24, 25, 0, 26, 10, 0, -95, 51, 52, 53, 54, // State 10
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 11
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 12
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 13
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 14
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 15
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 16
        49, 0, 0, 9, -42, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, // State 18
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 19
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22, 0, 0,
        23, 24, 25, 0, 26, 10, 0, -97, 51, 52, 53, 54, // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, // State 21
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, // State 23
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 24
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 25
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 26
        0, -25, -25, 0, -25, 74, -25, -25, -25, 0, 0, 75, 0, -25, -25, -25, 0, -25, -25, -25, 0,
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, // State 27
        0, 70, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 71, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, // State 28
        0, -46, -46, 0, -46, 0, 57, -46, 58, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46, 0, -46, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, // State 29
        0, -53, -53, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 64, 65, 0, -53, 66, 67, 0, -53, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, -53, 0, 0, 0, 0, // State 30
        0, 0, 59, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -86, -86, -86, 0, 0, 0, 0, // State 31
        49, 0, 0, 9, -44, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 10, 0, 0, 0, 0, 0, 0, // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 10, 0, 0, 0, 0, 0, 0, // State 34
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 35
        49, 0, 0, 9, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 10, 0, 0, 51, 52, 53, 54, // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 10, 0, 0, 0, 0, 0, 0, // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 10, 0, 0, 0, 0, 0, 0, // State 38
        0, -88, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, 0, -88, -88, -88,
        -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, -88, -88, 0, 0, 0, 0,
        // State 39
        0, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33,
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 41
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, -74, -74, 0, -74, -74, -74,
        -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0,
        // State 42
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, -75, -75, 0, -75, -75, -75,
        -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0,
        // State 43
        0, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35,
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0,
        // State 44
        0, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32,
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0,
        // State 45
        0, -113, -113, 17, -113, -113, -113, -113, -113, 0, 18, -113, 0, -113, -113, -113, 0, -113,
        -113, -113, 19, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, 0, 0, 0, 0,
        // State 46
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, -73, -73, 0, -73, -73, -73, 0, -73, -73, -73,
        -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0,
        // State 47
        0, -81, -81, 0, -81, -81, -81, -81, -81, 0, 0, -81, 0, -81, -81, -81, 0, -81, -81, -81, 0,
        -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, -81, 0, 0, 0, 0, // State 48
        -115, 0, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -115, 0, 0, -115, -115, -115, -115, // State 49
        -116, 0, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -116, 0, 0, -116, -116, -116, -116, // State 50
        0, -108, -108, -108, -108, -108, -108, -108, -108, 0, -108, -108, 0, -108, -108, -108, 0,
        -108, -108, -108, -108, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, -108, -108, 0, 0,
        0, 0, // State 51
        0, -60, -60, -60, -60, -60, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60,
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, -60, 0, 0, 0, 0,
        // State 52
        0, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65, -65,
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, -65, 0, 0, 0, 0,
        // State 53
        0, -83, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, 0, -83, -83, -83, -83, -83, -83,
        -83, -83, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, -83, -83, 0, 0, 0, 0,
        // State 54
        -23, 0, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23, // State 55
        -22, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -22, 0, 0, -22, -22, -22, -22, // State 56
        -87, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, -87, // State 57
        -78, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, -78, // State 58
        -76, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, -76, // State 59
        -48, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -48, 0, 0, -48, -48, -48, -48, // State 60
        -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -50, 0, 0, -50, -50, -50, -50, // State 61
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -47, 0, 0, -47, -47, -47, -47, // State 62
        -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -49, 0, 0, -49, -49, -49, -49, // State 63
        -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -71, 0, 0, -71, -71, -71, -71, // State 64
        -72, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -72, 0, 0, -72, -72, -72, -72, // State 65
        -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -63, 0, 0, -63, -63, -63, -63, // State 66
        -64, 0, 0, -64, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -64, 0, 0, -64, -64, -64, -64, // State 67
        -54, 0, 0, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -54, 0, 0, -54, -54, -54, -54, // State 68
        -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -55, 0, 0, -55, -55, -55, -55, // State 69
        -84, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, -84, // State 70
        -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -56, 0, 0, -56, -56, -56, -56, // State 71
        -80, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, -80, // State 72
        -79, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, -79, // State 73
        -109, 0, 0, -109, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -109, 0, 0, -109, -109, -109, -109, // State 74
        -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -51, 0, 0, -51, -51, -51, -51, // State 75
        -77, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, -77, // State 76
        0, -114, -114, 0, -114, -114, -114, -114, -114, 0, 0, -114, 0, -114, -114, -114, 0, -114,
        -114, -114, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, -114, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 89, 0, 0, 0, 0, // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, // State 81
        0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, 0,
        -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, -82, -82, 0, 0, 0, 0, // State 82
        0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 83
        0, 0, 0, 0, -41, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 84
        0, -91, -91, -91, -91, -91, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91,
        -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 86
        0, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34,
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, // State 88
        0, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36,
        -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0,
        // State 89
        -20, 0, 0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        0, 0, -20, -20, -20, 0, -20, -20, 0, -20, -20, -20, -20, -20, // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, // State 94
        0, 0, 0, 0, -43, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 95
        0, -90, -90, -90, -90, -90, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90,
        -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, -90, -90, 0, 0, 0, 0,
        // State 96
        -15, 0, 0, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, // State 97
        0, -89, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89,
        -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, -89, -89, 0, 0, 0, 0,
        // State 98
        -21, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21,
        0, 0, -21, -21, -21, 0, -21, -21, 0, -21, -21, -21, -21, -21, // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, // State 101
        -16, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, -16, // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 41 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0, // State 1
        -45, // State 2
        -85, // State 3
        -52, // State 4
        -26, // State 5
        -24, // State 6
        -57, // State 7
        0, // State 8
        0, // State 9
        0, // State 10
        0, // State 11
        0, // State 12
        0, // State 13
        0, // State 14
        0, // State 15
        0, // State 16
        0, // State 17
        0, // State 18
        0, // State 19
        0, // State 20
        0, // State 21
        0, // State 22
        0, // State 23
        0, // State 24
        0, // State 25
        0, // State 26
        -25, // State 27
        -27, // State 28
        -46, // State 29
        -53, // State 30
        -86, // State 31
        0, // State 32
        0, // State 33
        0, // State 34
        0, // State 35
        0, // State 36
        0, // State 37
        0, // State 38
        -88, // State 39
        -33, // State 40
        -117, // State 41
        -74, // State 42
        -75, // State 43
        -35, // State 44
        -32, // State 45
        -113, // State 46
        -73, // State 47
        -81, // State 48
        0, // State 49
        0, // State 50
        -108, // State 51
        -60, // State 52
        -65, // State 53
        -83, // State 54
        0, // State 55
        0, // State 56
        0, // State 57
        0, // State 58
        0, // State 59
        0, // State 60
        0, // State 61
        0, // State 62
        0, // State 63
        0, // State 64
        0, // State 65
        0, // State 66
        0, // State 67
        0, // State 68
        0, // State 69
        0, // State 70
        0, // State 71
        0, // State 72
        0, // State 73
        0, // State 74
        0, // State 75
        0, // State 76
        -114, // State 77
        0, // State 78
        0, // State 79
        0, // State 80
        0, // State 81
        -82, // State 82
        0, // State 83
        0, // State 84
        -91, // State 85
        0, // State 86
        -34, // State 87
        0, // State 88
        -36, // State 89
        0, // State 90
        0, // State 91
        0, // State 92
        0, // State 93
        0, // State 94
        0, // State 95
        -90, // State 96
        0, // State 97
        -89, // State 98
        0, // State 99
        0, // State 100
        0, // State 101
        0, // State 102
        0, // State 103
        0, // State 104
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            9 => 31,
            12 => 19,
            13 => 10,
            14 => match state {
                12 => 28,
                _ => 1,
            },
            15 => match state {
                15 => 30,
                _ => 2,
            },
            19 => 38,
            20 => match state {
                32 => 99,
                33 => 100,
                36 => 103,
                37 => 104,
                _ => 39,
            },
            22 => 82,
            23 => match state {
                13 => 29,
                _ => 3,
            },
            24 => 12,
            25 => 71,
            26 => match state {
                11 => 27,
                _ => 4,
            },
            27 => 13,
            28 => 67,
            29 => match state {
                21 => 32,
                25 => 33,
                34 => 36,
                0 => 40,
                8 => 77,
                16 => 83,
                18 => 85,
                23 => 92,
                24 => 93,
                31 => 94,
                35 => 102,
                _ => 78,
            },
            31 => 41,
            33 => 59,
            34 => 60,
            35 => 42,
            39 => 61,
            40 => 62,
            41 => 43,
            42 => 11,
            43 => 15,
            44 => 54,
            45 => 14,
            46 => match state {
                10 => 26,
                _ => 5,
            },
            47 => match state {
                17 => 84,
                20 => 90,
                22 => 91,
                _ => 44,
            },
            48 => 68,
            49 => 6,
            50 => 55,
            51 => 45,
            53 => 79,
            54 => match state {
                19 => 87,
                _ => 80,
            },
            56 => 46,
            57 => 72,
            59 => match state {
                7 => 76,
                14 => 81,
                _ => 47,
            },
            60 => 7,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""bool""###,
            r###""else""###,
            r###""fn""###,
            r###""for""###,
            r###""if""###,
            r###""in""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""while""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[+-]?[0-9]+[.][0-9]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-z]+"#"###,
        ];
        __TERMINAL
            .iter()
            .enumerate()
            .filter_map(|(index, terminal)| {
                let next_state = __action(__state, index);
                if next_state == 0 {
                    None
                } else {
                    Some(alloc::string::ToString::to_string(terminal))
                }
            })
            .collect()
    }
    pub(crate) struct __StateMachine<'input> {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Expr<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
            Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
            0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 41 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<'input>(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize> {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(17, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            Token(25, _) if true => Some(21),
            Token(26, _) if true => Some(22),
            Token(27, _) if true => Some(23),
            Token(28, _) if true => Some(24),
            Token(29, _) if true => Some(25),
            Token(30, _) if true => Some(26),
            Token(31, _) if true => Some(27),
            Token(32, _) if true => Some(28),
            Token(33, _) if true => Some(29),
            Token(34, _) if true => Some(30),
            Token(35, _) if true => Some(31),
            Token(36, _) if true => Some(32),
            Token(37, _) if true => Some(33),
            Token(38, _) if true => Some(34),
            Token(39, _) if true => Some(35),
            Token(40, _) if true => Some(36),
            Token(0, _) if true => Some(37),
            Token(1, _) if true => Some(38),
            Token(2, _) if true => Some(39),
            Token(3, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<'input>(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input> {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
            | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34
            | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(4, __tok0)
                | Token(5, __tok0)
                | Token(6, __tok0)
                | Token(7, __tok0)
                | Token(8, __tok0)
                | Token(9, __tok0)
                | Token(10, __tok0)
                | Token(11, __tok0)
                | Token(12, __tok0)
                | Token(13, __tok0)
                | Token(14, __tok0)
                | Token(15, __tok0)
                | Token(16, __tok0)
                | Token(17, __tok0)
                | Token(18, __tok0)
                | Token(19, __tok0)
                | Token(20, __tok0)
                | Token(21, __tok0)
                | Token(22, __tok0)
                | Token(23, __tok0)
                | Token(24, __tok0)
                | Token(25, __tok0)
                | Token(26, __tok0)
                | Token(27, __tok0)
                | Token(28, __tok0)
                | Token(29, __tok0)
                | Token(30, __tok0)
                | Token(31, __tok0)
                | Token(32, __tok0)
                | Token(33, __tok0)
                | Token(34, __tok0)
                | Token(35, __tok0)
                | Token(36, __tok0)
                | Token(37, __tok0)
                | Token(38, __tok0)
                | Token(39, __tok0)
                | Token(40, __tok0)
                | Token(0, __tok0)
                | Token(1, __tok0)
                | Token(2, __tok0)
                | Token(3, __tok0)
                    if true =>
                {
                    __Symbol::Variant0(__tok0)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            let __builder = super::__intern_token::new_builder();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<'input>(
            &self,
            input: &'input str,
        ) -> Result<Expr<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<'input>(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Expr<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => __reduce0(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            1 => __reduce1(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            2 => __reduce2(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            3 => __reduce3(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            4 => __reduce4(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            5 => __reduce5(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            6 => __reduce6(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            7 => __reduce7(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            8 => __reduce8(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            9 => __reduce9(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            10 => __reduce10(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            11 => __reduce11(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            12 => __reduce12(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            13 => __reduce13(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            14 => __reduce14(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            15 => __reduce15(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            16 => __reduce16(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            17 => __reduce17(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            18 => __reduce18(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            19 => __reduce19(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            20 => __reduce20(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            21 => __reduce21(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            22 => __reduce22(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            23 => __reduce23(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            24 => __reduce24(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            25 => __reduce25(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            26 => __reduce26(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            27 => __reduce27(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            28 => __reduce28(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            29 => __reduce29(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            30 => __reduce30(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            31 => __reduce31(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            32 => __reduce32(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            33 => __reduce33(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            34 => __reduce34(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            35 => __reduce35(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            36 => __reduce36(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            37 => __reduce37(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            38 => __reduce38(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            39 => __reduce39(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            40 => __reduce40(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            41 => __reduce41(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            42 => __reduce42(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            43 => __reduce43(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            44 => __reduce44(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            45 => __reduce45(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            46 => __reduce46(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            47 => __reduce47(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            48 => __reduce48(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            49 => __reduce49(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            50 => __reduce50(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            51 => __reduce51(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            52 => __reduce52(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            53 => __reduce53(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            54 => __reduce54(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            55 => __reduce55(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            56 => __reduce56(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            57 => __reduce57(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            58 => __reduce58(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            59 => __reduce59(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            60 => __reduce60(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            61 => __reduce61(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            62 => __reduce62(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            63 => __reduce63(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            64 => __reduce64(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            65 => __reduce65(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            66 => __reduce66(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            67 => __reduce67(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            68 => __reduce68(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            69 => __reduce69(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            70 => __reduce70(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            71 => __reduce71(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            72 => __reduce72(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            73 => __reduce73(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            74 => __reduce74(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            75 => __reduce75(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            76 => __reduce76(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            77 => __reduce77(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            78 => __reduce78(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            79 => __reduce79(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            80 => __reduce80(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            81 => __reduce81(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            82 => __reduce82(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            83 => __reduce83(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            84 => __reduce84(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            85 => __reduce85(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            86 => __reduce86(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            87 => __reduce87(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            88 => __reduce88(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            89 => __reduce89(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            90 => __reduce90(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            91 => __reduce91(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            92 => __reduce92(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            93 => __reduce93(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            94 => __reduce94(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            95 => __reduce95(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            96 => __reduce96(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            97 => __reduce97(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            98 => __reduce98(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            99 => __reduce99(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            100 => __reduce100(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            101 => __reduce101(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            102 => __reduce102(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            103 => __reduce103(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            104 => __reduce104(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            105 => __reduce105(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            106 => __reduce106(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            107 => __reduce107(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            108 => __reduce108(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            109 => __reduce109(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            110 => __reduce110(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            111 => __reduce111(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            112 => __reduce112(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            113 => __reduce113(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            114 => __reduce114(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            115 => __reduce115(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            116 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                return Some(Ok(__nt));
            }
            117 => __reduce117(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            118 => __reduce118(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            119 => __reduce119(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            120 => __reduce120(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            _ => panic!("invalid action code {}", __action),
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Arg<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant11<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, BinOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant7<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Expr<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant16<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Function<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant17<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Item<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant19<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Literal<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant20<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Program<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant9<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Stmt<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant1<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Type, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant22<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, UnaryOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant13<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant14<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant3<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant6<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant8<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant18<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant10<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant12<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant15<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant21<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant2<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Type>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant4<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Vec<Stmt<'input>>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant0<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, &'input str, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    pub(crate) fn __reduce0<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>) = "->", Type => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? = "->", Type => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? =  => ActionFn(82);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>) = "else", Body => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? = "else", Body => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? =  => ActionFn(77);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",") = Arg, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* =  => ActionFn(90);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action90(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(91);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = Arg, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",") = Expr, "," => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action102(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* =  => ActionFn(100);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(101);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = Expr, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";") = Stmt, ";" => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* =  => ActionFn(95);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action95(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* = (<Stmt> ";")+ => ActionFn(96);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = Stmt, ";" => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = (<Stmt> ";")+, Stmt, ";" => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce21<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Plus => ActionFn(69);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Minus => ActionFn(70);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = MultiplicitveExpr => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = AdditiveExpr, AdditionOp, MultiplicitveExpr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = EqualityExpr => ActionFn(25);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = AndExpr, LogicalAnd, EqualityExpr => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg = Name, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? = Arg => ActionFn(88);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? =  => ActionFn(89);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce30<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Args = "(", Comma<Arg>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce31<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Name => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Body => ActionFn(42);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = "(", Expr, ")" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce34<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Literal => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Body = "{", Semi<Stmt>, "}" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = Arg => ActionFn(127);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce37<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> =  => ActionFn(128);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action128(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce38<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(129);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce39<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = Expr => ActionFn(131);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce41<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> =  => ActionFn(132);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action132(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce42<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(134);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = AdditiveExpr => ActionFn(29);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce45<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = ComparisonExpr, ComparisonOp, AdditiveExpr => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce46<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThan => ActionFn(65);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce47<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThan => ActionFn(66);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce48<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThanEquals => ActionFn(67);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce49<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThanEquals => ActionFn(68);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce50<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Devide = "/" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce51<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = ComparisonExpr => ActionFn(27);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce53<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = Equals => ActionFn(63);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce54<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = NotEquals => ActionFn(64);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Equals = "==" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce56<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr = OrExpr => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce57<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? = Expr => ActionFn(98);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce58<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? =  => ActionFn(99);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce59<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // FloatLit = r#"[+-]?[0-9]+[.][0-9]*"# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, "->", Type, Body => ActionFn(110);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action110(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (6, 32)
    }
    pub(crate) fn __reduce61<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, Body => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce62<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThan = ">" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThanEquals = ">=" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // IntLit = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item = Function => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce66<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* =  => ActionFn(84);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* = Item+ => ActionFn(85);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item+, Item => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce70<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThan = "<" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThanEquals = "<=" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = StringLit => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = FloatLit => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = IntLit => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalAnd = "&&" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce76<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalOr = "||" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce77<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Minus = "-" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Times => ActionFn(71);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce79<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Devide => ActionFn(72);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = UnaryExpr => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce81<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = MultiplicitveExpr, MultiplicationOp, UnaryExpr => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce82<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Name = r#"[a-z]+"# => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce83<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // NotEquals = "!=" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce84<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = AndExpr => ActionFn(23);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce85<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = OrExpr, LogicalOr, AndExpr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce86<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Plus = "+" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce87<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = BaseExpr => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce88<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(38);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce89<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "(", Comma<Expr>, ")" => ActionFn(39);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce90<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, ".", Name => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 51)
    }
    pub(crate) fn __reduce91<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program =  => ActionFn(135);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action135(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce92<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program = Item+ => ActionFn(136);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce93<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = Stmt => ActionFn(137);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce94<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> =  => ActionFn(138);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action138(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce95<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+, Stmt => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce96<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+ => ActionFn(140);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce97<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = Expr => ActionFn(15);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce98<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "let", Name, "=", Expr => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 54)
    }
    pub(crate) fn __reduce99<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body, "else", Body => ActionFn(113);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action113(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce100<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce101<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "for", Name, "in", Expr, Body => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce102<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "print", Expr => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce103<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "while", Expr, Body => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce104<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "return", Expr => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce105<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? = Stmt => ActionFn(93);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? =  => ActionFn(94);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action94(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 55)
    }
    pub(crate) fn __reduce107<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // StringLit = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Times = "*" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce109<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "int" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce110<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "bool" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "string" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce112<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = PostfixExpr => ActionFn(35);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce113<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = UnaryOp, UnaryExpr => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce114<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "!" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce115<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "-" => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce117<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Function = Function => ActionFn(2);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce118<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Item = Item => ActionFn(1);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce119<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce120<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Stmt = Stmt => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 65)
    }
}
pub use self::__parse__Expr::ExprParser;

mod __parse__Function {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens
    )]

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input> {
        Variant0(&'input str),
        Variant1(Type),
        Variant2(core::option::Option<Type>),
        Variant3(Vec<Stmt<'input>>),
        Variant4(core::option::Option<Vec<Stmt<'input>>>),
        Variant5(Arg<'input>),
        Variant6(alloc::vec::Vec<Arg<'input>>),
        Variant7(Expr<'input>),
        Variant8(alloc::vec::Vec<Expr<'input>>),
        Variant9(Stmt<'input>),
        Variant10(alloc::vec::Vec<Stmt<'input>>),
        Variant11(BinOp),
        Variant12(core::option::Option<Arg<'input>>),
        Variant13(Vec<Arg<'input>>),
        Variant14(Vec<Expr<'input>>),
        Variant15(core::option::Option<Expr<'input>>),
        Variant16(Function<'input>),
        Variant17(Item<'input>),
        Variant18(alloc::vec::Vec<Item<'input>>),
        Variant19(Literal<'input>),
        Variant20(Program<'input>),
        Variant21(core::option::Option<Stmt<'input>>),
        Variant22(UnaryOp),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 2
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 4
        0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 54, 0,
        0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, // State 6
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0,
        21, 22, 23, 0, 24, 7, 0, -95, 70, 71, 72, 48, // State 7
        0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 9
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0,
        21, 22, 23, 0, 24, 7, 0, -97, 70, 71, 72, 48, // State 10
        0, -45, -45, 0, -45, 0, 80, -45, 81, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45, 0, -45, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, // State 11
        0, 0, 82, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -85, -85, -85, 0, 0, 0, 0, // State 12
        0, -52, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 87, 88, 0, -52, 89, 90, 0, -52, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, -52, -52, 0, 0, 0, 0, // State 13
        0, 93, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, 0, 0, 0, 94, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, // State 14
        0, -24, -24, 0, -24, 97, -24, -24, -24, 0, 0, 98, 0, -24, -24, -24, 0, -24, -24, -24, 0,
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, // State 15
        0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -57, 99, -57, 0, 0, 0, 0, // State 16
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 17
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 19
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 21
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 22
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 23
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 54, 0,
        0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, // State 25
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 26
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 27
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 28
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 29
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 30
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 31
        68, 0, 0, 18, -42, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, // State 33
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 36
        0, -25, -25, 0, -25, 97, -25, -25, -25, 0, 0, 98, 0, -25, -25, -25, 0, -25, -25, -25, 0,
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, // State 37
        0, 93, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 94, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, // State 38
        0, -46, -46, 0, -46, 0, 80, -46, 81, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46, 0, -46, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, // State 39
        0, -53, -53, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 87, 88, 0, -53, 89, 90, 0, -53, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, -53, 0, 0, 0, 0, // State 40
        0, 0, 82, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -86, -86, -86, 0, 0, 0, 0, // State 41
        68, 0, 0, 18, -44, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 42
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 43
        68, 0, 0, 18, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 70, 71, 72, 48, // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 47
        0, -83, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, -83, -83, -83, -83, -83, -83, -83,
        -83, -83, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, -83, -83, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 49
        0, 0, 0, 0, -37, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 50
        0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 52
        0, 0, 0, 0, -111, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, // State 53
        0, 0, 0, 0, -110, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, // State 54
        0, 0, 0, 0, -112, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, // State 55
        0, -88, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, 0, -88, -88, -88,
        -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, -88, -88, 0, 0, 0, 0,
        // State 56
        0, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33,
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, // State 58
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, -74, -74, 0, -74, -74, -74,
        -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0,
        // State 59
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, -75, -75, 0, -75, -75, -75,
        -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0,
        // State 60
        0, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35,
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0,
        // State 61
        0, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32,
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0,
        // State 62
        0, -113, -113, 32, -113, -113, -113, -113, -113, 0, 33, -113, 0, -113, -113, -113, 0, -113,
        -113, -113, 34, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 100, 0, 0, 0, 0, // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, // State 65
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, -73, -73, 0, -73, -73, -73, 0, -73, -73, -73,
        -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0,
        // State 66
        0, -81, -81, 0, -81, -81, -81, -81, -81, 0, 0, -81, 0, -81, -81, -81, 0, -81, -81, -81, 0,
        -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, -81, 0, 0, 0, 0, // State 67
        -115, 0, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -115, 0, 0, -115, -115, -115, -115, // State 68
        -116, 0, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -116, 0, 0, -116, -116, -116, -116, // State 69
        0, -108, -108, -108, -108, -108, -108, -108, -108, 0, -108, -108, 0, -108, -108, -108, 0,
        -108, -108, -108, -108, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, -108, -108, 0, 0,
        0, 0, // State 70
        0, -60, -60, -60, -60, -60, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60,
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, -60, 0, 0, 0, 0,
        // State 71
        0, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65, -65,
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, -65, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, -39, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 73
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, // State 77
        -23, 0, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23, // State 78
        -22, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -22, 0, 0, -22, -22, -22, -22, // State 79
        -87, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, -87, // State 80
        -78, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, -78, // State 81
        -76, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, -76, // State 82
        -48, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -48, 0, 0, -48, -48, -48, -48, // State 83
        -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -50, 0, 0, -50, -50, -50, -50, // State 84
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -47, 0, 0, -47, -47, -47, -47, // State 85
        -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -49, 0, 0, -49, -49, -49, -49, // State 86
        -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -71, 0, 0, -71, -71, -71, -71, // State 87
        -72, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -72, 0, 0, -72, -72, -72, -72, // State 88
        -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -63, 0, 0, -63, -63, -63, -63, // State 89
        -64, 0, 0, -64, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -64, 0, 0, -64, -64, -64, -64, // State 90
        -54, 0, 0, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -54, 0, 0, -54, -54, -54, -54, // State 91
        -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -55, 0, 0, -55, -55, -55, -55, // State 92
        -84, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, -84, // State 93
        -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -56, 0, 0, -56, -56, -56, -56, // State 94
        -80, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, -80, // State 95
        -79, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, -79, // State 96
        -109, 0, 0, -109, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -109, 0, 0, -109, -109, -109, -109, // State 97
        -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -51, 0, 0, -51, -51, -51, -51, // State 98
        -77, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, -77, // State 99
        0, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36,
        -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0,
        // State 100
        -20, 0, 0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        0, 0, -20, -20, -20, 0, -20, -20, 0, -20, -20, -20, -20, -20, // State 101
        0, -114, -114, 0, -114, -114, -114, -114, -114, 0, 0, -114, 0, -114, -114, -114, 0, -114,
        -114, -114, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, -114, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, // State 107
        0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, // State 108
        0, 0, 0, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 109
        -21, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21,
        0, 0, -21, -21, -21, 0, -21, -21, 0, -21, -21, -21, -21, -21, // State 110
        0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, 0,
        -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, -82, -82, 0, 0, 0, 0, // State 111
        0, 0, 0, 0, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 112
        0, 0, 0, 0, -41, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 113
        0, -91, -91, -91, -91, -91, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91,
        -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 115
        0, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34,
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, // State 118
        0, 0, 0, 0, -43, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 119
        0, -90, -90, -90, -90, -90, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90,
        -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, -90, -90, 0, 0, 0, 0,
        // State 120
        -15, 0, 0, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, // State 121
        0, -89, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89,
        -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, -89, -89, 0, 0, 0, 0,
        // State 122
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, // State 123
        -16, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, -16, // State 124
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 41 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0, // State 1
        0, // State 2
        0, // State 3
        0, // State 4
        0, // State 5
        0, // State 6
        0, // State 7
        0, // State 8
        0, // State 9
        0, // State 10
        0, // State 11
        0, // State 12
        0, // State 13
        0, // State 14
        0, // State 15
        0, // State 16
        0, // State 17
        0, // State 18
        0, // State 19
        0, // State 20
        0, // State 21
        0, // State 22
        0, // State 23
        0, // State 24
        0, // State 25
        0, // State 26
        0, // State 27
        0, // State 28
        0, // State 29
        0, // State 30
        0, // State 31
        0, // State 32
        0, // State 33
        0, // State 34
        0, // State 35
        0, // State 36
        0, // State 37
        0, // State 38
        0, // State 39
        0, // State 40
        0, // State 41
        0, // State 42
        0, // State 43
        0, // State 44
        0, // State 45
        0, // State 46
        -118, // State 47
        0, // State 48
        -62, // State 49
        0, // State 50
        0, // State 51
        0, // State 52
        0, // State 53
        0, // State 54
        0, // State 55
        0, // State 56
        0, // State 57
        0, // State 58
        0, // State 59
        0, // State 60
        0, // State 61
        0, // State 62
        0, // State 63
        0, // State 64
        0, // State 65
        0, // State 66
        0, // State 67
        0, // State 68
        0, // State 69
        0, // State 70
        0, // State 71
        0, // State 72
        0, // State 73
        0, // State 74
        0, // State 75
        -61, // State 76
        0, // State 77
        0, // State 78
        0, // State 79
        0, // State 80
        0, // State 81
        0, // State 82
        0, // State 83
        0, // State 84
        0, // State 85
        0, // State 86
        0, // State 87
        0, // State 88
        0, // State 89
        0, // State 90
        0, // State 91
        0, // State 92
        0, // State 93
        0, // State 94
        0, // State 95
        0, // State 96
        0, // State 97
        0, // State 98
        0, // State 99
        -36, // State 100
        0, // State 101
        0, // State 102
        0, // State 103
        0, // State 104
        0, // State 105
        0, // State 106
        0, // State 107
        0, // State 108
        0, // State 109
        0, // State 110
        0, // State 111
        0, // State 112
        0, // State 113
        0, // State 114
        0, // State 115
        0, // State 116
        0, // State 117
        0, // State 118
        0, // State 119
        0, // State 120
        0, // State 121
        0, // State 122
        0, // State 123
        0, // State 124
        0, // State 125
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            6 => 7,
            9 => 41,
            12 => 9,
            13 => 25,
            14 => match state {
                27 => 38,
                _ => 10,
            },
            15 => match state {
                30 => 40,
                _ => 11,
            },
            16 => match state {
                7 => 72,
                _ => 49,
            },
            18 => 3,
            19 => 55,
            20 => match state {
                3 => 48,
                8 => 75,
                34 => 116,
                35 => 117,
                44 => 124,
                45 => 125,
                _ => 56,
            },
            21 => 50,
            22 => 111,
            23 => match state {
                28 => 39,
                _ => 12,
            },
            24 => 27,
            25 => 94,
            26 => match state {
                26 => 37,
                _ => 13,
            },
            27 => 28,
            28 => 90,
            29 => match state {
                19 => 34,
                23 => 35,
                42 => 44,
                17 => 102,
                21 => 105,
                22 => 106,
                31 => 112,
                33 => 114,
                41 => 118,
                43 => 122,
                _ => 57,
            },
            31 => 58,
            32 => 46,
            33 => 82,
            34 => 83,
            35 => 59,
            39 => 84,
            40 => 85,
            41 => 60,
            42 => 26,
            43 => 30,
            44 => 77,
            45 => 29,
            46 => match state {
                25 => 36,
                _ => 14,
            },
            47 => match state {
                1 => 2,
                4 | 7 => 51,
                18 => 103,
                20 => 104,
                32 => 113,
                _ => 61,
            },
            48 => 91,
            49 => 15,
            50 => 78,
            51 => 62,
            53 => 63,
            54 => match state {
                9 => 76,
                _ => 64,
            },
            56 => 65,
            57 => 95,
            58 => match state {
                24 => 108,
                _ => 8,
            },
            59 => match state {
                16 => 101,
                29 => 110,
                _ => 66,
            },
            60 => 16,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""bool""###,
            r###""else""###,
            r###""fn""###,
            r###""for""###,
            r###""if""###,
            r###""in""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""while""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[+-]?[0-9]+[.][0-9]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-z]+"#"###,
        ];
        __TERMINAL
            .iter()
            .enumerate()
            .filter_map(|(index, terminal)| {
                let next_state = __action(__state, index);
                if next_state == 0 {
                    None
                } else {
                    Some(alloc::string::ToString::to_string(terminal))
                }
            })
            .collect()
    }
    pub(crate) struct __StateMachine<'input> {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Function<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
            Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
            0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 41 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<'input>(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize> {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(17, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            Token(25, _) if true => Some(21),
            Token(26, _) if true => Some(22),
            Token(27, _) if true => Some(23),
            Token(28, _) if true => Some(24),
            Token(29, _) if true => Some(25),
            Token(30, _) if true => Some(26),
            Token(31, _) if true => Some(27),
            Token(32, _) if true => Some(28),
            Token(33, _) if true => Some(29),
            Token(34, _) if true => Some(30),
            Token(35, _) if true => Some(31),
            Token(36, _) if true => Some(32),
            Token(37, _) if true => Some(33),
            Token(38, _) if true => Some(34),
            Token(39, _) if true => Some(35),
            Token(40, _) if true => Some(36),
            Token(0, _) if true => Some(37),
            Token(1, _) if true => Some(38),
            Token(2, _) if true => Some(39),
            Token(3, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<'input>(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input> {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
            | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34
            | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(4, __tok0)
                | Token(5, __tok0)
                | Token(6, __tok0)
                | Token(7, __tok0)
                | Token(8, __tok0)
                | Token(9, __tok0)
                | Token(10, __tok0)
                | Token(11, __tok0)
                | Token(12, __tok0)
                | Token(13, __tok0)
                | Token(14, __tok0)
                | Token(15, __tok0)
                | Token(16, __tok0)
                | Token(17, __tok0)
                | Token(18, __tok0)
                | Token(19, __tok0)
                | Token(20, __tok0)
                | Token(21, __tok0)
                | Token(22, __tok0)
                | Token(23, __tok0)
                | Token(24, __tok0)
                | Token(25, __tok0)
                | Token(26, __tok0)
                | Token(27, __tok0)
                | Token(28, __tok0)
                | Token(29, __tok0)
                | Token(30, __tok0)
                | Token(31, __tok0)
                | Token(32, __tok0)
                | Token(33, __tok0)
                | Token(34, __tok0)
                | Token(35, __tok0)
                | Token(36, __tok0)
                | Token(37, __tok0)
                | Token(38, __tok0)
                | Token(39, __tok0)
                | Token(40, __tok0)
                | Token(0, __tok0)
                | Token(1, __tok0)
                | Token(2, __tok0)
                | Token(3, __tok0)
                    if true =>
                {
                    __Symbol::Variant0(__tok0)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct FunctionParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl FunctionParser {
        pub fn new() -> FunctionParser {
            let __builder = super::__intern_token::new_builder();
            FunctionParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<'input>(
            &self,
            input: &'input str,
        ) -> Result<Function<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<'input>(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<
        Result<Function<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
    > {
        let (__pop_states, __nonterminal) = match __action {
            0 => __reduce0(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            1 => __reduce1(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            2 => __reduce2(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            3 => __reduce3(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            4 => __reduce4(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            5 => __reduce5(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            6 => __reduce6(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            7 => __reduce7(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            8 => __reduce8(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            9 => __reduce9(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            10 => __reduce10(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            11 => __reduce11(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            12 => __reduce12(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            13 => __reduce13(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            14 => __reduce14(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            15 => __reduce15(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            16 => __reduce16(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            17 => __reduce17(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            18 => __reduce18(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            19 => __reduce19(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            20 => __reduce20(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            21 => __reduce21(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            22 => __reduce22(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            23 => __reduce23(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            24 => __reduce24(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            25 => __reduce25(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            26 => __reduce26(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            27 => __reduce27(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            28 => __reduce28(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            29 => __reduce29(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            30 => __reduce30(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            31 => __reduce31(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            32 => __reduce32(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            33 => __reduce33(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            34 => __reduce34(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            35 => __reduce35(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            36 => __reduce36(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            37 => __reduce37(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            38 => __reduce38(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            39 => __reduce39(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            40 => __reduce40(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            41 => __reduce41(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            42 => __reduce42(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            43 => __reduce43(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            44 => __reduce44(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            45 => __reduce45(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            46 => __reduce46(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            47 => __reduce47(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            48 => __reduce48(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            49 => __reduce49(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            50 => __reduce50(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            51 => __reduce51(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            52 => __reduce52(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            53 => __reduce53(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            54 => __reduce54(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            55 => __reduce55(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            56 => __reduce56(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            57 => __reduce57(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            58 => __reduce58(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            59 => __reduce59(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            60 => __reduce60(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            61 => __reduce61(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            62 => __reduce62(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            63 => __reduce63(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            64 => __reduce64(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            65 => __reduce65(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            66 => __reduce66(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            67 => __reduce67(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            68 => __reduce68(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            69 => __reduce69(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            70 => __reduce70(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            71 => __reduce71(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            72 => __reduce72(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            73 => __reduce73(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            74 => __reduce74(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            75 => __reduce75(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            76 => __reduce76(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            77 => __reduce77(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            78 => __reduce78(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            79 => __reduce79(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            80 => __reduce80(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            81 => __reduce81(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            82 => __reduce82(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            83 => __reduce83(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            84 => __reduce84(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            85 => __reduce85(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            86 => __reduce86(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            87 => __reduce87(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            88 => __reduce88(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            89 => __reduce89(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            90 => __reduce90(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            91 => __reduce91(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            92 => __reduce92(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            93 => __reduce93(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            94 => __reduce94(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            95 => __reduce95(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            96 => __reduce96(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            97 => __reduce97(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            98 => __reduce98(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            99 => __reduce99(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            100 => __reduce100(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            101 => __reduce101(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            102 => __reduce102(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            103 => __reduce103(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            104 => __reduce104(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            105 => __reduce105(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            106 => __reduce106(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            107 => __reduce107(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            108 => __reduce108(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            109 => __reduce109(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            110 => __reduce110(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            111 => __reduce111(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            112 => __reduce112(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            113 => __reduce113(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            114 => __reduce114(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            115 => __reduce115(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            116 => __reduce116(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            117 => {
                // __Function = Function => ActionFn(2);
                let __sym0 = __pop_Variant16(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            118 => __reduce118(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            119 => __reduce119(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            120 => __reduce120(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            _ => panic!("invalid action code {}", __action),
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Arg<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant11<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, BinOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant7<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Expr<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant16<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Function<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant17<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Item<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant19<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Literal<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant20<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Program<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant9<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Stmt<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant1<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Type, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant22<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, UnaryOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant13<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant14<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant3<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant6<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant8<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant18<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant10<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant12<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant15<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant21<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant2<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Type>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant4<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Vec<Stmt<'input>>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant0<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, &'input str, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    pub(crate) fn __reduce0<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>) = "->", Type => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? = "->", Type => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? =  => ActionFn(82);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>) = "else", Body => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? = "else", Body => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? =  => ActionFn(77);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",") = Arg, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* =  => ActionFn(90);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action90(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(91);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = Arg, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",") = Expr, "," => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action102(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* =  => ActionFn(100);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(101);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = Expr, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";") = Stmt, ";" => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* =  => ActionFn(95);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action95(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* = (<Stmt> ";")+ => ActionFn(96);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = Stmt, ";" => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = (<Stmt> ";")+, Stmt, ";" => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce21<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Plus => ActionFn(69);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Minus => ActionFn(70);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = MultiplicitveExpr => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = AdditiveExpr, AdditionOp, MultiplicitveExpr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = EqualityExpr => ActionFn(25);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = AndExpr, LogicalAnd, EqualityExpr => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg = Name, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? = Arg => ActionFn(88);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? =  => ActionFn(89);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce30<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Args = "(", Comma<Arg>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce31<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Name => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Body => ActionFn(42);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = "(", Expr, ")" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce34<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Literal => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Body = "{", Semi<Stmt>, "}" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = Arg => ActionFn(127);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce37<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> =  => ActionFn(128);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action128(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce38<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(129);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce39<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = Expr => ActionFn(131);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce41<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> =  => ActionFn(132);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action132(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce42<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(134);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = AdditiveExpr => ActionFn(29);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce45<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = ComparisonExpr, ComparisonOp, AdditiveExpr => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce46<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThan => ActionFn(65);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce47<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThan => ActionFn(66);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce48<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThanEquals => ActionFn(67);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce49<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThanEquals => ActionFn(68);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce50<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Devide = "/" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce51<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = ComparisonExpr => ActionFn(27);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce53<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = Equals => ActionFn(63);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce54<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = NotEquals => ActionFn(64);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Equals = "==" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce56<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr = OrExpr => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce57<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? = Expr => ActionFn(98);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce58<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? =  => ActionFn(99);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce59<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // FloatLit = r#"[+-]?[0-9]+[.][0-9]*"# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, "->", Type, Body => ActionFn(110);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action110(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (6, 32)
    }
    pub(crate) fn __reduce61<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, Body => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce62<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThan = ">" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThanEquals = ">=" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // IntLit = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item = Function => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce66<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* =  => ActionFn(84);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* = Item+ => ActionFn(85);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item+, Item => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce70<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThan = "<" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThanEquals = "<=" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = StringLit => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = FloatLit => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = IntLit => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalAnd = "&&" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce76<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalOr = "||" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce77<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Minus = "-" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Times => ActionFn(71);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce79<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Devide => ActionFn(72);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = UnaryExpr => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce81<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = MultiplicitveExpr, MultiplicationOp, UnaryExpr => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce82<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Name = r#"[a-z]+"# => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce83<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // NotEquals = "!=" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce84<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = AndExpr => ActionFn(23);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce85<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = OrExpr, LogicalOr, AndExpr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce86<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Plus = "+" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce87<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = BaseExpr => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce88<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(38);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce89<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "(", Comma<Expr>, ")" => ActionFn(39);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce90<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, ".", Name => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 51)
    }
    pub(crate) fn __reduce91<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program =  => ActionFn(135);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action135(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce92<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program = Item+ => ActionFn(136);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce93<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = Stmt => ActionFn(137);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce94<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> =  => ActionFn(138);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action138(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce95<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+, Stmt => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce96<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+ => ActionFn(140);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce97<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = Expr => ActionFn(15);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce98<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "let", Name, "=", Expr => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 54)
    }
    pub(crate) fn __reduce99<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body, "else", Body => ActionFn(113);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action113(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce100<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce101<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "for", Name, "in", Expr, Body => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce102<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "print", Expr => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce103<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "while", Expr, Body => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce104<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "return", Expr => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce105<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? = Stmt => ActionFn(93);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? =  => ActionFn(94);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action94(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 55)
    }
    pub(crate) fn __reduce107<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // StringLit = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Times = "*" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce109<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "int" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce110<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "bool" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "string" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce112<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = PostfixExpr => ActionFn(35);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce113<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = UnaryOp, UnaryExpr => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce114<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "!" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce115<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "-" => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce116<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce118<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Item = Item => ActionFn(1);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce119<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce120<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Stmt = Stmt => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 65)
    }
}
pub use self::__parse__Function::FunctionParser;

mod __parse__Item {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens
    )]

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input> {
        Variant0(&'input str),
        Variant1(Type),
        Variant2(core::option::Option<Type>),
        Variant3(Vec<Stmt<'input>>),
        Variant4(core::option::Option<Vec<Stmt<'input>>>),
        Variant5(Arg<'input>),
        Variant6(alloc::vec::Vec<Arg<'input>>),
        Variant7(Expr<'input>),
        Variant8(alloc::vec::Vec<Expr<'input>>),
        Variant9(Stmt<'input>),
        Variant10(alloc::vec::Vec<Stmt<'input>>),
        Variant11(BinOp),
        Variant12(core::option::Option<Arg<'input>>),
        Variant13(Vec<Arg<'input>>),
        Variant14(Vec<Expr<'input>>),
        Variant15(core::option::Option<Expr<'input>>),
        Variant16(Function<'input>),
        Variant17(Item<'input>),
        Variant18(alloc::vec::Vec<Item<'input>>),
        Variant19(Literal<'input>),
        Variant20(Program<'input>),
        Variant21(core::option::Option<Stmt<'input>>),
        Variant22(UnaryOp),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 2
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 4
        0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 55, 0,
        0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, // State 6
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0,
        21, 22, 23, 0, 24, 7, 0, -95, 71, 72, 73, 49, // State 7
        0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 9
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0,
        21, 22, 23, 0, 24, 7, 0, -97, 71, 72, 73, 49, // State 10
        0, -45, -45, 0, -45, 0, 81, -45, 82, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45, 0, -45, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, // State 11
        0, 0, 83, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -85, -85, -85, 0, 0, 0, 0, // State 12
        0, -52, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 88, 89, 0, -52, 90, 91, 0, -52, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, -52, -52, 0, 0, 0, 0, // State 13
        0, 94, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, 0, 0, 0, 95, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, // State 14
        0, -24, -24, 0, -24, 98, -24, -24, -24, 0, 0, 99, 0, -24, -24, -24, 0, -24, -24, -24, 0,
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, // State 15
        0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -57, 100, -57, 0, 0, 0, 0, // State 16
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 17
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 19
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 21
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 22
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 23
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 55, 0,
        0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, // State 25
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 26
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 27
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 28
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 29
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 30
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 31
        69, 0, 0, 18, -42, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, // State 33
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 36
        0, -25, -25, 0, -25, 98, -25, -25, -25, 0, 0, 99, 0, -25, -25, -25, 0, -25, -25, -25, 0,
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, // State 37
        0, 94, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 95, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, // State 38
        0, -46, -46, 0, -46, 0, 81, -46, 82, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46, 0, -46, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, // State 39
        0, -53, -53, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 88, 89, 0, -53, 90, 91, 0, -53, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, -53, 0, 0, 0, 0, // State 40
        0, 0, 83, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -86, -86, -86, 0, 0, 0, 0, // State 41
        69, 0, 0, 18, -44, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 42
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 43
        69, 0, 0, 18, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 71, 72, 73, 49, // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 48
        0, -83, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, -83, -83, -83, -83, -83, -83, -83,
        -83, -83, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, -83, -83, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 50
        0, 0, 0, 0, -37, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 51
        0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 53
        0, 0, 0, 0, -111, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, // State 54
        0, 0, 0, 0, -110, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, // State 55
        0, 0, 0, 0, -112, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, // State 56
        0, -88, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, 0, -88, -88, -88,
        -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, -88, -88, 0, 0, 0, 0,
        // State 57
        0, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33,
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, // State 59
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, -74, -74, 0, -74, -74, -74,
        -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0,
        // State 60
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, -75, -75, 0, -75, -75, -75,
        -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0,
        // State 61
        0, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35,
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0,
        // State 62
        0, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32,
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0,
        // State 63
        0, -113, -113, 32, -113, -113, -113, -113, -113, 0, 33, -113, 0, -113, -113, -113, 0, -113,
        -113, -113, 34, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 101, 0, 0, 0, 0, // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, // State 66
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, -73, -73, 0, -73, -73, -73, 0, -73, -73, -73,
        -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0,
        // State 67
        0, -81, -81, 0, -81, -81, -81, -81, -81, 0, 0, -81, 0, -81, -81, -81, 0, -81, -81, -81, 0,
        -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, -81, 0, 0, 0, 0, // State 68
        -115, 0, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -115, 0, 0, -115, -115, -115, -115, // State 69
        -116, 0, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -116, 0, 0, -116, -116, -116, -116, // State 70
        0, -108, -108, -108, -108, -108, -108, -108, -108, 0, -108, -108, 0, -108, -108, -108, 0,
        -108, -108, -108, -108, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, -108, -108, 0, 0,
        0, 0, // State 71
        0, -60, -60, -60, -60, -60, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60,
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, -60, 0, 0, 0, 0,
        // State 72
        0, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65, -65,
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, -65, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, -39, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 74
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, // State 78
        -23, 0, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23, // State 79
        -22, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -22, 0, 0, -22, -22, -22, -22, // State 80
        -87, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, -87, // State 81
        -78, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, -78, // State 82
        -76, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, -76, // State 83
        -48, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -48, 0, 0, -48, -48, -48, -48, // State 84
        -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -50, 0, 0, -50, -50, -50, -50, // State 85
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -47, 0, 0, -47, -47, -47, -47, // State 86
        -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -49, 0, 0, -49, -49, -49, -49, // State 87
        -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -71, 0, 0, -71, -71, -71, -71, // State 88
        -72, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -72, 0, 0, -72, -72, -72, -72, // State 89
        -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -63, 0, 0, -63, -63, -63, -63, // State 90
        -64, 0, 0, -64, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -64, 0, 0, -64, -64, -64, -64, // State 91
        -54, 0, 0, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -54, 0, 0, -54, -54, -54, -54, // State 92
        -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -55, 0, 0, -55, -55, -55, -55, // State 93
        -84, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, -84, // State 94
        -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -56, 0, 0, -56, -56, -56, -56, // State 95
        -80, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, -80, // State 96
        -79, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, -79, // State 97
        -109, 0, 0, -109, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -109, 0, 0, -109, -109, -109, -109, // State 98
        -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -51, 0, 0, -51, -51, -51, -51, // State 99
        -77, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, -77, // State 100
        0, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36,
        -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0,
        // State 101
        -20, 0, 0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        0, 0, -20, -20, -20, 0, -20, -20, 0, -20, -20, -20, -20, -20, // State 102
        0, -114, -114, 0, -114, -114, -114, -114, -114, 0, 0, -114, 0, -114, -114, -114, 0, -114,
        -114, -114, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, -114, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, // State 108
        0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, // State 109
        0, 0, 0, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 110
        -21, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21,
        0, 0, -21, -21, -21, 0, -21, -21, 0, -21, -21, -21, -21, -21, // State 111
        0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, 0,
        -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, -82, -82, 0, 0, 0, 0, // State 112
        0, 0, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 113
        0, 0, 0, 0, -41, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 114
        0, -91, -91, -91, -91, -91, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91,
        -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 116
        0, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34,
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, // State 119
        0, 0, 0, 0, -43, 0, 0, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 120
        0, -90, -90, -90, -90, -90, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90,
        -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, -90, -90, 0, 0, 0, 0,
        // State 121
        -15, 0, 0, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, // State 122
        0, -89, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89,
        -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, -89, -89, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, // State 124
        -16, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, -16, // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, // State 126
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 41 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0, // State 1
        0, // State 2
        0, // State 3
        0, // State 4
        0, // State 5
        0, // State 6
        0, // State 7
        0, // State 8
        0, // State 9
        0, // State 10
        0, // State 11
        0, // State 12
        0, // State 13
        0, // State 14
        0, // State 15
        0, // State 16
        0, // State 17
        0, // State 18
        0, // State 19
        0, // State 20
        0, // State 21
        0, // State 22
        0, // State 23
        0, // State 24
        0, // State 25
        0, // State 26
        0, // State 27
        0, // State 28
        0, // State 29
        0, // State 30
        0, // State 31
        0, // State 32
        0, // State 33
        0, // State 34
        0, // State 35
        0, // State 36
        0, // State 37
        0, // State 38
        0, // State 39
        0, // State 40
        0, // State 41
        0, // State 42
        0, // State 43
        0, // State 44
        0, // State 45
        0, // State 46
        -66, // State 47
        -119, // State 48
        0, // State 49
        -62, // State 50
        0, // State 51
        0, // State 52
        0, // State 53
        0, // State 54
        0, // State 55
        0, // State 56
        0, // State 57
        0, // State 58
        0, // State 59
        0, // State 60
        0, // State 61
        0, // State 62
        0, // State 63
        0, // State 64
        0, // State 65
        0, // State 66
        0, // State 67
        0, // State 68
        0, // State 69
        0, // State 70
        0, // State 71
        0, // State 72
        0, // State 73
        0, // State 74
        0, // State 75
        0, // State 76
        -61, // State 77
        0, // State 78
        0, // State 79
        0, // State 80
        0, // State 81
        0, // State 82
        0, // State 83
        0, // State 84
        0, // State 85
        0, // State 86
        0, // State 87
        0, // State 88
        0, // State 89
        0, // State 90
        0, // State 91
        0, // State 92
        0, // State 93
        0, // State 94
        0, // State 95
        0, // State 96
        0, // State 97
        0, // State 98
        0, // State 99
        0, // State 100
        -36, // State 101
        0, // State 102
        0, // State 103
        0, // State 104
        0, // State 105
        0, // State 106
        0, // State 107
        0, // State 108
        0, // State 109
        0, // State 110
        0, // State 111
        0, // State 112
        0, // State 113
        0, // State 114
        0, // State 115
        0, // State 116
        0, // State 117
        0, // State 118
        0, // State 119
        0, // State 120
        0, // State 121
        0, // State 122
        0, // State 123
        0, // State 124
        0, // State 125
        0, // State 126
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            6 => 7,
            9 => 41,
            12 => 9,
            13 => 25,
            14 => match state {
                27 => 38,
                _ => 10,
            },
            15 => match state {
                30 => 40,
                _ => 11,
            },
            16 => match state {
                7 => 73,
                _ => 50,
            },
            18 => 3,
            19 => 56,
            20 => match state {
                3 => 49,
                8 => 76,
                34 => 117,
                35 => 118,
                44 => 125,
                45 => 126,
                _ => 57,
            },
            21 => 51,
            22 => 112,
            23 => match state {
                28 => 39,
                _ => 12,
            },
            24 => 27,
            25 => 95,
            26 => match state {
                26 => 37,
                _ => 13,
            },
            27 => 28,
            28 => 91,
            29 => match state {
                19 => 34,
                23 => 35,
                42 => 44,
                17 => 103,
                21 => 106,
                22 => 107,
                31 => 113,
                33 => 115,
                41 => 119,
                43 => 123,
                _ => 58,
            },
            31 => 59,
            32 => 46,
            33 => 83,
            34 => 84,
            35 => 60,
            36 => 47,
            39 => 85,
            40 => 86,
            41 => 61,
            42 => 26,
            43 => 30,
            44 => 78,
            45 => 29,
            46 => match state {
                25 => 36,
                _ => 14,
            },
            47 => match state {
                1 => 2,
                4 | 7 => 52,
                18 => 104,
                20 => 105,
                32 => 114,
                _ => 62,
            },
            48 => 92,
            49 => 15,
            50 => 79,
            51 => 63,
            53 => 64,
            54 => match state {
                9 => 77,
                _ => 65,
            },
            56 => 66,
            57 => 96,
            58 => match state {
                24 => 109,
                _ => 8,
            },
            59 => match state {
                16 => 102,
                29 => 111,
                _ => 67,
            },
            60 => 16,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""bool""###,
            r###""else""###,
            r###""fn""###,
            r###""for""###,
            r###""if""###,
            r###""in""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""while""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[+-]?[0-9]+[.][0-9]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-z]+"#"###,
        ];
        __TERMINAL
            .iter()
            .enumerate()
            .filter_map(|(index, terminal)| {
                let next_state = __action(__state, index);
                if next_state == 0 {
                    None
                } else {
                    Some(alloc::string::ToString::to_string(terminal))
                }
            })
            .collect()
    }
    pub(crate) struct __StateMachine<'input> {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Item<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
            Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
            0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 41 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<'input>(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize> {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(17, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            Token(25, _) if true => Some(21),
            Token(26, _) if true => Some(22),
            Token(27, _) if true => Some(23),
            Token(28, _) if true => Some(24),
            Token(29, _) if true => Some(25),
            Token(30, _) if true => Some(26),
            Token(31, _) if true => Some(27),
            Token(32, _) if true => Some(28),
            Token(33, _) if true => Some(29),
            Token(34, _) if true => Some(30),
            Token(35, _) if true => Some(31),
            Token(36, _) if true => Some(32),
            Token(37, _) if true => Some(33),
            Token(38, _) if true => Some(34),
            Token(39, _) if true => Some(35),
            Token(40, _) if true => Some(36),
            Token(0, _) if true => Some(37),
            Token(1, _) if true => Some(38),
            Token(2, _) if true => Some(39),
            Token(3, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<'input>(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input> {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
            | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34
            | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(4, __tok0)
                | Token(5, __tok0)
                | Token(6, __tok0)
                | Token(7, __tok0)
                | Token(8, __tok0)
                | Token(9, __tok0)
                | Token(10, __tok0)
                | Token(11, __tok0)
                | Token(12, __tok0)
                | Token(13, __tok0)
                | Token(14, __tok0)
                | Token(15, __tok0)
                | Token(16, __tok0)
                | Token(17, __tok0)
                | Token(18, __tok0)
                | Token(19, __tok0)
                | Token(20, __tok0)
                | Token(21, __tok0)
                | Token(22, __tok0)
                | Token(23, __tok0)
                | Token(24, __tok0)
                | Token(25, __tok0)
                | Token(26, __tok0)
                | Token(27, __tok0)
                | Token(28, __tok0)
                | Token(29, __tok0)
                | Token(30, __tok0)
                | Token(31, __tok0)
                | Token(32, __tok0)
                | Token(33, __tok0)
                | Token(34, __tok0)
                | Token(35, __tok0)
                | Token(36, __tok0)
                | Token(37, __tok0)
                | Token(38, __tok0)
                | Token(39, __tok0)
                | Token(40, __tok0)
                | Token(0, __tok0)
                | Token(1, __tok0)
                | Token(2, __tok0)
                | Token(3, __tok0)
                    if true =>
                {
                    __Symbol::Variant0(__tok0)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ItemParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ItemParser {
        pub fn new() -> ItemParser {
            let __builder = super::__intern_token::new_builder();
            ItemParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<'input>(
            &self,
            input: &'input str,
        ) -> Result<Item<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<'input>(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Item<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => __reduce0(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            1 => __reduce1(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            2 => __reduce2(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            3 => __reduce3(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            4 => __reduce4(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            5 => __reduce5(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            6 => __reduce6(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            7 => __reduce7(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            8 => __reduce8(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            9 => __reduce9(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            10 => __reduce10(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            11 => __reduce11(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            12 => __reduce12(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            13 => __reduce13(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            14 => __reduce14(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            15 => __reduce15(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            16 => __reduce16(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            17 => __reduce17(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            18 => __reduce18(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            19 => __reduce19(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            20 => __reduce20(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            21 => __reduce21(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            22 => __reduce22(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            23 => __reduce23(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            24 => __reduce24(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            25 => __reduce25(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            26 => __reduce26(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            27 => __reduce27(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            28 => __reduce28(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            29 => __reduce29(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            30 => __reduce30(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            31 => __reduce31(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            32 => __reduce32(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            33 => __reduce33(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            34 => __reduce34(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            35 => __reduce35(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            36 => __reduce36(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            37 => __reduce37(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            38 => __reduce38(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            39 => __reduce39(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            40 => __reduce40(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            41 => __reduce41(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            42 => __reduce42(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            43 => __reduce43(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            44 => __reduce44(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            45 => __reduce45(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            46 => __reduce46(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            47 => __reduce47(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            48 => __reduce48(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            49 => __reduce49(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            50 => __reduce50(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            51 => __reduce51(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            52 => __reduce52(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            53 => __reduce53(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            54 => __reduce54(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            55 => __reduce55(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            56 => __reduce56(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            57 => __reduce57(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            58 => __reduce58(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            59 => __reduce59(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            60 => __reduce60(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            61 => __reduce61(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            62 => __reduce62(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            63 => __reduce63(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            64 => __reduce64(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            65 => __reduce65(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            66 => __reduce66(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            67 => __reduce67(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            68 => __reduce68(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            69 => __reduce69(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            70 => __reduce70(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            71 => __reduce71(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            72 => __reduce72(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            73 => __reduce73(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            74 => __reduce74(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            75 => __reduce75(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            76 => __reduce76(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            77 => __reduce77(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            78 => __reduce78(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            79 => __reduce79(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            80 => __reduce80(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            81 => __reduce81(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            82 => __reduce82(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            83 => __reduce83(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            84 => __reduce84(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            85 => __reduce85(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            86 => __reduce86(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            87 => __reduce87(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            88 => __reduce88(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            89 => __reduce89(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            90 => __reduce90(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            91 => __reduce91(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            92 => __reduce92(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            93 => __reduce93(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            94 => __reduce94(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            95 => __reduce95(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            96 => __reduce96(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            97 => __reduce97(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            98 => __reduce98(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            99 => __reduce99(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            100 => __reduce100(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            101 => __reduce101(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            102 => __reduce102(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            103 => __reduce103(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            104 => __reduce104(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            105 => __reduce105(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            106 => __reduce106(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            107 => __reduce107(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            108 => __reduce108(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            109 => __reduce109(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            110 => __reduce110(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            111 => __reduce111(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            112 => __reduce112(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            113 => __reduce113(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            114 => __reduce114(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            115 => __reduce115(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            116 => __reduce116(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            117 => __reduce117(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            118 => {
                // __Item = Item => ActionFn(1);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            119 => __reduce119(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            120 => __reduce120(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            _ => panic!("invalid action code {}", __action),
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Arg<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant11<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, BinOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant7<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Expr<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant16<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Function<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant17<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Item<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant19<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Literal<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant20<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Program<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant9<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Stmt<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant1<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Type, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant22<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, UnaryOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant13<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant14<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant3<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant6<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant8<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant18<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant10<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant12<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant15<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant21<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant2<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Type>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant4<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Vec<Stmt<'input>>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant0<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, &'input str, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    pub(crate) fn __reduce0<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>) = "->", Type => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? = "->", Type => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? =  => ActionFn(82);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>) = "else", Body => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? = "else", Body => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? =  => ActionFn(77);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",") = Arg, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* =  => ActionFn(90);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action90(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(91);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = Arg, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",") = Expr, "," => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action102(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* =  => ActionFn(100);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(101);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = Expr, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";") = Stmt, ";" => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* =  => ActionFn(95);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action95(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* = (<Stmt> ";")+ => ActionFn(96);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = Stmt, ";" => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = (<Stmt> ";")+, Stmt, ";" => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce21<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Plus => ActionFn(69);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Minus => ActionFn(70);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = MultiplicitveExpr => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = AdditiveExpr, AdditionOp, MultiplicitveExpr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = EqualityExpr => ActionFn(25);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = AndExpr, LogicalAnd, EqualityExpr => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg = Name, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? = Arg => ActionFn(88);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? =  => ActionFn(89);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce30<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Args = "(", Comma<Arg>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce31<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Name => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Body => ActionFn(42);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = "(", Expr, ")" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce34<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Literal => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Body = "{", Semi<Stmt>, "}" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = Arg => ActionFn(127);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce37<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> =  => ActionFn(128);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action128(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce38<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(129);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce39<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = Expr => ActionFn(131);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce41<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> =  => ActionFn(132);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action132(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce42<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(134);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = AdditiveExpr => ActionFn(29);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce45<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = ComparisonExpr, ComparisonOp, AdditiveExpr => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce46<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThan => ActionFn(65);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce47<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThan => ActionFn(66);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce48<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThanEquals => ActionFn(67);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce49<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThanEquals => ActionFn(68);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce50<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Devide = "/" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce51<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = ComparisonExpr => ActionFn(27);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce53<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = Equals => ActionFn(63);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce54<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = NotEquals => ActionFn(64);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Equals = "==" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce56<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr = OrExpr => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce57<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? = Expr => ActionFn(98);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce58<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? =  => ActionFn(99);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce59<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // FloatLit = r#"[+-]?[0-9]+[.][0-9]*"# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, "->", Type, Body => ActionFn(110);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action110(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (6, 32)
    }
    pub(crate) fn __reduce61<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, Body => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce62<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThan = ">" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThanEquals = ">=" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // IntLit = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item = Function => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce66<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* =  => ActionFn(84);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* = Item+ => ActionFn(85);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item+, Item => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce70<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThan = "<" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThanEquals = "<=" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = StringLit => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = FloatLit => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = IntLit => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalAnd = "&&" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce76<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalOr = "||" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce77<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Minus = "-" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Times => ActionFn(71);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce79<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Devide => ActionFn(72);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = UnaryExpr => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce81<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = MultiplicitveExpr, MultiplicationOp, UnaryExpr => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce82<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Name = r#"[a-z]+"# => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce83<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // NotEquals = "!=" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce84<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = AndExpr => ActionFn(23);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce85<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = OrExpr, LogicalOr, AndExpr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce86<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Plus = "+" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce87<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = BaseExpr => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce88<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(38);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce89<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "(", Comma<Expr>, ")" => ActionFn(39);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce90<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, ".", Name => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 51)
    }
    pub(crate) fn __reduce91<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program =  => ActionFn(135);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action135(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce92<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program = Item+ => ActionFn(136);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce93<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = Stmt => ActionFn(137);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce94<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> =  => ActionFn(138);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action138(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce95<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+, Stmt => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce96<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+ => ActionFn(140);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce97<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = Expr => ActionFn(15);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce98<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "let", Name, "=", Expr => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 54)
    }
    pub(crate) fn __reduce99<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body, "else", Body => ActionFn(113);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action113(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce100<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce101<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "for", Name, "in", Expr, Body => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce102<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "print", Expr => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce103<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "while", Expr, Body => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce104<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "return", Expr => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce105<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? = Stmt => ActionFn(93);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? =  => ActionFn(94);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action94(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 55)
    }
    pub(crate) fn __reduce107<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // StringLit = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Times = "*" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce109<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "int" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce110<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "bool" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "string" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce112<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = PostfixExpr => ActionFn(35);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce113<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = UnaryOp, UnaryExpr => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce114<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "!" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce115<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "-" => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce116<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce117<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Function = Function => ActionFn(2);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce119<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce120<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Stmt = Stmt => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 65)
    }
}
pub use self::__parse__Item::ItemParser;

mod __parse__Program {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens
    )]

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input> {
        Variant0(&'input str),
        Variant1(Type),
        Variant2(core::option::Option<Type>),
        Variant3(Vec<Stmt<'input>>),
        Variant4(core::option::Option<Vec<Stmt<'input>>>),
        Variant5(Arg<'input>),
        Variant6(alloc::vec::Vec<Arg<'input>>),
        Variant7(Expr<'input>),
        Variant8(alloc::vec::Vec<Expr<'input>>),
        Variant9(Stmt<'input>),
        Variant10(alloc::vec::Vec<Stmt<'input>>),
        Variant11(BinOp),
        Variant12(core::option::Option<Arg<'input>>),
        Variant13(Vec<Arg<'input>>),
        Variant14(Vec<Expr<'input>>),
        Variant15(core::option::Option<Expr<'input>>),
        Variant16(Function<'input>),
        Variant17(Item<'input>),
        Variant18(alloc::vec::Vec<Item<'input>>),
        Variant19(Literal<'input>),
        Variant20(Program<'input>),
        Variant21(core::option::Option<Stmt<'input>>),
        Variant22(UnaryOp),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 3
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 5
        0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 58, 0,
        0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, // State 7
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 21, 0, 0,
        22, 23, 24, 0, 25, 8, 0, -95, 74, 75, 76, 52, // State 8
        0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 10
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 21, 0, 0,
        22, 23, 24, 0, 25, 8, 0, -97, 74, 75, 76, 52, // State 11
        0, -45, -45, 0, -45, 0, 84, -45, 85, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45, 0, -45, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, // State 12
        0, 0, 86, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -85, -85, -85, 0, 0, 0, 0, // State 13
        0, -52, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 91, 92, 0, -52, 93, 94, 0, -52, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, -52, -52, 0, 0, 0, 0, // State 14
        0, 97, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, 0, 0, 0, 98, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, // State 15
        0, -24, -24, 0, -24, 101, -24, -24, -24, 0, 0, 102, 0, -24, -24, -24, 0, -24, -24, -24, 0,
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, // State 16
        0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -57, 103, -57, 0, 0, 0, 0, // State 17
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 18
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 20
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 22
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 23
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 24
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 58, 0,
        0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, // State 26
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 27
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 28
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 29
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 30
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 31
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 32
        72, 0, 0, 19, -42, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // State 34
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 37
        0, -25, -25, 0, -25, 101, -25, -25, -25, 0, 0, 102, 0, -25, -25, -25, 0, -25, -25, -25, 0,
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, // State 38
        0, 97, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 98, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, // State 39
        0, -46, -46, 0, -46, 0, 84, -46, 85, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46, 0, -46, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, // State 40
        0, -53, -53, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 91, 92, 0, -53, 93, 94, 0, -53, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, -53, 0, 0, 0, 0, // State 41
        0, 0, 86, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -86, -86, -86, 0, 0, 0, 0, // State 42
        72, 0, 0, 19, -44, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 43
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 44
        72, 0, 0, 19, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 0, 0, 74, 75, 76, 52, // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 51
        0, -83, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, -83, -83, -83, -83, -83, -83, -83,
        -83, -83, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, -83, -83, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 53
        0, 0, 0, 0, -37, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 54
        0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 56
        0, 0, 0, 0, -111, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, // State 57
        0, 0, 0, 0, -110, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, // State 58
        0, 0, 0, 0, -112, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, // State 59
        0, -88, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, 0, -88, -88, -88,
        -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, -88, -88, 0, 0, 0, 0,
        // State 60
        0, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33,
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, // State 62
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, -74, -74, 0, -74, -74, -74,
        -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0,
        // State 63
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, -75, -75, 0, -75, -75, -75,
        -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0,
        // State 64
        0, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35,
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0,
        // State 65
        0, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32,
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0,
        // State 66
        0, -113, -113, 33, -113, -113, -113, -113, -113, 0, 34, -113, 0, -113, -113, -113, 0, -113,
        -113, -113, 35, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 104, 0, 0, 0, 0, // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, // State 69
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, -73, -73, 0, -73, -73, -73, 0, -73, -73, -73,
        -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0,
        // State 70
        0, -81, -81, 0, -81, -81, -81, -81, -81, 0, 0, -81, 0, -81, -81, -81, 0, -81, -81, -81, 0,
        -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, -81, 0, 0, 0, 0, // State 71
        -115, 0, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -115, 0, 0, -115, -115, -115, -115, // State 72
        -116, 0, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -116, 0, 0, -116, -116, -116, -116, // State 73
        0, -108, -108, -108, -108, -108, -108, -108, -108, 0, -108, -108, 0, -108, -108, -108, 0,
        -108, -108, -108, -108, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, -108, -108, 0, 0,
        0, 0, // State 74
        0, -60, -60, -60, -60, -60, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60,
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, -60, 0, 0, 0, 0,
        // State 75
        0, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65, -65,
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, -65, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, -39, 0, 0, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 77
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, // State 81
        -23, 0, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23, // State 82
        -22, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -22, 0, 0, -22, -22, -22, -22, // State 83
        -87, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, -87, // State 84
        -78, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, -78, // State 85
        -76, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, -76, // State 86
        -48, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -48, 0, 0, -48, -48, -48, -48, // State 87
        -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -50, 0, 0, -50, -50, -50, -50, // State 88
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -47, 0, 0, -47, -47, -47, -47, // State 89
        -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -49, 0, 0, -49, -49, -49, -49, // State 90
        -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -71, 0, 0, -71, -71, -71, -71, // State 91
        -72, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -72, 0, 0, -72, -72, -72, -72, // State 92
        -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -63, 0, 0, -63, -63, -63, -63, // State 93
        -64, 0, 0, -64, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -64, 0, 0, -64, -64, -64, -64, // State 94
        -54, 0, 0, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -54, 0, 0, -54, -54, -54, -54, // State 95
        -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -55, 0, 0, -55, -55, -55, -55, // State 96
        -84, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, -84, // State 97
        -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -56, 0, 0, -56, -56, -56, -56, // State 98
        -80, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, -80, // State 99
        -79, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, -79, // State 100
        -109, 0, 0, -109, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -109, 0, 0, -109, -109, -109, -109, // State 101
        -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -51, 0, 0, -51, -51, -51, -51, // State 102
        -77, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, -77, // State 103
        0, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36,
        -36, -36, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0,
        // State 104
        -20, 0, 0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        0, 0, -20, -20, -20, 0, -20, -20, 0, -20, -20, -20, -20, -20, // State 105
        0, -114, -114, 0, -114, -114, -114, -114, -114, 0, 0, -114, 0, -114, -114, -114, 0, -114,
        -114, -114, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, -114, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, // State 111
        0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, // State 112
        0, 0, 0, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 113
        -21, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21,
        0, 0, -21, -21, -21, 0, -21, -21, 0, -21, -21, -21, -21, -21, // State 114
        0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, 0,
        -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, -82, -82, 0, 0, 0, 0, // State 115
        0, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 116
        0, 0, 0, 0, -41, 0, 0, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 117
        0, -91, -91, -91, -91, -91, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91,
        -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 119
        0, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34,
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, // State 122
        0, 0, 0, 0, -43, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 123
        0, -90, -90, -90, -90, -90, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90,
        -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, -90, -90, 0, 0, 0, 0,
        // State 124
        -15, 0, 0, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, // State 125
        0, -89, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89,
        -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, -89, -89, 0, 0, 0, 0,
        // State 126
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, // State 127
        -16, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, -16, // State 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, // State 129
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 41 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -92, // State 1
        -93, // State 2
        0, // State 3
        0, // State 4
        0, // State 5
        0, // State 6
        0, // State 7
        0, // State 8
        0, // State 9
        0, // State 10
        0, // State 11
        0, // State 12
        0, // State 13
        0, // State 14
        0, // State 15
        0, // State 16
        0, // State 17
        0, // State 18
        0, // State 19
        0, // State 20
        0, // State 21
        0, // State 22
        0, // State 23
        0, // State 24
        0, // State 25
        0, // State 26
        0, // State 27
        0, // State 28
        0, // State 29
        0, // State 30
        0, // State 31
        0, // State 32
        0, // State 33
        0, // State 34
        0, // State 35
        0, // State 36
        0, // State 37
        0, // State 38
        0, // State 39
        0, // State 40
        0, // State 41
        0, // State 42
        0, // State 43
        0, // State 44
        0, // State 45
        0, // State 46
        0, // State 47
        -66, // State 48
        -69, // State 49
        -120, // State 50
        -70, // State 51
        0, // State 52
        -62, // State 53
        0, // State 54
        0, // State 55
        0, // State 56
        0, // State 57
        0, // State 58
        0, // State 59
        0, // State 60
        0, // State 61
        0, // State 62
        0, // State 63
        0, // State 64
        0, // State 65
        0, // State 66
        0, // State 67
        0, // State 68
        0, // State 69
        0, // State 70
        0, // State 71
        0, // State 72
        0, // State 73
        0, // State 74
        0, // State 75
        0, // State 76
        0, // State 77
        0, // State 78
        0, // State 79
        -61, // State 80
        0, // State 81
        0, // State 82
        0, // State 83
        0, // State 84
        0, // State 85
        0, // State 86
        0, // State 87
        0, // State 88
        0, // State 89
        0, // State 90
        0, // State 91
        0, // State 92
        0, // State 93
        0, // State 94
        0, // State 95
        0, // State 96
        0, // State 97
        0, // State 98
        0, // State 99
        0, // State 100
        0, // State 101
        0, // State 102
        0, // State 103
        -36, // State 104
        0, // State 105
        0, // State 106
        0, // State 107
        0, // State 108
        0, // State 109
        0, // State 110
        0, // State 111
        0, // State 112
        0, // State 113
        0, // State 114
        0, // State 115
        0, // State 116
        0, // State 117
        0, // State 118
        0, // State 119
        0, // State 120
        0, // State 121
        0, // State 122
        0, // State 123
        0, // State 124
        0, // State 125
        0, // State 126
        0, // State 127
        0, // State 128
        0, // State 129
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            6 => 8,
            9 => 42,
            12 => 10,
            13 => 26,
            14 => match state {
                28 => 39,
                _ => 11,
            },
            15 => match state {
                31 => 41,
                _ => 12,
            },
            16 => match state {
                8 => 76,
                _ => 53,
            },
            18 => 4,
            19 => 59,
            20 => match state {
                4 => 52,
                9 => 79,
                35 => 120,
                36 => 121,
                45 => 128,
                46 => 129,
                _ => 60,
            },
            21 => 54,
            22 => 115,
            23 => match state {
                29 => 40,
                _ => 13,
            },
            24 => 28,
            25 => 98,
            26 => match state {
                27 => 38,
                _ => 14,
            },
            27 => 29,
            28 => 94,
            29 => match state {
                20 => 35,
                24 => 36,
                43 => 45,
                18 => 106,
                22 => 109,
                23 => 110,
                32 => 116,
                34 => 118,
                42 => 122,
                44 => 126,
                _ => 61,
            },
            31 => 62,
            32 => 47,
            33 => 86,
            34 => 87,
            35 => 63,
            36 => match state {
                1 => 50,
                _ => 48,
            },
            38 => 1,
            39 => 88,
            40 => 89,
            41 => 64,
            42 => 27,
            43 => 31,
            44 => 81,
            45 => 30,
            46 => match state {
                26 => 37,
                _ => 15,
            },
            47 => match state {
                2 => 3,
                5 | 8 => 55,
                19 => 107,
                21 => 108,
                33 => 117,
                _ => 65,
            },
            48 => 95,
            49 => 16,
            50 => 82,
            51 => 66,
            52 => 49,
            53 => 67,
            54 => match state {
                10 => 80,
                _ => 68,
            },
            56 => 69,
            57 => 99,
            58 => match state {
                25 => 112,
                _ => 9,
            },
            59 => match state {
                17 => 105,
                30 => 114,
                _ => 70,
            },
            60 => 17,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""bool""###,
            r###""else""###,
            r###""fn""###,
            r###""for""###,
            r###""if""###,
            r###""in""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""while""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[+-]?[0-9]+[.][0-9]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-z]+"#"###,
        ];
        __TERMINAL
            .iter()
            .enumerate()
            .filter_map(|(index, terminal)| {
                let next_state = __action(__state, index);
                if next_state == 0 {
                    None
                } else {
                    Some(alloc::string::ToString::to_string(terminal))
                }
            })
            .collect()
    }
    pub(crate) struct __StateMachine<'input> {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Program<'input>;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
            Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
            0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 41 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<'input>(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize> {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(17, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            Token(25, _) if true => Some(21),
            Token(26, _) if true => Some(22),
            Token(27, _) if true => Some(23),
            Token(28, _) if true => Some(24),
            Token(29, _) if true => Some(25),
            Token(30, _) if true => Some(26),
            Token(31, _) if true => Some(27),
            Token(32, _) if true => Some(28),
            Token(33, _) if true => Some(29),
            Token(34, _) if true => Some(30),
            Token(35, _) if true => Some(31),
            Token(36, _) if true => Some(32),
            Token(37, _) if true => Some(33),
            Token(38, _) if true => Some(34),
            Token(39, _) if true => Some(35),
            Token(40, _) if true => Some(36),
            Token(0, _) if true => Some(37),
            Token(1, _) if true => Some(38),
            Token(2, _) if true => Some(39),
            Token(3, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<'input>(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input> {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
            | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34
            | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(4, __tok0)
                | Token(5, __tok0)
                | Token(6, __tok0)
                | Token(7, __tok0)
                | Token(8, __tok0)
                | Token(9, __tok0)
                | Token(10, __tok0)
                | Token(11, __tok0)
                | Token(12, __tok0)
                | Token(13, __tok0)
                | Token(14, __tok0)
                | Token(15, __tok0)
                | Token(16, __tok0)
                | Token(17, __tok0)
                | Token(18, __tok0)
                | Token(19, __tok0)
                | Token(20, __tok0)
                | Token(21, __tok0)
                | Token(22, __tok0)
                | Token(23, __tok0)
                | Token(24, __tok0)
                | Token(25, __tok0)
                | Token(26, __tok0)
                | Token(27, __tok0)
                | Token(28, __tok0)
                | Token(29, __tok0)
                | Token(30, __tok0)
                | Token(31, __tok0)
                | Token(32, __tok0)
                | Token(33, __tok0)
                | Token(34, __tok0)
                | Token(35, __tok0)
                | Token(36, __tok0)
                | Token(37, __tok0)
                | Token(38, __tok0)
                | Token(39, __tok0)
                | Token(40, __tok0)
                | Token(0, __tok0)
                | Token(1, __tok0)
                | Token(2, __tok0)
                | Token(3, __tok0)
                    if true =>
                {
                    __Symbol::Variant0(__tok0)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<'input>(
            &self,
            input: &'input str,
        ) -> Result<Program<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<'input>(
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<
        Result<Program<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
    > {
        let (__pop_states, __nonterminal) = match __action {
            0 => __reduce0(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            1 => __reduce1(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            2 => __reduce2(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            3 => __reduce3(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            4 => __reduce4(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            5 => __reduce5(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            6 => __reduce6(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            7 => __reduce7(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            8 => __reduce8(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            9 => __reduce9(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            10 => __reduce10(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            11 => __reduce11(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            12 => __reduce12(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            13 => __reduce13(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            14 => __reduce14(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            15 => __reduce15(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            16 => __reduce16(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            17 => __reduce17(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            18 => __reduce18(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            19 => __reduce19(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            20 => __reduce20(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            21 => __reduce21(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            22 => __reduce22(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            23 => __reduce23(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            24 => __reduce24(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            25 => __reduce25(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            26 => __reduce26(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            27 => __reduce27(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            28 => __reduce28(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            29 => __reduce29(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            30 => __reduce30(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            31 => __reduce31(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            32 => __reduce32(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            33 => __reduce33(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            34 => __reduce34(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            35 => __reduce35(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            36 => __reduce36(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            37 => __reduce37(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            38 => __reduce38(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            39 => __reduce39(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            40 => __reduce40(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            41 => __reduce41(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            42 => __reduce42(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            43 => __reduce43(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            44 => __reduce44(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            45 => __reduce45(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            46 => __reduce46(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            47 => __reduce47(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            48 => __reduce48(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            49 => __reduce49(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            50 => __reduce50(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            51 => __reduce51(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            52 => __reduce52(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            53 => __reduce53(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            54 => __reduce54(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            55 => __reduce55(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            56 => __reduce56(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            57 => __reduce57(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            58 => __reduce58(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            59 => __reduce59(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            60 => __reduce60(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            61 => __reduce61(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            62 => __reduce62(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            63 => __reduce63(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            64 => __reduce64(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            65 => __reduce65(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            66 => __reduce66(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            67 => __reduce67(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            68 => __reduce68(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            69 => __reduce69(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            70 => __reduce70(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            71 => __reduce71(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            72 => __reduce72(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            73 => __reduce73(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            74 => __reduce74(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            75 => __reduce75(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            76 => __reduce76(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            77 => __reduce77(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            78 => __reduce78(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            79 => __reduce79(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            80 => __reduce80(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            81 => __reduce81(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            82 => __reduce82(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            83 => __reduce83(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            84 => __reduce84(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            85 => __reduce85(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            86 => __reduce86(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            87 => __reduce87(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            88 => __reduce88(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            89 => __reduce89(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            90 => __reduce90(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            91 => __reduce91(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            92 => __reduce92(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            93 => __reduce93(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            94 => __reduce94(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            95 => __reduce95(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            96 => __reduce96(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            97 => __reduce97(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            98 => __reduce98(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            99 => __reduce99(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            100 => __reduce100(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            101 => __reduce101(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            102 => __reduce102(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            103 => __reduce103(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            104 => __reduce104(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            105 => __reduce105(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            106 => __reduce106(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            107 => __reduce107(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            108 => __reduce108(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            109 => __reduce109(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            110 => __reduce110(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            111 => __reduce111(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            112 => __reduce112(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            113 => __reduce113(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            114 => __reduce114(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            115 => __reduce115(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            116 => __reduce116(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            117 => __reduce117(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            118 => __reduce118(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            119 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant20(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            120 => __reduce120(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            _ => panic!("invalid action code {}", __action),
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Arg<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant11<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, BinOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant7<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Expr<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant16<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Function<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant17<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Item<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant19<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Literal<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant20<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Program<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant9<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Stmt<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant1<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Type, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant22<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, UnaryOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant13<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant14<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant3<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant6<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant8<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant18<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant10<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant12<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant15<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant21<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant2<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Type>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant4<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Vec<Stmt<'input>>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant0<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, &'input str, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    pub(crate) fn __reduce0<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>) = "->", Type => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? = "->", Type => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? =  => ActionFn(82);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>) = "else", Body => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? = "else", Body => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? =  => ActionFn(77);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",") = Arg, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* =  => ActionFn(90);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action90(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(91);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = Arg, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",") = Expr, "," => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action102(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* =  => ActionFn(100);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(101);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = Expr, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";") = Stmt, ";" => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* =  => ActionFn(95);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action95(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* = (<Stmt> ";")+ => ActionFn(96);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = Stmt, ";" => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = (<Stmt> ";")+, Stmt, ";" => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce21<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Plus => ActionFn(69);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Minus => ActionFn(70);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = MultiplicitveExpr => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = AdditiveExpr, AdditionOp, MultiplicitveExpr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = EqualityExpr => ActionFn(25);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = AndExpr, LogicalAnd, EqualityExpr => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg = Name, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? = Arg => ActionFn(88);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? =  => ActionFn(89);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce30<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Args = "(", Comma<Arg>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce31<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Name => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Body => ActionFn(42);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = "(", Expr, ")" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce34<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Literal => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Body = "{", Semi<Stmt>, "}" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = Arg => ActionFn(127);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce37<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> =  => ActionFn(128);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action128(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce38<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(129);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce39<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = Expr => ActionFn(131);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce41<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> =  => ActionFn(132);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action132(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce42<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(134);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = AdditiveExpr => ActionFn(29);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce45<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = ComparisonExpr, ComparisonOp, AdditiveExpr => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce46<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThan => ActionFn(65);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce47<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThan => ActionFn(66);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce48<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThanEquals => ActionFn(67);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce49<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThanEquals => ActionFn(68);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce50<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Devide = "/" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce51<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = ComparisonExpr => ActionFn(27);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce53<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = Equals => ActionFn(63);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce54<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = NotEquals => ActionFn(64);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Equals = "==" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce56<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr = OrExpr => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce57<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? = Expr => ActionFn(98);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce58<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? =  => ActionFn(99);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce59<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // FloatLit = r#"[+-]?[0-9]+[.][0-9]*"# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, "->", Type, Body => ActionFn(110);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action110(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (6, 32)
    }
    pub(crate) fn __reduce61<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, Body => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce62<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThan = ">" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThanEquals = ">=" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // IntLit = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item = Function => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce66<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* =  => ActionFn(84);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* = Item+ => ActionFn(85);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item+, Item => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce70<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThan = "<" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThanEquals = "<=" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = StringLit => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = FloatLit => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = IntLit => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalAnd = "&&" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce76<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalOr = "||" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce77<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Minus = "-" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Times => ActionFn(71);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce79<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Devide => ActionFn(72);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = UnaryExpr => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce81<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = MultiplicitveExpr, MultiplicationOp, UnaryExpr => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce82<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Name = r#"[a-z]+"# => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce83<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // NotEquals = "!=" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce84<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = AndExpr => ActionFn(23);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce85<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = OrExpr, LogicalOr, AndExpr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce86<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Plus = "+" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce87<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = BaseExpr => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce88<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(38);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce89<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "(", Comma<Expr>, ")" => ActionFn(39);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce90<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, ".", Name => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 51)
    }
    pub(crate) fn __reduce91<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program =  => ActionFn(135);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action135(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce92<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program = Item+ => ActionFn(136);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce93<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = Stmt => ActionFn(137);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce94<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> =  => ActionFn(138);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action138(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce95<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+, Stmt => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce96<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+ => ActionFn(140);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce97<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = Expr => ActionFn(15);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce98<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "let", Name, "=", Expr => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 54)
    }
    pub(crate) fn __reduce99<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body, "else", Body => ActionFn(113);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action113(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce100<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce101<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "for", Name, "in", Expr, Body => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce102<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "print", Expr => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce103<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "while", Expr, Body => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce104<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "return", Expr => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce105<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? = Stmt => ActionFn(93);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? =  => ActionFn(94);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action94(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 55)
    }
    pub(crate) fn __reduce107<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // StringLit = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Times = "*" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce109<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "int" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce110<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "bool" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "string" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce112<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = PostfixExpr => ActionFn(35);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce113<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = UnaryOp, UnaryExpr => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce114<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "!" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce115<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "-" => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce116<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce117<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Function = Function => ActionFn(2);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce118<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Item = Item => ActionFn(1);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce120<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Stmt = Stmt => ActionFn(3);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 65)
    }
}
pub use self::__parse__Program::ProgramParser;

mod __parse__Stmt {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens
    )]

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input> {
        Variant0(&'input str),
        Variant1(Type),
        Variant2(core::option::Option<Type>),
        Variant3(Vec<Stmt<'input>>),
        Variant4(core::option::Option<Vec<Stmt<'input>>>),
        Variant5(Arg<'input>),
        Variant6(alloc::vec::Vec<Arg<'input>>),
        Variant7(Expr<'input>),
        Variant8(alloc::vec::Vec<Expr<'input>>),
        Variant9(Stmt<'input>),
        Variant10(alloc::vec::Vec<Stmt<'input>>),
        Variant11(BinOp),
        Variant12(core::option::Option<Arg<'input>>),
        Variant13(Vec<Arg<'input>>),
        Variant14(Vec<Expr<'input>>),
        Variant15(core::option::Option<Expr<'input>>),
        Variant16(Function<'input>),
        Variant17(Item<'input>),
        Variant18(alloc::vec::Vec<Item<'input>>),
        Variant19(Literal<'input>),
        Variant20(Program<'input>),
        Variant21(core::option::Option<Stmt<'input>>),
        Variant22(UnaryOp),
    }
    const __ACTION: &[i8] = &[
        // State 0
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0, 0,
        12, 13, 14, 0, 15, 16, 0, 0, 52, 53, 54, 55, // State 1
        0, -45, -45, 0, -45, 0, 58, -45, 59, 0, 0, 0, 0, -45, -45, -45, 0, -45, -45, -45, 0, -45, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, // State 2
        0, 0, 60, 0, -85, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -85, -85, -85, 0, 0, 0, 0, // State 3
        0, -52, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 65, 66, 0, -52, 67, 68, 0, -52, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, -52, -52, 0, 0, 0, 0, // State 4
        0, 71, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, 0, 0, 0, 72, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, // State 5
        0, -24, -24, 0, -24, 75, -24, -24, -24, 0, 0, 76, 0, -24, -24, -24, 0, -24, -24, -24, 0,
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, // State 6
        0, 0, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -57, 77, -57, 0, 0, 0, 0, // State 7
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 8
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, // State 10
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, // State 12
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 13
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 14
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 15
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0, 0,
        12, 13, 14, 0, 15, 16, 0, -95, 52, 53, 54, 55, // State 16
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 17
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 18
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 19
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 20
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 21
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 22
        50, 0, 0, 9, -42, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, // State 24
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, // State 27
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0, 0,
        12, 13, 14, 0, 15, 16, 0, -97, 52, 53, 54, 55, // State 28
        0, -25, -25, 0, -25, 75, -25, -25, -25, 0, 0, 76, 0, -25, -25, -25, 0, -25, -25, -25, 0,
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, // State 29
        0, 71, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 72, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, // State 30
        0, -46, -46, 0, -46, 0, 58, -46, 59, 0, 0, 0, 0, -46, -46, -46, 0, -46, -46, -46, 0, -46, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, // State 31
        0, -53, -53, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 65, 66, 0, -53, 67, 68, 0, -53, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, -53, 0, 0, 0, 0, // State 32
        0, 0, 60, 0, -86, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -86, -86, -86, 0, 0, 0, 0, // State 33
        50, 0, 0, 9, -44, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 34
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 35
        50, 0, 0, 9, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 16, 0, 0, 52, 53, 54, 55, // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, // State 38
        0, -88, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, 0, -88, -88, -88,
        -88, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, -88, -88, 0, 0, 0, 0,
        // State 39
        0, -33, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33,
        -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, // State 41
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, -74, -74, 0, -74, -74, -74,
        -74, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, -74, -74, 0, 0, 0, 0,
        // State 42
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, -75, -75, 0, -75, -75, -75,
        -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, -75, -75, 0, 0, 0, 0,
        // State 43
        0, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35,
        -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0,
        // State 44
        0, -32, -32, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32,
        -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0,
        // State 45
        0, -113, -113, 23, -113, -113, -113, -113, -113, 0, 24, -113, 0, -113, -113, -113, 0, -113,
        -113, -113, 25, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 47
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, -73, -73, 0, -73, -73, -73, 0, -73, -73, -73,
        -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, -73, -73, 0, 0, 0, 0,
        // State 48
        0, -81, -81, 0, -81, -81, -81, -81, -81, 0, 0, -81, 0, -81, -81, -81, 0, -81, -81, -81, 0,
        -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, -81, -81, 0, 0, 0, 0, // State 49
        -115, 0, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -115, 0, 0, -115, -115, -115, -115, // State 50
        -116, 0, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -116, 0, 0, -116, -116, -116, -116, // State 51
        0, -108, -108, -108, -108, -108, -108, -108, -108, 0, -108, -108, 0, -108, -108, -108, 0,
        -108, -108, -108, -108, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, -108, -108, 0, 0,
        0, 0, // State 52
        0, -60, -60, -60, -60, -60, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60,
        -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, -60, 0, 0, 0, 0,
        // State 53
        0, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65, -65,
        -65, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, -65, -65, 0, 0, 0, 0,
        // State 54
        0, -83, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, 0, -83, -83, -83, -83, -83, -83,
        -83, -83, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, -83, -83, 0, 0, 0, 0,
        // State 55
        -23, 0, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -23, 0, 0, -23, -23, -23, -23, // State 56
        -22, 0, 0, -22, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -22, 0, 0, -22, -22, -22, -22, // State 57
        -87, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -87, 0, 0, -87, -87, -87, -87, // State 58
        -78, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -78, 0, 0, -78, -78, -78, -78, // State 59
        -76, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -76, 0, 0, -76, -76, -76, -76, // State 60
        -48, 0, 0, -48, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -48, 0, 0, -48, -48, -48, -48, // State 61
        -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -50, 0, 0, -50, -50, -50, -50, // State 62
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -47, 0, 0, -47, -47, -47, -47, // State 63
        -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -49, 0, 0, -49, -49, -49, -49, // State 64
        -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -71, 0, 0, -71, -71, -71, -71, // State 65
        -72, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -72, 0, 0, -72, -72, -72, -72, // State 66
        -63, 0, 0, -63, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -63, 0, 0, -63, -63, -63, -63, // State 67
        -64, 0, 0, -64, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -64, 0, 0, -64, -64, -64, -64, // State 68
        -54, 0, 0, -54, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -54, 0, 0, -54, -54, -54, -54, // State 69
        -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -55, 0, 0, -55, -55, -55, -55, // State 70
        -84, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -84, 0, 0, -84, -84, -84, -84, // State 71
        -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -56, 0, 0, -56, -56, -56, -56, // State 72
        -80, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -80, 0, 0, -80, -80, -80, -80, // State 73
        -79, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -79, 0, 0, -79, -79, -79, -79, // State 74
        -109, 0, 0, -109, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -109, 0, 0, -109, -109, -109, -109, // State 75
        -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -51, 0, 0, -51, -51, -51, -51, // State 76
        -77, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -77, 0, 0, -77, -77, -77, -77, // State 77
        0, -114, -114, 0, -114, -114, -114, -114, -114, 0, 0, -114, 0, -114, -114, -114, 0, -114,
        -114, -114, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, -114, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 95, 0, 0, 0, 0, // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, // State 85
        0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, 0,
        -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, -82, -82, 0, 0, 0, 0, // State 86
        0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 87
        0, 0, 0, 0, -41, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 88
        0, -91, -91, -91, -91, -91, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 0, -91, -91, -91,
        -91, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 90
        0, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34,
        -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, // State 94
        0, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, 0, -36, -36, -36, 0, -36, -36, -36,
        -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0,
        // State 95
        -20, 0, 0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        0, 0, -20, -20, -20, 0, -20, -20, 0, -20, -20, -20, -20, -20, // State 96
        0, 0, 0, 0, -43, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // State 97
        0, -90, -90, -90, -90, -90, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 0, -90, -90, -90,
        -90, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, -90, -90, 0, 0, 0, 0,
        // State 98
        -15, 0, 0, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, // State 99
        0, -89, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, 0, -89, -89, -89,
        -89, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, -89, -89, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, // State 101
        -21, 0, 0, -21, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21,
        0, 0, -21, -21, -21, 0, -21, -21, 0, -21, -21, -21, -21, -21, // State 102
        -16, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, -16, // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 41 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0, // State 1
        -45, // State 2
        -85, // State 3
        -52, // State 4
        -26, // State 5
        -24, // State 6
        -57, // State 7
        0, // State 8
        0, // State 9
        0, // State 10
        0, // State 11
        0, // State 12
        0, // State 13
        0, // State 14
        0, // State 15
        0, // State 16
        0, // State 17
        0, // State 18
        0, // State 19
        0, // State 20
        0, // State 21
        0, // State 22
        0, // State 23
        0, // State 24
        0, // State 25
        0, // State 26
        0, // State 27
        0, // State 28
        -25, // State 29
        -27, // State 30
        -46, // State 31
        -53, // State 32
        -86, // State 33
        0, // State 34
        0, // State 35
        0, // State 36
        0, // State 37
        0, // State 38
        -88, // State 39
        -33, // State 40
        -98, // State 41
        -74, // State 42
        -75, // State 43
        -35, // State 44
        -32, // State 45
        -113, // State 46
        -121, // State 47
        -73, // State 48
        -81, // State 49
        0, // State 50
        0, // State 51
        -108, // State 52
        -60, // State 53
        -65, // State 54
        -83, // State 55
        0, // State 56
        0, // State 57
        0, // State 58
        0, // State 59
        0, // State 60
        0, // State 61
        0, // State 62
        0, // State 63
        0, // State 64
        0, // State 65
        0, // State 66
        0, // State 67
        0, // State 68
        0, // State 69
        0, // State 70
        0, // State 71
        0, // State 72
        0, // State 73
        0, // State 74
        0, // State 75
        0, // State 76
        0, // State 77
        -114, // State 78
        0, // State 79
        0, // State 80
        0, // State 81
        -103, // State 82
        -105, // State 83
        0, // State 84
        0, // State 85
        -82, // State 86
        0, // State 87
        0, // State 88
        -91, // State 89
        0, // State 90
        -34, // State 91
        -101, // State 92
        -104, // State 93
        0, // State 94
        -36, // State 95
        0, // State 96
        0, // State 97
        -90, // State 98
        0, // State 99
        -89, // State 100
        -99, // State 101
        0, // State 102
        0, // State 103
        -102, // State 104
        -100,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            9 => 33,
            12 => 27,
            13 => 16,
            14 => match state {
                18 => 30,
                _ => 1,
            },
            15 => match state {
                21 => 32,
                _ => 2,
            },
            19 => 38,
            20 => match state {
                25 => 91,
                26 => 92,
                36 => 103,
                37 => 104,
                _ => 39,
            },
            22 => 86,
            23 => match state {
                19 => 31,
                _ => 3,
            },
            24 => 18,
            25 => 72,
            26 => match state {
                17 => 29,
                _ => 4,
            },
            27 => 19,
            28 => 68,
            29 => match state {
                10 => 25,
                14 => 26,
                34 => 36,
                8 => 78,
                12 => 81,
                13 => 82,
                22 => 87,
                24 => 89,
                33 => 96,
                35 => 100,
                _ => 40,
            },
            31 => 41,
            33 => 60,
            34 => 61,
            35 => 42,
            39 => 62,
            40 => 63,
            41 => 43,
            42 => 17,
            43 => 21,
            44 => 55,
            45 => 20,
            46 => match state {
                16 => 28,
                _ => 5,
            },
            47 => match state {
                9 => 79,
                11 => 80,
                23 => 88,
                _ => 44,
            },
            48 => 69,
            49 => 6,
            50 => 56,
            51 => 45,
            53 => 83,
            54 => match state {
                15 => 84,
                27 => 93,
                _ => 46,
            },
            56 => 47,
            57 => 73,
            59 => match state {
                7 => 77,
                20 => 85,
                _ => 48,
            },
            60 => 7,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""bool""###,
            r###""else""###,
            r###""fn""###,
            r###""for""###,
            r###""if""###,
            r###""in""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""while""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"\\\\])*\""#"###,
            r###"r#"[+-]?[0-9]+[.][0-9]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-z]+"#"###,
        ];
        __TERMINAL
            .iter()
            .enumerate()
            .filter_map(|(index, terminal)| {
                let next_state = __action(__state, index);
                if next_state == 0 {
                    None
                } else {
                    Some(alloc::string::ToString::to_string(terminal))
                }
            })
            .collect()
    }
    pub(crate) struct __StateMachine<'input> {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Stmt<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
            Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
            0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 41 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<'input>(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize> {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(17, _) if true => Some(13),
            Token(18, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(20, _) if true => Some(16),
            Token(21, _) if true => Some(17),
            Token(22, _) if true => Some(18),
            Token(23, _) if true => Some(19),
            Token(24, _) if true => Some(20),
            Token(25, _) if true => Some(21),
            Token(26, _) if true => Some(22),
            Token(27, _) if true => Some(23),
            Token(28, _) if true => Some(24),
            Token(29, _) if true => Some(25),
            Token(30, _) if true => Some(26),
            Token(31, _) if true => Some(27),
            Token(32, _) if true => Some(28),
            Token(33, _) if true => Some(29),
            Token(34, _) if true => Some(30),
            Token(35, _) if true => Some(31),
            Token(36, _) if true => Some(32),
            Token(37, _) if true => Some(33),
            Token(38, _) if true => Some(34),
            Token(39, _) if true => Some(35),
            Token(40, _) if true => Some(36),
            Token(0, _) if true => Some(37),
            Token(1, _) if true => Some(38),
            Token(2, _) if true => Some(39),
            Token(3, _) if true => Some(40),
            _ => None,
        }
    }
    fn __token_to_symbol<'input>(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input> {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
            | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34
            | 35 | 36 | 37 | 38 | 39 | 40 => match __token {
                Token(4, __tok0)
                | Token(5, __tok0)
                | Token(6, __tok0)
                | Token(7, __tok0)
                | Token(8, __tok0)
                | Token(9, __tok0)
                | Token(10, __tok0)
                | Token(11, __tok0)
                | Token(12, __tok0)
                | Token(13, __tok0)
                | Token(14, __tok0)
                | Token(15, __tok0)
                | Token(16, __tok0)
                | Token(17, __tok0)
                | Token(18, __tok0)
                | Token(19, __tok0)
                | Token(20, __tok0)
                | Token(21, __tok0)
                | Token(22, __tok0)
                | Token(23, __tok0)
                | Token(24, __tok0)
                | Token(25, __tok0)
                | Token(26, __tok0)
                | Token(27, __tok0)
                | Token(28, __tok0)
                | Token(29, __tok0)
                | Token(30, __tok0)
                | Token(31, __tok0)
                | Token(32, __tok0)
                | Token(33, __tok0)
                | Token(34, __tok0)
                | Token(35, __tok0)
                | Token(36, __tok0)
                | Token(37, __tok0)
                | Token(38, __tok0)
                | Token(39, __tok0)
                | Token(40, __tok0)
                | Token(0, __tok0)
                | Token(1, __tok0)
                | Token(2, __tok0)
                | Token(3, __tok0)
                    if true =>
                {
                    __Symbol::Variant0(__tok0)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct StmtParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl StmtParser {
        pub fn new() -> StmtParser {
            let __builder = super::__intern_token::new_builder();
            StmtParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<'input>(
            &self,
            input: &'input str,
        ) -> Result<Stmt<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<'input>(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Stmt<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => __reduce0(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            1 => __reduce1(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            2 => __reduce2(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            3 => __reduce3(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            4 => __reduce4(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            5 => __reduce5(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            6 => __reduce6(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            7 => __reduce7(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            8 => __reduce8(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            9 => __reduce9(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            10 => __reduce10(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            11 => __reduce11(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            12 => __reduce12(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            13 => __reduce13(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            14 => __reduce14(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            15 => __reduce15(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            16 => __reduce16(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            17 => __reduce17(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            18 => __reduce18(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            19 => __reduce19(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            20 => __reduce20(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            21 => __reduce21(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            22 => __reduce22(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            23 => __reduce23(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            24 => __reduce24(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            25 => __reduce25(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            26 => __reduce26(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            27 => __reduce27(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            28 => __reduce28(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            29 => __reduce29(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            30 => __reduce30(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            31 => __reduce31(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            32 => __reduce32(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            33 => __reduce33(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            34 => __reduce34(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            35 => __reduce35(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            36 => __reduce36(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            37 => __reduce37(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            38 => __reduce38(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            39 => __reduce39(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            40 => __reduce40(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            41 => __reduce41(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            42 => __reduce42(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            43 => __reduce43(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            44 => __reduce44(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            45 => __reduce45(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            46 => __reduce46(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            47 => __reduce47(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            48 => __reduce48(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            49 => __reduce49(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            50 => __reduce50(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            51 => __reduce51(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            52 => __reduce52(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            53 => __reduce53(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            54 => __reduce54(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            55 => __reduce55(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            56 => __reduce56(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            57 => __reduce57(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            58 => __reduce58(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            59 => __reduce59(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            60 => __reduce60(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            61 => __reduce61(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            62 => __reduce62(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            63 => __reduce63(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            64 => __reduce64(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            65 => __reduce65(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            66 => __reduce66(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            67 => __reduce67(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            68 => __reduce68(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            69 => __reduce69(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            70 => __reduce70(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            71 => __reduce71(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            72 => __reduce72(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            73 => __reduce73(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            74 => __reduce74(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            75 => __reduce75(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            76 => __reduce76(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            77 => __reduce77(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            78 => __reduce78(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            79 => __reduce79(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            80 => __reduce80(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            81 => __reduce81(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            82 => __reduce82(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            83 => __reduce83(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            84 => __reduce84(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            85 => __reduce85(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            86 => __reduce86(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            87 => __reduce87(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            88 => __reduce88(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            89 => __reduce89(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            90 => __reduce90(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            91 => __reduce91(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            92 => __reduce92(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            93 => __reduce93(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            94 => __reduce94(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            95 => __reduce95(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            96 => __reduce96(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            97 => __reduce97(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            98 => __reduce98(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            99 => __reduce99(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            100 => __reduce100(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            101 => __reduce101(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            102 => __reduce102(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            103 => __reduce103(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            104 => __reduce104(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            105 => __reduce105(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            106 => __reduce106(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            107 => __reduce107(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            108 => __reduce108(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            109 => __reduce109(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            110 => __reduce110(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            111 => __reduce111(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            112 => __reduce112(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            113 => __reduce113(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            114 => __reduce114(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            115 => __reduce115(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            116 => __reduce116(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            117 => __reduce117(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            118 => __reduce118(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            119 => __reduce119(
                input,
                __lookahead_start,
                __symbols,
                core::marker::PhantomData::<(&())>,
            ),
            120 => {
                // __Stmt = Stmt => ActionFn(3);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action),
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Arg<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant11<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, BinOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant7<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Expr<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant16<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Function<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant17<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Item<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant19<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Literal<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant20<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Program<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant9<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Stmt<'input>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant1<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Type, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant22<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, UnaryOp, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant13<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant14<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant3<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant6<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant8<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant18<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant10<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, alloc::vec::Vec<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant12<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Arg<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant15<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Expr<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant21<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Stmt<'input>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant2<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Type>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant4<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, core::option::Option<Vec<Stmt<'input>>>, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    fn __pop_Variant0<'input>(
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
    ) -> (usize, &'input str, usize) {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch(),
        }
    }
    pub(crate) fn __reduce0<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>) = "->", Type => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? = "->", Type => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action109(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("->" <Type>)? =  => ActionFn(82);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>) = "else", Body => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? = "else", Body => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action112(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ("else" <Body>)? =  => ActionFn(77);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",") = Arg, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action92(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* =  => ActionFn(90);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action90(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(91);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = Arg, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action115(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",") = Expr, "," => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action102(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* =  => ActionFn(100);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(101);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action101(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = Expr, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action119(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action120(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";") = Stmt, ";" => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* =  => ActionFn(95);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action95(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")* = (<Stmt> ";")+ => ActionFn(96);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = Stmt, ";" => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // (<Stmt> ";")+ = (<Stmt> ";")+, Stmt, ";" => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action124(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce21<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Plus => ActionFn(69);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditionOp = Minus => ActionFn(70);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = MultiplicitveExpr => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AdditiveExpr = AdditiveExpr, AdditionOp, MultiplicitveExpr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = EqualityExpr => ActionFn(25);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // AndExpr = AndExpr, LogicalAnd, EqualityExpr => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce27<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg = Name, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce28<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? = Arg => ActionFn(88);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Arg? =  => ActionFn(89);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce30<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Args = "(", Comma<Arg>, ")" => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce31<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Name => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Body => ActionFn(42);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = "(", Expr, ")" => ActionFn(43);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action43(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce34<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // BaseExpr = Literal => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Body = "{", Semi<Stmt>, "}" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = Arg => ActionFn(127);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce37<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> =  => ActionFn(128);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action128(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce38<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(129);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce39<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = Expr => ActionFn(131);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action131(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce41<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> =  => ActionFn(132);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action132(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce42<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action133(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(134);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = AdditiveExpr => ActionFn(29);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce45<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonExpr = ComparisonExpr, ComparisonOp, AdditiveExpr => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce46<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThan => ActionFn(65);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce47<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThan => ActionFn(66);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce48<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = LessThanEquals => ActionFn(67);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce49<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // ComparisonOp = GreaterThanEquals => ActionFn(68);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce50<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Devide = "/" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce51<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = ComparisonExpr => ActionFn(27);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce52<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityExpr = EqualityExpr, EqualityOp, ComparisonExpr => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce53<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = Equals => ActionFn(63);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce54<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // EqualityOp = NotEquals => ActionFn(64);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce55<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Equals = "==" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce56<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr = OrExpr => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce57<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? = Expr => ActionFn(98);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce58<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Expr? =  => ActionFn(99);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action99(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce59<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // FloatLit = r#"[+-]?[0-9]+[.][0-9]*"# => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce60<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, "->", Type, Body => ActionFn(110);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action110(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (6, 32)
    }
    pub(crate) fn __reduce61<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Function = "fn", Name, Args, Body => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 32)
    }
    pub(crate) fn __reduce62<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThan = ">" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce63<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // GreaterThanEquals = ">=" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // IntLit = r#"[0-9]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item = Function => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce66<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* =  => ActionFn(84);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action84(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 37)
    }
    pub(crate) fn __reduce67<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item* = Item+ => ActionFn(85);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item => ActionFn(86);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Item+ = Item+, Item => ActionFn(87);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action87(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 38)
    }
    pub(crate) fn __reduce70<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThan = "<" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action55(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LessThanEquals = "<=" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = StringLit => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = FloatLit => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Literal = IntLit => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalAnd = "&&" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce76<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // LogicalOr = "||" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce77<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Minus = "-" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce78<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Times => ActionFn(71);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce79<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicationOp = Devide => ActionFn(72);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce80<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = UnaryExpr => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce81<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // MultiplicitveExpr = MultiplicitveExpr, MultiplicationOp, UnaryExpr => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action34(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce82<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Name = r#"[a-z]+"# => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce83<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // NotEquals = "!=" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce84<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = AndExpr => ActionFn(23);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce85<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // OrExpr = OrExpr, LogicalOr, AndExpr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce86<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Plus = "+" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce87<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = BaseExpr => ActionFn(37);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce88<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(38);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce89<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, "(", Comma<Expr>, ")" => ActionFn(39);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 51)
    }
    pub(crate) fn __reduce90<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // PostfixExpr = PostfixExpr, ".", Name => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 51)
    }
    pub(crate) fn __reduce91<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program =  => ActionFn(135);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action135(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce92<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Program = Item+ => ActionFn(136);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce93<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = Stmt => ActionFn(137);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce94<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> =  => ActionFn(138);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action138(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce95<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+, Stmt => ActionFn(139);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action139(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 53)
    }
    pub(crate) fn __reduce96<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Semi<Stmt> = (<Stmt> ";")+ => ActionFn(140);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce97<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = Expr => ActionFn(15);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce98<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "let", Name, "=", Expr => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 54)
    }
    pub(crate) fn __reduce99<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body, "else", Body => ActionFn(113);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action113(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce100<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "if", Expr, Body => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce101<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "for", Name, "in", Expr, Body => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action18(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 54)
    }
    pub(crate) fn __reduce102<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "print", Expr => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce103<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "while", Expr, Body => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 54)
    }
    pub(crate) fn __reduce104<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt = "return", Expr => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce105<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? = Stmt => ActionFn(93);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce106<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Stmt? =  => ActionFn(94);
        let __start = __lookahead_start
            .cloned()
            .or_else(|| __symbols.last().map(|s| s.2.clone()))
            .unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action94(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 55)
    }
    pub(crate) fn __reduce107<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // StringLit = r#"\"(\\\\.|[^\"\\\\])*\""# => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce108<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Times = "*" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce109<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "int" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce110<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "bool" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce111<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // Type = "string" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce112<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = PostfixExpr => ActionFn(35);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce113<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryExpr = UnaryOp, UnaryExpr => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce114<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "!" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce115<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // UnaryOp = "-" => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce116<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce117<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Function = Function => ActionFn(2);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce118<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Item = Item => ActionFn(1);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce119<'input>(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize) {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
}
pub use self::__parse__Stmt::StmtParser;

mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate alloc;
    extern crate core;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            (
                "^(\"(\\\\[\u{0}-\t\u{b}-\u{10ffff}]|[\u{0}-!\\#-\\[\\]-\u{10ffff}])*\")",
                false,
            ),
            ("^([\\+\\-]?[0-9]+[\\.][0-9]*)", false),
            ("^([0-9]+)", false),
            ("^([a-z]+)", false),
            ("^(!)", false),
            ("^(!=)", false),
            ("^(\\&\\&)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(,)", false),
            ("^(\\-)", false),
            ("^(\\->)", false),
            ("^(\\.)", false),
            ("^(/)", false),
            ("^(:)", false),
            ("^(;)", false),
            ("^(<)", false),
            ("^(<=)", false),
            ("^(=)", false),
            ("^(==)", false),
            ("^(>)", false),
            ("^(>=)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(bool)", false),
            ("^(else)", false),
            ("^(fn)", false),
            ("^(for)", false),
            ("^(if)", false),
            ("^(in)", false),
            ("^(int)", false),
            ("^(let)", false),
            ("^(print)", false),
            ("^(return)", false),
            ("^(string)", false),
            ("^(while)", false),
            ("^(\\{)", false),
            ("^(\\|\\|)", false),
            ("^(\\})", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<'input>(
    input: &'input str,
    (_, __0, _): (usize, Program<'input>, usize),
) -> Program<'input> {
    __0
}

#[allow(unused_variables)]
fn __action1<'input>(
    input: &'input str,
    (_, __0, _): (usize, Item<'input>, usize),
) -> Item<'input> {
    __0
}

#[allow(unused_variables)]
fn __action2<'input>(
    input: &'input str,
    (_, __0, _): (usize, Function<'input>, usize),
) -> Function<'input> {
    __0
}

#[allow(unused_variables)]
fn __action3<'input>(
    input: &'input str,
    (_, __0, _): (usize, Stmt<'input>, usize),
) -> Stmt<'input> {
    __0
}

#[allow(unused_variables)]
fn __action4<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action5<'input>(
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> Program<'input> {
    __0
}

#[allow(unused_variables)]
fn __action6<'input>(
    input: &'input str,
    (_, __0, _): (usize, Function<'input>, usize),
) -> Item<'input> {
    Item::Function(__0)
}

#[allow(unused_variables)]
fn __action7<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<Arg<'input>>, usize),
    (_, ret, _): (usize, core::option::Option<Type>, usize),
    (_, body, _): (usize, Vec<Stmt<'input>>, usize),
) -> Function<'input> {
    Function {
        name: name,
        args: args,
        ret: ret,
        body: body,
    }
}

#[allow(unused_variables)]
fn __action8<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Arg<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Arg<'input>> {
    __0
}

#[allow(unused_variables)]
fn __action9<'input>(
    input: &'input str,
    (_, name, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Type, usize),
) -> Arg<'input> {
    Arg { name: name, ty: ty }
}

#[allow(unused_variables)]
fn __action10<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> Type {
    Type::Int
}

#[allow(unused_variables)]
fn __action11<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> Type {
    Type::Bool
}

#[allow(unused_variables)]
fn __action12<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> Type {
    Type::String
}

#[allow(unused_variables)]
fn __action13<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> &'input str {
    __0
}

#[allow(unused_variables)]
fn __action14<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Stmt<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Stmt<'input>> {
    __0
}

#[allow(unused_variables)]
fn __action15<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Stmt<'input> {
    Stmt::Expr(__0)
}

#[allow(unused_variables)]
fn __action16<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr<'input>, usize),
) -> Stmt<'input> {
    Stmt::Let(__0, __1)
}

#[allow(unused_variables)]
fn __action17<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, __1, _): (usize, Vec<Stmt<'input>>, usize),
    (_, __2, _): (usize, core::option::Option<Vec<Stmt<'input>>>, usize),
) -> Stmt<'input> {
    Stmt::If(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action18<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr<'input>, usize),
    (_, __2, _): (usize, Vec<Stmt<'input>>, usize),
) -> Stmt<'input> {
    Stmt::For(__0, __1, __2)
}

#[allow(unused_variables)]
fn __action19<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Stmt<'input> {
    Stmt::Print(__0)
}

#[allow(unused_variables)]
fn __action20<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, __1, _): (usize, Vec<Stmt<'input>>, usize),
) -> Stmt<'input> {
    Stmt::While(__0, __1)
}

#[allow(unused_variables)]
fn __action21<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Stmt<'input> {
    Stmt::Return(__0)
}

#[allow(unused_variables)]
fn __action22<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action23<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action24<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action25<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action26<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action27<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action28<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action29<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action30<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action31<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action32<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action33<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action34<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, o, _): (usize, BinOp, usize),
    (_, r, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::BinOp(Box::new(l), o, Box::new(r))
}

#[allow(unused_variables)]
fn __action35<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action36<'input>(
    input: &'input str,
    (_, o, _): (usize, UnaryOp, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    Expr::UnaryOp(o, Box::new(e))
}

#[allow(unused_variables)]
fn __action37<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action38<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input> {
    Expr::ArrayAccess(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action39<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Vec<Expr<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input> {
    Expr::Call(Box::new(l), r)
}

#[allow(unused_variables)]
fn __action40<'input>(
    input: &'input str,
    (_, l, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, &'input str, usize),
) -> Expr<'input> {
    Expr::FieldAccess(Box::new(l), r)
}

#[allow(unused_variables)]
fn __action41<'input>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expr<'input> {
    Expr::Var(__0)
}

#[allow(unused_variables)]
fn __action42<'input>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Stmt<'input>>, usize),
) -> Expr<'input> {
    Expr::Block(__0)
}

#[allow(unused_variables)]
fn __action43<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action44<'input>(
    input: &'input str,
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Expr<'input> {
    Expr::Literal(__0)
}

#[allow(unused_variables)]
fn __action45<'input>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Literal<'input> {
    Literal::String(&s[1..s.len() - 1])
}

#[allow(unused_variables)]
fn __action46<'input>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input> {
    Literal::Float(__0.parse().unwrap())
}

#[allow(unused_variables)]
fn __action47<'input>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal<'input> {
    Literal::Integer(__0.parse().unwrap())
}

#[allow(unused_variables)]
fn __action48<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> &'input str {
    __0
}

#[allow(unused_variables)]
fn __action49<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> &'input str {
    __0
}

#[allow(unused_variables)]
fn __action50<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> &'input str {
    __0
}

#[allow(unused_variables)]
fn __action51<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::LogicalOr
}

#[allow(unused_variables)]
fn __action52<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::LogicalAnd
}

#[allow(unused_variables)]
fn __action53<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::Equals
}

#[allow(unused_variables)]
fn __action54<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::NotEquals
}

#[allow(unused_variables)]
fn __action55<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::LessThan
}

#[allow(unused_variables)]
fn __action56<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::GreaterThan
}

#[allow(unused_variables)]
fn __action57<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::LessThanEquals
}

#[allow(unused_variables)]
fn __action58<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::GreaterThanEquals
}

#[allow(unused_variables)]
fn __action59<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::Plus
}

#[allow(unused_variables)]
fn __action60<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::Minus
}

#[allow(unused_variables)]
fn __action61<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::Times
}

#[allow(unused_variables)]
fn __action62<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> BinOp {
    BinOp::Devide
}

#[allow(unused_variables)]
fn __action63<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action64<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action65<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action66<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action67<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action68<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action69<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action70<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action71<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action72<'input>(input: &'input str, (_, __0, _): (usize, BinOp, usize)) -> BinOp {
    __0
}

#[allow(unused_variables)]
fn __action73<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> UnaryOp {
    UnaryOp::Not
}

#[allow(unused_variables)]
fn __action74<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> UnaryOp {
    UnaryOp::Minus
}

#[allow(unused_variables)]
fn __action75<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
    (_, e, _): (usize, core::option::Option<Expr<'input>>, usize),
) -> Vec<Expr<'input>> {
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action76<'input>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Stmt<'input>>, usize),
) -> core::option::Option<Vec<Stmt<'input>>> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action77<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Vec<Stmt<'input>>> {
    None
}

#[allow(unused_variables)]
fn __action78<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Stmt<'input>>, usize),
) -> Vec<Stmt<'input>> {
    __0
}

#[allow(unused_variables)]
fn __action79<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Stmt<'input>>, usize),
    (_, e, _): (usize, core::option::Option<Stmt<'input>>, usize),
) -> Vec<Stmt<'input>> {
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action80<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
    (_, e, _): (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>> {
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action81<'input>(
    input: &'input str,
    (_, __0, _): (usize, Type, usize),
) -> core::option::Option<Type> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action82<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Type> {
    None
}

#[allow(unused_variables)]
fn __action83<'input>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type {
    __0
}

#[allow(unused_variables)]
fn __action84<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Item<'input>> {
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action85<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> alloc::vec::Vec<Item<'input>> {
    v
}

#[allow(unused_variables)]
fn __action86<'input>(
    input: &'input str,
    (_, __0, _): (usize, Item<'input>, usize),
) -> alloc::vec::Vec<Item<'input>> {
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action87<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
    (_, e, _): (usize, Item<'input>, usize),
) -> alloc::vec::Vec<Item<'input>> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action88<'input>(
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
) -> core::option::Option<Arg<'input>> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action89<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Arg<'input>> {
    None
}

#[allow(unused_variables)]
fn __action90<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Arg<'input>> {
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action91<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
) -> alloc::vec::Vec<Arg<'input>> {
    v
}

#[allow(unused_variables)]
fn __action92<'input>(
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Arg<'input> {
    __0
}

#[allow(unused_variables)]
fn __action93<'input>(
    input: &'input str,
    (_, __0, _): (usize, Stmt<'input>, usize),
) -> core::option::Option<Stmt<'input>> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action94<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Stmt<'input>> {
    None
}

#[allow(unused_variables)]
fn __action95<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Stmt<'input>> {
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action96<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Stmt<'input>>, usize),
) -> alloc::vec::Vec<Stmt<'input>> {
    v
}

#[allow(unused_variables)]
fn __action97<'input>(
    input: &'input str,
    (_, __0, _): (usize, Stmt<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Stmt<'input> {
    __0
}

#[allow(unused_variables)]
fn __action98<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> core::option::Option<Expr<'input>> {
    Some(__0)
}

#[allow(unused_variables)]
fn __action99<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Expr<'input>> {
    None
}

#[allow(unused_variables)]
fn __action100<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Expr<'input>> {
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action101<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
) -> alloc::vec::Vec<Expr<'input>> {
    v
}

#[allow(unused_variables)]
fn __action102<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr<'input> {
    __0
}

#[allow(unused_variables)]
fn __action103<'input>(
    input: &'input str,
    (_, __0, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>> {
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action104<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr<'input>>, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> alloc::vec::Vec<Expr<'input>> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action105<'input>(
    input: &'input str,
    (_, __0, _): (usize, Stmt<'input>, usize),
) -> alloc::vec::Vec<Stmt<'input>> {
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action106<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Stmt<'input>>, usize),
    (_, e, _): (usize, Stmt<'input>, usize),
) -> alloc::vec::Vec<Stmt<'input>> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action107<'input>(
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
) -> alloc::vec::Vec<Arg<'input>> {
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action108<'input>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
    (_, e, _): (usize, Arg<'input>, usize),
) -> alloc::vec::Vec<Arg<'input>> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(unused_variables)]
fn __action109<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Type, usize),
) -> core::option::Option<Type> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action83(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action81(input, __temp0)
}

#[allow(unused_variables)]
fn __action110<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Arg<'input>>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Type, usize),
    __5: (usize, Vec<Stmt<'input>>, usize),
) -> Function<'input> {
    let __start0 = __3.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action109(input, __3, __4);
    let __temp0 = (__start0, __temp0, __end0);
    __action7(input, __0, __1, __2, __temp0, __5)
}

#[allow(unused_variables)]
fn __action111<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Arg<'input>>, usize),
    __3: (usize, Vec<Stmt<'input>>, usize),
) -> Function<'input> {
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action82(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action7(input, __0, __1, __2, __temp0, __3)
}

#[allow(unused_variables)]
fn __action112<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<Stmt<'input>>, usize),
) -> core::option::Option<Vec<Stmt<'input>>> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action78(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action76(input, __temp0)
}

#[allow(unused_variables)]
fn __action113<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr<'input>, usize),
    __2: (usize, Vec<Stmt<'input>>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Vec<Stmt<'input>>, usize),
) -> Stmt<'input> {
    let __start0 = __3.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action112(input, __3, __4);
    let __temp0 = (__start0, __temp0, __end0);
    __action17(input, __0, __1, __2, __temp0)
}

#[allow(unused_variables)]
fn __action114<'input>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr<'input>, usize),
    __2: (usize, Vec<Stmt<'input>>, usize),
) -> Stmt<'input> {
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action77(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action17(input, __0, __1, __2, __temp0)
}

#[allow(unused_variables)]
fn __action115<'input>(
    input: &'input str,
    __0: (usize, Arg<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Arg<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action92(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action107(input, __temp0)
}

#[allow(unused_variables)]
fn __action116<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, Arg<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Arg<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action92(input, __1, __2);
    let __temp0 = (__start0, __temp0, __end0);
    __action108(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action117<'input>(
    input: &'input str,
    __0: (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action90(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action80(input, __temp0, __0)
}

#[allow(unused_variables)]
fn __action118<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action91(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action80(input, __temp0, __1)
}

#[allow(unused_variables)]
fn __action119<'input>(
    input: &'input str,
    __0: (usize, Expr<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action102(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action103(input, __temp0)
}

#[allow(unused_variables)]
fn __action120<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, Expr<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action102(input, __1, __2);
    let __temp0 = (__start0, __temp0, __end0);
    __action104(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action121<'input>(
    input: &'input str,
    __0: (usize, core::option::Option<Expr<'input>>, usize),
) -> Vec<Expr<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action100(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action75(input, __temp0, __0)
}

#[allow(unused_variables)]
fn __action122<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, core::option::Option<Expr<'input>>, usize),
) -> Vec<Expr<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action101(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action75(input, __temp0, __1)
}

#[allow(unused_variables)]
fn __action123<'input>(
    input: &'input str,
    __0: (usize, Stmt<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Stmt<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action97(input, __0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action105(input, __temp0)
}

#[allow(unused_variables)]
fn __action124<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Stmt<'input>>, usize),
    __1: (usize, Stmt<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Stmt<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action97(input, __1, __2);
    let __temp0 = (__start0, __temp0, __end0);
    __action106(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action125<'input>(
    input: &'input str,
    __0: (usize, core::option::Option<Stmt<'input>>, usize),
) -> Vec<Stmt<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action95(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action79(input, __temp0, __0)
}

#[allow(unused_variables)]
fn __action126<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Stmt<'input>>, usize),
    __1: (usize, core::option::Option<Stmt<'input>>, usize),
) -> Vec<Stmt<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action96(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action79(input, __temp0, __1)
}

#[allow(unused_variables)]
fn __action127<'input>(input: &'input str, __0: (usize, Arg<'input>, usize)) -> Vec<Arg<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action88(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action117(input, __temp0)
}

#[allow(unused_variables)]
fn __action128<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Arg<'input>> {
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action89(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action117(input, __temp0)
}

#[allow(unused_variables)]
fn __action129<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, Arg<'input>, usize),
) -> Vec<Arg<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action88(input, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action118(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action130<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
) -> Vec<Arg<'input>> {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action89(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action118(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action131<'input>(input: &'input str, __0: (usize, Expr<'input>, usize)) -> Vec<Expr<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action98(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action121(input, __temp0)
}

#[allow(unused_variables)]
fn __action132<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expr<'input>> {
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action99(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action121(input, __temp0)
}

#[allow(unused_variables)]
fn __action133<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
    __1: (usize, Expr<'input>, usize),
) -> Vec<Expr<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action98(input, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action122(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action134<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr<'input>>, usize),
) -> Vec<Expr<'input>> {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action99(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action122(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action135<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program<'input> {
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action84(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action5(input, __temp0)
}

#[allow(unused_variables)]
fn __action136<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> Program<'input> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action85(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action5(input, __temp0)
}

#[allow(unused_variables)]
fn __action137<'input>(input: &'input str, __0: (usize, Stmt<'input>, usize)) -> Vec<Stmt<'input>> {
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action93(input, __0);
    let __temp0 = (__start0, __temp0, __end0);
    __action125(input, __temp0)
}

#[allow(unused_variables)]
fn __action138<'input>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Stmt<'input>> {
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action94(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action125(input, __temp0)
}

#[allow(unused_variables)]
fn __action139<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Stmt<'input>>, usize),
    __1: (usize, Stmt<'input>, usize),
) -> Vec<Stmt<'input>> {
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action93(input, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action126(input, __0, __temp0)
}

#[allow(unused_variables)]
fn __action140<'input>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Stmt<'input>>, usize),
) -> Vec<Stmt<'input>> {
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action94(input, &__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action126(input, __0, __temp0)
}

pub trait __ToTriple<'input> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Token<'input>, usize),
        __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
    >;
}

impl<'input> __ToTriple<'input> for (usize, Token<'input>, usize) {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Token<'input>, usize),
        __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
    > {
        Ok(value)
    }
}
impl<'input> __ToTriple<'input> for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Token<'input>, usize),
        __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
    > {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
