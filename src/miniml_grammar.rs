#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{Type, Expr, TopLevelCmd, Tok};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Type, Expr, TopLevelCmd, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Term<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expr, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Term(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Colon(Tok),
        Else(Tok),
        Equal(Tok),
        False(Expr),
        Fun(Tok),
        If(Tok),
        Int(Expr),
        Is(Tok),
        Less(Tok),
        Let(Tok),
        Lparen(Tok),
        Minus(Tok),
        Plus(Tok),
        Rparen(Tok),
        Semicolon2(Tok),
        Tarrow(Tok),
        Tbool(Type),
        Term(Expr),
        Then(Tok),
        Times(Tok),
        Tint(Type),
        True(Expr),
        ____Term(Expr),
    }

    // State 0
    //   Term = (*) True [EOF]
    //   True = (*) "true" [EOF]
    //   __Term = (*) Term [EOF]
    //
    //   "true" -> Shift(S3)
    //
    //   Term -> S1
    //   True -> S2
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (19, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::True(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Term = Term (*) [EOF]
    //
    //   EOF -> Reduce(__Term = Term => Call(ActionFn(0));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Expr>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Term(__nt)));
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
    //   Term = True (*) [EOF]
    //
    //   EOF -> Reduce(Term = True => Call(ActionFn(1));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Expr>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   True = "true" (*) [EOF]
    //
    //   EOF -> Reduce(True = "true" => Call(ActionFn(5));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::True(__nt)));
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
pub use self::__parse__Term::parse_Term;
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
                        '(' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        ':' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        ';' => {
                            __current_state = 8;
                            continue;
                        }
                        '<' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_state = 12;
                            continue;
                        }
                        'f' => {
                            __current_state = 13;
                            continue;
                        }
                        'i' => {
                            __current_state = 14;
                            continue;
                        }
                        'l' => {
                            __current_state = 15;
                            continue;
                        }
                        't' => {
                            __current_state = 16;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '>' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 18;
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
                        '0' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 6;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ';' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 19;
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
                        'o' => {
                            __current_state = 20;
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
                        'l' => {
                            __current_state = 21;
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
                        'a' => {
                            __current_state = 22;
                            continue;
                        }
                        'u' => {
                            __current_state = 23;
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
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        'n' => {
                            __current_state = 25;
                            continue;
                        }
                        's' => {
                            __current_match = Some((16, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 27;
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
                        'h' => {
                            __current_state = 28;
                            continue;
                        }
                        'r' => {
                            __current_state = 29;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'o' => {
                            __current_state = 30;
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
                        's' => {
                            __current_state = 31;
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
                        'l' => {
                            __current_state = 32;
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
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 33;
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
                        't' => {
                            __current_match = Some((15, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        't' => {
                            __current_match = Some((17, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_state = 36;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'u' => {
                            __current_state = 37;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        's' => {
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'e' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
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
    __0: Expr,
) -> Expr
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Expr,
) -> Expr
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Expr
{
    Expr::Int(i64::from_str(s).unwrap())
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Type
{
    Type::Int
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Type
{
    Type::Bool
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Expr
{
    Expr::Bool(true)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Expr
{
    Expr::Bool(false)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Fun
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Is
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::If
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Then
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Else
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Let
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Semicolon2
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Equal
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Less
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Tarrow
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Colon
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Lparen
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Rparen
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Plus
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Minus
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Times
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
