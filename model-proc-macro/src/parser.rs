// auto-generated: "lalrpop 0.19.8"
// sha3: 6c99215a868db4dcacf9529d7eb3127527f8210558d2742f50c4275007321234
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Model {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use crate::ast::*;
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
        Variant1(Field),
        Variant2(alloc::vec::Vec<Field>),
        Variant3(String),
        Variant4(core::option::Option<String>),
        Variant5(Vec<Field>),
        Variant6(core::option::Option<Field>),
        Variant7(bool),
        Variant8(FieldForeignNode),
        Variant9(FieldRelation),
        Variant10(FieldProperty),
        Variant11(core::option::Option<&'input str>),
        Variant12(Model),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, -28, 0, -17, 0, -17, -17, 24,
        // State 2
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, -30, 0, -17, 0, -17, -17, 24,
        // State 4
        0, 0, 0, 0, 0, 0, 16, 0, 6, 7, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, -28, 0, -17, 0, -17, -17, 24,
        // State 8
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        -22, -22, -22, -22, -22, 0, 0, -22, -22, -22, 0,
        // State 16
        0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0,
        // State 17
        28, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 18
        -11, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 19
        -13, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0,
        // State 20
        -10, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 21
        -12, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, -16, 0, -16, -16, 0,
        // State 24
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        30, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -4, 0, -4, 0, -4, -4, -4,
        // State 28
        -20, 9, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, -5, 0, -5, 0, -5, -5, -5,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 32
        0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0,
        // State 38
        -19, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 39
        -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 11 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
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
        -31,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
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
        -26,
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
        -25,
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
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 3,
            5 => match state {
                7 => 32,
                _ => 16,
            },
            6 => match state {
                3 => 25,
                _ => 17,
            },
            8 => 4,
            9 => 18,
            10 => 19,
            11 => 20,
            12 => 21,
            13 => match state {
                2 => 24,
                4 => 28,
                5 => 30,
                6 => 31,
                8 => 33,
                9 => 36,
                10 => 37,
                11 => 38,
                12 => 39,
                _ => 13,
            },
            15 => 14,
            16 => 22,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###"",""###,
            r###""<""###,
            r###"">""###,
            r###""{""###,
            r###""}""###,
            r###"r#"[0-9]+"#"###,
            r###"IdentifierRegex"###,
            r###"KeywordAs"###,
            r###"KeywordIncomingEdge"###,
            r###"KeywordOutgoingEdge"###,
            r###"KeywordPub"###,
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
        type Success = Model;
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
            __action(state, 11 - 1)
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
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(12, _) if true => Some(7),
            Token(11, _) if true => Some(8),
            Token(10, _) if true => Some(9),
            Token(13, _) if true => Some(10),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(9, __tok0) | Token(6, __tok0) | Token(12, __tok0) | Token(11, __tok0) | Token(10, __tok0) | Token(13, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ModelParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ModelParser {
        pub fn new() -> ModelParser {
            let __builder = super::__intern_token::new_builder();
            ModelParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Model, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Model,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                // __Model = Model => ActionFn(0);
                let __sym0 = __pop_Variant12(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
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
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FieldForeignNode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FieldProperty, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FieldRelation, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Model, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Field>, usize)
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
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
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
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",") = Field, "," => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")* =  => ActionFn(21);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action21::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")* = (<Field> ",")+ => ActionFn(22);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")+ = Field, "," => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (KeywordAs <Identifier>) = KeywordAs, Identifier => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (KeywordAs <Identifier>)? = KeywordAs, Identifier => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (KeywordAs <Identifier>)? =  => ActionFn(17);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action17::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CommaSeparatedFields = TrailingComma<Field> => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field = FieldProperty => ActionFn(3);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field = FieldForeignNode => ActionFn(4);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field = FieldRelation => ActionFn(5);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field = FieldForeignRelation => ActionFn(6);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field? = Field => ActionFn(19);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field? =  => ActionFn(20);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action20::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldEncapsulation = KeywordPub => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldEncapsulation =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldForeignNode = FieldEncapsulation, Identifier, "<", Identifier, ">" => ActionFn(8);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (5, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldForeignRelation = FieldEncapsulation, KeywordIncomingEdge, Identifier, KeywordIncomingEdge, Identifier, KeywordAs, Identifier => ActionFn(10);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant3(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (7, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldProperty = FieldEncapsulation, Identifier => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FieldRelation = FieldEncapsulation, KeywordOutgoingEdge, Identifier, KeywordOutgoingEdge, Identifier, KeywordAs, Identifier => ActionFn(9);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant3(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant3(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (7, 12)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Identifier = IdentifierRegex => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // KeywordPub? = KeywordPub => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // KeywordPub? =  => ActionFn(14);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action14::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Model = Identifier, KeywordAs, Identifier, "{", CommaSeparatedFields, "}" => ActionFn(31);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action31::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Model = Identifier, "{", CommaSeparatedFields, "}" => ActionFn(32);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action32::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<Field> = Field => ActionFn(33);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<Field> =  => ActionFn(34);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action34::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<Field> = (<Field> ",")+, Field => ActionFn(35);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action35::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TrailingComma<Field> = (<Field> ",")+ => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 16)
    }
}
pub use self::__parse__Model::ModelParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(,)", false),
            ("^(<)", false),
            ("^(>)", false),
            ("^(\\{)", false),
            ("^(\\})", false),
            ("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]*)", true),
            ("^([0-9A-Z_a-zªµºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬˮ\u{300}-ʹͶ-ͷͺ-ͽͿΆΈ-ΊΌΎ-ΡΣ-ϵϷ-ҁ\u{483}-ԯԱ-Ֆՙՠ-ֈ\u{591}-\u{5bd}\u{5bf}\u{5c1}-\u{5c2}\u{5c4}-\u{5c5}\u{5c7}א-תׯ-ײ\u{610}-\u{61a}ؠ-٩ٮ-ۓە-\u{6dc}\u{6df}-\u{6e8}\u{6ea}-ۼۿܐ-\u{74a}ݍ-ޱ߀-ߵߺ\u{7fd}ࠀ-\u{82d}ࡀ-\u{85b}ࡠ-ࡪࡰ-ࢇࢉ-ࢎ\u{898}-\u{8e1}\u{8e3}-\u{963}०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রলশ-হ\u{9bc}-\u{9c4}ে-ৈো-ৎ\u{9d7}ড়-ঢ়য়-\u{9e3}০-ৱৼ\u{9fe}\u{a01}-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ\u{a3c}ਾ-\u{a42}\u{a47}-\u{a48}\u{a4b}-\u{a4d}\u{a51}ਖ਼-ੜਫ਼੦-\u{a75}\u{a81}-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ\u{abc}-\u{ac5}\u{ac7}-ૉો-\u{acd}ૐૠ-\u{ae3}૦-૯ૹ-\u{aff}\u{b01}-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ\u{b3c}-\u{b44}େ-ୈୋ-\u{b4d}\u{b55}-\u{b57}ଡ଼-ଢ଼ୟ-\u{b63}୦-୯ୱ\u{b82}-ஃஅ-ஊஎ-ஐஒ-கங-சஜஞ-டண-தந-பம-ஹ\u{bbe}-ூெ-ைொ-\u{bcd}ௐ\u{bd7}௦-௯\u{c00}-ఌఎ-ఐఒ-నప-హ\u{c3c}-ౄ\u{c46}-\u{c48}\u{c4a}-\u{c4d}\u{c55}-\u{c56}ౘ-ౚౝౠ-\u{c63}౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ\u{cbc}-ೄ\u{cc6}-ೈೊ-\u{ccd}\u{cd5}-\u{cd6}ೝ-ೞೠ-\u{ce3}೦-೯ೱ-ೳ\u{d00}-ഌഎ-ഐഒ-\u{d44}െ-ൈൊ-ൎൔ-\u{d57}ൟ-\u{d63}൦-൯ൺ-ൿ\u{d81}-ඃඅ-ඖක-නඳ-රලව-ෆ\u{dca}\u{dcf}-\u{dd4}\u{dd6}ෘ-\u{ddf}෦-෯ෲ-ෳก-\u{e3a}เ-\u{e4e}๐-๙ກ-ຂຄຆ-ຊຌ-ຣລວ-ຽເ-ໄໆ\u{ec8}-\u{ece}໐-໙ໜ-ໟༀ\u{f18}-\u{f19}༠-༩\u{f35}\u{f37}\u{f39}༾-ཇཉ-ཬ\u{f71}-\u{f84}\u{f86}-\u{f97}\u{f99}-\u{fbc}\u{fc6}က-၉ၐ-\u{109d}Ⴀ-ჅჇჍა-ჺჼ-ቈቊ-ቍቐ-ቖቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ\u{135d}-\u{135f}ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-᜕ᜟ-᜴ᝀ-\u{1753}ᝠ-ᝬᝮ-ᝰ\u{1772}-\u{1773}ក-\u{17d3}ៗៜ-\u{17dd}០-៩\u{180b}-\u{180d}\u{180f}-᠙ᠠ-ᡸᢀ-ᢪᢰ-ᣵᤀ-ᤞ\u{1920}-ᤫᤰ-\u{193b}᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-\u{1a1b}ᨠ-\u{1a5e}\u{1a60}-\u{1a7c}\u{1a7f}-᪉᪐-᪙ᪧ\u{1ab0}-\u{1ace}\u{1b00}-ᭌ᭐-᭙\u{1b6b}-\u{1b73}\u{1b80}-᯳ᰀ-\u{1c37}᱀-᱉ᱍ-ᱽᲀ-ᲈᲐ-ᲺᲽ-Ჿ\u{1cd0}-\u{1cd2}\u{1cd4}-ᳺᴀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙὛὝὟ-ώᾀ-ᾴᾶ-ᾼιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔ⁱⁿₐ-ₜ\u{20d0}-\u{20f0}ℂℇℊ-ℓℕℙ-ℝℤΩℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎⅠ-ↈⒶ-ⓩⰀ-ⳤⳫ-ⳳⴀ-ⴥⴧⴭⴰ-ⵧⵯ\u{2d7f}-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞ\u{2de0}-\u{2dff}ⸯ々-〇〡-\u{302f}〱-〵〸-〼ぁ-ゖ\u{3099}-\u{309a}ゝ-ゟァ-ヺー-ヿㄅ-ㄯㄱ-ㆎㆠ-ㆿㇰ-ㇿ㐀-䶿一-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-\u{a672}\u{a674}-\u{a67d}ꙿ-\u{a6f1}ꜗ-ꜟꜢ-ꞈꞋ-ꟊꟐ-ꟑꟓꟕ-ꟙꟲ-ꠧ\u{a82c}ꡀ-ꡳꢀ-\u{a8c5}꣐-꣙\u{a8e0}-ꣷꣻꣽ-\u{a92d}ꤰ-꥓ꥠ-ꥼ\u{a980}-꧀ꧏ-꧙ꧠ-ꧾꨀ-\u{aa36}ꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-\u{aaf6}ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭩꭰ-ꯪ꯬-\u{abed}꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ\u{fe00}-\u{fe0f}\u{fe20}-\u{fe2f}︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴\u{101fd}𐊀-𐊜𐊠-𐋐\u{102e0}𐌀-𐌟𐌭-𐍊𐍐-\u{1037a}𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐕰-𐕺𐕼-𐖊𐖌-𐖒𐖔-𐖕𐖗-𐖡𐖣-𐖱𐖳-𐖹𐖻-𐖼𐘀-𐜶𐝀-𐝕𐝠-𐝧𐞀-𐞅𐞇-𐞰𐞲-𐞺𐠀-𐠅𐠈𐠊-𐠵𐠷-𐠸𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-\u{10a03}\u{10a05}-\u{10a06}\u{10a0c}-𐨓𐨕-𐨗𐨙-𐨵\u{10a38}-\u{10a3a}\u{10a3f}𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-\u{10ae6}𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𐴀-\u{10d27}𐴰-𐴹𐺀-𐺩\u{10eab}-\u{10eac}𐺰-𐺱\u{10efd}-𐼜𐼧𐼰-\u{10f50}𐽰-\u{10f85}𐾰-𐿄𐿠-𐿶𑀀-\u{11046}𑁦-𑁵\u{1107f}-\u{110ba}\u{110c2}𑃐-𑃨𑃰-𑃹\u{11100}-\u{11134}𑄶-𑄿𑅄-𑅇𑅐-\u{11173}𑅶\u{11180}-𑇄\u{111c9}-\u{111cc}𑇎-𑇚𑇜𑈀-𑈑𑈓-\u{11237}\u{1123e}-\u{11241}𑊀-𑊆𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-\u{112ea}𑋰-𑋹\u{11300}-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹\u{1133b}-𑍄𑍇-𑍈𑍋-𑍍𑍐\u{11357}𑍝-𑍣\u{11366}-\u{1136c}\u{11370}-\u{11374}𑐀-𑑊𑑐-𑑙\u{1145e}-𑑡𑒀-𑓅𑓇𑓐-𑓙𑖀-\u{115b5}𑖸-\u{115c0}𑗘-\u{115dd}𑘀-\u{11640}𑙄𑙐-𑙙𑚀-𑚸𑛀-𑛉𑜀-𑜚\u{1171d}-\u{1172b}𑜰-𑜹𑝀-𑝆𑠀-\u{1183a}𑢠-𑣩𑣿-𑤆𑤉𑤌-𑤓𑤕-𑤖𑤘-𑤵𑤷-𑤸\u{1193b}-\u{11943}𑥐-𑥙𑦠-𑦧𑦪-\u{119d7}\u{119da}-𑧡𑧣-𑧤𑨀-\u{11a3e}\u{11a47}𑩐-\u{11a99}𑪝𑪰-𑫸𑰀-𑰈𑰊-\u{11c36}\u{11c38}-𑱀𑱐-𑱙𑱲-𑲏\u{11c92}-\u{11ca7}𑲩-\u{11cb6}𑴀-𑴆𑴈-𑴉𑴋-\u{11d36}\u{11d3a}\u{11d3c}-\u{11d3d}\u{11d3f}-\u{11d47}𑵐-𑵙𑵠-𑵥𑵧-𑵨𑵪-𑶎\u{11d90}-\u{11d91}𑶓-𑶘𑶠-𑶩𑻠-𑻶\u{11f00}-𑼐𑼒-\u{11f3a}𑼾-\u{11f42}𑽐-𑽙𑾰𒀀-𒎙𒐀-𒑮𒒀-𒕃𒾐-𒿰𓀀-𓐯\u{13440}-\u{13455}𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖩰-𖪾𖫀-𖫉𖫐-𖫭\u{16af0}-\u{16af4}𖬀-\u{16b36}𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖹀-𖹿𖼀-𖽊\u{16f4f}-𖾇\u{16f8f}-𖾟𖿠-𖿡𖿣-\u{16fe4}𖿰-𖿱𗀀-𘟷𘠀-𘳕𘴀-𘴈𚿰-𚿳𚿵-𚿻𚿽-𚿾𛀀-𛄢𛄲𛅐-𛅒𛅕𛅤-𛅧𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙\u{1bc9d}-\u{1bc9e}\u{1cf00}-\u{1cf2d}\u{1cf30}-\u{1cf46}\u{1d165}-\u{1d169}𝅭-\u{1d172}\u{1d17b}-\u{1d182}\u{1d185}-\u{1d18b}\u{1d1aa}-\u{1d1ad}\u{1d242}-\u{1d244}𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿\u{1da00}-\u{1da36}\u{1da3b}-\u{1da6c}\u{1da75}\u{1da84}\u{1da9b}-\u{1da9f}\u{1daa1}-\u{1daaf}𝼀-𝼞𝼥-𝼪\u{1e000}-\u{1e006}\u{1e008}-\u{1e018}\u{1e01b}-\u{1e021}\u{1e023}-\u{1e024}\u{1e026}-\u{1e02a}𞀰-𞁭\u{1e08f}𞄀-𞄬\u{1e130}-𞄽𞅀-𞅉𞅎𞊐-\u{1e2ae}𞋀-𞋹𞓐-𞓹𞟠-𞟦𞟨-𞟫𞟭-𞟮𞟰-𞟾𞠀-𞣄\u{1e8d0}-\u{1e8d6}𞤀-𞥋𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤𞸧𞸩-𞸲𞸴-𞸷𞸹𞸻𞹂𞹇𞹉𞹋𞹍-𞹏𞹑-𞹒𞹔𞹗𞹙𞹛𞹝𞹟𞹡-𞹢𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉🯰-🯹𠀀-𪛟𪜀-𫜹𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀𰀀-𱍊𱍐-𲎯\u{e0100}-\u{e01ef}]+)", false),
            ("^(//[\0-\t\u{b}-\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("^(/\\*[\0-\\)\\+-\u{10ffff}]*[\0-\\.0-\u{10ffff}]*(\\*/)[\n\r]*)", true),
            ("^([0-9]+)", false),
            ("^(\\->)", false),
            ("^(<\\-)", false),
            ("^(as)", false),
            ("^(pub)", false),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Model, usize),
) -> Model
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, alias, _): (usize, core::option::Option<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, fields, _): (usize, Vec<Field>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Model
{
    Model { name, fields, alias }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Field>, usize),
) -> Vec<Field>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, FieldProperty, usize),
) -> Field
{
    Field::Property(__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, FieldForeignNode, usize),
) -> Field
{
    Field::ForeignNode(__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, FieldRelation, usize),
) -> Field
{
    Field::Relation(__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, FieldRelation, usize),
) -> Field
{
    Field::Relation(__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, is_public, _): (usize, bool, usize),
    (_, name, _): (usize, String, usize),
) -> FieldProperty
{
    FieldProperty { name, is_public }
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, is_public, _): (usize, bool, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, foreign_type, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> FieldForeignNode
{
    FieldForeignNode { name, foreign_type, is_public }
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, is_public, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, foreign_type, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, alias, _): (usize, String, usize),
) -> FieldRelation
{
    FieldRelation { name, foreign_type, alias, relation_type: FieldRelationType::OutgoingEdge, is_public }
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, is_public, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, foreign_type, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, alias, _): (usize, String, usize),
) -> FieldRelation
{
    FieldRelation { name, foreign_type, alias, relation_type: FieldRelationType::IncomingEdge, is_public }
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, is_public, _): (usize, core::option::Option<&'input str>, usize),
) -> bool
{
    is_public.is_some()
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> core::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Field>, usize),
    (_, e, _): (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    match e {
      None => v,
      Some(e) => {
          v.push(e);
          v
      }
  }
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> core::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<String>
{
    None
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> core::option::Option<Field>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Field>
{
    None
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Field>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Field>, usize),
) -> alloc::vec::Vec<Field>
{
    v
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Field
{
    __0
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> alloc::vec::Vec<Field>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> alloc::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action23(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action21(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action22(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> core::option::Option<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Vec<Field>, usize),
    __5: (usize, &'input str, usize),
) -> Model
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __0,
        __temp0,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Field>, usize),
    __3: (usize, &'input str, usize),
) -> Model
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action19(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> bool
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action13(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> bool
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action14(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
