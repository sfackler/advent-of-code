#![allow(unused_imports)]
#![allow(unused_variables)]
use ast::{Source, Gate, Op};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__gates {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::{Source, Gate, Op};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_gates<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Gate<'input>>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____gates(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        Gate(Gate<'input>),
        Gate_2a(::std::vec::Vec<Gate<'input>>),
        Gate_2b(::std::vec::Vec<Gate<'input>>),
        ShiftWidth(u8),
        Signal(u16),
        Source(Source<'input>),
        Wire(&'input str),
        ____gates(Vec<Gate<'input>>),
        gates(Vec<Gate<'input>>),
    }

    // State 0
    //   Gate = (*) Source "->" Wire [EOF]
    //   Gate = (*) Source "->" Wire ["NOT"]
    //   Gate = (*) Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "AND" Source "->" Wire [EOF]
    //   Gate = (*) Source "AND" Source "->" Wire ["NOT"]
    //   Gate = (*) Source "AND" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "AND" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "OR" Source "->" Wire [EOF]
    //   Gate = (*) Source "OR" Source "->" Wire ["NOT"]
    //   Gate = (*) Source "OR" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "OR" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) "NOT" Source "->" Wire [EOF]
    //   Gate = (*) "NOT" Source "->" Wire ["NOT"]
    //   Gate = (*) "NOT" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) "NOT" Source "->" Wire [r#"[a-z]+"#]
    //   Gate+ = (*) Gate [EOF]
    //   Gate+ = (*) Gate ["NOT"]
    //   Gate+ = (*) Gate [r#"[0-9]+"#]
    //   Gate+ = (*) Gate [r#"[a-z]+"#]
    //   Gate+ = (*) Gate+ Gate [EOF]
    //   Gate+ = (*) Gate+ Gate ["NOT"]
    //   Gate+ = (*) Gate+ Gate [r#"[0-9]+"#]
    //   Gate+ = (*) Gate+ Gate [r#"[a-z]+"#]
    //   Signal = (*) r#"[0-9]+"# ["->"]
    //   Signal = (*) r#"[0-9]+"# ["AND"]
    //   Signal = (*) r#"[0-9]+"# ["LSHIFT"]
    //   Signal = (*) r#"[0-9]+"# ["OR"]
    //   Signal = (*) r#"[0-9]+"# ["RSHIFT"]
    //   Source = (*) Signal ["->"]
    //   Source = (*) Signal ["AND"]
    //   Source = (*) Signal ["LSHIFT"]
    //   Source = (*) Signal ["OR"]
    //   Source = (*) Signal ["RSHIFT"]
    //   Source = (*) Wire ["->"]
    //   Source = (*) Wire ["AND"]
    //   Source = (*) Wire ["LSHIFT"]
    //   Source = (*) Wire ["OR"]
    //   Source = (*) Wire ["RSHIFT"]
    //   Wire = (*) r#"[a-z]+"# ["->"]
    //   Wire = (*) r#"[a-z]+"# ["AND"]
    //   Wire = (*) r#"[a-z]+"# ["LSHIFT"]
    //   Wire = (*) r#"[a-z]+"# ["OR"]
    //   Wire = (*) r#"[a-z]+"# ["RSHIFT"]
    //   __gates = (*) gates [EOF]
    //   gates = (*) [EOF]
    //   gates = (*) Gate+ [EOF]
    //
    //   EOF -> Reduce(gates =  => ActionFn(17);)
    //   "NOT" -> Shift(S7)
    //   r#"[0-9]+"# -> Shift(S8)
    //   r#"[a-z]+"# -> Shift(S9)
    //
    //   Gate -> S1
    //   Gate+ -> S2
    //   Signal -> S3
    //   Source -> S4
    //   Wire -> S5
    //   gates -> S6
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym0));
            }
            None => {
                let __nt = super::__action17(input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::gates(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Gate(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Gate_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Signal(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Source(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Wire(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::gates(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Gate+ = Gate (*) [EOF]
    //   Gate+ = Gate (*) ["NOT"]
    //   Gate+ = Gate (*) [r#"[0-9]+"#]
    //   Gate+ = Gate (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate+ = Gate => ActionFn(15);)
    //   "NOT" -> Reduce(Gate+ = Gate => ActionFn(15);)
    //   r#"[0-9]+"# -> Reduce(Gate+ = Gate => ActionFn(15);)
    //   r#"[a-z]+"# -> Reduce(Gate+ = Gate => ActionFn(15);)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Gate<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Gate = (*) Source "->" Wire [EOF]
    //   Gate = (*) Source "->" Wire ["NOT"]
    //   Gate = (*) Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "AND" Source "->" Wire [EOF]
    //   Gate = (*) Source "AND" Source "->" Wire ["NOT"]
    //   Gate = (*) Source "AND" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "AND" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "LSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "OR" Source "->" Wire [EOF]
    //   Gate = (*) Source "OR" Source "->" Wire ["NOT"]
    //   Gate = (*) Source "OR" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "OR" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) Source "RSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   Gate = (*) "NOT" Source "->" Wire [EOF]
    //   Gate = (*) "NOT" Source "->" Wire ["NOT"]
    //   Gate = (*) "NOT" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = (*) "NOT" Source "->" Wire [r#"[a-z]+"#]
    //   Gate+ = Gate+ (*) Gate [EOF]
    //   Gate+ = Gate+ (*) Gate ["NOT"]
    //   Gate+ = Gate+ (*) Gate [r#"[0-9]+"#]
    //   Gate+ = Gate+ (*) Gate [r#"[a-z]+"#]
    //   Signal = (*) r#"[0-9]+"# ["->"]
    //   Signal = (*) r#"[0-9]+"# ["AND"]
    //   Signal = (*) r#"[0-9]+"# ["LSHIFT"]
    //   Signal = (*) r#"[0-9]+"# ["OR"]
    //   Signal = (*) r#"[0-9]+"# ["RSHIFT"]
    //   Source = (*) Signal ["->"]
    //   Source = (*) Signal ["AND"]
    //   Source = (*) Signal ["LSHIFT"]
    //   Source = (*) Signal ["OR"]
    //   Source = (*) Signal ["RSHIFT"]
    //   Source = (*) Wire ["->"]
    //   Source = (*) Wire ["AND"]
    //   Source = (*) Wire ["LSHIFT"]
    //   Source = (*) Wire ["OR"]
    //   Source = (*) Wire ["RSHIFT"]
    //   Wire = (*) r#"[a-z]+"# ["->"]
    //   Wire = (*) r#"[a-z]+"# ["AND"]
    //   Wire = (*) r#"[a-z]+"# ["LSHIFT"]
    //   Wire = (*) r#"[a-z]+"# ["OR"]
    //   Wire = (*) r#"[a-z]+"# ["RSHIFT"]
    //   gates = Gate+ (*) [EOF]
    //
    //   EOF -> Reduce(gates = Gate+ => ActionFn(18);)
    //   "NOT" -> Shift(S7)
    //   r#"[0-9]+"# -> Shift(S8)
    //   r#"[a-z]+"# -> Shift(S9)
    //
    //   Gate -> S10
    //   Signal -> S3
    //   Source -> S4
    //   Wire -> S5
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Gate<'input>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::gates(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Gate(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Signal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Source(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Wire(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Source = Signal (*) ["->"]
    //   Source = Signal (*) ["AND"]
    //   Source = Signal (*) ["LSHIFT"]
    //   Source = Signal (*) ["OR"]
    //   Source = Signal (*) ["RSHIFT"]
    //
    //   "->" -> Reduce(Source = Signal => ActionFn(9);)
    //   "AND" -> Reduce(Source = Signal => ActionFn(9);)
    //   "LSHIFT" -> Reduce(Source = Signal => ActionFn(9);)
    //   "OR" -> Reduce(Source = Signal => ActionFn(9);)
    //   "RSHIFT" -> Reduce(Source = Signal => ActionFn(9);)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u16>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Source(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Gate = Source (*) "->" Wire [EOF]
    //   Gate = Source (*) "->" Wire ["NOT"]
    //   Gate = Source (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = Source (*) "->" Wire [r#"[a-z]+"#]
    //   Gate = Source (*) "AND" Source "->" Wire [EOF]
    //   Gate = Source (*) "AND" Source "->" Wire ["NOT"]
    //   Gate = Source (*) "AND" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = Source (*) "AND" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = Source (*) "LSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = Source (*) "LSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = Source (*) "LSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = Source (*) "LSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   Gate = Source (*) "OR" Source "->" Wire [EOF]
    //   Gate = Source (*) "OR" Source "->" Wire ["NOT"]
    //   Gate = Source (*) "OR" Source "->" Wire [r#"[0-9]+"#]
    //   Gate = Source (*) "OR" Source "->" Wire [r#"[a-z]+"#]
    //   Gate = Source (*) "RSHIFT" ShiftWidth "->" Wire [EOF]
    //   Gate = Source (*) "RSHIFT" ShiftWidth "->" Wire ["NOT"]
    //   Gate = Source (*) "RSHIFT" ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = Source (*) "RSHIFT" ShiftWidth "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S11)
    //   "AND" -> Shift(S12)
    //   "LSHIFT" -> Shift(S13)
    //   "OR" -> Shift(S14)
    //   "RSHIFT" -> Shift(S15)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 5
    //   Source = Wire (*) ["->"]
    //   Source = Wire (*) ["AND"]
    //   Source = Wire (*) ["LSHIFT"]
    //   Source = Wire (*) ["OR"]
    //   Source = Wire (*) ["RSHIFT"]
    //
    //   "->" -> Reduce(Source = Wire => ActionFn(8);)
    //   "AND" -> Reduce(Source = Wire => ActionFn(8);)
    //   "LSHIFT" -> Reduce(Source = Wire => ActionFn(8);)
    //   "OR" -> Reduce(Source = Wire => ActionFn(8);)
    //   "RSHIFT" -> Reduce(Source = Wire => ActionFn(8);)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Source(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   __gates = gates (*) [EOF]
    //
    //   EOF -> Reduce(__gates = gates => ActionFn(0);)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Gate<'input>>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____gates(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Gate = "NOT" (*) Source "->" Wire [EOF]
    //   Gate = "NOT" (*) Source "->" Wire ["NOT"]
    //   Gate = "NOT" (*) Source "->" Wire [r#"[0-9]+"#]
    //   Gate = "NOT" (*) Source "->" Wire [r#"[a-z]+"#]
    //   Signal = (*) r#"[0-9]+"# ["->"]
    //   Source = (*) Signal ["->"]
    //   Source = (*) Wire ["->"]
    //   Wire = (*) r#"[a-z]+"# ["->"]
    //
    //   r#"[0-9]+"# -> Shift(S19)
    //   r#"[a-z]+"# -> Shift(S20)
    //
    //   Signal -> S16
    //   Source -> S17
    //   Wire -> S18
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Signal(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Source(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Wire(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Signal = r#"[0-9]+"# (*) ["->"]
    //   Signal = r#"[0-9]+"# (*) ["AND"]
    //   Signal = r#"[0-9]+"# (*) ["LSHIFT"]
    //   Signal = r#"[0-9]+"# (*) ["OR"]
    //   Signal = r#"[0-9]+"# (*) ["RSHIFT"]
    //
    //   "->" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //   "AND" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //   "LSHIFT" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //   "OR" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //   "RSHIFT" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Signal(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Wire = r#"[a-z]+"# (*) ["->"]
    //   Wire = r#"[a-z]+"# (*) ["AND"]
    //   Wire = r#"[a-z]+"# (*) ["LSHIFT"]
    //   Wire = r#"[a-z]+"# (*) ["OR"]
    //   Wire = r#"[a-z]+"# (*) ["RSHIFT"]
    //
    //   "->" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   "AND" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   "LSHIFT" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   "OR" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   "RSHIFT" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Wire(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   Gate+ = Gate+ Gate (*) [EOF]
    //   Gate+ = Gate+ Gate (*) ["NOT"]
    //   Gate+ = Gate+ Gate (*) [r#"[0-9]+"#]
    //   Gate+ = Gate+ Gate (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate+ = Gate+, Gate => ActionFn(16);)
    //   "NOT" -> Reduce(Gate+ = Gate+, Gate => ActionFn(16);)
    //   r#"[0-9]+"# -> Reduce(Gate+ = Gate+, Gate => ActionFn(16);)
    //   r#"[a-z]+"# -> Reduce(Gate+ = Gate+, Gate => ActionFn(16);)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Gate<'input>>>,
        __sym1: &mut Option<Gate<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action16(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   Gate = Source "->" (*) Wire [EOF]
    //   Gate = Source "->" (*) Wire ["NOT"]
    //   Gate = Source "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = Source "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S21
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Gate = Source "AND" (*) Source "->" Wire [EOF]
    //   Gate = Source "AND" (*) Source "->" Wire ["NOT"]
    //   Gate = Source "AND" (*) Source "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "AND" (*) Source "->" Wire [r#"[a-z]+"#]
    //   Signal = (*) r#"[0-9]+"# ["->"]
    //   Source = (*) Signal ["->"]
    //   Source = (*) Wire ["->"]
    //   Wire = (*) r#"[a-z]+"# ["->"]
    //
    //   r#"[0-9]+"# -> Shift(S19)
    //   r#"[a-z]+"# -> Shift(S20)
    //
    //   Signal -> S16
    //   Source -> S23
    //   Wire -> S18
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Signal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Source(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Wire(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 13
    //   Gate = Source "LSHIFT" (*) ShiftWidth "->" Wire [EOF]
    //   Gate = Source "LSHIFT" (*) ShiftWidth "->" Wire ["NOT"]
    //   Gate = Source "LSHIFT" (*) ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "LSHIFT" (*) ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   ShiftWidth = (*) r#"[0-9]+"# ["->"]
    //
    //   r#"[0-9]+"# -> Shift(S25)
    //
    //   ShiftWidth -> S24
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ShiftWidth(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Gate = Source "OR" (*) Source "->" Wire [EOF]
    //   Gate = Source "OR" (*) Source "->" Wire ["NOT"]
    //   Gate = Source "OR" (*) Source "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "OR" (*) Source "->" Wire [r#"[a-z]+"#]
    //   Signal = (*) r#"[0-9]+"# ["->"]
    //   Source = (*) Signal ["->"]
    //   Source = (*) Wire ["->"]
    //   Wire = (*) r#"[a-z]+"# ["->"]
    //
    //   r#"[0-9]+"# -> Shift(S19)
    //   r#"[a-z]+"# -> Shift(S20)
    //
    //   Signal -> S16
    //   Source -> S26
    //   Wire -> S18
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Signal(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Source(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Wire(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Gate = Source "RSHIFT" (*) ShiftWidth "->" Wire [EOF]
    //   Gate = Source "RSHIFT" (*) ShiftWidth "->" Wire ["NOT"]
    //   Gate = Source "RSHIFT" (*) ShiftWidth "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "RSHIFT" (*) ShiftWidth "->" Wire [r#"[a-z]+"#]
    //   ShiftWidth = (*) r#"[0-9]+"# ["->"]
    //
    //   r#"[0-9]+"# -> Shift(S25)
    //
    //   ShiftWidth -> S27
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ShiftWidth(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Source = Signal (*) ["->"]
    //
    //   "->" -> Reduce(Source = Signal => ActionFn(9);)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u16>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Source(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   Gate = "NOT" Source (*) "->" Wire [EOF]
    //   Gate = "NOT" Source (*) "->" Wire ["NOT"]
    //   Gate = "NOT" Source (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = "NOT" Source (*) "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S28)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Source<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Source = Wire (*) ["->"]
    //
    //   "->" -> Reduce(Source = Wire => ActionFn(8);)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Source(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Signal = r#"[0-9]+"# (*) ["->"]
    //
    //   "->" -> Reduce(Signal = r#"[0-9]+"# => ActionFn(11);)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Signal(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Wire = r#"[a-z]+"# (*) ["->"]
    //
    //   "->" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Wire(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   Gate = Source "->" Wire (*) [EOF]
    //   Gate = Source "->" Wire (*) ["NOT"]
    //   Gate = Source "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = Source "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = Source, "->", Wire => ActionFn(4);)
    //   "NOT" -> Reduce(Gate = Source, "->", Wire => ActionFn(4);)
    //   r#"[0-9]+"# -> Reduce(Gate = Source, "->", Wire => ActionFn(4);)
    //   r#"[a-z]+"# -> Reduce(Gate = Source, "->", Wire => ActionFn(4);)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Wire = r#"[a-z]+"# (*) [EOF]
    //   Wire = r#"[a-z]+"# (*) ["NOT"]
    //   Wire = r#"[a-z]+"# (*) [r#"[0-9]+"#]
    //   Wire = r#"[a-z]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   "NOT" -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   r#"[0-9]+"# -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //   r#"[a-z]+"# -> Reduce(Wire = r#"[a-z]+"# => ActionFn(10);)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Wire(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   Gate = Source "AND" Source (*) "->" Wire [EOF]
    //   Gate = Source "AND" Source (*) "->" Wire ["NOT"]
    //   Gate = Source "AND" Source (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "AND" Source (*) "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S29)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Gate = Source "LSHIFT" ShiftWidth (*) "->" Wire [EOF]
    //   Gate = Source "LSHIFT" ShiftWidth (*) "->" Wire ["NOT"]
    //   Gate = Source "LSHIFT" ShiftWidth (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "LSHIFT" ShiftWidth (*) "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S30)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 25
    //   ShiftWidth = r#"[0-9]+"# (*) ["->"]
    //
    //   "->" -> Reduce(ShiftWidth = r#"[0-9]+"# => ActionFn(12);)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action12(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ShiftWidth(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   Gate = Source "OR" Source (*) "->" Wire [EOF]
    //   Gate = Source "OR" Source (*) "->" Wire ["NOT"]
    //   Gate = Source "OR" Source (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "OR" Source (*) "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S31)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Gate = Source "RSHIFT" ShiftWidth (*) "->" Wire [EOF]
    //   Gate = Source "RSHIFT" ShiftWidth (*) "->" Wire ["NOT"]
    //   Gate = Source "RSHIFT" ShiftWidth (*) "->" Wire [r#"[0-9]+"#]
    //   Gate = Source "RSHIFT" ShiftWidth (*) "->" Wire [r#"[a-z]+"#]
    //
    //   "->" -> Shift(S32)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 28
    //   Gate = "NOT" Source "->" (*) Wire [EOF]
    //   Gate = "NOT" Source "->" (*) Wire ["NOT"]
    //   Gate = "NOT" Source "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = "NOT" Source "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S33
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Source<'input>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Gate = Source "AND" Source "->" (*) Wire [EOF]
    //   Gate = Source "AND" Source "->" (*) Wire ["NOT"]
    //   Gate = Source "AND" Source "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = Source "AND" Source "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S34
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 30
    //   Gate = Source "LSHIFT" ShiftWidth "->" (*) Wire [EOF]
    //   Gate = Source "LSHIFT" ShiftWidth "->" (*) Wire ["NOT"]
    //   Gate = Source "LSHIFT" ShiftWidth "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = Source "LSHIFT" ShiftWidth "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S35
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Gate = Source "OR" Source "->" (*) Wire [EOF]
    //   Gate = Source "OR" Source "->" (*) Wire ["NOT"]
    //   Gate = Source "OR" Source "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = Source "OR" Source "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S36
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 32
    //   Gate = Source "RSHIFT" ShiftWidth "->" (*) Wire [EOF]
    //   Gate = Source "RSHIFT" ShiftWidth "->" (*) Wire ["NOT"]
    //   Gate = Source "RSHIFT" ShiftWidth "->" (*) Wire [r#"[0-9]+"#]
    //   Gate = Source "RSHIFT" ShiftWidth "->" (*) Wire [r#"[a-z]+"#]
    //   Wire = (*) r#"[a-z]+"# [EOF]
    //   Wire = (*) r#"[a-z]+"# ["NOT"]
    //   Wire = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Wire = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[a-z]+"# -> Shift(S22)
    //
    //   Wire -> S37
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Wire(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 33
    //   Gate = "NOT" Source "->" Wire (*) [EOF]
    //   Gate = "NOT" Source "->" Wire (*) ["NOT"]
    //   Gate = "NOT" Source "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = "NOT" Source "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = "NOT", Source, "->", Wire => ActionFn(5);)
    //   "NOT" -> Reduce(Gate = "NOT", Source, "->", Wire => ActionFn(5);)
    //   r#"[0-9]+"# -> Reduce(Gate = "NOT", Source, "->", Wire => ActionFn(5);)
    //   r#"[a-z]+"# -> Reduce(Gate = "NOT", Source, "->", Wire => ActionFn(5);)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Source<'input>>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 34
    //   Gate = Source "AND" Source "->" Wire (*) [EOF]
    //   Gate = Source "AND" Source "->" Wire (*) ["NOT"]
    //   Gate = Source "AND" Source "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = Source "AND" Source "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = Source, "AND", Source, "->", Wire => ActionFn(6);)
    //   "NOT" -> Reduce(Gate = Source, "AND", Source, "->", Wire => ActionFn(6);)
    //   r#"[0-9]+"# -> Reduce(Gate = Source, "AND", Source, "->", Wire => ActionFn(6);)
    //   r#"[a-z]+"# -> Reduce(Gate = Source, "AND", Source, "->", Wire => ActionFn(6);)
    //
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 35
    //   Gate = Source "LSHIFT" ShiftWidth "->" Wire (*) [EOF]
    //   Gate = Source "LSHIFT" ShiftWidth "->" Wire (*) ["NOT"]
    //   Gate = Source "LSHIFT" ShiftWidth "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = Source "LSHIFT" ShiftWidth "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = Source, "LSHIFT", ShiftWidth, "->", Wire => ActionFn(7);)
    //   "NOT" -> Reduce(Gate = Source, "LSHIFT", ShiftWidth, "->", Wire => ActionFn(7);)
    //   r#"[0-9]+"# -> Reduce(Gate = Source, "LSHIFT", ShiftWidth, "->", Wire => ActionFn(7);)
    //   r#"[a-z]+"# -> Reduce(Gate = Source, "LSHIFT", ShiftWidth, "->", Wire => ActionFn(7);)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 36
    //   Gate = Source "OR" Source "->" Wire (*) [EOF]
    //   Gate = Source "OR" Source "->" Wire (*) ["NOT"]
    //   Gate = Source "OR" Source "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = Source "OR" Source "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = Source, "OR", Source, "->", Wire => ActionFn(3);)
    //   "NOT" -> Reduce(Gate = Source, "OR", Source, "->", Wire => ActionFn(3);)
    //   r#"[0-9]+"# -> Reduce(Gate = Source, "OR", Source, "->", Wire => ActionFn(3);)
    //   r#"[a-z]+"# -> Reduce(Gate = Source, "OR", Source, "->", Wire => ActionFn(3);)
    //
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Source<'input>>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 37
    //   Gate = Source "RSHIFT" ShiftWidth "->" Wire (*) [EOF]
    //   Gate = Source "RSHIFT" ShiftWidth "->" Wire (*) ["NOT"]
    //   Gate = Source "RSHIFT" ShiftWidth "->" Wire (*) [r#"[0-9]+"#]
    //   Gate = Source "RSHIFT" ShiftWidth "->" Wire (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Gate = Source, "RSHIFT", ShiftWidth, "->", Wire => ActionFn(2);)
    //   "NOT" -> Reduce(Gate = Source, "RSHIFT", ShiftWidth, "->", Wire => ActionFn(2);)
    //   r#"[0-9]+"# -> Reduce(Gate = Source, "RSHIFT", ShiftWidth, "->", Wire => ActionFn(2);)
    //   r#"[a-z]+"# -> Reduce(Gate = Source, "RSHIFT", ShiftWidth, "->", Wire => ActionFn(2);)
    //
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Source<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<u8>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (3, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Gate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__gates::parse_gates;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '-' => {
                            __current_state = 1;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        'A' => {
                            __current_state = 3;
                            continue;
                        }
                        'L' => {
                            __current_state = 4;
                            continue;
                        }
                        'N' => {
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_state = 6;
                            continue;
                        }
                        'R' => {
                            __current_state = 7;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        's' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        't' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '>' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'N' => {
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'S' => {
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'O' => {
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'R' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'S' => {
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        's' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        't' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'D' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'H' => {
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'H' => {
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 20;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'I' => {
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'F' => {
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'F' => {
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'T' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Vec<Gate<'input>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Gate<'input>>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Gate<'input>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Gate<'input>>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: u8,
    _: &'input str,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::rshift(__0, __1, __2)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: Source<'input>,
    _: &'input str,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::or(__0, __1, __2)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::pass(__0, __1)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::not(__0, __1)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: Source<'input>,
    _: &'input str,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::and(__0, __1, __2)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: Source<'input>,
    _: &'input str,
    __1: u8,
    _: &'input str,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Gate<'input>
{
    Gate::lshift(__0, __1, __2)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Source<'input>
{
    Source::Wire(__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: u16,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Source<'input>
{
    Source::Lit(__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> &'input str
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> u16
{
    __0.parse().unwrap()
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> u8
{
    __0.parse().unwrap()
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Gate<'input>>
{
    vec![]
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Gate<'input>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Gate<'input>>
{
    v
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: Gate<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Gate<'input>>
{
    vec![__0]
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Gate<'input>>,
    e: Gate<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Gate<'input>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Gate<'input>>
{
    let __temp0 = __action13(
        input,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Gate<'input>>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Gate<'input>>
{
    let __temp0 = __action14(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
