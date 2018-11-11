use std::collections::HashMap;
use std::ops::*; // adds math operations to f64
use token::{Stack, Token};
use error::{Error, Result};

type Bif = fn(&[Token]) -> Result<Token>; // short for Built-In Function

pub struct Callback {
    pub arity: usize,
    pub func: Bif,
}

lazy_static! {
    pub static ref BIFS: HashMap<&'static str, Callback> = {
        let mut map: HashMap<&'static str, Callback> = HashMap::new();
        map.insert("+",   Callback { arity: 2, func: |args| apply_num_binop(args, f64::add)});
        map.insert("-",   Callback { arity: 2, func: |args| apply_num_binop(args, f64::sub)});
        map.insert("*",   Callback { arity: 2, func: |args| apply_num_binop(args, f64::mul)});
        map.insert("/",   Callback { arity: 2, func: |args| apply_num_binop(args, f64::div)});
        map.insert("neg", Callback { arity: 1, func: |args| apply_num_unop(args, f64::neg)});
        map.insert("sin", Callback { arity: 1, func: |args| apply_num_unop(args, f64::sin)});
        map.insert("cos", Callback { arity: 1, func: |args| apply_num_unop(args, f64::cos)});
        map.insert("tan", Callback { arity: 1, func: |args| apply_num_unop(args, f64::tan)});
        //map.insert("copy", |args| )
        map.insert("if", Callback { arity: 3, func: handle_if});
        map
    };
}

/// Parses an n-ary operator
pub fn parse_symbol(stack: &mut Stack, symbol: &str) -> Result<()> {
    // get the callback stored in BIFS, if available
    let callback = BIFS.get(symbol).ok_or(Error::UnknownIdentifier)?;

    let mut args = Vec::new();

    // if the arity is greater than the available data, error
    if stack.len() < callback.arity {
        return Err(Error::ArityMismatch);
    }

    // add the required number of tokens to the args vector
    for _ in 0 .. callback.arity {
        let token = stack.pop_back().unwrap();
        args.push(token);
    };

    args.reverse(); // ensure the args appear in the right order

    // call the inner function
    let fun = callback.func;
    let result = fun(&args)?;

    // if the result is a scope, append it to the stack instead of pushing it
    if let Token::Scope(mut result_stack) = result {
        stack.append(&mut result_stack);
    }
    // if the result isn't a void, add the result to the stack
    else if result != Token::Void {
        stack.push_back(result);
    }

    Ok(())
}

/// Checks if the expected arity matches the actual one.
/// Returns Error::ArityMismatch if it doesn't, and nothing if all is fine.
fn check_arity(expected: usize, actual: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::ArityMismatch)
    }
}

/// Applies a binary numeric operation and returns the resulting token.
fn apply_num_binop(args: &[Token], func: fn(f64, f64) -> f64) -> Result<Token> {
    check_arity(2, args.len())?;

    let a = args[0].get_number()?;
    let b = args[1].get_number()?;

    let result = func(a, b);

    Ok(Token::Number(result))
}

/// Applies a unary numeric operation and returns the resulting token.
fn apply_num_unop(args: &[Token], func: fn(f64) -> f64) -> Result<Token> {
    check_arity(1, args.len())?;

    let a = args[0].get_number()?;
    let result = func(a);

    Ok(Token::Number(result))
}

/// Applies an if-statement operation and returns the corresponding token
fn handle_if(args: &[Token]) -> Result<Token> {
    check_arity(3, args.len())?;

    // if-statements are written [then] [else] cond if
    let then_case = args[0].clone();
    let else_case = args[1].clone();
    let condition = args[2].get_boolean()?;

    if condition {
        Ok(then_case)
    } else {
        Ok(else_case)
    }
}