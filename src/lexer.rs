use std::collections::{HashMap};
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
        map.insert("+", Callback { arity: 2, func: |args| apply_num_binop(args, f64::add)});
        map.insert("-", Callback { arity: 2, func: |args| apply_num_binop(args, f64::sub)});
        map.insert("*", Callback { arity: 2, func: |args| apply_num_binop(args, f64::mul)});
        map.insert("/", Callback { arity: 2, func: |args| apply_num_binop(args, f64::div)});
        map.insert("neg", Callback { arity: 1, func: |args| apply_num_unop(args, f64::neg)});
        map.insert("sin", Callback { arity: 1, func: |args| apply_num_unop(args, f64::sin)});
        map.insert("cos", Callback { arity: 1, func: |args| apply_num_unop(args, f64::cos)});
        map.insert("tan", Callback { arity: 1, func: |args| apply_num_unop(args, f64::tan)});
        //map.insert("copy", |args| )
        map.insert("if", Callback { arity: 3, func: handle_if});
        map
    };
}

/// Checks if the expected arity matches the actual one.
/// Returns Error::ArityMismatch if it doesn't, and nothing if all is fine.
pub fn check_arity(expected: usize, actual: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::ArityMismatch)
    }
}

pub fn apply_num_binop(args: &[Token], func: fn(f64, f64) -> f64) -> Result<Token> {
    check_arity(2, args.len())?;

    let a = args[0].get_number()?;
    let b = args[1].get_number()?;

    let result = func(a, b);

    Ok(Token::Number(result))
}

pub fn apply_num_unop(args: &[Token], func: fn(f64) -> f64) -> Result<Token> {
    check_arity(1, args.len())?;

    let a = args[0].get_number()?;
    let result = func(a);

    Ok(Token::Number(result))
}

pub fn handle_if(args: &[Token]) -> Result<Token> {
    check_arity(3, args.len())?;

    // Bools are written [then] [else] cond if
    let then_case = args[0].clone();
    let else_case = args[1].clone();
    let condition = args[2].get_boolean()?;

    if condition {
        Ok(then_case)
    } else {
        Ok(else_case)
    }
}