use std::collections::{HashMap, VecDeque};
use std::ops::*;
use std::result;
use token::{Stack, Token};
use error::{Error, Result};

type Bif = fn(&[Token]) -> Result<Token>; // short for Built-In Function

lazy_static! {
    static ref BIFS: HashMap<&'static str, Bif> = {
        let mut map: HashMap<&'static str, Bif> = HashMap::new();
        map.insert("+", |args| apply_numeric_binop(args, f64::add));
        map.insert("-", |args| apply_numeric_binop(args, f64::sub));
        map.insert("*", |args| apply_numeric_binop(args, f64::mul));
        map.insert("/", |args| apply_numeric_binop(args, f64::div));
        //map.insert("copy", |args| )
        map
    };
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