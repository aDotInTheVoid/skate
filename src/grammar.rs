// auto-generated: "lalrpop 0.19.5"
// sha3: 4d18574ffcb2eacd450a48f3f7e7ade24f15ff8f67a1391d24ad47c68d3531
use crate::ast::*;
use crate::diagnostics::FileId;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast::*;
    use crate::diagnostics::FileId;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Spanned<Type>),
        Variant2(core::option::Option<Spanned<Type>>),
        Variant3(Arg<'input>),
        Variant4(alloc::vec::Vec<Arg<'input>>),
        Variant5(Spanned<Stmt<'input>>),
        Variant6(alloc::vec::Vec<Spanned<Stmt<'input>>>),
        Variant7(usize),
        Variant8(core::option::Option<Arg<'input>>),
        Variant9(Vec<Arg<'input>>),
        Variant10(Body<'input>),
        Variant11(Spanned<RawExpr<'input>>),
        Variant12(Function<'input>),
        Variant13(Item<'input>),
        Variant14(alloc::vec::Vec<Item<'input>>),
        Variant15(Spanned<&'input str>),
        Variant16(Program<'input>),
        Variant17(RawExpr<'input>),
        Variant18(Vec<Spanned<Stmt<'input>>>),
        Variant19(Spanned<Vec<Arg<'input>>>),
        Variant20(Spanned<Function<'input>>),
        Variant21(Spanned<Vec<Spanned<Stmt<'input>>>>),
        Variant22(Stmt<'input>),
        Variant23(Type),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 3
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 34, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 0, -43, 24,
        // State 7
        0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 0, -44, 24,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 0, 0, 0, 34, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 0, -43, 24,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -48, -48, -48, 0, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0,
        // State 22
        -38, -38, -38, 0, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0,
        // State 23
        -42, -42, -42, 0, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0,
        // State 24
        0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0,
        // State 25
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 26
        0, -26, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -20, -20, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0,
        // State 30
        0, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0,
        // State 31
        0, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0,
        // State 32
        0, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0,
        // State 33
        0, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 35
        0, 0, 0, 0, 0, -52, 15, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0,
        // State 36
        0, 0, 0, 0, 0, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0,
        // State 37
        0, 0, 0, 0, 0, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0,
        // State 39
        0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0,
        // State 41
        0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -28, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10,
        // State 45
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0,
        // State 46
        0, 0, 0, 0, 0, -52, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, 0, 0, -15, -15,
        // State 50
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11,
        // State 54
        0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16, 0, 0, -16, -16,
        // State 57
        0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -39,
        // State 1
        -40,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -46,
        // State 17
        -36,
        // State 18
        -60,
        // State 19
        -33,
        // State 20
        -37,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        -32,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -31,
        // State 59
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            6 => 7,
            9 => 8,
            12 => match state {
                7 => 43,
                _ => 26,
            },
            14 => 24,
            15 => match state {
                13 => 55,
                _ => 34,
            },
            16 => 27,
            17 => match state {
                8 => 46,
                10 => 51,
                11 => 52,
                14 => 57,
                15 => 59,
                _ => 35,
            },
            18 => 16,
            19 => match state {
                1 => 20,
                _ => 17,
            },
            21 => 1,
            22 => match state {
                2 => 3,
                4 | 7 => 28,
                9 => 50,
                _ => 36,
            },
            23 => 18,
            24 => 37,
            25 => 21,
            26 => 38,
            27 => 25,
            28 => 19,
            29 => 39,
            30 => 22,
            31 => 40,
            32 => match state {
                8 => 47,
                _ => 41,
            },
            33 => match state {
                12 => 54,
                _ => 29,
            },
            34 => 42,
            35 => 30,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""->""###,
            r###"":""###,
            r###"";""###,
            r###""=""###,
            r###""bool""###,
            r###""fn""###,
            r###""int""###,
            r###""let""###,
            r###""print""###,
            r###""return""###,
            r###""string""###,
            r###""{""###,
            r###""}""###,
            r###"r#"[a-zA-Z][a-zA-Z0-9_]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        file_id: FileId,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Program<'input>;
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
            __action(state, 17 - 1)
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
                self.file_id,
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
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(6, _) if true => Some(5),
            Token(7, _) if true => Some(6),
            Token(8, _) if true => Some(7),
            Token(9, _) if true => Some(8),
            Token(10, _) if true => Some(9),
            Token(11, _) if true => Some(10),
            Token(12, _) if true => Some(11),
            Token(13, _) if true => Some(12),
            Token(14, _) if true => Some(13),
            Token(15, _) if true => Some(14),
            Token(16, _) if true => Some(15),
            Token(0, _) if true => Some(16),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
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
        pub fn parse<
            'input,
        >(
            &self,
            file_id: FileId,
            input: &'input str,
        ) -> Result<Program<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    file_id,
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Program<'input>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(file_id, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant16(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(file_id, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Arg<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Body<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Function<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Item<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RawExpr<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Function<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<RawExpr<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Stmt<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Vec<Arg<'input>>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Vec<Spanned<Stmt<'input>>>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Arg<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Spanned<Stmt<'input>>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Arg<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Item<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Arg<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Spanned<Type>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" <Spaned<Type>>) = "->", Spaned<Type> => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" <Spaned<Type>>)? = "->", Spaned<Type> => ActionFn(53);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" <Spaned<Type>>)? =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (":" <Spaned<Type>>) = ":", Spaned<Type> => ActionFn(27);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (":" <Spaned<Type>>)? = ":", Spaned<Type> => ActionFn(56);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action56::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (":" <Spaned<Type>>)? =  => ActionFn(26);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action26::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg> ",") = Arg, "," => ActionFn(43);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg> ",")* =  => ActionFn(41);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action41::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg> ",")* = (<Arg> ",")+ => ActionFn(42);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg> ",")+ = Arg, "," => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg> ",")+ = (<Arg> ",")+, Arg, "," => ActionFn(60);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(file_id, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spaned<Stmt>> ";") = Spaned<Stmt>, ";" => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spaned<Stmt>> ";")* =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spaned<Stmt>> ";")* = (<Spaned<Stmt>> ";")+ => ActionFn(45);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spaned<Stmt>> ";")+ = Spaned<Stmt>, ";" => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Spaned<Stmt>> ";")+ = (<Spaned<Stmt>> ";")+, Spaned<Stmt>, ";" => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(file_id, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(47);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action47::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Arg = Name, ":", Spaned<Type> => ActionFn(57);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action57::<>(file_id, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Arg = Name => ActionFn(58);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Arg? = Arg => ActionFn(39);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Arg? =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Args = "(", Comma<Arg>, ")" => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(file_id, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Body = Expr => ActionFn(11);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Body = Spaned<Semi<Spaned<Stmt>>> => ActionFn(12);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Arg> = Arg => ActionFn(81);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Arg> =  => ActionFn(82);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Arg> = (<Arg> ",")+, Arg => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Arg> = (<Arg> ",")+ => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Spaned<RExpr> => ActionFn(18);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = "fn", Name, Spaned<Args>, "->", Spaned<Type>, "{", Body, "}" => ActionFn(54);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant10(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action54::<>(file_id, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (8, 18)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = "fn", Name, Spaned<Args>, "{", Body, "}" => ActionFn(55);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action55::<>(file_id, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 18)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Item = Spaned<Function> => ActionFn(2);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Item* =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 20)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Item* = Item+ => ActionFn(36);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Item+ = Item => ActionFn(37);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Item+ = Item+, Item => ActionFn(38);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action38::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Name = Spaned<RawName> => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(85);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action85::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Item+ => ActionFn(86);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RExpr = Name => ActionFn(19);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RawName = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Semi<Spaned<Stmt>> =  => ActionFn(65);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action65::<>(file_id, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Semi<Spaned<Stmt>> = (<Spaned<Stmt>> ";")+ => ActionFn(66);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<Args> = Args => ActionFn(74);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<Function> = Function => ActionFn(75);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<RExpr> = RExpr => ActionFn(76);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<RawName> = RawName => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<Semi<Spaned<Stmt>>> = Semi<Spaned<Stmt>> => ActionFn(78);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<Stmt> = Stmt => ActionFn(79);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Spaned<Type> = Type => ActionFn(80);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action80::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr => ActionFn(13);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = "let", Name, "=", Expr => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(file_id, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (4, 34)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = Expr, "=", Expr => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(file_id, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = "print", Expr => ActionFn(16);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 34)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = "return", Expr => ActionFn(17);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action17::<>(file_id, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 34)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "int" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "bool" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        file_id: FileId,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "string" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(file_id, input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 35)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::*;
    use crate::diagnostics::FileId;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([A-Za-z][0-9A-Z_a-z]*)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^(\\->)", false),
            ("^(:)", false),
            ("^(;)", false),
            ("^(=)", false),
            ("^(bool)", false),
            ("^(fn)", false),
            ("^(int)", false),
            ("^(let)", false),
            ("^(print)", false),
            ("^(return)", false),
            ("^(string)", false),
            ("^(\\{)", false),
            ("^(\\})", false),
            ("^(//[\u{0}-\t\u{b}-\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Program<'input>, usize),
) -> Program<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> Program<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Function<'input>>, usize),
) -> Item<'input>
{
    Item::Function(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, Spanned<&'input str>, usize),
    (_, args, _): (usize, Spanned<Vec<Arg<'input>>>, usize),
    (_, ret, _): (usize, core::option::Option<Spanned<Type>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Body<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Function<'input>
{
    Function{name:name, args:args, ret:ret, body:body}
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Arg<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Arg<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, name, _): (usize, Spanned<&'input str>, usize),
    (_, ty, _): (usize, core::option::Option<Spanned<Type>>, usize),
) -> Arg<'input>
{
    Arg{name:name, ty:ty}
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Int
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Bool
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::String
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<&'input str>, usize),
) -> Spanned<&'input str>
{
    __0
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Body<'input>
{
    Body::Expr(__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Vec<Spanned<Stmt<'input>>>>, usize),
) -> Body<'input>
{
    Body::Block(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Stmt<'input>
{
    Stmt::Expr(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Spanned<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Stmt<'input>
{
    Stmt::Let(__0, __1)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Stmt<'input>
{
    Stmt::Assign(__0, __1)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Stmt<'input>
{
    Stmt::Print(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Stmt<'input>
{
    Stmt::Return(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<RawExpr<'input>>, usize),
) -> Spanned<RawExpr<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<&'input str>, usize),
) -> RawExpr<'input>
{
    RawExpr::Var(__0)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, RawExpr<'input>, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<RawExpr<'input>>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, Vec<Spanned<Stmt<'input>>>, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<Vec<Spanned<Stmt<'input>>>>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize),
) -> Vec<Spanned<Stmt<'input>>>
{
    v
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, Stmt<'input>, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<Stmt<'input>>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<&'input str>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Type>, usize),
) -> core::option::Option<Spanned<Type>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Spanned<Type>>
{
    None
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Spanned<Type>, usize),
) -> Spanned<Type>
{
    __0
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
    (_, e, _): (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>>
{
    match e {
        None=> v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Type>, usize),
) -> core::option::Option<Spanned<Type>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Spanned<Type>>
{
    None
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Spanned<Type>, usize),
) -> Spanned<Type>
{
    __0
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, Type, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<Type>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, Vec<Arg<'input>>, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<Vec<Arg<'input>>>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, node, _): (usize, Function<'input>, usize),
    (_, end, _): (usize, usize, usize),
) -> Spanned<Function<'input>>
{
    Spanned { node, span: Span { start, end, file_id } }
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Item<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> alloc::vec::Vec<Item<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Item<'input>, usize),
) -> alloc::vec::Vec<Item<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Item<'input>>, usize),
    (_, e, _): (usize, Item<'input>, usize),
) -> alloc::vec::Vec<Item<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
) -> core::option::Option<Arg<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Arg<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Arg<'input>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
) -> alloc::vec::Vec<Arg<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Arg<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize),
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    v
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Stmt<'input>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Spanned<Stmt<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Spanned<Stmt<'input>>, usize),
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize),
    (_, e, _): (usize, Spanned<Stmt<'input>>, usize),
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, __0, _): (usize, Arg<'input>, usize),
) -> alloc::vec::Vec<Arg<'input>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Arg<'input>>, usize),
    (_, e, _): (usize, Arg<'input>, usize),
) -> alloc::vec::Vec<Arg<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Spanned<Type>, usize),
) -> core::option::Option<Spanned<Type>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action31(
        file_id,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Spanned<&'input str>, usize),
    __2: (usize, Spanned<Vec<Arg<'input>>>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Spanned<Type>, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Body<'input>, usize),
    __7: (usize, &'input str, usize),
) -> Function<'input>
{
    let __start0 = __3.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action53(
        file_id,
        input,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        file_id,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Spanned<&'input str>, usize),
    __2: (usize, Spanned<Vec<Arg<'input>>>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Body<'input>, usize),
    __5: (usize, &'input str, usize),
) -> Function<'input>
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action30(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        file_id,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Spanned<Type>, usize),
) -> core::option::Option<Spanned<Type>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action27(
        file_id,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Spanned<&'input str>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Spanned<Type>, usize),
) -> Arg<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        file_id,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Spanned<&'input str>, usize),
) -> Arg<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Arg<'input>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Arg<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        file_id,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, Arg<'input>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Arg<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action43(
        file_id,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action41(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        file_id,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, core::option::Option<Arg<'input>>, usize),
) -> Vec<Arg<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        file_id,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        file_id,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Spanned<Stmt<'input>>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        file_id,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize),
    __1: (usize, Spanned<Stmt<'input>>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Spanned<Stmt<'input>>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action46(
        file_id,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Spanned<Stmt<'input>>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action44(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Spanned<Stmt<'input>>>, usize),
) -> Vec<Spanned<Stmt<'input>>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action45(
        file_id,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Vec<Arg<'input>>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Vec<Arg<'input>>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Function<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Function<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, RawExpr<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<RawExpr<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Spanned<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Vec<Spanned<Stmt<'input>>>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Vec<Spanned<Stmt<'input>>>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Stmt<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Stmt<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Type, usize),
    __1: (usize, usize, usize),
) -> Spanned<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action48(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        file_id,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Vec<Arg<'input>>, usize),
) -> Spanned<Vec<Arg<'input>>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Function<'input>, usize),
) -> Spanned<Function<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, RawExpr<'input>, usize),
) -> Spanned<RawExpr<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Spanned<&'input str>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Vec<Spanned<Stmt<'input>>>, usize),
) -> Spanned<Vec<Spanned<Stmt<'input>>>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Stmt<'input>, usize),
) -> Spanned<Stmt<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Type, usize),
) -> Spanned<Type>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, Arg<'input>, usize),
) -> Vec<Arg<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        file_id,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Arg<'input>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action40(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
    __1: (usize, Arg<'input>, usize),
) -> Vec<Arg<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        file_id,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Arg<'input>>, usize),
) -> Vec<Arg<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        file_id,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program<'input>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action35(
        file_id,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        file_id,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
>(
    file_id: FileId,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Item<'input>>, usize),
) -> Program<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        file_id,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        file_id,
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
