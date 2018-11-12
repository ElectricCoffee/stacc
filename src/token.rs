use std::collections::VecDeque;
use error::*;
use scope::Scope;

pub type Stack = VecDeque<Token>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Symbol(String),
    String(String),
    Scope(Scope),
    Boolean(bool),
    Void,
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