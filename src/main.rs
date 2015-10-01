pub mod miniml_grammar;
pub mod ast;

use ast::{Type, Expr, TopLevelCmd};
type Name = String;

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;

/////////  type_check.ml ////////////////

fn check(ctx: &mut HashMap<Name,Type>, t: &Type, e: &Expr) -> Result<(), String> {
    let t_ctx = try!(type_of(ctx, e));
    if *t == t_ctx {
        Ok(())
    } else {
        Err(format!("{} has type of {} but is used as if it has type {}.", e, t, t_ctx))
    }
}

fn type_of<'a, 'b>(ctx: &'a mut HashMap<Name,Type>, e: &'b Expr) -> Result<Type, String> {
    match *e {
        Expr::Var(ref name) => {
            if let Some(t) = ctx.get(name) {
                Ok(t.clone())
            } else {
                Err(format!("Variable name {} not found.", name))
            }
        },
        Expr::Int(_) => Ok(Type::Int),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Times(ref e1, ref e2) => {try!(check(ctx, &Type::Int, e1)); try!(check(ctx, &Type::Int, e2)); Ok(Type::Int) },
        Expr::Plus(ref e1, ref e2) => {try!(check(ctx, &Type::Int, e1)); try!(check(ctx, &Type::Int, e2)); Ok(Type::Int) },
        Expr::Minus(ref e1, ref e2) => {try!(check(ctx, &Type::Int, e1)); try!(check(ctx, &Type::Int, e2)); Ok(Type::Int) },
        Expr::Equal(ref e1, ref e2) => {try!(check(ctx, &Type::Int, e1)); try!(check(ctx, &Type::Int, e2)); Ok(Type::Bool) },
        Expr::Less(ref e1, ref e2) => {try!(check(ctx, &Type::Int, e1)); try!(check(ctx, &Type::Int, e2)); Ok(Type::Bool) },
        Expr::If(ref e1, ref e2, ref e3) => {
            try!(check(ctx, &Type::Bool, e1));
            let t2 = try!(type_of(ctx, e2));
            try!(check(ctx, &t2, e3));
            Ok(t2)
        },
        Expr::Fn(ref f, ref x, ref ty1, ref ty2, ref e) => {
            ctx.insert((*f).clone(), Type::Arrow(Box::new((*ty1).clone()), Box::new((*ty2).clone())));
            ctx.insert((*x).clone(), (*ty1).clone());
            try!(check(ctx, ty2, e));
            Ok(Type::Arrow(Box::new((*ty1).clone()), Box::new((*ty2).clone())))
        },
        Expr::Apply(ref e1, ref e2) => {
            let ty = try!(type_of(ctx, e1));
            match ty {
                Type::Arrow(ref ty1, ref ty2) => {
                    try!(check(ctx, ty1, e2));
                    Ok((**ty2).clone())
                },
                wrong_type => Err(format!("{} is used as a function but its type is {}", e1, wrong_type))
            }
        }
    }
}

///////// end type check ////////////

///////// machine.ml ///////////////

// type Name = String above
type Frame = Vec<Instr>;
type Environ = HashMap<Name, Mvalue>;

#[derive(Clone, Debug)]
enum Mvalue {
    Int(i64),
    Bool(bool),
    Closure(Name, Frame, Environ)
}

impl fmt::Display for Mvalue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mvalue::Int(i) => write!(f, "{}", i),
            Mvalue::Bool(b) => write!(f, "{}", b),
            Mvalue::Closure(_, _, _) => write!(f, "<fun>")
        }
    }
}

#[derive(Clone, Debug)]
enum Instr {
    Mult,
    Add,
    Sub,
    Equal,
    Less,
    Var(Name),
    Int(i64),
    Bool(bool),
    Closure(Name, Name, Frame),
    Branch(Frame, Frame),
    Call,
    PopEnv,
}

fn lookup(x: &Name, envs: &mut Vec<Environ>) -> Result<Mvalue, String> {
    if let Some(env) = envs.get_mut(0) {
        if let Some(v) = env.get(x) {
            Ok((*v).clone())
        } else {
            Err(format!("Variable {} not found in environment!", x))
        }
    } else {
        return Err("Can't look up. No environment in stack".to_string())
    }
}


fn pop_bool(s: &mut Vec<Mvalue>) -> Result<bool, String> {
    if let Some(val) = s.pop() {
        match val {
            Mvalue::Bool(b) => Ok(b),
            _ => Err(format!("Bool expected, but got {}", val))
        }
    } else {
        Err("No values on stack.".to_string())
    }
}

fn pop_app(s: &mut Vec<Mvalue>) -> Result<(Name, Frame, Environ, Mvalue), String> {
    if let (Some(v1), Some(v2)) = (s.pop(), s.pop()) {
        match v2 {
            Mvalue::Closure(n, f, e) => Ok((n, f, e, v1)),
            _ => Err(format!("Value and closure expected, got {} and {}", v1, v2))
        }
    } else {
        Err("Two elements expected on stack, but were not there.".to_string())
    }
}

fn mult(s: &mut Vec<Mvalue>) -> Result<(), String> {
    if let (Some(Mvalue::Int(i1)), Some(Mvalue::Int(i2))) = (s.pop(), s.pop()) {
        s.push(Mvalue::Int(i1*i2));
        Ok(())
    } else {
        Err("To ints expected for mult, but not on stack".to_string())
    }
}

fn add(s: &mut Vec<Mvalue>) -> Result<(), String> {
    if let (Some(Mvalue::Int(i1)), Some(Mvalue::Int(i2))) = (s.pop(), s.pop()) {
        s.push(Mvalue::Int(i1+i2));
        Ok(())
    } else {
        Err("To ints expected for add, but not on stack".to_string())
    }
}

fn sub(s: &mut Vec<Mvalue>) -> Result<(), String> {
    if let (Some(Mvalue::Int(i1)), Some(Mvalue::Int(i2))) = (s.pop(), s.pop()) {
        s.push(Mvalue::Int(i1-i2));
        Ok(())
    } else {
        Err("To ints expected for sub, but not on stack".to_string())
    }
}

fn equal(s: &mut Vec<Mvalue>) -> Result<(), String> {
    if let (Some(Mvalue::Int(i1)), Some(Mvalue::Int(i2))) = (s.pop(), s.pop()) {
        s.push(Mvalue::Bool(i1 == i2));
        Ok(())
    } else {
        Err("To ints expected for equal, but not on stack".to_string())
    }
}

fn less(s: &mut Vec<Mvalue>) -> Result<(), String> {
    if let (Some(Mvalue::Int(i1)), Some(Mvalue::Int(i2))) = (s.pop(), s.pop()) {
        s.push(Mvalue::Bool(i1 < i2));
        Ok(())
    } else {
        Err("To ints expected for less, but not on stack".to_string())
    }
}

fn exec(instr: Instr, frms: &mut Vec<Frame>, stack: &mut Vec<Mvalue>, envs: &mut Vec<Environ>) -> Result<(), String> {
    match instr {
        // arithemetic
        Instr::Mult => { mult(stack) }
        Instr::Add => { add(stack) }
        Instr::Sub => { sub(stack) }
        Instr::Equal => { equal(stack) }
        Instr::Less => { less(stack) }

        // pushing values onto stack
        Instr::Var(name) => {
            let val = try!(lookup(&name, envs));
            Ok(stack.push(val))
        }
        Instr::Int(i) => { Ok(stack.push(Mvalue::Int(i))) }
        Instr::Bool(b) => { Ok(stack.push(Mvalue::Bool(b))) }
        Instr::Closure(f, x, frm) => {
            let env = try!((*envs).first().ok_or("No environments in closure.".to_string()));
            let mut c = Mvalue::Closure(x, frm, (*env).clone());
            let c2 = c.clone();

            match c {
                Mvalue::Closure(_, _, ref mut e) => {
                    e.insert(f, c2);
                },
                _ => unreachable!()
            }
            Ok(stack.push(c))
        }
        Instr::Branch(f1, f2) => {
            let b = try!(pop_bool(stack));

            if b {
                Ok(frms.push(f1))
            } else {
                Ok(frms.push(f2))
            }
        }
        Instr::Call => {
            let (x, f, mut e, v) = try!(pop_app(stack));
            (*frms).push(f);
            e.insert(x, v);
            (*envs).push(e);
            Ok(())
        }
        Instr::PopEnv => {
            if let Some(_) = (*envs).pop() {
                Ok(())
            } else {
                Err("No environment to pop.".to_string())
            }
        }
    }
}

fn run(frm: Frame, env: &mut Environ) -> Result<Mvalue, String> {
    let mut frms: Vec<Frame> = vec![frm];
    let mut stack: Vec<Mvalue> = vec![];
    let mut envs: Vec<Environ> = vec![env.clone()];

    loop {
        if frms.len() == 0 && stack.len() == 1 {
            return Ok(stack.pop().unwrap())
        } else if frms.len() >= 1 && frms.first().unwrap().len() >=1 {
            let mut frm = frms.pop().unwrap();
            let i = try!(frm.pop().ok_or("No instructions in this frame."));
            frms.push(frm);
            try!(exec(i, &mut frms, &mut stack, &mut envs));
        } else if frms.len() >= 1 && frms.first().unwrap().len() == 0 {
            frms.pop();
        } else {
            return Err("Illegal end of program.".to_string())
        }
    }
}

/////////////// end machine /////////////////////

///////////// compile.ml ////////////////////

// MiniML compiler Expr -> Instr
fn compile(expr: Expr) -> Vec<Instr> {
    match expr {
        Expr::Var(x) => vec![Instr::Var(x)],
        Expr::Int(k) => vec![Instr::Int(k)],
        Expr::Bool(b) => vec![Instr::Bool(b)],
        Expr::Times(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Mult]);
            instrs
        }
        Expr::Plus(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Add]);
            instrs
        }
        Expr::Minus(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Sub]);
            instrs
        }
        Expr::Equal(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Equal]);
            instrs
        }
        Expr::Less(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Less]);
            instrs
        }
        Expr::If(e1, e2, e3) => {
            let mut cond_instrs = compile(*e1);
            let true_instrs = compile(*e2);
            let false_instrs = compile(*e3);
            cond_instrs.extend(vec![Instr::Branch(true_instrs, false_instrs)]);
            cond_instrs
        }
        Expr::Fn(f, x, _, _, e) => {
            let mut func_body = compile(*e);
            func_body.extend(vec![Instr::PopEnv]);
            vec![Instr::Closure((*f).to_string(), (*x).to_string(), func_body)]
        }

        Expr::Apply(e1, e2) => {
            let mut instrs = compile(*e1);
            instrs.extend(compile(*e2));
            instrs.extend(vec![Instr::Call]);
            instrs
        }
    }
}
////////////// end compile /////////////

////////// miniml.ml //////////////

//[exec_cmd (ctx, env) cmd] executes the toplevel command [cmd] and
// returns the new context-environment pair and a string representing the
// result of evaluation.
fn exec_cmd(ctx: &mut HashMap<Name, Type>, env: &mut Environ, cmd: TopLevelCmd) -> Result<String, String> {
    match cmd {
        TopLevelCmd::Expr(e) => {
            let ty = try!(type_of(ctx, &e));
            let frm = compile(e);
            let v = try!(run(frm, env));
            Ok(format!("- : {} = {}", ty, v))
        }
        TopLevelCmd::Def(x, e) => {
            let ty = try!(type_of(ctx, &e));
            let frm = compile(e);
            let v = try!(run(frm, env));
            ctx.insert(x.clone(), ty.clone());
            env.insert(x.clone(), v.clone());
            Ok(format!("{} : {} = {}", x, ty, v))
        }
    }
}

fn exec_cmds(ctx: &mut HashMap<Name, Type>, env: &mut Environ, cmds: Vec<TopLevelCmd>) -> Result<String, String> {
    let mut output: String = String::new();
    for cmd in cmds.into_iter() {
        let o = try!(exec_cmd(ctx, env, cmd));
        output = output + "\n";
        output = output + &o;
    }
    Ok(output)
}

fn shell(ctx: &mut HashMap<Name, Type>, env: &mut Environ) {
    println!("Welcome to MiniML.");
    loop {
        print!("MiniML> ");
        if let Err(x) = io::stdout().flush() {
            println!("could not flush: {}", x);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Could not read line.");

        if let Ok(cmds) = parse(input) {
            match exec_cmds(ctx, env, cmds) {
                Ok(out) => println!("{}", out),
                Err(err) => println!("{}", err)
            }
        }
    }
}

fn parse(input: String) -> Result<Vec<TopLevelCmd>, ()> {
    match miniml_grammar::parse_Toplevel(&input) {
        Ok(ast) => Ok(ast),
        Err(err) => {
            println!("Parse Error: {:?}", err);
            Err(())
        }
    }
}

////////// end miniml ///////////

fn main() {
    let mut ctx: HashMap<Name, Type> = HashMap::new();
    let mut env: Environ = HashMap::new();
    shell(&mut ctx, &mut env);
}


// #[test]
// fn test_parse_type() {
//     println!("{:?}", miniml_grammar::parse_Type("int -> int"));
// }

#[test]
fn calculator1() {
    assert!(miniml_grammar::parse_Term("22").is_ok());
}
