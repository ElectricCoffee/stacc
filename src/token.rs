use std::collections::VecDeque;
use error::*;
use scope::Scope;
use uuid::Uuid;

pub type Stack = VecDeque<Token>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),    // Keeps track of numeric values (May need to be replaced by a custom value) 1, 2.0, -3, etc.
    Symbol(String), // Holds the names of symbols (subject to change) if, for, $a, $value, etc
    String(String), // Holds strings, "you get the idea"
    Scope(Scope),   // Used for nested scopes, (...)
    List(Stack),    // Used for lists, [...]
    Boolean(bool),  // Used for booleans, true and false
    Id(Uuid),
    Void,           // Used for operations that don't produce any results
}

impl Token {
    /// Attempts to get a token as a number.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_number(&self) -> Result<f64> {
        match self {
            Token::Number(n) => Ok(*n),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a symbol string.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_symbol(&self) -> Result<String> {
        match self {
            Token::Symbol(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Token::Symbol(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a string.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_string(&self) -> Result<String> {
        match self {
            Token::String(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Token::String(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a scope.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_scope(&self) -> Result<Scope> {
        match self {
            Token::Scope(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_scope(&self) -> bool {
        match self {
            Token::Scope(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a scope.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_list(&self) -> Result<Stack> {
        match self {
            Token::List(l) => Ok(l.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Token::List(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a boolean.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_boolean(&self) -> Result<bool> {
        match self {
            Token::Boolean(b) => Ok(*b),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            _ => false,
        }
    }
}