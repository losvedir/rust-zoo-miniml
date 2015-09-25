#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{Type, Expr, TopLevelCmd, Tok};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Bexpr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Type, Expr, TopLevelCmd, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Bexpr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Expr>, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Bexpr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        App(Box<Expr>),
        Arith(Box<Expr>),
        Bexpr(Box<Expr>),
        Boolean(Box<Expr>),
        Colon(Tok),
        Def(TopLevelCmd),
        Else(Tok),
        Equal(Tok),
        False(Tok),
        Fun(Tok),
        If(Tok),
        Int(i64),
        Is(Tok),
        Less(Tok),
        Let(Tok),
        Lparen(Tok),
        Minus(Tok),
        Nonapp(Box<Expr>),
        Plus(Tok),
        Rparen(Tok),
        Semicolon2(Tok),
        Tarrow(Tok),
        Tbool(Tok),
        Then(Tok),
        Times(Tok),
        Tint(Tok),
        Toplevel(Vec<TopLevelCmd>),
        True(Tok),
        Ty(Type),
        Ty1(Type),
        Var(Expr),
        ____Bexpr(Box<Expr>),
        ____Toplevel(Vec<TopLevelCmd>),
        ____Ty(Type),
    }

    // State 0
    //   Bexpr = (*) "unimplemented!" [EOF]
    //   __Bexpr = (*) Bexpr [EOF]
    //
    //   "unimplemented!" -> Shift(S2)
    //
    //   Bexpr -> S1
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
            Some((_, (20, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state2(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::Bexpr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   __Bexpr = Bexpr (*) [EOF]
    //
    //   EOF -> Reduce(__Bexpr = Bexpr => Call(ActionFn(1));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Box<Expr>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Bexpr(__nt)));
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
    //   Bexpr = "unimplemented!" (*) [EOF]
    //
    //   EOF -> Reduce(Bexpr = "unimplemented!" => Call(ActionFn(28));)
    //
    pub fn __state2<
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
                let __nt = super::__action28(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Bexpr(__nt)));
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
pub use self::__parse__Bexpr::parse_Bexpr;

mod __parse__Toplevel {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Type, Expr, TopLevelCmd, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Toplevel<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<TopLevelCmd>, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Toplevel(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        App(Box<Expr>),
        Arith(Box<Expr>),
        Bexpr(Box<Expr>),
        Boolean(Box<Expr>),
        Colon(Tok),
        Def(TopLevelCmd),
        Else(Tok),
        Equal(Tok),
        False(Tok),
        Fun(Tok),
        If(Tok),
        Int(i64),
        Is(Tok),
        Less(Tok),
        Let(Tok),
        Lparen(Tok),
        Minus(Tok),
        Nonapp(Box<Expr>),
        Plus(Tok),
        Rparen(Tok),
        Semicolon2(Tok),
        Tarrow(Tok),
        Tbool(Tok),
        Then(Tok),
        Times(Tok),
        Tint(Tok),
        Toplevel(Vec<TopLevelCmd>),
        True(Tok),
        Ty(Type),
        Ty1(Type),
        Var(Expr),
        ____Bexpr(Box<Expr>),
        ____Toplevel(Vec<TopLevelCmd>),
        ____Ty(Type),
    }

    // State 0
    //   Def = (*) "unimplemented!" [EOF]
    //   Def = (*) "unimplemented!" [";;"]
    //   Toplevel = (*) Def [EOF]
    //   Toplevel = (*) Def ";;" [EOF]
    //   __Toplevel = (*) Toplevel [EOF]
    //
    //   "unimplemented!" -> Shift(S3)
    //
    //   Def -> S1
    //   Toplevel -> S2
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
            Some((_, (20, __tok0), __loc)) => {
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
                __Nonterminal::Def(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Toplevel(__nt) => {
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
    //   Toplevel = Def (*) [EOF]
    //   Toplevel = Def (*) ";;" [EOF]
    //
    //   EOF -> Reduce(Toplevel = Def => Call(ActionFn(25));)
    //   ";;" -> Shift(S4)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<TopLevelCmd>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Toplevel(__nt)));
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

    // State 2
    //   __Toplevel = Toplevel (*) [EOF]
    //
    //   EOF -> Reduce(__Toplevel = Toplevel => Call(ActionFn(0));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<TopLevelCmd>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Toplevel(__nt)));
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
    //   Def = "unimplemented!" (*) [EOF]
    //   Def = "unimplemented!" (*) [";;"]
    //
    //   EOF -> Reduce(Def = "unimplemented!" => Call(ActionFn(27));)
    //   ";;" -> Reduce(Def = "unimplemented!" => Call(ActionFn(27));)
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
            None |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Def(__nt)));
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
    //   Toplevel = Def ";;" (*) [EOF]
    //
    //   EOF -> Reduce(Toplevel = Def, ";;" => Call(ActionFn(26));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<TopLevelCmd>,
        __sym1: &mut Option<&'input str>,
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
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action26(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Toplevel(__nt)));
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
pub use self::__parse__Toplevel::parse_Toplevel;

mod __parse__Ty {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Type, Expr, TopLevelCmd, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Ty<
        'input,
    >(
        input: &'input str,
    ) -> Result<Type, __ParseError<usize,(usize, &'input str),()>>
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
            (_, None, __Nonterminal::____Ty(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        App(Box<Expr>),
        Arith(Box<Expr>),
        Bexpr(Box<Expr>),
        Boolean(Box<Expr>),
        Colon(Tok),
        Def(TopLevelCmd),
        Else(Tok),
        Equal(Tok),
        False(Tok),
        Fun(Tok),
        If(Tok),
        Int(i64),
        Is(Tok),
        Less(Tok),
        Let(Tok),
        Lparen(Tok),
        Minus(Tok),
        Nonapp(Box<Expr>),
        Plus(Tok),
        Rparen(Tok),
        Semicolon2(Tok),
        Tarrow(Tok),
        Tbool(Tok),
        Then(Tok),
        Times(Tok),
        Tint(Tok),
        Toplevel(Vec<TopLevelCmd>),
        True(Tok),
        Ty(Type),
        Ty1(Type),
        Var(Expr),
        ____Bexpr(Box<Expr>),
        ____Toplevel(Vec<TopLevelCmd>),
        ____Ty(Type),
    }

    // State 0
    //   Ty = (*) Ty1 [EOF]
    //   Ty = (*) Ty1 "->" Ty1 [EOF]
    //   Ty1 = (*) "(" Ty ")" [EOF]
    //   Ty1 = (*) "(" Ty ")" ["->"]
    //   Ty1 = (*) "bool" [EOF]
    //   Ty1 = (*) "bool" ["->"]
    //   Ty1 = (*) "int" [EOF]
    //   Ty1 = (*) "int" ["->"]
    //   __Ty = (*) Ty [EOF]
    //
    //   "(" -> Shift(S3)
    //   "bool" -> Shift(S4)
    //   "int" -> Shift(S5)
    //
    //   Ty -> S1
    //   Ty1 -> S2
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
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::Ty(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Ty1(__nt) => {
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
    //   __Ty = Ty (*) [EOF]
    //
    //   EOF -> Reduce(__Ty = Ty => Call(ActionFn(2));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Ty(__nt)));
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
    //   Ty = Ty1 (*) [EOF]
    //   Ty = Ty1 (*) "->" Ty1 [EOF]
    //
    //   EOF -> Reduce(Ty = Ty1 => Call(ActionFn(42));)
    //   "->" -> Shift(S6)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action42(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty(__nt)));
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

    // State 3
    //   Ty = (*) Ty1 [")"]
    //   Ty = (*) Ty1 "->" Ty1 [")"]
    //   Ty1 = (*) "(" Ty ")" [")"]
    //   Ty1 = (*) "(" Ty ")" ["->"]
    //   Ty1 = "(" (*) Ty ")" [EOF]
    //   Ty1 = "(" (*) Ty ")" ["->"]
    //   Ty1 = (*) "bool" [")"]
    //   Ty1 = (*) "bool" ["->"]
    //   Ty1 = (*) "int" [")"]
    //   Ty1 = (*) "int" ["->"]
    //
    //   "(" -> Shift(S9)
    //   "bool" -> Shift(S10)
    //   "int" -> Shift(S11)
    //
    //   Ty -> S7
    //   Ty1 -> S8
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
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Ty(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Ty1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Ty1 = "bool" (*) [EOF]
    //   Ty1 = "bool" (*) ["->"]
    //
    //   EOF -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //   "->" -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //
    pub fn __state4<
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
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action45(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Ty1 = "int" (*) [EOF]
    //   Ty1 = "int" (*) ["->"]
    //
    //   EOF -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //   "->" -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //
    pub fn __state5<
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
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action44(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
    //   Ty = Ty1 "->" (*) Ty1 [EOF]
    //   Ty1 = (*) "(" Ty ")" [EOF]
    //   Ty1 = (*) "bool" [EOF]
    //   Ty1 = (*) "int" [EOF]
    //
    //   "(" -> Shift(S13)
    //   "bool" -> Shift(S14)
    //   "int" -> Shift(S15)
    //
    //   Ty1 -> S12
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Type>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Ty1(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Ty1 = "(" Ty (*) ")" [EOF]
    //   Ty1 = "(" Ty (*) ")" ["->"]
    //
    //   ")" -> Shift(S16)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 8
    //   Ty = Ty1 (*) [")"]
    //   Ty = Ty1 (*) "->" Ty1 [")"]
    //
    //   ")" -> Reduce(Ty = Ty1 => Call(ActionFn(42));)
    //   "->" -> Shift(S17)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action42(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty(__nt)));
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

    // State 9
    //   Ty = (*) Ty1 [")"]
    //   Ty = (*) Ty1 "->" Ty1 [")"]
    //   Ty1 = (*) "(" Ty ")" [")"]
    //   Ty1 = (*) "(" Ty ")" ["->"]
    //   Ty1 = "(" (*) Ty ")" [")"]
    //   Ty1 = "(" (*) Ty ")" ["->"]
    //   Ty1 = (*) "bool" [")"]
    //   Ty1 = (*) "bool" ["->"]
    //   Ty1 = (*) "int" [")"]
    //   Ty1 = (*) "int" ["->"]
    //
    //   "(" -> Shift(S9)
    //   "bool" -> Shift(S10)
    //   "int" -> Shift(S11)
    //
    //   Ty -> S18
    //   Ty1 -> S8
    pub fn __state9<
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
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Ty(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Ty1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Ty1 = "bool" (*) [")"]
    //   Ty1 = "bool" (*) ["->"]
    //
    //   ")" -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //   "->" -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //
    pub fn __state10<
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
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action45(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
    //   Ty1 = "int" (*) [")"]
    //   Ty1 = "int" (*) ["->"]
    //
    //   ")" -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //   "->" -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //
    pub fn __state11<
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
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action44(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Ty = Ty1 "->" Ty1 (*) [EOF]
    //
    //   EOF -> Reduce(Ty = Ty1, "->", Ty1 => Call(ActionFn(43));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Type>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action43(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Ty = (*) Ty1 [")"]
    //   Ty = (*) Ty1 "->" Ty1 [")"]
    //   Ty1 = (*) "(" Ty ")" [")"]
    //   Ty1 = (*) "(" Ty ")" ["->"]
    //   Ty1 = "(" (*) Ty ")" [EOF]
    //   Ty1 = (*) "bool" [")"]
    //   Ty1 = (*) "bool" ["->"]
    //   Ty1 = (*) "int" [")"]
    //   Ty1 = (*) "int" ["->"]
    //
    //   "(" -> Shift(S9)
    //   "bool" -> Shift(S10)
    //   "int" -> Shift(S11)
    //
    //   Ty -> S19
    //   Ty1 -> S8
    pub fn __state13<
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
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Ty(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Ty1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Ty1 = "bool" (*) [EOF]
    //
    //   EOF -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //
    pub fn __state14<
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
                let __nt = super::__action45(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Ty1 = "int" (*) [EOF]
    //
    //   EOF -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //
    pub fn __state15<
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
                let __nt = super::__action44(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   Ty1 = "(" Ty ")" (*) [EOF]
    //   Ty1 = "(" Ty ")" (*) ["->"]
    //
    //   EOF -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //   "->" -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action46(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
    //   Ty = Ty1 "->" (*) Ty1 [")"]
    //   Ty1 = (*) "(" Ty ")" [")"]
    //   Ty1 = (*) "bool" [")"]
    //   Ty1 = (*) "int" [")"]
    //
    //   "(" -> Shift(S21)
    //   "bool" -> Shift(S22)
    //   "int" -> Shift(S23)
    //
    //   Ty1 -> S20
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Type>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2));
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
                __Nonterminal::Ty1(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Ty1 = "(" Ty (*) ")" [")"]
    //   Ty1 = "(" Ty (*) ")" ["->"]
    //
    //   ")" -> Shift(S24)
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
        __sym1: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 19
    //   Ty1 = "(" Ty (*) ")" [EOF]
    //
    //   ")" -> Shift(S25)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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

    // State 20
    //   Ty = Ty1 "->" Ty1 (*) [")"]
    //
    //   ")" -> Reduce(Ty = Ty1, "->", Ty1 => Call(ActionFn(43));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Type>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action43(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty(__nt)));
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
    //   Ty = (*) Ty1 [")"]
    //   Ty = (*) Ty1 "->" Ty1 [")"]
    //   Ty1 = (*) "(" Ty ")" [")"]
    //   Ty1 = (*) "(" Ty ")" ["->"]
    //   Ty1 = "(" (*) Ty ")" [")"]
    //   Ty1 = (*) "bool" [")"]
    //   Ty1 = (*) "bool" ["->"]
    //   Ty1 = (*) "int" [")"]
    //   Ty1 = (*) "int" ["->"]
    //
    //   "(" -> Shift(S9)
    //   "bool" -> Shift(S10)
    //   "int" -> Shift(S11)
    //
    //   Ty -> S26
    //   Ty1 -> S8
    pub fn __state21<
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
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (15, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Ty(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Ty1(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
    //   Ty1 = "bool" (*) [")"]
    //
    //   ")" -> Reduce(Ty1 = "bool" => Call(ActionFn(45));)
    //
    pub fn __state22<
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
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action45(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
    //   Ty1 = "int" (*) [")"]
    //
    //   ")" -> Reduce(Ty1 = "int" => Call(ActionFn(44));)
    //
    pub fn __state23<
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
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action44(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   Ty1 = "(" Ty ")" (*) [")"]
    //   Ty1 = "(" Ty ")" (*) ["->"]
    //
    //   ")" -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //   "->" -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action46(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   Ty1 = "(" Ty ")" (*) [EOF]
    //
    //   EOF -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
        __sym2: &mut Option<&'input str>,
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
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action46(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
    //   Ty1 = "(" Ty (*) ")" [")"]
    //
    //   ")" -> Shift(S27)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
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
    //   Ty1 = "(" Ty ")" (*) [")"]
    //
    //   ")" -> Reduce(Ty1 = "(", Ty, ")" => Call(ActionFn(46));)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Type>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action46(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ty1(__nt)));
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
pub use self::__parse__Ty::parse_Ty;
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
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((21, __index + 1));
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                            __current_state = 20;
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
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((21, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((21, __index + 1));
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
                            __current_state = 21;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((16, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((15, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((17, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((18, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((19, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
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
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 47;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 49;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 50;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 51;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 52;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 53;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 54;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 55;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 57;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '!' => {
                            __current_match = Some((20, __index + 1));
                            __current_state = 58;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        's' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        't' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((22, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
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
    __0: Vec<TopLevelCmd>,
) -> Vec<TopLevelCmd>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
) -> Box<Expr>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Type,
) -> Type
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> i64
{
    i64::from_str(s).unwrap()
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Tint
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Tbool
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::True
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::False
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Fun
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Is
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::If
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Then
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Else
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Let
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Semicolon2
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Equal
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Less
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Tarrow
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Colon
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Lparen
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Rparen
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Plus
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Minus
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Tok
{
    Tok::Times
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> Expr
{
    Expr::Var(s.to_string())
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    __0: TopLevelCmd,
) -> Vec<TopLevelCmd>
{
    vec![__0]
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    __0: TopLevelCmd,
    _: &'input str,
) -> Vec<TopLevelCmd>
{
    vec![__0]
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> TopLevelCmd
{
    TopLevelCmd::Expr(Expr::Var("foo".to_string()))
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Box<Expr>
{
    Box::new(Expr::Int(3))
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Apply(__0, __1))
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Apply(__0, __1))
}

pub fn __action31<
    'input,
>(
    input: &'input str,
    __0: Expr,
) -> Box<Expr>
{
    Box::new(__0)
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Box<Expr>
{
    Box::new(Expr::Bool(true))
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Box<Expr>
{
    Box::new(Expr::Bool(false))
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    __0: i64,
) -> Box<Expr>
{
    Box::new(Expr::Int(__0))
}

pub fn __action35<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Box<Expr>,
    _: &'input str,
) -> Box<Expr>
{
    __0
}

pub fn __action36<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: i64,
) -> Box<Expr>
{
    Box::new(Expr::Int(__0))
}

pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Plus(__0, __1))
}

pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Minus(__0, __1))
}

pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Times(__0, __1))
}

pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Equal(__0, __1))
}

pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: Box<Expr>,
    _: &'input str,
    __1: Box<Expr>,
) -> Box<Expr>
{
    Box::new(Expr::Less(__0, __1))
}

pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: Type,
) -> Type
{
    (__0)
}

pub fn __action43<
    'input,
>(
    input: &'input str,
    t1: Type,
    _: &'input str,
    t2: Type,
) -> Type
{
    Type::Arrow(Box::new(t1), Box::new(t2))
}

pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Type
{
    Type::Int
}

pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: &'input str,
) -> Type
{
    Type::Bool
}

pub fn __action46<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Type,
    _: &'input str,
) -> Type
{
    (__0)
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
