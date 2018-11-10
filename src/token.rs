use std::collections::VecDeque;
use error::*;

pub type Stack = VecDeque<Token>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Symbol(String),
    String(String),
    Scope(Stack),
    Boolean(bool),
    Void,
}

impl Token {
    pub fn get_number(&self) -> Result<f64> {
        match self {
            Token::Number(n) => Ok(*n),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn get_symbol(&self) -> Result<String> {
        match self {
            Token::Symbol(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn get_string(&self) -> Result<String> {
        match self {
            Token::String(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn get_scope(&self) -> Result<Stack> {
        match self {
            Token::Scope(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn get_boolean(&self) -> Result<bool> {
        match self {
            Token::Boolean(b) => Ok(*b),
            _ => Err(Error::ArgumentMismatch),
        }
    }
}