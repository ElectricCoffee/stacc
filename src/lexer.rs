use std::collections::{HashMap, VecDeque};
use std::ops::*;
use token::{Token};
use error::{Error, Result};

type Bif = fn(&[Token]) -> Result<Token>; // short for Built-In Function

pub struct Callback {
    pub arity: usize,
    pub func: Bif,
}

lazy_static! {
    pub static ref BIFS: HashMap<&'static str, Callback> = {
        let mut map: HashMap<&'static str, Callback> = HashMap::new();
        map.insert("+", Callback { arity: 2, func: |args| apply_numeric_binop(args, f64::add)});
        map.insert("-", Callback { arity: 2, func: |args| apply_numeric_binop(args, f64::sub)});
        map.insert("*", Callback { arity: 2, func: |args| apply_numeric_binop(args, f64::mul)});
        map.insert("/", Callback { arity: 2, func: |args| apply_numeric_binop(args, f64::div)});
        //map.insert("copy", |args| )
        map
    };
}

fn check_arity(expected: usize, actual: usize) -> Result<()> {
    if actual >= expected {
        Ok(())
    } else {
        Err(Error::ArityMismatch)
    }
}

fn apply_numeric_binop(args: &[Token], func: fn(f64, f64) -> f64) -> Result<Token> {
    if args.len() < 2 {
        return Err(Error::InvalidToken);
    }

    let a = args[0].get_number()?;
    let b = args[1].get_number()?;

    let result = func(a, b);

    Ok(Token::Number(result))
}