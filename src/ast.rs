use std::fmt;
use std::io::Write;

////////// From syntax.ml /////////

// Abstract Syntax

// Variable names
pub type Name = String;

// Types
#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    Arrow(Box<Type>, Box<Type>),
}

impl Clone for Type {
    fn clone(&self) -> Type {
        match *self {
            Type::Int => Type::Int,
            Type::Bool => Type::Bool,
            Type::Arrow(ref t1, ref t2) => {
                Type::Arrow(t1.clone(), t2.clone())
            }
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match *self {
            Type::Int => {
                match *other {
                    Type::Int => true,
                    _ => false
                }
            },
            Type::Bool => {
                match *other {
                    Type::Bool => true,
                    _ => false
                }
            },
            Type::Arrow(ref t1, ref t2) => {
                match *other {
                    Type::Arrow(ref o1, ref o2) => {
                        *t1 == *o1 && *t2 == *o2
                    },
                    _ => false
                }
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", rec_fmt_type(&self, -1))
    }
}

fn rec_fmt_type(t: &Type, n: i8) -> String {
    let (m, str) = match *t {
        Type::Int => (2, "int".to_string()),
        Type::Bool => (2, "bool".to_string()),
        Type::Arrow(ref t1, ref t2) => (1, format!("{} -> {}", &rec_fmt_type(t1, 1), &rec_fmt_type(t2, 0))),
    };

    if m > n {
        str
    } else {
        format!("({})", &str)
    }
}

// Expressions
#[derive(Debug)]
pub enum Expr {
    Var(Name),
    Int(i64),
    Bool(bool),
    Times(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Fn(Name, Name, Type, Type, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &rec_fmt_expr(self, -1))
    }
}

fn rec_fmt_expr(t: &Expr, n: i8) -> String {
    let (m, str) = match *t {
        Expr::Int(i) => (7, i.to_string()),
        Expr::Bool(b) => (7, if b { "true".to_string() } else { "false".to_string() }),
        Expr::Var(ref name) => (7, name.to_string()),
        Expr::Apply(ref e1, ref e2) => (6, format!("{} {}", rec_fmt_expr(e1, 5), rec_fmt_expr(e2, 6))),
        Expr::Times(ref e1, ref e2) => (5, format!("{} * {}", rec_fmt_expr(e1, 4), rec_fmt_expr(e2, 5))),
        Expr::Plus(ref e1, ref e2) => (4, format!("{} + {}", rec_fmt_expr(e1, 3), rec_fmt_expr(e2, 4))),
        Expr::Minus(ref e1, ref e2) => (4, format!("{} - {}", rec_fmt_expr(e1, 3), rec_fmt_expr(e2, 4))),
        Expr::Equal(ref e1, ref e2) => (3, format!("{} = {}", rec_fmt_expr(e1, 3), rec_fmt_expr(e2, 3))),
        Expr::Less(ref e1, ref e2) => (3, format!("{} < {}", rec_fmt_expr(e1, 3), rec_fmt_expr(e2, 3))),
        Expr::If(ref e1, ref e2, ref e3) => (2, format!("if {} then {} else {}",
                                                        rec_fmt_expr(e1, 3),
                                                        rec_fmt_expr(e2, 3),
                                                        rec_fmt_expr(e3, 3))),
        Expr::Fn(ref f, ref x, ref t1, ref t2, ref e) => (1, format!("fn {}({} : {}) : {} is {}",
                                                                       f.to_string(),
                                                                       t1,
                                                                       x.to_string(),
                                                                       t2,
                                                                       rec_fmt_expr(e, 0))),
    };

    if m > n {
        str
    } else {
        format!("({})", &str)
    }
}

// Toplevel commands
#[derive(Debug)]
pub enum TopLevelCmd {
    Expr(Expr),
    Def(Name, Expr)
}

////////// end syntax //////////////////


pub enum Tok {
    Int(i32),
    Tint,
    Tbool,
    True,
    False,
    Fun,
    Is,
    If,
    Then,
    Else,
    Let,
    Semicolon2,
    Equal,
    Less,
    Tarrow,
    Colon,
    Lparen,
    Rparen,
    Plus,
    Minus,
    Times,
    Var(String)
}
