use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Neg, Sub}, // adds math operations to f64
}; 
use token::Token;
use error::{Error, Result};
use scope::{self, Scope, StackFrames};
use tables::{self, ScopeTable};
use callback::Callback;

lazy_static! {
    /// A collection of all the parser's built-in functions.
    /// This collection includes mathematical operations, as well as if-statements, loops, and stack operations.
    pub static ref BIFS: HashMap<&'static str, Callback> = {
        let mut map: HashMap<&'static str, Callback> = HashMap::new();
        map.insert("+",    Callback { arity: 2, func: |_, _, args| apply_num_binop(args, f64::add) });
        map.insert("-",    Callback { arity: 2, func: |_, _, args| apply_num_binop(args, f64::sub) });
        map.insert("*",    Callback { arity: 2, func: |_, _, args| apply_num_binop(args, f64::mul) });
        map.insert("/",    Callback { arity: 2, func: |_, _, args| apply_num_binop(args, f64::div) });
        map.insert("neg",  Callback { arity: 1, func: |_, _, args| apply_num_unop(args, f64::neg) });
        map.insert("sin",  Callback { arity: 1, func: |_, _, args| apply_num_unop(args, f64::sin) });
        map.insert("cos",  Callback { arity: 1, func: |_, _, args| apply_num_unop(args, f64::cos) });
        map.insert("tan",  Callback { arity: 1, func: |_, _, args| apply_num_unop(args, f64::tan) });
        map.insert("copy", Callback { arity: 0, func: |_, scope, _| handle_duplicate(scope) });
        map.insert("if",   Callback { arity: 3, func: |_, _, args| handle_if(args) });
        map.insert("def",  Callback { arity: 2, func: handle_def });
        map.insert("set",  Callback { arity: 2, func: handle_set });
        map
    };
}

/// Parses a given token in the context of a scope table and the stack of stack frames
pub fn parse(table: &mut ScopeTable, frames: &mut StackFrames, token: Token) -> Result<()> {
    match token {
        Token::Symbol(symbol) => {
            // get the new frame in a local scope to avoid borrowing issues
            let new_frame = {
                let mut current_frame = scope::current_frame_mut(frames)?;
                parse_symbol(table, &mut current_frame, &symbol)?
            };

            // if there is a new frame, push the new frame to the top of the frame stack
            if let Some(new_frame) = new_frame {
                frames.push(new_frame);
            }
        },
        token => scope::current_frame_mut(frames)?.stack.push(token)
    }
    Ok(())
}

/// Parses an n-ary operator
/// Parameters: 
/// - `table` is the global scope table containing each scope's symbol tables
/// - `scope` is the current scope
/// - `symbol` is the symbol that needs parsing
pub fn parse_symbol(table: &mut ScopeTable, scope: &mut Scope, symbol: &str) -> Result<Option<Scope>> {
    let mut new_scope = None;
    // get the callback stored in BIFS, if available
    if let Some(callback) = BIFS.get(symbol) {
        let mut args = Vec::new();

        // if the arity is greater than the available data, error
        if scope.stack.len() < callback.arity {
            return Err(Error::ArityMismatch);
        }

        // add the required number of tokens to the args vector
        for _ in 0 .. callback.arity {
            let token = scope.stack.pop().unwrap();
            args.push(token);
        };

        args.reverse(); // ensure the args appear in the right order

        // call the inner function
        let result = callback.invoke(table, scope, &args)?;

        // if the result is a scope, append it to the stack instead of pushing it
        if let Token::Scope(result_scope) = result {
            new_scope = Some(result_scope);
        }
        // if the result isn't a void, add the result to the stack
        else if !result.is_void() {
            scope.stack.push(result);
        }
    }
    // if the first character in a symbol name is a $, assume it's a variable invocation
    else if symbol.starts_with('$') {
        let token = handle_invoke(table, scope, symbol)?;
        if let Token::Scope(result_scope) = token {
            new_scope = Some(result_scope);
        } else {
            scope.stack.push(token);
        }
    }
    // if the symbol is not a valid keyword, assume it's a variable name and add it to the stack 
    else {
        scope.stack.push(Token::Symbol(symbol.into()));
    }

    Ok(new_scope)
}

/// Applies a binary numeric operation and returns the resulting token.
fn apply_num_binop(args: &[Token], func: fn(f64, f64) -> f64) -> Result<Token> {
    let a = args[0].get_number()?;
    let b = args[1].get_number()?;

    let result = func(a, b);

    Ok(Token::Number(result))
}

/// Applies a unary numeric operation and returns the resulting token.
fn apply_num_unop(args: &[Token], func: fn(f64) -> f64) -> Result<Token> {
    let a = args[0].get_number()?;
    let result = func(a);

    Ok(Token::Number(result))
}

/// Duplicates the topmost element on the stack and returns it.
/// Technically a 0-arity function, as it doesn't consume anything on the stack
fn handle_duplicate(scope: &mut Scope) -> Result<Token> {
    scope.stack.last().cloned().ok_or(Error::EmptyStack)
}

/// Applies an if-statement operation and returns the corresponding token
fn handle_if(args: &[Token]) -> Result<Token> {
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

/// Defines a variable in the program, saving its data and value in the symbol table
fn handle_def(table: &mut ScopeTable, scope: &mut Scope, args: &[Token]) -> Result<Token> {
    let value = args[0].clone();
    let name  = args[1].get_symbol()?;
    let id = scope.id();
    let sym_table = table
        .get_mut(&id)
        .unwrap_or_else(|| Scope::invalid_id_panic(id));

    let name = format!("${}", name); // prefix $ onto the name to mark it as a variable

    // if the value already exists, rewrite it; if not, add it
    sym_table.insert(name, value);

    Ok(Token::Void)
}

/// Handles setting a new value to a given symbol
fn handle_set(table: &mut ScopeTable, scope: &mut Scope, args: &[Token]) -> Result<Token> {
    let value = args[0].clone();
    let name  = args[1].get_symbol()?;
    let id = scope.id();

    let sym_table = tables::find_symbol(table, id, &name)?;

    sym_table.insert(name, value);

    Ok(Token::Void)
}

/// Finds a variable in the symbol table if it's available; if not it returns an error
fn handle_invoke(table: &mut ScopeTable, scope: &mut Scope, symbol: &str) -> Result<Token> {
    let id = scope.id();
    let sym_table = tables::find_symbol(table, id, symbol)?;

    let result = sym_table.get(symbol).unwrap().to_owned();

    Ok(result)
}